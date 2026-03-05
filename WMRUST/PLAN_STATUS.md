# Plan Status — WhoreMaster Renewal Rust Recreation

> See [PROJECT_PLAN.md](PROJECT_PLAN.md) for full phase details, technology stack, and verification checklist.

## Phase Summary

| Phase | Name | Status | Notes |
|-------|------|--------|-------|
| 0 | Rust Setup & SDL2 Hello World | **COMPLETE** | Workspace scaffolded by architect; all 6 crates compile |
| 1 | Data Model & XML Loading | **COMPLETE** | All XML deserializers, Gold system, trait parser — 14 tests passing |
| 2 | UI Widget System & Screen Framework | **COMPLETE** | All 8 widgets render, XML screen loader, FontCache, TextureCache, wm-app wired |
| 3 | Core Game Logic | **COMPLETE** | 7 managers, 64 jobs, turn processor, combat, inventory — 37 tests passing |
| 4 | Lua Scripting & .script Conversion | **COMPLETE** | mlua engine, 41 opcodes, wm.* API, trigger system — 16 tests passing |
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

### Phase 3 — Core Game Logic (`wm-game`)
- **7 manager modules:**
  - **player.rs** — Scale200 algorithm, disposition/suspicion/customer_fear adjustments (3 tests)
  - **girls.rs** — GirlManager: full stat/skill/trait management, rebel value, pregnancy, level-up, ask price, temp stat decay (5 tests)
  - **brothel.rs** — Brothel struct + BrothelManager: add/remove/assign/transfer girls, room limits (2 tests)
  - **gangs.rs** — Gang struct + GangManager: hire/fire, 11 mission types, weekly recruit system (3 tests)
  - **customers.rs** — CustomerGenerator: generate_customers/generate_customer from town wealth formulas (2 tests)
  - **dungeon.rs** — DungeonManager: stat change tables, torture mechanics, injury/scarring, miscarriage (4 tests)
  - **rivals.rs** — Rival AI: income/upkeep/spending/attack cycles, RivalManager: process/takeover/spawn (3 tests)
- **2 utility modules:**
  - **combat.rs** — gang_brawl(), girl_vs_gang(), girl_escape_odds(), player_combat() (2 tests)
  - **inventory.rs** — Equip/unequip with slot limits (Ring=8, weapons=2, armor=1, etc.), consumable handling, apply/reverse stat effects (4 tests)
- **Job system — 11 files, 64 jobs registered via JobDispatcher:**
  - **jobs/mod.rs** — Job trait + JobDispatcher with register_all() auto-registration
  - **jobs/general.rs** — 10 jobs: Resting, Training, Cleaning, Security, Advertising, Matron, Torturer, ExploreCatacombs, BeastCapture, BeastCarer (2 tests)
  - **jobs/brothel_jobs.rs** — 4 jobs: WhoreBrothel, WhoreStreets, BrothelStripper, Masseuse with shared girl_fucks() helper (2 tests)
  - **jobs/bar.rs** — 5 jobs: Barmaid, Waitress, Stripper, WhoreBar, Singer with shared bar_work_common() (1 test)
  - **jobs/gambling.rs** — 5 jobs: CustomerService, WhoreGambHall, Dealer, Entertainment, XXXEntertainment
  - **jobs/movie.rs** — 8 jobs with shared film_common() helper
  - **jobs/community.rs** — 5 jobs: CommunityService, FeedPoor, Counselor, ChurchWork, RecycleTrash
  - **jobs/drug_lab.rs** — 4 jobs: DrugCourier, DrugDealing, DrugTesting, DrugProduction
  - **jobs/alchemy.rs** — 3 jobs: AlchemyAssistant, AlchemyBrewer, AlchemyResearch
  - **jobs/arena.rs** — 5 jobs: ArenaFight, ArenaTraining, CityGuard, Escort, CentreStage
  - **jobs/training.rs** — 10 jobs with shared teach_skill() helper
  - **jobs/clinic.rs** — 5 jobs: Nurse, Doctor, Surgeon, ClinicTraining, Therapy
- **Integration modules:**
  - **state.rs** — GameState: aggregates all managers, config, gold, items, beasts, week counter
  - **turn.rs** — TurnProcessor: process_week → pre_shift_updates → day/night shifts → end_of_week (wages, tax, gangs, rivals, dungeon, level-ups, runaways, pregnancy, bank interest) (2 tests)
- **37 unit tests** all passing, zero clippy warnings

### Phase 4 — Lua Scripting & .script Conversion (`wm-script`)
- **script_converter.rs** — Binary .script parser + Lua code generator
  - `parse_script(data)` reads the legacy binary format (linked actions with typed entries)
  - `actions_to_lua(actions)` converts all 41 opcodes to idiomatic Lua calls via `wm.*` API
  - `convert_all_scripts(dir)` batch-converts every `.script` file in a directory to `.lua`
  - Handles all entry types: _NONE(0), _TEXT(1), _BOOL(2), _INT(3), _FLOAT(4), _CHOICE(5)
  - 2 tests: empty script parsing, real .script file conversion from Resources/Scripts/
- **api.rs** — ScriptContext + full wm.* Lua API registration
  - `ScriptContext` struct: girl stats/skills/flags/traits, global flags, message queue, choices, dungeon ops, action signals
  - `SharedContext = Arc<Mutex<ScriptContext>>` for thread-safe Lua↔Rust bridge
  - `register_api(lua, ctx)` registers full wm namespace: wm.message, wm.choice_box, wm.player.*, wm.global.*, wm.girl.* (stat/skill/trait/flag/checks/sex acts), wm.dungeon.*
  - Helper functions for stat/skill name→index mapping
- **lua_engine.rs** — Lua VM wrapper with context management
  - `LuaEngine { lua, ctx }` with `new()`, `exec_file()`, `exec_str()`, `context()`, `reset_context()`
  - 9 tests: creation, message API, girl stat/trait APIs, global flags, player, game_over, Intro.lua exec, context reset
- **triggers.rs** — GlobalTriggers.xml loader + trigger evaluator
  - `TriggerSystem` with `load()`, `evaluate()`, `process_next()`, `check_for_trigger()`
  - XML parsing via quick-xml for all 13 trigger types
  - `TriggerEvalContext` for condition checking (girl stats/skills, global flags, money, etc.)
  - 5 tests: GlobalTriggers.xml parsing, flag evaluation, skill evaluation, once-only, check_for_trigger
- **16 unit tests** all passing, 1 clippy warning (dead_code on entry_type field)

## Build Verification (as of Phase 4 completion)
- `cargo test -p wm-script` — **16 passed**, 0 failed
- `cargo test -p wm-game` — **37 passed**, 0 failed
- `cargo check` — all 6 crates compile and link

## Next Up: Phase 5
All Game Screens in `wm-ui` — see [PROJECT_PLAN.md](PROJECT_PLAN.md#phase-5-all-game-screens) for full task breakdown including 12+ game screens wired to real game data.
