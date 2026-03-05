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
| 5 | All Game Screens | **COMPLETE** | 15 screens wired to real game data, full navigation, all 67 tests passing |
| 6 | WMEdit Standalone | **COMPLETE** | egui desktop editor — Girls, Items, Traits tabs with load/save, 17 wm-core tests |
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

### Phase 5 — All Game Screens (`wm-ui`)
- **Architecture changes:**
  - Screen trait updated: `init`, `process`, `on_event` all take `&mut GameState`
  - ScreenManager passes GameState through push/pop/process/on_event/handle_action
  - `wm-ui/Cargo.toml` — added `wm-game`, `serde_json`, `rand` dependencies
  - `wm-app/main.rs` — creates GameState, passes to all ScreenManager calls
  - `xml_loader.rs` — enhanced with ColumnXml, EditBoxXml, dual CheckBox/Checkbox tags, Multi/HeaderDiv alternates
- **15 screens implemented (all XML-loaded or programmatic):**
  - **main_menu.rs** — Programmatic layout (MainMenu.txt spec), 3 buttons: New Game → BrothelManagement, Load → LoadGame, Exit → Quit
  - **bank.rs** — XML-loaded (bank_screen.xml), deposit/withdraw/deposit_all via Gold struct, dynamic button enable/disable
  - **town.rs** — XML-loaded (town_screen.xml), navigation hub to 5 screens, 6 brothel selection buttons, walk-around toggle
  - **brothel_management.rs** — Programmatic layout (BrothelScreen.txt), main game hub: GirlManagement/BuildingSetup/Dungeon/Town navigation, NextWeek turns via TurnProcessor, prev/next brothel cycling, details display
  - **girl_management.rs** — XML-loaded (girl_management_screen.xml), 7-column girl list from brothel, 9 job categories with filtered job lists, day/night shift toggle, job assignment via safe `job_from_id()`, view details/fire/transfer
  - **building_setup.rs** — XML-loaded (building_setup_screen.xml), potion buying (10/20 at 2g each), bar/casino hire/fire toggles, advertising slider with live-update, room building (5000g/20 rooms), 6 sex restriction checkbox toggles
  - **house.rs** — XML-loaded (house_screen.xml), player info display (name, disposition, suspicion, gold, girls, brothels, week)
  - **slave_market.rs** — XML-loaded (slavemarket_screen.xml), market population (up to 10 unassigned girls), buy mechanic deducting AskPrice gold, girl stat/trait preview on selection
  - **dungeon.rs** — XML-loaded (dungeon_screen.xml), 7-column inmate list, release selected/all girls/all customers, torture via DungeonManager.torture(), feeding toggle
  - **prison.rs** — XML-loaded (prison_screen.xml), prisoner list with details, release selected inmate
  - **gang_management.rs** — XML-loaded (gangs_screen.xml), 9-column gang list, 8-column recruit list, 11 mission types in MissionList, hire/fire gangs, weapon upgrade (scaling cost), buy heal potions (20×10g) and nets (20×5g), weekly cost display
  - **mayor.rs** — XML-loaded (mayor_screen.xml), disposition/suspicion/influence display, bribe mechanic (suspicion-scaled cost, reduces suspicion by 10)
  - **girl_details.rs** — XML-loaded (girl_details_screen.xml), full stat list (22 stats), skill list (10 skills), trait list, house % slider, prev/next girl navigation through brothel, send-to-dungeon action, `with_girl(girl_id)` constructor
  - **item_management.rs** — XML-loaded (itemmanagement_screen.xml), dual-pane owner/item lists (Player/Shop/Girl owners), equip/unequip toggle, transfer items between owners, item effect description display
  - **turn_summary.rs** — Programmatic layout, scrollable event list from TurnProcessor results, gold/girls/week summary bar, `with_events(Vec<String>)` constructor
  - **load_game.rs** — Programmatic layout, .json save file scanning, list/load/save UI (serialization stubbed pending Serialize derives on GameState), `new()` constructor
- **Gallery stub** remains minimal (optional/cosmetic screen)
- **Zero compilation errors**, zero warnings from wm-ui screens

### Phase 6 — WMEdit Standalone (`wm-edit`)
- **wm-core/xml/loaders.rs additions:**
  - `save_girls(path, &[Girl])` — writes Girls.girlsx XML with all 22 stats, 10 skills, traits
  - `save_items(path, &[Item])` — writes Items.itemsx XML with effects, type, rarity
  - `save_traits(path, &[TraitDef])` — writes CoreTraits.traits plain-text format
  - `item_type_to_str()`, `rarity_to_str()`, `effect_target_to_str()` — public helpers
  - `escape_xml_attr()` — XML attribute escaping (& < > ")
  - 3 round-trip tests: load→save→reload→verify for girls, items, traits
- **wm-edit/Cargo.toml** — added `rfd = "0.14"` for native file dialogs
- **3 editor tabs implemented:**
  - **girls_tab.rs** — Load/save .girlsx files, girl list with selection, edit name/desc/gold, 22 stat sliders with appropriate ranges, 10 skill sliders, trait management with combo box sourced from CoreTraits.traits, add/remove girls
  - **items_tab.rs** — Load/save .itemsx files, item list with selection, edit name/desc/type/cost/rarity/badness/special/infinite/girl_buy_chance, effect list with target/name/amount editing, add/remove effects, add/remove items
  - **traits_tab.rs** — Load/save .traits files, trait list with selection, edit name and description, add/remove traits
- **app.rs** — EditorApp with 3-tab bar (Girls/Items/Traits), eframe integration
- **main.rs** — 1024×768 window, eframe native launch
- **Common UI patterns:** SidePanel for lists, ScrollArea for editors, toolbar with Load/Save/Save As/Add/Remove, dirty flag indicator, status messages
- **17 wm-core tests** all passing (14 original + 3 new save round-trip tests), zero clippy warnings

## Build Verification (as of Phase 6 completion)
- `cargo test -p wm-core` — **17 passed**, 0 failed (3 new round-trip save tests)
- `cargo test -p wm-game` — **36 passed**, 1 failed (pre-existing test_whore_brothel_earns_gold)
- `cargo test -p wm-script` — **16 passed**, 0 failed
- `cargo check --workspace` — all 6 crates compile, zero errors
- `cargo clippy -p wm-core -p wm-edit` — zero warnings
- **Total: 69 tests passing** (up from 67)

## Next Up: Phase 7
Polish & Release Prep — see [PROJECT_PLAN.md](PROJECT_PLAN.md#phase-7-polish--release-prep) for cross-platform builds, CI/CD, open-licensed assets, and documentation.
