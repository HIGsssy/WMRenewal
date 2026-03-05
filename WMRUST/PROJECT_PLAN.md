# WhoreMaster Renewal — Rust Recreation Project Plan

## Overview

Recreate WhoreMaster Renewal from scratch in Rust, using the existing C++ codebase (~142 source files) and game data assets as reference. The game is a turn-based business management sim originally built on SDL 1.2, Lua 5.2, TinyXML, and a custom UI framework. The Rust version uses SDL2, serde + quick-xml, mlua, and a custom widget system. All existing XML data files, 300+ image assets, and Lua scripts reuse directly.

## Decisions

| # | Decision | Choice |
|---|----------|--------|
| 1 | Path | Full recreation in Rust |
| 2 | Language | Rust (learning curve accounted for in phasing) |
| 3 | Save compatibility | No — fresh save format |
| 4 | .script binary files | Reverse-engineer (format documented in source, 9 small files) then convert to Lua |
| 5 | WMEdit | Standalone Rust + egui desktop app |
| 6 | Legal | Use existing assets during dev; clear before public release |

## Technology Stack

| Need | Crate | Notes |
|------|-------|-------|
| Windowing + 2D Rendering | `sdl2` (rust-sdl2) | Mature SDL2 bindings, closest to original |
| Image loading | `sdl2::image` | PNG/JPG, matches original |
| Font rendering | `sdl2::ttf` | TTF via SDL2_ttf |
| Graphics primitives | `sdl2::gfx` | Rects, lines |
| XML data parsing | `quick-xml` + `serde` | Derive directly to Rust structs |
| Lua scripting | `mlua` | Lua 5.4, most actively maintained |
| Save/load serialization | `serde` + `serde_json` | Fresh format, no legacy compat needed |
| Logging | `tracing` + `tracing-subscriber` | Structured logging |
| Error handling | `anyhow` + `thiserror` | Ergonomic error types |
| Editor UI (WMEdit) | `eframe` + `egui` | Immediate-mode GUI, cross-platform |
| RNG | `rand` | Game randomness |

## Prerequisites (Install Before Phase 0)

1. **Rust toolchain**: Install via [rustup.rs](https://rustup.rs). This gives you `rustc`, `cargo`, and `rustup`.
2. **C compiler + CMake**: Required for the SDL2 `bundled` feature (compiles SDL2 from C source). On Windows, install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (select "Desktop development with C++") and [CMake](https://cmake.org/download/).
3. **VS Code extensions**: `rust-analyzer` (Rust language support), `Even Better TOML` (Cargo.toml editing).

> **Why `bundled`?** The `sdl2` crate needs SDL2 development libraries. Rather than manually downloading DLLs and setting `LIB`/`PATH`, the `bundled` cargo feature compiles SDL2 from source automatically. This is the simplest path on Windows and makes CI/CD trivial.

## Cargo Workspace Structure

```
WMRUST/
├── Cargo.toml              # Workspace root
├── PROJECT_PLAN.md
├── ARCHITECT_PROMPT.md
├── BUILDER_PROMPT.md
├── crates/
│   ├── wm-app/             # Main game binary (entry point + game loop)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs         # SDL2 init, game loop, wires wm-ui + wm-game + wm-script
│   │
│   ├── wm-core/            # Shared data model, enums, XML loaders
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── enums.rs        # Stat, Skill, JobType, ItemType, Trait enums
│   │       ├── girl.rs         # Girl struct, stat/skill arrays
│   │       ├── item.rs         # Item, Effect structs
│   │       ├── room.rs         # Room/Facility structs
│   │       ├── config.rs       # GameConfig from config.xml
│   │       ├── gold.rs         # Gold/economy categorical tracking
│   │       ├── traits.rs       # Trait definitions + parser
│   │       └── xml/
│   │           ├── mod.rs
│   │           ├── items.rs    # Items.itemsx deserializer
│   │           ├── girls.rs    # Girls.girlsx deserializer
│   │           ├── rooms.rs    # Rooms.roomsx deserializer
│   │           ├── config.rs   # config.xml deserializer
│   │           └── screen.rs   # Screen XML layout deserializer
│   │
│   ├── wm-game/            # Game logic (no rendering)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state.rs        # GameState: owns all managers
│   │       ├── brothel.rs      # Brothel struct + manager
│   │       ├── girl_manager.rs # GirlManager: girl lifecycle
│   │       ├── gang_manager.rs # GangManager: gangs, recruitment
│   │       ├── customer.rs     # CustomerGenerator
│   │       ├── dungeon.rs      # DungeonManager
│   │       ├── rival.rs        # RivalManager
│   │       ├── player.rs       # Player struct
│   │       ├── turn.rs         # TurnProcessor: weekly cycle orchestration
│   │       ├── combat.rs       # Gang fight resolution
│   │       ├── pregnancy.rs    # Pregnancy/maternity system
│   │       ├── inventory.rs    # Inventory management
│   │       └── jobs/
│   │           ├── mod.rs      # Job trait + JobDispatcher
│   │           ├── whore.rs
│   │           ├── bar.rs
│   │           ├── stripper.rs
│   │           ├── security.rs
│   │           ├── matron.rs
│   │           ├── torturer.rs
│   │           ├── advertising.rs
│   │           ├── cleaning.rs
│   │           ├── hall.rs
│   │           ├── fluffer.rs
│   │           ├── masseuse.rs
│   │           ├── freetime.rs
│   │           ├── beast_care.rs
│   │           ├── beast_capture.rs
│   │           └── explore_catacombs.rs
│   │
│   ├── wm-ui/              # SDL2 rendering + widget system
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── graphics.rs     # SDL2 init, render loop, surface/texture mgmt
│   │       ├── font.rs         # Font loading + text rendering
│   │       ├── resources.rs    # Image cache / resource manager
│   │       ├── widget/
│   │       │   ├── mod.rs      # Widget enum, WidgetId, WidgetStore
│   │       │   ├── button.rs   # 3-state button (off/on/disabled)
│   │       │   ├── text_item.rs# Multi-line text with scrollbar
│   │       │   ├── list_box.rs # Multi-select, sortable, multi-column
│   │       │   ├── edit_box.rs # Single-line text input
│   │       │   ├── check_box.rs
│   │       │   ├── slider.rs   # Integer range drag
│   │       │   ├── scroll_bar.rs
│   │       │   └── image_item.rs # Static/animated image display
│   │       ├── screen/
│   │       │   ├── mod.rs      # Screen trait + ScreenManager (push/pop stack)
│   │       │   ├── main_menu.rs
│   │       │   ├── town.rs
│   │       │   ├── building_management.rs
│   │       │   ├── building_setup.rs
│   │       │   ├── girl_management.rs
│   │       │   ├── girl_details.rs
│   │       │   ├── slave_market.rs
│   │       │   ├── dungeon.rs
│   │       │   ├── gangs.rs
│   │       │   ├── bank.rs
│   │       │   ├── house.rs
│   │       │   ├── prison.rs
│   │       │   ├── mayor.rs
│   │       │   ├── item_management.rs
│   │       │   ├── turn_summary.rs
│   │       │   └── load_save.rs
│   │       └── xml_loader.rs   # Parse Interface/*.xml → WidgetStore
│   │
│   ├── wm-script/           # Lua scripting + .script converter
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── lua_engine.rs   # mlua setup, wm.* API registration
│   │       ├── api.rs          # All wm.* Lua functions
│   │       ├── triggers.rs     # TriggerSystem: GlobalTriggers.xml loader + evaluator
│   │       └── script_converter.rs  # .script binary → Lua converter (tool)
│   │
│   └── wm-edit/             # Standalone editor (egui)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── app.rs          # eframe App impl
│           ├── girls_tab.rs
│           ├── items_tab.rs
│           └── traits_tab.rs
│
└── resources/               # Symlink to game resources for dev
    └── (see Resource Path Strategy below)
```

## Resource Path Strategy

Game data lives at `../WhoreMasterRenewal/Resources/`. Rather than copying or hardcoding paths:

1. **Create a symlink** in the workspace root: `WMRUST/resources/ → ../WhoreMasterRenewal/Resources/`
   - Windows (admin PowerShell): `New-Item -ItemType SymbolicLink -Path resources -Target ..\WhoreMasterRenewal\Resources`
2. **Define a constant** in `wm-core/src/lib.rs`:
   ```rust
   /// Base path to game resources. Configurable via WM_RESOURCES_PATH env var.
   pub fn resources_path() -> std::path::PathBuf {
       std::env::var("WM_RESOURCES_PATH")
           .map(std::path::PathBuf::from)
           .unwrap_or_else(|_| std::path::PathBuf::from("resources"))
   }
   ```
3. **Tests** use `resources_path()` — works both locally (via symlink) and in CI (via env var).
4. **Runtime binary** uses the same function — set `WM_RESOURCES_PATH` or place the resources dir next to the executable.

## C++ Reference Codebase Location

All original source is at `../WhoreMasterRenewal/` relative to WMRUST:
- `src/` — 142 C++ source files
- `Resources/` — Game data, UI definitions, images, scripts
- `original WM files/` — Original game files for additional reference
- `Docs&Tools/` — Design document, script editor, ScriptCommands.txt

## Phase Plan

### Phase 0: Rust Setup & SDL2 Hello World
**Deliverables:** Cargo workspace compiles, SDL2 window opens, renders an image + text.
**Key files:** `Cargo.toml` (workspace), `crates/wm-app/src/main.rs`, `crates/wm-ui/src/graphics.rs`
**Reference:** `../WhoreMasterRenewal/src/CGraphics.h` (SDL init), `../WhoreMasterRenewal/src/main.cpp` (game loop)
**Rust concepts:** ownership, borrowing, Result, match, modules, cargo

**Critical: SDL2 texture lifetimes.** In rust-sdl2, `Texture` borrows from `TextureCreator`, which borrows from `Canvas`. Storing textures in structs requires careful lifetime design. The recommended pattern is a `TextureManager` that owns the `TextureCreator` and a `HashMap<String, Texture>` — both in the same struct using self-referential patterns (or the `sdl2` `unsafe_textures` feature which makes `Texture: 'static`). The architect must design this upfront.

### Phase 1: Data Model & XML Loading
**Deliverables:** All game data files deserialize into typed Rust structs with passing unit tests.
**Crate:** `wm-core`
**Data files to load:**
- `Resources/Data/Items.itemsx` — Items with nested Effects
- `Resources/Data/config.xml` — Game configuration (12 sections)
- `Resources/Data/Rooms.roomsx` — 39 facility types with Functions
- `Resources/Data/CoreTraits.traits` — Plain-text trait definitions
- `Resources/Characters/Girls.girlsx` — Character definitions (22 stats, 10 skills, traits)
- `Resources/Characters/Random*.rgirlsx` — Random girl templates

**Key enums to define:**
- `Stat` — 22 variants: Charisma, Happiness, Libido, Constitution, Intelligence, Confidence, Mana, Agility, Fame, Level, AskPrice, HousePerc, Exp, Age, Obedience, Spirit, Beauty, Tiredness, Health, PCFear, PCLove, PCHate
- `Skill` — 10 variants: Anal, BDSM, NormalSex, Beastiality, Group, Lesbian, Service, Strip, Combat, Magic
- `JobType` — 30+ variants (one per Work*.cpp file)
- `ItemType` — Food, Ring, Necklace, Dress, Underwear, Shoes, SmallWeapon, etc.

**Reference:** `Girl.hpp` (stat/skill lists), `cGold.h` (economy), `sConfig.h` (config struct)
**Rust concepts:** enums, derive macros, serde, Vec, unit tests

**Critical: `quick-xml` serde attribute syntax.** When deserializing XML attributes (not child elements), `quick-xml`'s serde integration requires the `@` prefix on field names:
```rust
#[derive(Deserialize)]
struct Item {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@Cost")]
    cost: i32,
    // Child elements do NOT get the @ prefix:
    #[serde(rename = "Effect", default)]
    effects: Vec<Effect>,
}
```
Without the `@` prefix, attribute deserialization silently fails. This applies to every XML struct in the project.

### Phase 2: UI Widget System & Screen Framework
**Deliverables:** 8 widget types render correctly; Screen trait + ScreenManager; XML screen loader; 2 proof-of-concept screens (Main Menu, Bank).
**Crate:** `wm-ui`

**Widget types (from C++ codebase):**
| Widget | C++ Reference | Key Features |
|--------|--------------|--------------|
| Button | `cButton.h` | 3-state (off/on/disabled), 3 image layers, click detection |
| TextItem | `cTextItem.h` | Multi-line, auto-scroll, text wrapping |
| ListBox | `cListBox.h` | Multi-select (Shift/Ctrl), up to 6 columns, sortable headers, color-coded rows, double-click |
| EditBox | `cEditBox.h` | Single-line text input, focus state |
| CheckBox | `cCheckBox.h` | Boolean toggle with label |
| Slider | `cSlider.h` | Integer range, drag, live-update |
| ScrollBar | `cScrollBar.h` | Vertical, item-based, linked to ListBox/TextItem |
| ImageItem | `cImageItem.h` | Static PNG or animated sprite sheet |

**Screen XML format** (example from `bank_screen.xml`):
```xml
<Screen>
  <Window Name="Bank Window" XPos="280" YPos="152" Width="240" Height="296" Border="1" />
  <Image Name="BankIcon" File="Bank.png" XPos="6" YPos="6" Width="32" Height="32" />
  <Text Name="ScreenHeader" Text="Town Bank" XPos="70" YPos="8" Width="100" Height="32" FontSize="20" />
  <Button Name="DepositButton" Image="Deposit" XPos="40" YPos="112" Width="160" Height="32" Transparency="true" Scale="true" />
</Screen>
```

**Screen lifecycle** (from `cInterfaceWindowXML` pattern):
1. `init()` — Load XML, create widgets, cache widget IDs, set initial state
2. `process()` — Called each frame: update display, check events
3. `on_event()` — Handle button clicks/widget interactions by WidgetId

**Reference:** `cInterfaceWindow.h`, `InterfaceWindowXML.hpp`, `cWindowManager.h`, `cScreenBank.cpp`
**Rust concepts:** traits, dyn Trait, Box<dyn Screen>, HashMap, closures

### Phase 3: Core Game Logic
**Deliverables:** All game simulation systems functional and unit-tested (no UI required).
**Crate:** `wm-game`

**Managers to implement (decomposing the C++ God Object `BrothelManager`):**
| Manager | C++ Reference | Responsibility |
|---------|--------------|----------------|
| GameState | `main.cpp` globals | Owns all managers, replaces 20+ globals |
| BrothelManager | `BrothelManager.hpp` | Brothel CRUD, girl assignment |
| GirlManager | `GirlManager.hpp`, `cGirls.h` | Girl lifecycle, stat queries, trait checks |
| GangManager | `GangManager.hpp`, `cGangs.h` | Gang recruitment, training, missions |
| CustomerGenerator | `cCustomers.h` | Random customer gen from town wealth |
| DungeonManager | `cDungeon.h` | Prisoner tracking, torture mechanics |
| RivalManager | `cRival.h` | Rival AI (extracted from BrothelManager) |
| TurnProcessor | `BrothelManager.cpp` turn logic | Weekly cycle orchestration |
| JobDispatcher | `cJobManager.h` | Routes JobType → Job impl |

**Job trait pattern** (replacing C++ function pointer array):
```rust
pub trait Job {
    fn job_type(&self) -> JobType;
    fn process(&self, girl: &mut Girl, brothel: &Brothel, rng: &mut impl Rng) -> JobResult;
}
```

**Turn processing cycle** (reference: BrothelManager weekly processing):
1. Process all girl job assignments (day shift → night shift)
2. Process gang missions
3. Process rival actions
4. Generate customers
5. Calculate income/expenses via Gold system
6. Fire event triggers
7. Age/heal/stat decay

**Reference:** All `Work*.cpp` files, `cGirlGangFight.cpp`, `cGirlTorture.cpp`, `cPlayer.h`
**Rust concepts:** trait objects, iterators, rand crate, builder pattern

### Phase 4: Lua Scripting & .script Conversion
**Deliverables:** Lua engine with full wm.* API; all 9 .script files converted to Lua; trigger system operational.
**Crate:** `wm-script`

**Binary .script format** (from `cGameScript.h`):
```
sScript { m_Type: i32, m_NumEntries: i32, entries: [sScriptEntry], next: *sScript }
sScriptEntry { m_Type: i32, m_Value: i32, m_Var: u8 }
```
- 41 opcodes documented in `Docs&Tools/ScriptEditor/Data/ScriptCommands.txt`
- 9 files to convert: CustNoPay, CustGambCheat, RivalLose, NoMoney, DefaultInteractDetails, DefaultInteractDungeon, MeetTownDefault, TalkDetailsDefault, TalkDungeonDefault

**Lua API to expose** (covers all 41 C++ opcodes):
```lua
wm.message(text, color)
wm.choice_box(choices) → selected_index
wm.girl.get_stat(name) / wm.girl.set_stat(name, delta)
wm.girl.get_skill(name) / wm.girl.set_skill(name, delta)
wm.girl.has_trait(name) / wm.girl.add_trait(name) / wm.girl.remove_trait(name)
wm.girl.get_flag(id) / wm.girl.set_flag(id, value)
wm.global.get_flag(id) / wm.global.set_flag(id, value)
wm.player.set_suspicion(delta) / wm.player.set_disposition(delta)
wm.dungeon.add_customer(reason)
wm.dungeon.add_girl() / wm.dungeon.add_random_girls(count)
wm.give_random_special_item()
wm.game_over()
```

**Trigger system** (from `GlobalTriggers.xml`): 11 trigger types — Random, Shopping, Skill, Stat, Status, Money, Meet, Talk, WeeksPast, GlobalFlag, Kidnapped

**Reference:** `cGameScript.h/.cpp`, `cLuaScript.h/.cpp`, `cTriggers.h/.cpp`, `Resources/Scripts/`

### Phase 5: All Game Screens
**Deliverables:** All 10+ screens functional with real game data.
**Crate:** `wm-ui` (screen/ module)

**Screens in order of complexity:**
1. Main Menu — buttons only (simplest)
2. Bank — text + buttons + edit box
3. Town — navigation hub, building buttons, current brothel display
4. House / Prison — simple management screens
5. Gangs — recruit, train, assign missions (ListBox heavy)
6. Slave Market — buy girls, preview stats
7. Dungeon — prisoner list, torture/release
8. Building Management / Setup — girl list, job assignment, shift toggle
9. Item Management — inventory with drag/equip
10. Girl Details — stats, skills, traits, inventory (most complex screen)
11. Turn Summary — color-coded event list (most gameplay-critical)
12. Load / Save — new save format via serde

**Reference:** All `cScreen*.cpp/.h` files, all `Resources/Interface/*.xml` files

### Phase 6: WMEdit Standalone
**Deliverables:** Desktop editor that loads, edits, and saves all game data XML files.
**Crate:** `wm-edit` (depends on `wm-core` for shared types)

**Tabs:**
- Girls — edit stats, skills, traits, descriptions
- Items — edit effects, costs, rarity, descriptions
- Traits — view/edit trait definitions

**Reference:** `../WMEdit/WM Girls Generator/Form1.cs` (C# WinForms, .NET 3.5)

### Phase 7: Polish & Release Prep
**Deliverables:** Cross-platform builds, CI/CD, open-licensed assets, documentation.

**Tasks:**
1. Replace proprietary fonts (Segoe UI, Comic Sans) with open alternatives
2. Review character assets for copyright (Cammy White, Chun-Li dirs)
3. Cross-platform CI: GitHub Actions — cargo build, test, clippy
4. Release packaging with resources bundled
5. Modding documentation for Lua API
6. README + contribution guide

## Verification Checklist

- [ ] `cargo build` + `cargo clippy` — zero warnings
- [ ] All XML data files deserialize correctly (unit tests per format)
- [ ] All 10+ screens render with correct layout from XML definitions
- [ ] 10-turn simulation produces correct stat/gold/event outcomes
- [ ] Lua scripts execute; triggers fire; converted scripts match C++ behavior
- [ ] ListBox: multi-select, sort, scroll all functional
- [ ] Builds and runs on Windows + Linux
- [ ] WMEdit loads, edits, saves all data files without data loss
- [ ] `cargo test` — full suite passes

## Scope Estimates

| Phase | ~Lines of Rust | Complexity |
|-------|---------------|------------|
| 0 — Scaffold | 200 | Low |
| 1 — Data Model | 1,500–2,000 | Low–Med |
| 2 — UI Widgets | 4,000–6,000 | **High** |
| 3 — Game Logic | 8,000–12,000 | **High** |
| 4 — Scripting | 2,000–3,000 | Medium |
| 5 — All Screens | 5,000–8,000 | Medium |
| 6 — WMEdit | 2,000–3,000 | Medium |
| 7 — Polish | 500–1,000 | Low |
| **Total** | **~23K–36K** | |
