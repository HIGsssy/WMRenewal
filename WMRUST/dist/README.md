# WhoreMaster Renewal — Rust Recreation

A full recreation of WhoreMaster Renewal in Rust, using SDL2 for rendering, mlua for Lua scripting, and egui for the standalone editor.

## Project Structure

```
WMRUST/
├── crates/
│   ├── wm-app/      # Game binary (SDL2 window, game loop, screen management)
│   ├── wm-core/     # Shared data model, XML loaders, enums, Gold system
│   ├── wm-game/     # Game logic: managers, jobs, turn processing, combat
│   ├── wm-ui/       # SDL2 widget system, screen implementations, font/texture caching
│   ├── wm-script/   # Lua engine, wm.* API, trigger system, .script converter
│   └── wm-edit/     # Standalone egui editor for game data files
├── assets/
│   └── fonts/       # Bundled open-source fonts (DejaVu Sans)
├── resources/       # Symlink to game resource files
├── docs/            # Documentation
├── scripts/         # Build and packaging scripts
└── .github/         # CI/CD workflows
```

## Prerequisites

1. **Rust toolchain** (1.75+): Install via [rustup.rs](https://rustup.rs)
2. **C compiler + CMake**: Required for SDL2 bundled build
   - **Windows**: [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (Desktop C++ workload) + [CMake](https://cmake.org/download/)
   - **Linux**: `sudo apt install build-essential cmake libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev`
   - **macOS**: `xcode-select --install && brew install cmake sdl2 sdl2_image sdl2_ttf`
3. **Game resources**: The `resources/` directory must contain the game data files. Create a symlink:
   - **Windows (admin PowerShell):** `New-Item -ItemType SymbolicLink -Path resources -Target ..\WhoreMasterRenewal\Resources`
   - **Linux/macOS:** `ln -s ../WhoreMasterRenewal/Resources resources`

## Building

```bash
# Build everything
cargo build --workspace

# Build release binaries
cargo build --release -p wm-app    # Game
cargo build --release -p wm-edit   # Editor

# Run the game
cargo run -p wm-app

# Run the editor
cargo run -p wm-edit
```

### Windows Build Notes

SDL2_image and SDL2_ttf development libraries must be available. If using the bundled SDL2 feature but external SDL2_image/TTF:

```powershell
# Set environment variables (adjust paths as needed)
$env:CMAKE_GENERATOR = "Visual Studio 17 2022"
$env:LIB = "$env:LIB;$PWD\sdl2-libs"
```

## Running Tests

```bash
# All tests
cargo test --workspace

# Per-crate tests
cargo test -p wm-core     # Data model & XML loaders (17 tests)
cargo test -p wm-game     # Game logic (36 tests)
cargo test -p wm-script   # Lua engine & triggers (16 tests)

# Lint
cargo clippy --workspace -- -D warnings

# Format check
cargo fmt --all -- --check
```

## Crate Overview

| Crate | Purpose | Key Types |
|-------|---------|-----------|
| **wm-core** | Data model, XML loading | `Girl`, `Item`, `Room`, `Gold`, `GameConfig`, `TraitDef` |
| **wm-game** | Game simulation | `GameState`, `TurnProcessor`, `JobDispatcher`, `BrothelManager`, `GangManager` |
| **wm-ui** | SDL2 rendering & screens | `Graphics`, `FontCache`, `Screen`, `ScreenManager`, 8 widget types |
| **wm-script** | Lua scripting | `LuaEngine`, `ScriptContext`, `TriggerSystem`, `ScriptConverter` |
| **wm-app** | Game entry point | `main()` — SDL2 init, game loop |
| **wm-edit** | Data file editor | `EditorApp` — Girls/Items/Traits tabs |

## Game Features

- **15 screens**: Main menu, town, brothel management, girl management, building setup, bank, house, slave market, dungeon, prison, gang management, mayor, girl details, item management, turn summary
- **64 jobs** across 11 categories: brothel, bar, gambling, movie, community, drug lab, alchemy, arena, training, clinic, general
- **7 game managers**: Player, Girls, Brothels, Gangs, Customers, Dungeon, Rivals
- **Lua scripting**: Full `wm.*` API with 41 opcodes, trigger system
- **Data-driven**: All UI layouts from XML, all game data from XML/traits files

## Modding

See [docs/LUA_MODDING_GUIDE.md](docs/LUA_MODDING_GUIDE.md) for the Lua scripting API reference and modding tutorial.

## License

This project recreates WhoreMaster Renewal. The original game was released under the GPL. The Rust code in this repository is licensed under **GPLv3**.

Bundled fonts (DejaVu Sans family) are licensed under the Bitstream Vera license — see `assets/fonts/LICENSE-DejaVu.txt`.

See [docs/ASSET_REVIEW.md](docs/ASSET_REVIEW.md) for the copyright status of game assets.
