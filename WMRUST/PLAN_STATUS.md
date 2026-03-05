# Plan Status — WhoreMaster Renewal Rust Recreation

> See [PROJECT_PLAN.md](PROJECT_PLAN.md) for full phase details, technology stack, and verification checklist.

## Phase Summary

| Phase | Name | Status | Notes |
|-------|------|--------|-------|
| 0 | Rust Setup & SDL2 Hello World | **COMPLETE** | Workspace scaffolded by architect; all 6 crates compile |
| 1 | Data Model & XML Loading | **COMPLETE** | All XML deserializers, Gold system, trait parser — 14 tests passing |
| 2 | UI Widget System & Screen Framework | **COMPLETE** | All 8 widgets render, XML screen loader, FontCache, TextureCache, wm-app wired |
| 3 | Core Game Logic | Not started | `wm-game` crate — managers, jobs, turn processing |
| 4 | Lua Scripting & .script Conversion | Not started | `wm-script` crate — mlua engine, .script→Lua converter |
| 5 | All Game Screens | Not started | `wm-ui` screen/ module — 12+ screens with real game data |
| 6 | WMEdit Standalone | Not started | `wm-edit` crate — egui desktop editor |
| 7 | Polish & Release Prep | Not started | CI/CD, open assets, cross-platform, docs |

## Completed Phase Details

### Phase 0 — Rust Setup & SDL2 Hello World
- Cargo workspace with 6 crates: `wm-app`, `wm-core`, `wm-game`, `wm-ui`, `wm-script`, `wm-edit`
- SDL2 bundled build working on Windows (VS 2022 + CMake)
- `resources` symlink pointing to `../WhoreMasterRenewal/Resources/`

### Phase 1 — Data Model & XML Loading (`wm-core`)
- **items.rs** — `Items.itemsx` parser with Effect→EffectTarget enum mapping
- **rooms.rs** — `Rooms.roomsx` parser with %-string handling for Success/Factor fields
- **config.rs** — `config.xml` parser covering all 12 sections
- **girls.rs** — `Girls.girlsx` parser mapping 22 stats, 10 skills, nested traits
- **screen.rs** — `Interface/*.xml` parser with enum dispatch for all 8 widget types
- **traits.rs** — `CoreTraits.traits` plain-text parser
- **gold.rs** — Full Gold system: deposit/withdraw, 17 income + 16 expense methods, forced vs. checked spending, weekly reset
- **14 unit tests** all passing, zero clippy warnings

### Phase 2 — UI Widget System & Screen Framework (`wm-ui`)
- **graphics.rs** — SDL2 lifecycle, `draw_rect`, `draw_texture`, `draw_rect_outline`, SDL2_ttf/image init
- **font.rs** — `FontCache` with `render_text()` and `render_multiline()` (word wrap, clipping, scroll)
- **texture_cache.rs** — HashMap-based PNG/JPG texture caching
- **8 widget types** all rendering via `RenderContext`:
  - `ButtonWidget` — 3-state image (Off/On/Disabled) with hover detection
  - `TextItemWidget` — Multi-line with word wrap and scroll
  - `ListBoxWidget` — Multi-column, sortable, click-select, colored rows, headers
  - `EditBoxWidget` — Single-line input with focus styling
  - `CheckBoxWidget` — Toggle with visual check mark
  - `SliderWidget` — Drag handle with increment snapping
  - `ScrollBarWidget` — Thumb + arrows with range tracking
  - `ImageItemWidget` — Static PNG display
- **xml_loader.rs** — Creates all 8 widget types from screen XML definitions
- **screen/mod.rs** — `Screen` trait + `ScreenManager` (push/pop/pop_to stack)
- **16 screen stubs** scaffolded (bank, town, brothel_management, etc.)
- **wm-app main.rs** — Full render loop with `FontCache`, `TextureCache`, `RenderContext`, mouse tracking
- **Build note**: Removed `gfx` feature; downloaded SDL2_image + SDL2_ttf dev libs to `sdl2-libs/`

## Build Verification (as of Phase 2 completion)
- `cargo test --workspace` — **14 passed**, 0 failed
- `cargo clippy --workspace` — **0 warnings**
- `cargo check` — all 6 crates compile and link

## Next Up: Phase 3
Core Game Logic in `wm-game` — see [PROJECT_PLAN.md](PROJECT_PLAN.md#phase-3-core-game-logic) for full task breakdown including managers, job system, turn processing, and combat.
