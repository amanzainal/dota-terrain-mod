<p align="center">
  <h1 align="center">Dota Terrain Mod</h1>
  <p align="center">
    Play Dota 2 with any terrain skin — even ones you don't own.
    <br />
    <a href="https://github.com/ObsoleteXero/Dota-Terrain-Mod/releases"><strong>Download Latest Release »</strong></a>
    <br />
    <br />
    <a href="https://github.com/ObsoleteXero/Dota-Terrain-Mod/issues">Report Bug</a>
    ·
    <a href="https://github.com/ObsoleteXero/Dota-Terrain-Mod/issues">Request Feature</a>
  </p>
</p>

<p align="center">
  <a href="https://github.com/ObsoleteXero/Dota-Terrain-Mod/releases"><img src="https://img.shields.io/github/v/release/ObsoleteXero/Dota-Terrain-Mod?style=flat-square&color=blue" alt="Release"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="License"></a>
  <img src="https://img.shields.io/badge/rust-2021-orange?style=flat-square&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux-lightgrey?style=flat-square" alt="Platform">
</p>

---

## About

Dota Terrain Mod is a lightweight desktop application that patches Dota 2's default map VPK with any custom terrain of your choice. It writes the patched content to a separate `tempcontent` directory — **no original game files are modified**.

Built with [Iced](https://iced.rs) for a native, dark-themed GUI that matches the Dota 2 aesthetic.

## Features

- **11 Terrain Skins** — Desert, Immortal Gardens, Overgrown Empire, Reef's Edge, Emerald Abyss, and more
- **One-Click Patching** — Select a terrain, click Apply, and launch Dota 2
- **Non-Destructive** — Patched VPK is written to `dota_tempcontent/`; original files stay untouched
- **Auto-Detection** — Automatically locates your Steam and Dota 2 installation (Windows registry / Linux config)
- **Image Previews** — Terrain thumbnails are downloaded asynchronously and cached locally
- **Cross-Platform** — Full support for Windows and Linux

## Getting Started

### Prerequisites

- [Steam](https://store.steampowered.com/) with Dota 2 installed
- [Rust](https://www.rust-lang.org/tools/install) toolchain (only if building from source)

### Download

Grab the latest pre-built binary from the [Releases](https://github.com/ObsoleteXero/Dota-Terrain-Mod/releases) page.

### Build from Source

```bash
git clone https://github.com/ObsoleteXero/Dota-Terrain-Mod.git
cd Dota-Terrain-Mod
cargo build --release
```

The compiled binary will be at:
- **Linux:** `target/release/dota-terrain-mod`
- **Windows:** `target/release/dota-terrain-mod.exe`

## Usage

1. Launch the application.
2. Select a terrain from the preview grid.
3. Click **Apply Terrain** — the VPK is patched in the background.
4. Click **Launch Dota 2** — the game opens with the custom terrain loaded.

The app automatically passes the `-language tempcontent` launch parameter so Dota 2 picks up the patched content.

## Available Terrains

| Terrain | Origin |
|---|---|
| Desert | TI5 Battle Pass |
| The King's New Journey | New Bloom 2017 |
| Immortal Gardens | TI6 Battle Pass |
| Reef's Edge | TI7 Battle Pass |
| The Emerald Abyss | TI8 Battle Pass |
| Overgrown Empire | TI9 Battle Pass |
| Sanctums of the Divine | TI10 Battle Pass |
| Autumn | Dota Plus Seasonal |
| Winter | Dota Plus Seasonal |
| Spring | Dota Plus Seasonal |
| Summer | Dota Plus Seasonal |

## How It Works

1. **Detect** — Locates Steam and Dota 2 via the Windows registry or Linux config files (`libraryfolders.vdf`).
2. **Unpack** — Reads the base terrain (`dota.vpk`) and the selected custom terrain VPK.
3. **Patch** — Merges custom terrain files over the base, remapping internal paths so Dota 2 loads the override.
4. **Repack** — Writes the patched VPK to `dota_tempcontent/maps/dota.vpk` with valid CRC32 and MD5 checksums.
5. **Launch** — Opens Dota 2 via the Steam protocol with the `-language tempcontent` flag.

## Project Structure

```
src/
├── main.rs              # Entry point, Iced app state & view
├── utils.rs             # Steam/Dota 2 path detection (Windows & Linux)
├── vpk.rs               # VPK archive parsing, patching, and repacking
└── gui/
    ├── mod.rs            # Module re-exports
    ├── terrain.rs        # Terrain metadata and definitions
    ├── style.rs          # Dark theme and UI component styles
    └── images.rs         # Async image downloading and caching
```

## Contributing

Contributions are welcome. Please open an issue first to discuss proposed changes.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m 'Add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for details.

## Acknowledgements

- VPK parsing logic based on [ValvePython/vpk](https://github.com/ValvePython/vpk/)
- GUI built with [Iced](https://iced.rs)
