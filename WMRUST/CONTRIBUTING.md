# Contributing to WhoreMaster Renewal (Rust)

Thank you for your interest in contributing! This document covers the workflow and standards for the project.

## Getting Started

1. Fork the repository and clone your fork
2. Set up the [prerequisites](README.md#prerequisites)
3. Create the `resources/` symlink (see README)
4. Run `cargo build --workspace` to verify your setup
5. Run `cargo test --workspace` to confirm tests pass

## Development Workflow

1. Create a feature branch from `develop`: `git checkout -b feature/my-change develop`
2. Make your changes
3. Run the full check suite before committing:
   ```bash
   cargo fmt --all
   cargo clippy --workspace -- -D warnings
   cargo test --workspace
   ```
4. Commit with clear, descriptive messages
5. Push and open a pull request against `develop`

## Code Standards

### Rust Style

- **Format**: Run `cargo fmt` before committing. CI enforces formatting.
- **Lints**: Zero clippy warnings required. CI runs `cargo clippy -- -D warnings`.
- **No `unsafe`**: Unless absolutely necessary and well-documented.
- **Error handling**: Use `Result` over panics. `anyhow` for application code, `thiserror` for library errors.
- **Testing**: Every new module should include `#[cfg(test)] mod tests { ... }`. Use real game data files in tests where possible.

### Architecture Rules

- **`wm-core`** has no dependencies on other workspace crates — it's the foundation.
- **`wm-game`** depends only on `wm-core`. No rendering code.
- **`wm-ui`** depends on `wm-core` and `wm-game`. All SDL2 rendering lives here.
- **`wm-script`** depends on `wm-core`. Lua engine and trigger system.
- **`wm-app`** wires everything together. Minimal logic — delegates to other crates.
- **`wm-edit`** depends on `wm-core`. Standalone editor, no SDL2 dependency.

### Resource Paths

Always use `wm_core::resources_path()` to resolve game data files. Never hardcode absolute paths. Tests should work both with the `resources/` symlink and via the `WM_RESOURCES_PATH` environment variable.

## What to Work On

### Good First Issues

- Add missing XML parser tests for edge cases
- Improve error messages in data loaders
- Add keyboard shortcuts to the editor (`wm-edit`)

### Larger Contributions

- **Cross-platform testing** — macOS builds, Linux packaging
- **Save/load system** — Implement `Serialize`/`Deserialize` on `GameState` for full save support
- **Animated images** — Sprite sheet support in `ImageItemWidget`
- **Sound system** — SDL2_mixer integration for background music and effects
- **Gallery screen** — Character image gallery (currently a stub)

## Reporting Bugs

Open an issue with:
1. Steps to reproduce
2. Expected behavior
3. Actual behavior
4. OS and Rust version (`rustc --version`)

## Asset Contributions

If contributing game assets (images, scripts, data files):
- **Do not submit copyrighted content** (no fan art of trademarked characters)
- All image contributions must be original work or explicitly licensed for redistribution
- Lua scripts are welcome — see [docs/LUA_MODDING_GUIDE.md](docs/LUA_MODDING_GUIDE.md)

## Code of Conduct

Be respectful and constructive. Focus on the code, not the person.
