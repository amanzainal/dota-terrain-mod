# Dota Terrain Mod

A desktop application that lets you play Dota 2 with any custom terrain — even ones you don't own. It patches the default map's VPK archive with the selected terrain and loads it through Dota 2's content override system.

![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey)

## Features

- **11 Custom Terrains** — Desert, Immortal Gardens, Overgrown Empire, Reef's Edge, and more
- **One-Click Patching** — Select a terrain, hit Apply, and launch Dota 2
- **Dark-Themed GUI** — Built with [Iced](https://iced.rs), styled to match the Dota 2 aesthetic
- **Auto-Detection** — Automatically finds your Steam and Dota 2 installation
- **Image Previews** — Terrain preview images downloaded and cached locally
- **Cross-Platform** — Windows and Linux support

## Screenshot

> *Coming soon*

## Installation

### Pre-built Binaries

Download the latest release from the [Releases](https://github.com/ObsoleteXero/Dota-Terrain-Mod/releases) page.

### Build from Source

**Requirements:** [Rust](https://www.rust-lang.org/tools/install) (2021 edition or later)

```bash
git clone https://github.com/ObsoleteXero/Dota-Terrain-Mod.git
cd Dota-Terrain-Mod
cargo build --release
```

The compiled binary will be at `target/release/dota-terrain-mod` (Linux) or `target/release/dota-terrain-mod.exe` (Windows).

## Usage

**Prerequisites:** Steam and Dota 2 must be installed.

1. Launch the application
2. Select a terrain from the grid
3. Click **Apply Terrain** and wait for patching to complete
4. Click **Launch Dota 2** — the game starts with the custom terrain loaded

The app automatically sets the required `-language tempcontent` launch parameter when opening Dota 2.

## Available Terrains

| Terrain | Source |
|---|---|
| Desert | The International 2015 |
| The King's New Journey | Winter Battle Pass 2016 |
| Immortal Gardens | The International 2016 |
| Reef's Edge | The International 2017 |
| Overgrown Empire | The International 2019 |
| Sanctums of the Divine | The International 2020 |
| The Emerald Abyss | — |
| Seasonal Terrain — Autumn | Seasonal |
| Seasonal Terrain — Winter | Seasonal |
| Seasonal Terrain — Spring | Seasonal |
| Seasonal Terrain — Summer | Seasonal |

## How It Works

1. **Detect** — Locates Steam and Dota 2 via the Windows registry or Linux config files
2. **Unpack** — Reads the base terrain (`dota.vpk`) and selected custom terrain VPK in parallel
3. **Patch** — Merges custom terrain files over the base terrain, remapping filenames so Dota 2 loads the override
4. **Repack** — Writes the patched VPK to `dota_tempcontent/maps/dota.vpk` with valid CRC32/MD5 checksums
5. **Launch** — Opens Dota 2 via Steam with the `-language tempcontent` flag to load the patched content

No game files are modified — the patched VPK is written to a separate `tempcontent` directory.

## Project Structure

```
src/
├── main.rs           # Application entry point, GUI state management
├── utils.rs          # Steam/Dota 2 path detection
├── vpk.rs            # VPK archive parsing, patching, and repacking
└── gui/
    ├── mod.rs        # Module exports
    ├── terrain.rs    # Terrain definitions and metadata
    ├── style.rs      # Dark theme and component styling
    └── images.rs     # Async image downloading and caching
```

## Acknowledgements

- VPK unpacking and repacking based on [ValvePython/vpk](https://github.com/ValvePython/vpk/)

## License

Distributed under the [MIT License](LICENSE). Use at your own risk.
