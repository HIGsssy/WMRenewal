# Builder Agent Prompt — WhoreMaster Renewal Rust Recreation

## Your Role

You are the **builder** for the Rust recreation of WhoreMaster Renewal. The architect agent has already created the cargo workspace, all crate boundaries, type definitions, traits, and module stubs. Your job is to **fill in the implementations** — real game logic, working widget rendering, functional screens, Lua scripting, and the .script converter. Work one phase at a time, starting from whichever phase the user requests.

## Context

- **Project plan:** `WMRUST/PROJECT_PLAN.md` — full technology choices, phase breakdown, and folder structure
- **Architecture:** The workspace at `WMRUST/crates/` has 6 crates: `wm-app` (game binary), `wm-core`, `wm-game`, `wm-ui`, `wm-script`, `wm-edit`
- **C++ reference:** `../WhoreMasterRenewal/src/` (142 files), `../WhoreMasterRenewal/Resources/`
- **Original game files:** `../WhoreMasterRenewal/original WM files/` for additional reference

**Always read the existing Rust stub code before implementing.** The architect defined the types and signatures — respect them. If a type needs changing, explain why before modifying it.

## Implementation Phases

Work through these phases in order unless the user directs otherwise. Each phase produces working, tested code.

---

### Phase 1: Data Model & XML Loading (`wm-core`)

**Goal:** Every XML data file deserializes into typed Rust structs. Unit tests pass loading real game data.

**Tasks:**

1. **Implement XML deserializers** in `crates/wm-core/src/xml/`:

   > **CRITICAL: `quick-xml` serde attribute syntax.** When deserializing XML **attributes**, `quick-xml` requires the `@` prefix on the serde rename string. Without it, attribute deserialization silently fails:
   > ```rust
   > #[derive(Deserialize)]
   > struct Item {
   >     #[serde(rename = "@Name")]   // @ prefix = XML attribute
   >     name: String,
   >     #[serde(rename = "Effect", default)]  // NO @ = child element
   >     effects: Vec<Effect>,
   > }
   > ```
   > Apply this `@` prefix pattern to **every** struct that maps to an XML element with attributes.

   - `items.rs` — Parse `Resources/Data/Items.itemsx`. Root element `<Items>`, children `<Item>` with attributes + nested `<Effect>` elements. Map `What` attribute to `EffectTarget` enum, `Rarity` string to `Rarity` enum.
   - `rooms.rs` — Parse `Resources/Data/Rooms.roomsx`. Root `<Facilities>`, children `<Facility>` with nested `<Function>` elements.
   - `config.rs` — Parse `Resources/Data/config.xml`. Root `<config>`, children: `<Initial>`, `<Income>`, `<Expenses>`, `<Gambling>`, `<Tax>`, `<Pregnancy>`, `<Gangs>`, `<Prostitution>`, `<Items>`, `<Fonts>`, `<Debug>`. Note: percentage values like `"49%"` and `"6%"` need string-to-f32 parsing.
   - `girls.rs` — Parse `Resources/Characters/Girls.girlsx`. Each `<Girl>` has 22 stat attributes, 10 skill attributes, and nested `<Trait>` elements.
   - `screen.rs` — Parse `Resources/Interface/*.xml`. Root `<Screen>`, children: `<Window>`, `<Text>`, `<Button>`, `<Image>`, `<ListBox>`, `<CheckBox>`, `<Slider>`, `<EditBox>`. Each has `Name`, `XPos`, `YPos`, `Width`, `Height` and type-specific attributes.

2. **Implement `traits.rs`** — Parse `CoreTraits.traits`: plain text, alternating lines (name, description). Return `Vec<TraitDef>`.

3. **Implement `gold.rs`** — The Gold struct with methods:
   - `deposit(amount)`, `withdraw(amount) -> Result`
   - `add_income(category, amount)`, `add_expense(category, amount)`
   - `total_income()`, `total_expenses()`, `net_income()`
   - `reset_weekly()` — clear periodic counters
   - Reference: `../WhoreMasterRenewal/src/cGold.h` and `cGold.cpp`

4. **Write unit tests** in each module:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       #[test]
       fn test_load_items() {
           let items = load_items("../../WhoreMasterRenewal/Resources/Data/Items.itemsx").unwrap();
           assert!(items.len() > 50); // should have 100+ items
           let aids_cure = items.iter().find(|i| i.name == "AIDS Cure").unwrap();
           assert_eq!(aids_cure.cost, 3500);
           assert!(aids_cure.effects.len() >= 10);
       }
   }
   ```

**C++ reference files to consult:**
- `Girl.hpp` lines defining stat/skill enums and their order
- `sConfig.h` / `sConfig.cpp` for config loading patterns
- `cGold.h` / `cGold.cpp` for economy categories
- `cInventory.h` for item type handling
- `cTraits.h` for trait loading

**Verification:** `cargo test -p wm-core` passes, loading all real data files.

---

### Phase 2: UI Widget System (`wm-ui`)

**Goal:** All 8 widget types render correctly via SDL2. XML screen definitions load and display.

**Tasks:**

1. **Implement `graphics.rs`** — Full SDL2 lifecycle:
   - `Graphics::new()` — Init SDL2, create 800×600 window, create renderer
   - `begin_frame()` — `canvas.set_draw_color(black); canvas.clear()`
   - `end_frame()` — `canvas.present()`, frame timing (25 FPS = 40ms per frame)
   - `load_texture(path)` — Load PNG/JPG via sdl2::image
   - `draw_rect(rect, color)` — Filled rectangle
   - Reference: `CGraphics.h/.cpp` — init pattern, frame rate control

   > **SDL2 Texture Lifetimes:** The architect set up the `unsafe_textures` sdl2 feature, which makes `Texture: 'static` (no lifetime borrow from `TextureCreator`). This means you must call `.destroy()` on textures manually when done. In practice, use the `TextureCache` / `ResourceManager` struct (already stubbed) which drops all textures before destroying the `TextureCreator`. Never store raw `Texture` objects outside the cache.

2. **Implement `font.rs`** — SDL2_ttf wrapper:
   - `FontCache` — HashMap of (font_path, size) → loaded Font
   - `render_text(canvas, text, x, y, font_size, color)` — Multi-line support with word wrap
   - Reference: `cFont.h/.cpp`

3. **Implement `resources.rs`** — Image/texture cache:
   - `ResourceManager` — HashMap<String, Texture> with LRU eviction (cap at ~50 textures)
   - `get_texture(path)` — Load or return cached
   - Button images: look up `Resources/Buttons/{name}On.png`, `{name}Off.png`, `{name}Disabled.png`
   - Reference: `CResourceManager.h/.cpp`

4. **Implement each widget type** in `crates/wm-ui/src/widget/`:

   **`button.rs`** — Three-state image button:
   - States: Off (default), On (hover/pressed), Disabled
   - Load 3 images: `{Image}Off.png`, `{Image}On.png`, `{Image}Disabled.png` from `Resources/Buttons/`
   - `draw()`: render appropriate image based on state + mouse position
   - `handle_click(mouse_x, mouse_y) -> bool`: hit test against rect
   - Reference: `cButton.h/.cpp`

   **`text_item.rs`** — Multi-line read-only text:
   - Word-wrap text to fit width
   - Scroll offset for long text (linked to ScrollBar if present)
   - `set_text(text)`, `draw()`, `scroll(delta)`
   - Reference: `cTextItem.h/.cpp`

   **`list_box.rs`** — This is the most complex widget:
   - Items: `Vec<ListItem>` where `ListItem { columns: Vec<String>, color: ListColor, data: i32 }`
   - Multi-column: up to 6 columns with configurable offsets
   - Sortable: click column header to sort (ascending/descending toggle)
   - Multi-select: Ctrl+click for individual, Shift+click for range
   - Color-coded rows: `ListColor` enum (Blue, Red, Green, Yellow, DarkBlue)
   - Scrollbar integration: tracks visible window into item list
   - Double-click detection
   - Reference: `cListBox.h/.cpp` — study carefully, this is ~800 lines in C++

   **`edit_box.rs`** — Single-line text input:
   - Focus state (draw different background when focused)
   - Keyboard character accumulation
   - `get_text() -> &str`, `set_text(text)`, `clear()`
   - Reference: `cEditBox.h/.cpp`

   **`check_box.rs`** — Simple toggle:
   - Checked/unchecked state, label text, enabled/disabled
   - Reference: `cCheckBox.h/.cpp`

   **`slider.rs`** — Integer range with drag:
   - `min`, `max`, `value`, `increment`
   - Mouse drag to change value
   - Optional live-update mode (fires change on drag vs. on release)
   - Reference: `cSlider.h/.cpp`

   **`scroll_bar.rs`** — Vertical scroll:
   - `position`, `total_items`, `visible_items`
   - Up/down buttons + drag handle
   - Reference: `cScrollBar.h/.cpp`

   **`image_item.rs`** — Static image display:
   - Load PNG, draw at position, support hide/show
   - Optional animated sprite sheet support (rows × columns frame grid)
   - Reference: `cImageItem.h/.cpp`, `cAnimatedSurface.h`

5. **Implement `xml_loader.rs`** — Parse screen XMLs into WidgetStore:
   - Read `<Screen>` root, iterate child elements
   - For each `<Button>`, `<Text>`, `<Image>`, `<ListBox>`, etc.: create Widget, assign WidgetId, insert into WidgetStore
   - Map `Name` attribute to a name→id lookup for screens to use
   - Test with `bank_screen.xml` (simplest) and `town_screen.xml`

6. **Implement `screen/mod.rs`** — ScreenManager:
   - `push(screen)`, `pop()`, `pop_to(id)`
   - `current() -> &mut dyn Screen`
   - `process_current()`, `draw_current()`
   - Reference: `cWindowManager.h/.cpp`

**Verification:**
- Window opens at 800x600
- `bank_screen.xml` loads and renders: background, icon, text labels, 4 buttons
- Buttons highlight on hover, fire click events
- ListBox renders with sample data, scrolls, sorts by column header click

---

### Phase 3: Core Game Logic (`wm-game`)

**Goal:** Complete game simulation — testable without UI via unit/integration tests.

**Tasks:**

1. **Implement `girl_manager.rs`**:
   - `add_girl(girl)`, `remove_girl(id)`, `get_girl(id)`, `get_girl_mut(id)`
   - `find_by_name(name)`, `all_girls()`, `girls_at_brothel(brothel_id)`
   - Stat/skill modification with clamp (0–100 for most, some exceptions)
   - Trait add/remove/check
   - Reference: `GirlManager.hpp/.cpp`, `cGirls.h/.cpp`

2. **Implement `brothel.rs`**:
   - `Brothel` struct: name, rooms, assigned_girls (Vec of girl IDs), max_girls
   - `BrothelManager`: CRUD for brothels, girl assignment/unassignment
   - Reference: `Brothel.hpp`, `cBrothel.h`

3. **Implement `gang_manager.rs`**:
   - `Gang` struct: name, members, stats (combat, magic, intelligence)
   - Recruitment pool (configurable from `config.xml` Gangs section)
   - Mission types: guard, sabotage, recruit, capture
   - Weekly gang mission processing
   - Reference: `GangManager.hpp`, `cGangs.h/.cpp`

4. **Implement `customer.rs`**:
   - Random customer generation: stats based on town wealth
   - Customer preferences affecting girl matching
   - Reference: `cCustomers.h/.cpp`

5. **Implement `dungeon.rs`**:
   - Prisoner list (girls + customers)
   - Torture mechanics: stat changes, trait gain chances, week tracking
   - Release/enslave actions
   - Reference: `cDungeon.h/.cpp`, `cGirlTorture.h/.cpp`

6. **Implement `rival.rs`**:
   - Rival brothels with simple AI: expand, hire girls, attack player
   - Extracted from C++ BrothelManager's rival logic
   - Reference: `cRival.h/.cpp`, rival-related methods in `BrothelManager.cpp`

7. **Implement `player.rs`**:
   - `Player` struct: disposition, suspicion, customer_fear
   - Reference: `cPlayer.h/.cpp`

8. **Implement all jobs** in `jobs/`:
   - Each job struct implements `Job` trait
   - Key logic per job: girl stat checks → success/fail → gold earned → stat/skill changes → events
   - `whore.rs` is the most complex (customer satisfaction, payment, risk of assault)
   - `security.rs` affects brothel defense
   - `matron.rs` provides passive bonuses
   - Reference: Each `Work*.cpp` file maps to one job. Read them carefully for formulas.

9. **Implement `turn.rs`** — `TurnProcessor::process_week()`:
   - For each brothel: for each girl: process day job, process night job
   - Process all gang missions
   - Process rival AI actions
   - Generate customers → match to girls
   - Calculate gold: sum all income, subtract all expenses, apply tax
   - Fire triggers (delegate to wm-script when available; for now, collect trigger conditions)
   - Age girls, heal injured, process pregnancies
   - Reference: The weekly processing loop in `BrothelManager.cpp`

10. **Implement `combat.rs`**:
    - Gang vs gang fight resolution
    - Girl combat stats contribution
    - Casualty calculation
    - Reference: `cGirlGangFight.h/.cpp`

11. **Implement `inventory.rs`**:
    - Equip/unequip items to girls
    - Apply item effects to stats/skills
    - Auto-equip for combat (if config flag set)
    - Reference: `cInventory.h/.cpp`

**Verification:** `cargo test -p wm-game` passes:
- Create game state, add girls, assign jobs, run 10 turns
- Assert gold changes, stat changes, events generated
- Gang recruitment + mission → assert outcomes
- Dungeon torture → assert trait gain probability
- Customer generation → assert stat ranges match config

---

### Phase 4: Lua Scripting & .script Conversion (`wm-script`)

**Goal:** Working Lua engine, converted scripts, operational trigger system.

**Tasks:**

1. **Implement `script_converter.rs`** — Binary .script → Lua converter:
   - Parse binary format: `sScript { type: i32, num_entries: i32, entries: [...], next: ptr }`
   - Each entry: `sScriptEntry { type: i32, value: i32, var: u8 }`
   - Map 41 opcodes to Lua function calls (opcode table from `cGameScript.h`):

   | Opcode | C++ Name | Lua Output |
   |--------|----------|------------|
   | 0 | DIALOG | `wm.message("text", color)` |
   | 1 | INIT | `-- init block` |
   | 2 | ENDINIT | `-- end init` |
   | 3 | ENDSCRIPT | `-- end` |
   | 4 | SETVAR | `vars[N] = value` |
   | 5 | IFVAR | `if vars[N] op value then` |
   | 6 | ELSE | `else` |
   | 7 | ENDIF | `end` |
   | 8 | CHOICEBOX | `wm.choice_box({...})` |
   | 9 | IFCHOICE | `if choice == N then` |
   | 10 | SETPLAYERSUSPICION | `wm.player.set_suspicion(delta)` |
   | 11 | SETPLAYERDISPOSITION | `wm.player.set_disposition(delta)` |
   | 12 | SETGLOBAL | `wm.global.set_flag(id, true)` |
   | 13 | CLEARGLOBALFLAG | `wm.global.set_flag(id, false)` |
   | 14 | ADDTARGETGIRL | `wm.girl.add_target()` |
   | 15 | ADJUSTTARGETGIRLSTAT | `wm.girl.set_stat(name, delta)` |
   | 16 | SETGIRLFLAG | `wm.girl.set_flag(id, value)` |
   | 17 | IFGIRLFLAG | `if wm.girl.get_flag(id) then` |
   | 18 | IFGIRLSTAT | `if wm.girl.get_stat(name) op value then` |
   | 19 | IFGIRLSKILL | `if wm.girl.get_skill(name) op value then` |
   | 20 | IFHASTRAIT | `if wm.girl.has_trait(name) then` |
   | 21 | ADDCUSTTODUNGEON | `wm.dungeon.add_customer(reason)` |
   | 22 | ADDRANDOMGIRLTODUNGEON | `wm.dungeon.add_random_girl()` |
   | 23 | ADDMANYRANDOMGIRLS | `wm.dungeon.add_random_girls(count)` |
   | 24 | NORMALSEX | `wm.girl.sex("normal")` |
   | 25 | BEASTSEX | `wm.girl.sex("beast")` |
   | 26 | ANALSEX | `wm.girl.sex("anal")` |
   | 27 | BDSMSEX | `wm.girl.sex("bdsm")` |
   | 28 | PLAYERRAPETARGETGIRL | `wm.girl.rape()` |
   | 29 | TORTURETARGET | `wm.girl.torture()` |
   | 30 | SCOLDTARGET | `wm.girl.scold()` |
   | 31 | GAMEOVER | `wm.game_over()` |
   | 32 | GIVEPLAYERRANDOMSPECIALITEM | `wm.give_random_special_item()` |
   | 33 | IFPASSSKILLCHECK | `if wm.girl.skill_check(name, threshold) then` |
   | 34 | IFPASSSTATCHECK | `if wm.girl.stat_check(name, threshold) then` |
   | 35 | IFNOTDISOBEY | `if not wm.girl.disobeys() then` |
   | 36 | RESTART | `-- restart script` |
   | 37 | SETVARARANDOM | `vars[N] = math.random(min, max)` |
   | 38 | ACTIVATECHOICE | `wm.activate_choice(id)` |

   - Metadata for parameter types is in `../WhoreMasterRenewal/Docs&Tools/ScriptEditor/Data/ScriptCommands.txt`
   - Convert all 9 .script files, output to `resources/scripts/converted/`

2. **Implement `lua_engine.rs`**:
   - Create sandboxed `mlua::Lua` instance
   - Register `wm` table with all API functions from the opcode table
   - Each API function takes `&mut GameState` (via Lua UserData) and modifies game state
   - `run_script(path)` — load and execute a .lua file
   - `run_string(code)` — execute inline Lua

3. **Implement `api.rs`** — Each `wm.*` function:
   - `wm.message(text, color)` → push to event/message queue for UI display
   - `wm.choice_box(choices)` → block and return selected index (needs UI integration point)
   - `wm.girl.*` → operate on the "target girl" stored in script context
   - `wm.player.*` → modify player stats
   - `wm.global.*` → read/write global flags
   - `wm.dungeon.*` → add to dungeon
   - `wm.game_over()` → set game-over flag

4. **Implement `triggers.rs`**:
   - `TriggerSystem::load(path)` — parse `GlobalTriggers.xml`
   - Each trigger: type, condition params, script path
   - `evaluate(game_state)` → returns list of scripts to fire
   - 11 trigger types: Random (% chance per week), Shopping, Skill (girl stat threshold), Stat, Status, Money (gold threshold), Meet (location type), Talk (context), WeeksPast (week counter), GlobalFlag, Kidnapped
   - Reference: `cTriggers.h/.cpp`, `Resources/Scripts/GlobalTriggers.xml`

**Verification:**
- `script_converter` produces valid .lua for all 9 .script files
- Lua engine executes `Intro.lua` without errors
- `wm.message()` calls accumulate in message queue
- Trigger system loads GlobalTriggers.xml, evaluates conditions against game state
- `cargo test -p wm-script` passes

---

### Phase 5: All Game Screens (`wm-ui` screen module)

**Goal:** Every screen functional with real game data, connected to `wm-game` state.

**Tasks:** Implement each screen's `init()`, `process()`, and `on_event()`:

1. **`main_menu.rs`** — New Game, Load Game, Quit buttons. New Game → create GameState → push Town screen.

2. **`bank.rs`** — Display cash + bank balance. Deposit/Withdraw buttons → prompt for amount → update Gold. Reference: `cScreenBank.cpp`.

3. **`town.rs`** — Navigation hub. Buttons for each building (Brothel, Slave Market, Dungeon, Bank, House, Prison, Mayor). Display current brothel name. "Walk Around" button (chance to meet girl). "Next Week" button → trigger TurnProcessor. Reference: `cScreenTown.cpp`, `town_screen.xml`.

4. **`building_management.rs`** — Girl list (ListBox) for current brothel. Show name, job, stats in columns. Click girl → select. Buttons: Job assignment, Girl Details, Fire, Transfer. Reference: `cScreenBuildingManagement.cpp`, `girl_management_screen.xml`.

5. **`building_setup.rs`** — Manage rooms/facilities. Buy/sell rooms. Display space used/total. Reference: `cScreenBuildingSetup.cpp`, `building_setup_screen.xml`.

6. **`girl_details.rs`** — Most complex screen. Display all 22 stats, 10 skills as bars/numbers. Trait list. Inventory. Job assignment (day/night). House percentage slider. Reference: `cScreenGirlDetails.cpp`, `girl_details_screen.xml`.

7. **`slave_market.rs`** — List of available girls for purchase. Preview stats on selection. Buy button → deduct gold, add girl. Reference: `cScreenSlaveMarket.cpp`, `slavemarket_screen.xml`.

8. **`dungeon.rs`** — Prisoner list (girls + customers). Buttons: Torture, Release, Enslave (if girl). Reference: `cScreenDungeon.cpp`, `dungeon_screen.xml`.

9. **`gangs.rs`** — Hired gangs list + recruitable list. Recruit, assign mission, manage. Reference: `cScreenGangs.cpp`, `gangs_screen.xml`.

10. **`house.rs`** — Player house management. Reference: `cScreenHouse.cpp`, `house_screen.xml`.

11. **`prison.rs`** — Similar to dungeon but for convicted characters. Reference: `cScreenPrison.cpp`, `prison_screen.xml`.

12. **`mayor.rs`** — Political interaction. Reference: `cScreenMayor.cpp`, `mayor_screen.xml`.

13. **`item_management.rs`** — Full inventory screen. Item list, girl list, equip/unequip/use. Reference: `cScreenItemManagement.cpp`, `itemmanagement_screen.xml`.

14. **`turn_summary.rs`** — Display weekly results. Color-coded event list (red = critical, blue = warning). Gold summary. This is the screen players see most. Reference: `InterfaceProcesses.cpp` (turn summary logic).

15. **`load_save.rs`** — Save: serialize GameState via serde_json. Load: deserialize from file. List available .json save files.

**Verification:**
- Navigate Main Menu → New Game → Town → each building screen → Back
- Assign girl to job, run Next Week, see Turn Summary with events and gold changes
- Buy girl from Slave Market, verify she appears in brothel management
- Save game, quit, load game, verify state matches

---

### Phase 6: WMEdit Standalone (`wm-edit`)

**Goal:** Desktop editor for game data files.

**Tasks:**

1. **`app.rs`** — eframe App with tab bar: Girls | Items | Traits
2. **`girls_tab.rs`** — Load `Girls.girlsx`, display girl list, edit stats/skills/traits, save
3. **`items_tab.rs`** — Load `Items.itemsx`, display item list, edit effects/cost/rarity, save
4. **`traits_tab.rs`** — Load `CoreTraits.traits`, display trait list, edit descriptions

All editors use `wm-core` XML serializers (add `Serialize` derives + write functions). File picker for load/save paths.

**Verification:** Load each data file, make an edit, save, reload, verify edit persisted.

---

## General Guidelines

1. **Read the C++ reference** before implementing each module. Don't guess at game formulas — the C++ code has the exact math.
2. **Write tests as you go.** Every module should have `#[cfg(test)]` tests.
3. **Use idiomatic Rust.** No unsafe blocks. Prefer `Result` over panics. Use iterators. Pattern match on enums.
4. **Keep modules focused.** If a function grows beyond ~100 lines, decompose it.
5. **Respect the architect's types.** The struct/enum/trait definitions in the stubs are the contract. If you must change a type, document why.
6. **Run `cargo clippy` frequently.** Fix all warnings before moving to the next module.
7. **Resource paths:** Use `wm_core::resources_path()` to resolve game data file paths. Never hardcode `../../WhoreMasterRenewal/Resources/`. In test code, set the `WM_RESOURCES_PATH` env var or rely on the `resources/` symlink in the workspace root. Example:
   ```rust
   let items_path = wm_core::resources_path().join("Data/Items.itemsx");
   ```
8. **The `wm-app` binary** is the game's entry point (`crates/wm-app/src/main.rs`). It initializes SDL2, creates `GameState`, and runs the game loop. When adding new screens or systems, wire them into the `wm-app` main loop — not via standalone test binaries.
