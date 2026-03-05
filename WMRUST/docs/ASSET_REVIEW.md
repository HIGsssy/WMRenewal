# Asset Copyright Review

This document tracks the copyright status of assets used by WhoreMaster Renewal.

## Fonts

| Font | Status | License | Notes |
|------|--------|---------|-------|
| DejaVu Sans | **OK** | Bitstream Vera / public domain additions | Bundled in `assets/fonts/` |
| DejaVu Sans Mono | **OK** | Bitstream Vera / public domain additions | Bundled in `assets/fonts/` |
| Segoe UI (`segoeui.ttf`) | **REMOVE** | Microsoft proprietary | Legacy reference removed from defaults; fallback only |
| Comic Sans MS (`comic.ttf`) | **REMOVE** | Microsoft proprietary | Not referenced in Rust code |

## Character Image Directories

| Directory | Status | Issue |
|-----------|--------|-------|
| `Cammy White/` | **FLAGGED** | Named after Capcom's Street Fighter character — trademarked |
| `Chun-Li/` | **FLAGGED** | Named after Capcom's Street Fighter character — trademarked |
| `Cute Girl/` | **UNKNOWN** | Image provenance unknown; no trademark issues with name |
| `Dangerous Girl/` | **UNKNOWN** | Image provenance unknown; no trademark issues with name |

## Recommended Actions Before Public Release

1. **Do NOT bundle** `Cammy White/` or `Chun-Li/` directories in any public distribution.
2. **Do NOT bundle** proprietary fonts (`segoeui.ttf`, `comic.ttf`).
3. Create placeholder/original character images for any characters shipped with the game.
4. Audit `Cute Girl/` and `Dangerous Girl/` images for copyright — replace with original art if origin cannot be verified.
5. All UI images in `Resources/Buttons/`, `Resources/Images/`, `Resources/Interface/` should be reviewed for provenance.

## Game Data Files (XML)

The `.girlsx`, `.rgirlsx`, `.itemsx`, `.roomsx`, `.traits`, and `config.xml` files are game data authored by the WhoreMaster community under GPL. These are safe to redistribute under GPL terms.

## Scripts

Lua scripts and converted `.script` files are original game logic — redistributable under GPL.
