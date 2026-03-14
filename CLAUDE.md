# Dota 2 Mod Tool - Agent Knowledge Base

Comprehensive documentation of the project architecture, codebase, Dota 2 modding mechanics, and known issues. Written to eliminate redundant exploration by future agents.

## Project Overview

A **Rust desktop application** that lets users swap client-side only Dota 2 cosmetics (terrains, announcers, music packs, cursors, HUD skins, weather effects) by patching game files. Only the local player sees the changes.

- **Language**: Rust (edition 2021)
- **GUI Framework**: Iced 0.13 (Elm-architecture reactive GUI)
- **Async Runtime**: Tokio (multi-threaded)
- **Window Size**: 1060x720
- **License**: MIT
- **Original Author**: ObsoleteXero
- **Repository**: `github.com/ObsoleteXero/Dota-Terrain-Mod` (forked to `amanzainal/dota-terrain-mod`)

---

## File Map

```
dota-terrain-mod/
├── Cargo.toml                        # Dependencies and metadata
├── Cargo.lock                        # Dependency lock file
├── README.md                         # User-facing documentation
├── LICENSE                           # MIT
├── .gitignore
├── .github/workflows/rust.yml        # CI: builds Windows x86_64 release on tag push
└── src/
    ├── main.rs                       # App entry, state, messages, view, async patching (682 lines)
    ├── vpk.rs                        # VPK v2 parsing, patching, repacking, scanning (482 lines)
    ├── utils.rs                      # Steam/Dota 2 path detection (172 lines)
    └── gui/
        ├── mod.rs                    # Module re-exports
        ├── cosmetics.rs              # CosmeticCategory enum, CosmeticItem, auto-discovery (208 lines)
        ├── terrain.rs                # TerrainInfo struct, 11 hardcoded terrains (112 lines)
        ├── style.rs                  # Dark theme, card/button/sidebar styles (209 lines)
        └── images.rs                 # Async image download + disk caching (37 lines)
```

---

## Architecture Deep Dive

### Iced 0.13 Application Pattern

The app uses Iced's function-based pattern (NOT the older `Application` trait):

```rust
iced::application("Dota 2 Mod Tool", update, view)
    .theme(|_state| style::dark_theme())
    .window_size((1060.0, 720.0))
    .run_with(init)
```

Three top-level functions drive everything:
- `init() -> (App, Task<Message>)` — creates initial state, fires async tasks
- `update(app: &mut App, message: Message) -> Task<Message>` — handles all events
- `view(app: &App) -> Element<'_, Message>` — renders the entire UI

**Important**: Iced 0.13 uses `Task<Message>` (not `Command<Message>` from older versions). Async work is done via `Task::perform(future, message_mapper)`.

### App State (`src/main.rs:29-53`)

```rust
struct App {
    dota: Option<Dota>,                                    // Dota 2 install info (None if not found)
    init_error: Option<String>,                            // Shown as error screen
    active_tab: CosmeticCategory,                          // Which sidebar tab is active
    selected_terrain: Option<usize>,                       // Index into TERRAINS array
    selected_cosmetics: HashMap<CosmeticCategory, usize>,  // One selection per non-terrain category
    discovered_items: HashMap<CosmeticCategory, Vec<CosmeticItem>>,  // From pak01 scan
    images: HashMap<u32, image::Handle>,                   // Downloaded preview images by ID
    patch_status: PatchStatus,                             // Idle | Patching | Success | Error(String)
    scan_status: ScanStatus,                               // Scanning | Done | Error(String)
}
```

### Message Flow (`src/main.rs:58-68`)

```rust
enum Message {
    TabSelected(CosmeticCategory),        // Sidebar tab click
    TerrainSelected(usize),               // Terrain card click (index into TERRAINS)
    CosmeticSelected(CosmeticCategory, usize),  // Non-terrain card click (toggles selection)
    ImageLoaded(u32, Result<Vec<u8>, String>),   // Async image download complete
    ApplyMods,                            // "Apply Mods" button
    PatchComplete(Result<(), String>),     // Async patching done
    LaunchDota,                           // "Launch Dota 2" button
    ScanComplete(Result<HashMap<CosmeticCategory, Vec<CosmeticItem>>, String>),  // pak01 scan done
}
```

### GUI Layout

```
+------------------------------------------+
|  DOTA 2 MOD TOOL         (header)        |
+--------+---------------------------------+
| Terrain|                                 |
| Announ.|   Card grid for active tab      |
| Music  |   (scrollable, 4 per row)       |
| Cursor |   Cards are 185px wide          |
| HUD    |                                 |
| Weather|                                 |
+--------+---------------------------------+
| Terrain: X | Announcer: Y  | Apply | Run|
+------------------------------------------+
```

- **Sidebar** (140px): `CosmeticCategory::ALL` buttons with active highlighting. Green `*` indicator next to categories with selections.
- **Content area**: Switches based on `active_tab`. Terrain tab shows hardcoded `TERRAINS` with preview images. Other tabs show auto-discovered items from pak01 scan (or "Scanning..." / "No items found" states).
- **Bottom bar**: Selection summary, status text, "Apply Mods" + "Launch Dota 2" buttons.

---

## Two Distinct Patching Pipelines

### Pipeline 1: Terrain Patching (VPK merge)

Terrains are standalone VPK files in `{dota_path}/dota/maps/`. The tool merges a target terrain VPK onto the base `dota.vpk`.

**Flow** (`vpk::create_terrain` at `src/vpk.rs:468`):
1. Read base VPK (`dota/maps/dota.vpk`) and target terrain VPK (e.g., `dota/maps/dota_desert.vpk`) **in parallel** using `mpsc::channel` + `thread::spawn`
2. Call `patch_vpk()` which:
   - Finds the `.vmap_c` file in the target and renames it to `dota.vmap_c`
   - Merges: all target files kept, missing files filled from base
3. Call `create_vpk()` to repack into a new VPK v2 binary with proper CRC32 + MD5 checksums
4. Write result to `dota_tempcontent/maps/dota.vpk`

### Pipeline 2: Cosmetic Extraction (pak01 file extraction)

Non-terrain cosmetics (announcers, music, cursors, HUDs, weather) live inside `pak01_dir.vpk` (~3GB main game archive). The tool extracts specific files and places them in `dota_tempcontent/` where Dota 2 loads them as overrides.

**Flow** (`apply_all_async` at `src/main.rs:574`):
1. For each selected cosmetic, call `vpk::extract_files_by_prefix(pak01_path, &[prefix])`
2. Remap extracted file paths using `vpk::remap_path()` (e.g., `sounds/vo/announcer_dlc_bastion/` → `sounds/vo/announcer/`)
3. Write each file to `dota_tempcontent/{remapped_path}`

### Output Location & Launch

All patched/extracted content goes to `{dota_path}/dota_tempcontent/`. Original game files in `dota/` are **never modified**.

Dota 2 is launched with `-language tempcontent` which tells the engine to load from `dota_tempcontent/` as an override layer:
- Primary: `steam://rungameid/570//+language tempcontent`
- Fallback: `steam -applaunch 570 -language tempcontent`

---

## VPK v2 File Format (`src/vpk.rs`)

### Header (28 bytes)
| Offset | Size | Field | Expected |
|--------|------|-------|----------|
| 0 | 4 | signature | `0x55aa1234` |
| 4 | 4 | version | `2` |
| 8 | 4 | tree_length | variable |
| 12 | 4 | embed_chunk_length | variable |
| 16 | 4 | chunk_hashes_length | variable |
| 20 | 4 | self_hashes_length | `48` (3x MD5) |
| 24 | 4 | signature_length | variable |

### Index Tree Structure
Nested null-terminated strings: `extension → directory → filename`, with 18 bytes of metadata per file:
- Bytes 0-3: CRC32
- Bytes 4-5: preload_length
- Bytes 6-7: archive_index (`32767` = embedded in dir VPK)
- Bytes 8-11: archive_offset
- Bytes 12-15: file_length
- Bytes 16-17: suffix (`65535` = valid)

### VPK Struct (private)
```rust
struct VPK {
    _path: PathBuf,
    data: Cursor<Vec<u8>>,              // Entire file loaded into memory
    header: Option<VPKHeader>,
    index: HashMap<String, VPKMetadata>, // path → metadata
    files: HashMap<String, Vec<u8>>,     // path → file contents
}
```

### Public Functions
| Function | Purpose |
|----------|---------|
| `create_terrain(base_path, target_path) -> Vec<u8>` | Merge two terrain VPKs, return new VPK bytes |
| `scan_vpk_index(vpk_path) -> Vec<String>` | Parse index only, return all file paths (no data loaded) |
| `extract_files_by_prefix(vpk_path, prefixes) -> HashMap<String, Vec<u8>>` | Parse index, selectively load matching files |
| `remap_path(path, source_prefixes, dest_prefix) -> String` | Replace path prefix for output mapping |

### Critical Limitation: Full File Load

`VPK::new()` calls `f.read_to_end(&mut vpk_data)` — it loads the **entire VPK file into memory**. For terrain VPKs (small, ~50-200MB) this is fine. For `pak01_dir.vpk` (~3GB), the entire file gets loaded into RAM during `scan_vpk_index()` and `extract_files_by_prefix()`. This works but is memory-heavy.

**Potential improvement**: Implement streaming VPK parsing that only reads the header + tree (first few MB), then seeks to specific offsets for individual file extraction without loading the whole archive.

### Multi-Part VPK Limitation

`pak01_dir.vpk` is a **directory VPK** that references external data archives (`pak01_000.vpk`, `pak01_001.vpk`, etc.) via `archive_index`. The current code only handles entries where `archive_index == 32767` (embedded data). Files stored in external archives (`archive_index != 32767`) will have their offsets incorrectly resolved, producing corrupt extractions.

**This is the #1 bug to fix for non-terrain cosmetics to work correctly.** Most pak01 content lives in external archives, not embedded in the directory VPK.

---

## Cosmetic Auto-Discovery (`src/gui/cosmetics.rs`)

### CosmeticCategory Enum
```rust
pub enum CosmeticCategory {
    Terrain,    // display: "Terrains"   — uses hardcoded TERRAINS array, NOT discovery
    Announcer,  // display: "Announcers" — discovered from pak01
    Music,      // display: "Music"      — discovered from pak01
    Cursor,     // display: "Cursors"    — discovered from pak01
    Hud,        // display: "HUD Skins"  — discovered from pak01
    Weather,    // display: "Weather"    — discovered from pak01
}
```

`CosmeticCategory::ALL` is a `&'static [CosmeticCategory]` containing all 6 variants in display order.

### CosmeticItem Struct
```rust
pub struct CosmeticItem {
    pub id: u32,              // Auto-assigned starting at 100
    pub name: String,         // Prettified display name
    pub category: CosmeticCategory,
    pub vpk_prefix: String,   // Path prefix inside pak01_dir.vpk to extract
    pub output_prefix: String, // Destination prefix under dota_tempcontent/
}
```

### Discovery Patterns

`discover_cosmetics(vpk_index: &[String])` scans all file paths from the pak01 index and groups unique directories into items:

| Category | Path Pattern | Grouping Key (depth) | Strip Prefixes | Override Target |
|----------|-------------|---------------------|----------------|-----------------|
| Announcer | `sounds/vo/announcer*` | depth 2 (e.g., `announcer_dlc_bastion`) | `announcer_dlc_`, `announcer_` | `sounds/vo/announcer/` |
| Music | `sounds/music/*` (not `default`) | depth 2 | `valve_dota_`, `valve_` | `sounds/music/default/` |
| Cursor | `resource/cursor*` (not `cursor` itself) | depth 1 | `cursor_pack_`, `cursor_` | `resource/cursor/` |
| HUD | `resource/flash3/images/hud_skins/*` | depth 4 | `hud_skin_`, `hud_` | `resource/flash3/images/hud_skins/default/` |
| Weather | `particles/weather/*` | depth 2 | `weather_` | `particles/weather/{same}/` |

**Name prettification**: Strip prefix → split on `_` → title-case each word → join with spaces. E.g., `announcer_dlc_bastion` → `Bastion`.

Items are sorted alphabetically within each category.

---

## Terrain Data (`src/gui/terrain.rs`)

11 hardcoded terrains with static data. IDs 1-11.

| ID | Name | VPK File | Category | Source |
|----|------|----------|----------|--------|
| 1 | Desert Terrain | `dota_desert.vpk` | Premium | TI5 Battle Pass |
| 2 | The King's New Journey | `dota_journey.vpk` | Premium | New Bloom 2017 |
| 3 | Immortal Gardens | `dota_coloseum.vpk` | Premium | TI6 Battle Pass |
| 4 | Overgrown Empire | `dota_jungle.vpk` | Premium | TI9 Battle Pass |
| 5 | Reef's Edge | `dota_reef.vpk` | Premium | TI7 Battle Pass |
| 6 | Sanctums of the Divine | `dota_ti10.vpk` | Premium | TI10 Battle Pass |
| 7 | The Emerald Abyss | `dota_cavern.vpk` | Premium | TI8 Battle Pass |
| 8 | Autumn Terrain | `dota_autumn.vpk` | Seasonal | Dota Plus |
| 9 | Winter Terrain | `dota_winter.vpk` | Seasonal | Dota Plus |
| 10 | Spring Terrain | `dota_spring.vpk` | Seasonal | Dota Plus |
| 11 | Summer Terrain | `dota_summer.vpk` | Seasonal | Dota Plus |

Preview images sourced from `dota2.fandom.com/wiki/Special:Filepath/...`. Cached to `~/.cache/dota-terrain-mod/images/terrain_{id}.png`.

---

## Steam/Dota 2 Path Detection (`src/utils.rs`)

### Dota Struct
```rust
pub struct Dota {
    pub dota_path: PathBuf,      // e.g., /path/to/steamapps/common/dota 2 beta/game
    pub base_path: Option<PathBuf>,   // Unused in GUI mode
    pub target_path: Option<PathBuf>, // Unused in GUI mode
    pub out_path: Option<PathBuf>,    // Unused in GUI mode
}
```

### Detection Flow
1. **Windows**: Read `HKEY_CURRENT_USER\Software\Valve\Steam` → `SteamPath` value
2. **Linux**: Check `~/.local/share/Steam/config/libraryfolders.vdf`
3. Parse `libraryfolders.vdf` with regex to find library containing app ID `570` (Dota 2)
4. Build path: `{library}/steamapps/common/dota 2 beta/game`

### Key Game Paths (relative to `dota_path`)
```
dota/maps/dota.vpk           — Base terrain VPK
dota/maps/dota_desert.vpk    — Example terrain VPK
dota/pak01_dir.vpk           — Main game archive (~3GB, contains all cosmetics)
dota/pak01_000.vpk           — Data archive 0 (referenced by pak01_dir.vpk)
dota/pak01_001.vpk           — Data archive 1
...
dota_tempcontent/             — Override output directory (tool writes here)
dota_tempcontent/maps/dota.vpk  — Patched terrain output
```

---

## Styling System (`src/gui/style.rs`)

### Theme Palette
| Color | RGB | Usage |
|-------|-----|-------|
| Background | `(0.08, 0.08, 0.10)` | Main app background (near-black) |
| Text | `(0.92, 0.92, 0.92)` | Primary text (off-white) |
| Primary | `(0.83, 0.18, 0.18)` | Dota red, active tab border |
| Success | `(0.20, 0.78, 0.35)` | Green text for success messages |
| Danger | `(0.90, 0.20, 0.20)` | Red text for errors |

### Style Functions
All style functions follow the pattern `fn style_name(_theme: &Theme, ...) -> Style`:

**Containers**: `card_style`, `selected_card_style`, `placeholder_style`, `header_style`, `bottom_bar_style`, `sidebar_style`

**Buttons**: `apply_button_style` (red, disabled state), `launch_button_style` (blue/teal), `card_button_style` (transparent), `tab_button_style` (transparent, hover highlight), `active_tab_button_style` (red border + bright text)

---

## Dependencies (`Cargo.toml`)

| Crate | Version | Purpose |
|-------|---------|---------|
| `iced` | 0.13 (`image`, `tokio` features) | GUI framework |
| `tokio` | 1 (`rt-multi-thread`, `fs`) | Async runtime |
| `reqwest` | 0.12 (`rustls-tls`) | HTTP client for image downloads |
| `regex` | 1 | `libraryfolders.vdf` parsing |
| `crc` | 3 | CRC32 checksums for VPK files |
| `md-5` | 0.10 | MD5 hashing for VPK integrity |
| `open` | 5 | Cross-platform URL/process opening |
| `dirs` | 5 | User cache directory detection |
| `winreg` | 0.51 | Windows registry access (Windows only) |

---

## Git History & Branches

### Branches
- `master` — Original CLI-based implementation
- `claude/revamp-gui-FKS8m` — Full Iced GUI rewrite (terrain-only)
- `claude/add-item-selection-gui-g7Tw9` — **Current development branch** (multi-cosmetic support)

### Key Commits (chronological)
```
1841205 implement unpacking and repacking vpk         ← Original VPK implementation
ef72905 complete vpk implementation
51353c6 implement cli                                  ← CLI selector
94b4414 Add iced GUI with terrain grid, VPK patching  ← Full GUI rewrite
aa3ed32 Revamp README for production readiness
a9a509d Add multi-cosmetic item selection              ← Sidebar + auto-discovery
```

### CI/CD
GitHub Actions workflow (`.github/workflows/rust.yml`):
- Triggers on push to `main` or `v*` tags
- Builds Windows x86_64 GNU target
- Linux target commented out
- Uses `rust-build/rust-build.action@v1.4.3`
- Requires `RELEASE_TOKEN` secret

---

## Known Issues & Future Work

### Bug: Multi-Part VPK Archive Support
**Severity**: Critical for non-terrain cosmetics

The VPK parser only handles embedded file data (`archive_index == 32767`). `pak01_dir.vpk` stores most file data in external archives (`pak01_000.vpk`, `pak01_001.vpk`, etc.) referenced by `archive_index` values `0, 1, 2, ...`. Currently, `extract_files_by_prefix` reads from the wrong offset for these files, producing garbage data.

**Fix needed in `src/vpk.rs`**:
- When `archive_index != 32767`, open the corresponding `pak01_{archive_index:03}.vpk` file
- Read `file_length` bytes at `archive_offset` from that file
- The current `VPKMetadata::validate()` only adjusts offset for embedded files (index 32767)

### Memory Usage
`VPK::new()` loads entire files into memory. For `pak01_dir.vpk` (~3GB), this uses ~3GB RAM. Should implement streaming: read header+tree, then `seek()` to specific offsets for individual files.

### Missing Preview Images
Auto-discovered cosmetics (announcers, music, etc.) show "No Preview" placeholders. Could add:
- A mapping of known item names to Dota 2 wiki image URLs
- Or generate thumbnails from extracted assets

### No Tempcontent Cleanup
The `apply_all_async()` function doesn't clean `dota_tempcontent/` before writing. Stale files from a previous apply remain. Should wipe relevant subdirectories before each apply.

### Terrain Deselection
Clicking a selected terrain card doesn't deselect it (unlike cosmetic items which toggle). `TerrainSelected` always sets `selected_terrain = Some(idx)`.

### Error Handling in VPK Parser
`vpk.rs` uses `unwrap()` and `panic!()` extensively. These are caught by `std::panic::catch_unwind()` at the call sites, but proper `Result<>` error handling would be cleaner.

### utils.rs Module Structure
`utils.rs` wraps everything in a `pub mod utils { ... }` block then re-exports with `pub use self::utils::*`. This is unusual — the inner module is redundant.

---

## Dota 2 Client-Side Cosmetics Reference

These are cosmetic items that **only the local player sees** — other players in the game are unaffected:

| Type | What It Changes | VPK Location |
|------|----------------|--------------|
| Terrain | Map appearance/theme | Standalone VPKs in `dota/maps/` |
| Announcer | Game announcer voice lines | `pak01: sounds/vo/announcer*/` |
| Music Pack | Background music tracks | `pak01: sounds/music/*/` |
| Cursor Pack | Mouse cursor appearance | `pak01: resource/cursor*/` |
| HUD Skin | In-game UI skin/theme | `pak01: resource/flash3/images/hud_skins/*/` |
| Weather Effect | Map weather particles | `pak01: particles/weather/*/` |
| Loading Screen | Pre-game loading image | `pak01: resource/flash3/images/loadingscreens/` |

Loading screens are not yet implemented in the tool.

### How Dota 2 Content Override Works
Dota 2's `-language X` launch parameter tells the engine to check `dota_X/` as a content override directory before falling back to `dota/`. By using `-language tempcontent`, any file placed in `dota_tempcontent/` at the correct relative path will override the corresponding file in `dota/` or `pak01_dir.vpk`.

This is the same mechanism Valve uses for localization (e.g., `-language schinese` loads Chinese assets from `dota_schinese/`).

---

## Quick Reference for Common Tasks

### Adding a new terrain
Add an entry to the `TERRAINS` array in `src/gui/terrain.rs` with a unique `id` (1-99), the VPK filename, and a preview image URL.

### Adding a new cosmetic category
1. Add variant to `CosmeticCategory` enum in `cosmetics.rs`
2. Add to `CosmeticCategory::ALL` array
3. Add `display_name()` match arm
4. Add discovery pattern in `discover_cosmetics()`
5. Everything else (sidebar, tabs, patching) is driven by the enum automatically

### Changing the theme
Edit `src/gui/style.rs`. The `dark_theme()` function sets the Iced palette. Individual component styles are separate functions.

### Testing the VPK parser
The `VPK` struct has a `_save_file_data()` method (prefixed with `_` to suppress warnings) that extracts all files to disk. Useful for debugging.

### Building
```bash
cargo build              # Debug build
cargo build --release    # Release build (much smaller + faster)
```

Cross-compile for Windows from Linux:
```bash
cargo build --target x86_64-pc-windows-gnu --release
```
