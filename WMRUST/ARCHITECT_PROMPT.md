# Architect Agent Prompt — WhoreMaster Renewal Rust Recreation

## Your Role

You are the **architect** for the Rust recreation of WhoreMaster Renewal, a turn-based business management game. Your job is to lay the foundational groundwork: create the cargo workspace, define all crate boundaries, write all type definitions, traits, and module stubs, and set up the SDL2 rendering scaffold. You do NOT implement game logic or full screen behavior — the builder agent handles that. You create the skeleton that the builder fills in.

## Context

The original game is a C++ codebase at `../WhoreMasterRenewal/` with:
- **142 source files** in `src/` (C++11, SDL 1.2, Lua 5.2, TinyXML)
- **Game data** in `Resources/Data/` (XML: items, rooms, config, traits)
- **Character data** in `Resources/Characters/` (XML: girls with 22 stats, 10 skills, traits)
- **UI definitions** in `Resources/Interface/` (10 XML screen layouts + 3 legacy TXT files)
- **Scripts** in `Resources/Scripts/` (Lua files + 9 binary .script files + GlobalTriggers.xml)
- **Image assets** in `Resources/Buttons/` (300+ PNGs) and `Resources/Images/` (40+ backgrounds)

See `PROJECT_PLAN.md` in this directory for the complete plan, technology choices, and folder structure.

## What You Must Create

### 1. Cargo Workspace (`WMRUST/Cargo.toml`)

Create a workspace with 6 crates:
```toml
[workspace]
members = [
    "crates/wm-app",
    "crates/wm-core",
    "crates/wm-game",
    "crates/wm-ui",
    "crates/wm-script",
    "crates/wm-edit",
]
resolver = "2"
```

### 2. `wm-core` Crate — Shared Data Model

This crate has **zero rendering dependencies**. It defines all game data types and XML deserializers.

**Dependencies:** `serde`, `serde_derive`, `quick-xml` (with `serialize` feature), `thiserror`

> **CRITICAL: `quick-xml` serde attribute syntax.** When deserializing XML **attributes** (as opposed to child elements), `quick-xml`'s serde integration requires the `@` prefix on the rename string:
> ```rust
> #[derive(Deserialize)]
> struct Item {
>     #[serde(rename = "@Name")]   // <-- @ prefix for XML attributes
>     name: String,
>     #[serde(rename = "@Cost")]   // <-- @ prefix for XML attributes
>     cost: i32,
>     #[serde(rename = "Effect", default)]  // NO @ for child elements
>     effects: Vec<Effect>,
> }
> ```
> Without the `@` prefix, attribute deserialization silently fails (fields get default values). This applies to **every** XML struct in the project. Apply this pattern consistently.

**Files to create with full implementations:**

#### `crates/wm-core/src/enums.rs`
Define these enums with `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]`:

```rust
pub enum Stat {
    Charisma, Happiness, Libido, Constitution, Intelligence, Confidence,
    Mana, Agility, Fame, Level, AskPrice, HousePerc, Exp, Age,
    Obedience, Spirit, Beauty, Tiredness, Health, PCFear, PCLove, PCHate,
}
// Stat::COUNT = 22, used for fixed-size arrays

pub enum Skill {
    Anal, BDSM, NormalSex, Beastiality, Group, Lesbian,
    Service, Strip, Combat, Magic,
}
// Skill::COUNT = 10

pub enum JobType {
    Whore, Stripper, Bartender, Masseuse, Fluffer, Security,
    Matron, Torturer, BeastCarer, BeastCapturer, ExploreCatacombs,
    Advertising, Cleaning, Hall, FreetimeRest, FreetimeTown,
    // ... extend as needed from Work*.cpp files
}

pub enum ItemType {
    Food, Ring, Necklace, Dress, Underwear, Shoes, Hat,
    Helmet, SmallWeapon, LargeWeapon, Armor, Shield,
    Consumable, Makeup, Misc,
}

pub enum EffectTarget { Stat, Skill, Trait }

pub enum Rarity {
    Common,     // Shop always
    Shop50,     // Shop 50% chance
    Shop25,     // Shop 25%
    Shop05,     // Shop 5%
    Catacomb15, // Catacombs 15%
    ScriptOnly, // Only via scripts
    Reward,     // Objectives/scripts
}

pub enum TriggerType {
    Random, Shopping, Skill, Stat, Status, Money,
    Meet, Talk, WeeksPast, GlobalFlag, Kidnapped,
}

pub enum Shift { Day, Night }
```

#### `crates/wm-core/src/girl.rs`
```rust
pub struct Girl {
    pub name: String,
    pub desc: String,
    pub stats: [i32; Stat::COUNT],
    pub skills: [i32; Skill::COUNT],
    pub traits: Vec<String>,
    pub status: GirlStatus,
    pub job_day: Option<JobType>,
    pub job_night: Option<JobType>,
    pub flags: [bool; 30],       // 30 per-girl flags (from C++ sGirl)
    pub weeks_pregnant: i32,
    pub inventory: Vec<usize>,   // Item IDs
}

pub struct GirlStatus { /* pregnant, slave, controlled, poisoned, etc. */ }
```

Reference `Girl.hpp` and `cGirls.h` for the complete stat/skill/status model.

#### `crates/wm-core/src/item.rs`
```rust
pub struct Item {
    pub name: String,
    pub desc: String,
    pub item_type: ItemType,
    pub badness: i32,
    pub special: String,
    pub cost: i32,
    pub rarity: Rarity,
    pub infinite: bool,
    pub girl_buy_chance: i32,
    pub effects: Vec<Effect>,
}

pub struct Effect {
    pub target: EffectTarget,
    pub name: String,
    pub amount: i32,
}
```

XML format to parse (from `Items.itemsx`):
```xml
<Items>
  <Item Name="AIDS Cure" Desc="..." Type="Food" Badness="0" Special="None"
        Cost="3500" Rarity="Shop05" Infinite="false" GirlBuyChance="0">
    <Effect What="Skill" Name="Anal" Amount="-20" />
    <Effect What="Stat" Name="Libido" Amount="-75" />
    <Effect What="Trait" Name="AIDS" Amount="0" />
  </Item>
</Items>
```

#### `crates/wm-core/src/room.rs`
```rust
pub struct Room {
    pub name: String,
    pub desc: String,
    pub space: i32,
    pub provides: i32,
    pub price: i32,
    pub glitz: i32,
    pub min_glitz: i32,
    pub max_glitz: i32,
    pub functions: Vec<RoomFunction>,
}

pub struct RoomFunction {
    pub name: String,
    pub factor: Option<f32>,
    pub success: Option<f32>,
}
```

XML format (from `Rooms.roomsx`):
```xml
<Facilities>
  <Facility Name="Bedroom" Desc="..." Space="4" Provides="4" Price="100"
            MinGlitz="0" MaxGlitz="5" Glitz="0">
    <Function Name="Rest" />
    <Function Name="Whoring" />
  </Facility>
</Facilities>
```

#### `crates/wm-core/src/config.rs`
Define `GameConfig` struct matching all 12 sections of `config.xml`: Initial, Income, Expenses, Gambling, Tax, Pregnancy, Gangs, Prostitution, Items, Fonts, Debug. Reference the full config.xml in `Resources/Data/config.xml`.

#### `crates/wm-core/src/gold.rs`
```rust
pub struct Gold {
    pub cash_on_hand: i32,
    pub bank_balance: i32,
    pub income: GoldCategory,
    pub expenses: GoldCategory,
}

pub struct GoldCategory {
    pub brothel: i32,
    pub street: i32,
    pub extortion: i32,
    pub movie: i32,
    pub stripper: i32,
    pub barmaid: i32,
    pub slave_sales: i32,
    pub item_sales: i32,
    pub gambling: i32,
    // ... mirror cGold.h categories
}
```

#### `crates/wm-core/src/traits.rs`
Parse `CoreTraits.traits` — plain text, alternating lines: name, description.
```rust
pub struct TraitDef {
    pub name: String,
    pub description: String,
}
```

#### XML deserializers in `crates/wm-core/src/xml/`
Implement complete `quick-xml` + `serde` deserializers for each data file format. Each must:
- Handle the exact XML attribute names from the original files
- Map to the Rust structs defined above
- Include unit tests that load the real data files from `../../WhoreMasterRenewal/Resources/`

### 3. `wm-game` Crate — Game Logic Stubs

**Dependencies:** `wm-core`, `rand`, `thiserror`

Create module files with struct definitions and trait signatures. Do NOT implement game logic — just the public API surface.

#### `crates/wm-game/src/state.rs`
```rust
pub struct GameState {
    pub config: GameConfig,
    pub player: Player,
    pub gold: Gold,
    pub brothels: BrothelManager,
    pub girls: GirlManager,
    pub gangs: GangManager,
    pub customers: CustomerGenerator,
    pub dungeon: DungeonManager,
    pub rivals: RivalManager,
    pub global_flags: [bool; 5],
    pub week: u32,
}
```

#### `crates/wm-game/src/jobs/mod.rs`
```rust
pub trait Job {
    fn job_type(&self) -> JobType;
    fn process(&self, girl: &mut Girl, brothel: &Brothel, rng: &mut impl Rng) -> JobResult;
}

pub struct JobResult {
    pub gold_earned: i32,
    pub events: Vec<String>,
    pub stat_changes: Vec<(Stat, i32)>,
    pub skill_changes: Vec<(Skill, i32)>,
}

pub struct JobDispatcher { /* HashMap<JobType, Box<dyn Job>> */ }
```

Create stub files for each job in `jobs/` — empty struct implementing Job with `todo!()` body.

#### Other managers
Create stub structs with documented public method signatures for: `BrothelManager`, `GirlManager`, `GangManager`, `CustomerGenerator`, `DungeonManager`, `RivalManager`, `TurnProcessor`. Reference the corresponding C++ headers for method signatures.

### 4. `wm-ui` Crate — Rendering & Widget Stubs

**Dependencies:** `wm-core`, `sdl2` (with features: `bundled`, `image`, `ttf`, `gfx`), `thiserror`

> **Why `bundled`?** On Windows, the `bundled` feature compiles SDL2 from C source automatically, eliminating the need to manually install SDL2 development libraries and set `LIB`/`PATH`. Requires a C compiler (MSVC) and CMake.

#### `crates/wm-ui/src/graphics.rs`
**Fully implement** the SDL2 initialization and game loop scaffold:
```rust
pub struct Graphics {
    pub sdl_context: Sdl,
    pub canvas: WindowCanvas,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl Graphics {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self>;
    pub fn begin_frame(&mut self);      // Clear to black
    pub fn end_frame(&mut self);        // canvas.present()
    pub fn load_texture(&self, path: &str) -> Result<Texture>;
}
```
Reference `CGraphics.h/.cpp` — original uses 800x600, 25 FPS, SDL_Flip.

> **CRITICAL: SDL2 Texture Lifetimes.** In `rust-sdl2`, a `Texture` borrows from `TextureCreator`, which borrows from `Canvas`. This makes storing textures in structs difficult. Two recommended approaches:
>
> 1. **`unsafe_textures` feature** — add `features = ["unsafe_textures"]` to the `sdl2` dependency. This makes `Texture: 'static` by requiring manual `destroy()` calls. Simpler code, but you must ensure textures are destroyed before the `TextureCreator`.
>
> 2. **Single-owner `TextureManager`** — keep `TextureCreator` and all `Texture` objects in the same struct. Load textures through the manager, never hand out ownership.
>
> **Recommendation:** Use approach #1 (`unsafe_textures` feature) for simplicity. Add it to the sdl2 dependency:
> ```toml
> sdl2 = { version = "0.37", features = ["bundled", "image", "ttf", "gfx", "unsafe_textures"] }
> ```
> Then implement a `TextureCache` that loads and caches textures by path, and `Drop`s them before the `TextureCreator` is dropped.

#### `crates/wm-ui/src/widget/mod.rs`
Define the widget type system:
```rust
pub type WidgetId = u32;

pub enum Widget {
    Button(ButtonWidget),
    TextItem(TextItemWidget),
    ListBox(ListBoxWidget),
    EditBox(EditBoxWidget),
    CheckBox(CheckBoxWidget),
    Slider(SliderWidget),
    ScrollBar(ScrollBarWidget),
    ImageItem(ImageItemWidget),
}

pub struct WidgetStore {
    widgets: HashMap<WidgetId, Widget>,
    next_id: WidgetId,
}
```

Create stub files for each widget type with struct definitions and empty `draw()` / `handle_click()` methods.

#### `crates/wm-ui/src/screen/mod.rs`
```rust
pub enum ScreenAction {
    None,
    Push(Box<dyn Screen>),
    Pop,
    PopTo(ScreenId),
    Quit,
}

pub trait Screen {
    fn init(&mut self, widgets: &mut WidgetStore, game: &GameState);
    fn process(&mut self, widgets: &mut WidgetStore, game: &mut GameState) -> ScreenAction;
    fn on_event(&mut self, event: UiEvent, widgets: &mut WidgetStore, game: &mut GameState) -> ScreenAction;
}

pub struct ScreenManager {
    stack: Vec<Box<dyn Screen>>,
}
```

Create stub screen files for all 12+ screens.

#### `crates/wm-ui/src/xml_loader.rs`
Parse `Resources/Interface/*.xml` into `WidgetStore`. XML format:
```xml
<Screen>
  <Window Name="..." XPos="..." YPos="..." Width="..." Height="..." Border="..." />
  <Text Name="..." Text="..." XPos="..." YPos="..." Width="..." Height="..." FontSize="..." />
  <Button Name="..." Image="..." XPos="..." YPos="..." Width="..." Height="..." Transparency="..." Scale="..." />
  <Image Name="..." File="..." XPos="..." YPos="..." Width="..." Height="..." />
</Screen>
```

### 5. `wm-script` Crate — Stubs Only

**Dependencies:** `wm-core`, `mlua`, `quick-xml`, `thiserror`

Create module stubs for:
- `lua_engine.rs` — `LuaEngine` struct wrapping `mlua::Lua`
- `api.rs` — Stub functions for each `wm.*` API call
- `triggers.rs` — `TriggerSystem` struct with `load()` and `evaluate()` stubs
- `script_converter.rs` — `convert_script_to_lua(path: &Path) -> Result<String>` stub

### 6. `wm-edit` Crate — Stub Only

**Dependencies:** `wm-core`, `eframe`, `egui`

Create a minimal `main.rs` and `app.rs` that opens an egui window with placeholder tabs.

### 7. `wm-app` Crate — Main Game Binary

**Dependencies:** `wm-core`, `wm-game`, `wm-ui`, `wm-script`, `anyhow`, `tracing`, `tracing-subscriber`

This is the game's entry point — a **binary** crate (not a library).

#### `crates/wm-app/Cargo.toml`
```toml
[package]
name = "wm-app"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "whoremaster"
path = "src/main.rs"

[dependencies]
wm-core = { path = "../wm-core" }
wm-game = { path = "../wm-game" }
wm-ui = { path = "../wm-ui" }
wm-script = { path = "../wm-script" }
anyhow = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

#### `crates/wm-app/src/main.rs`
Create a minimal entry point that:
1. Initializes `tracing_subscriber`
2. Loads resources via `wm_core::resources_path()`
3. Creates `Graphics` (800x600 window)
4. Creates `GameState`
5. Runs a game loop: poll events → process current screen → render → cap to 25 FPS
6. Handles `ScreenAction::Quit` to exit

Use `todo!()` for anything that depends on the builder's implementations, but the binary must **compile and run** (opening a black window with a title).

#### Resource Path Helper
In `wm-core/src/lib.rs`, create a `resources_path()` function:
```rust
/// Base path to game resources. Override with WM_RESOURCES_PATH env var.
pub fn resources_path() -> std::path::PathBuf {
    std::env::var("WM_RESOURCES_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("resources"))
}
```
All crates must use this function (not hardcoded paths) when loading game data.

## What You Must NOT Do

- Do NOT implement game logic (job processing, turn simulation, combat, etc.)
- Do NOT implement full widget rendering (just struct definitions and method signatures)
- Do NOT implement screen behavior (just Screen trait impls with `todo!()`)
- Do NOT copy C++ code — write idiomatic Rust from scratch, using C++ only as reference

## Quality Requirements

1. `cargo build` must succeed for the entire workspace
2. `cargo clippy` must pass with zero warnings
3. All public types must have `#[derive(Debug, Clone)]` at minimum
4. All enums must have `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`
5. All serde-derived structs must round-trip correctly
6. XML deserializers in `wm-core` must include `#[cfg(test)]` modules that load real data files
7. The SDL2 game loop in `wm-app/src/main.rs` (not wm-ui) must actually open a window and render a black screen
8. Use `thiserror` for library crate errors, `anyhow` only in binary crates (`wm-app`, `wm-edit`)

## Key C++ References

When you need to understand the original's data model or API surface, consult:

| What | File |
|------|------|
| Stats/Skills enum values | `../WhoreMasterRenewal/src/Girl.hpp` |
| Full stat list (22) | `../WhoreMasterRenewal/src/Constants.h` |
| Gold categories | `../WhoreMasterRenewal/src/cGold.h` |
| Config structure | `../WhoreMasterRenewal/src/sConfig.h` |
| Job list | `../WhoreMasterRenewal/src/cJobManager.h` |
| Widget types | `../WhoreMasterRenewal/src/cButton.h`, `cListBox.h`, `cSlider.h`, etc. |
| Screen lifecycle | `../WhoreMasterRenewal/src/InterfaceWindowXML.hpp` |
| Screen examples | `../WhoreMasterRenewal/src/cScreenBank.cpp` |
| Window manager | `../WhoreMasterRenewal/src/cWindowManager.h` |
| Game loop | `../WhoreMasterRenewal/src/main.cpp` |
| Trigger types | `../WhoreMasterRenewal/src/cTriggers.h` |
| Script opcodes | `../WhoreMasterRenewal/Docs&Tools/ScriptEditor/Data/ScriptCommands.txt` |
| Items XML schema | `../WhoreMasterRenewal/Resources/Data/Items.itemsx` |
| Config XML | `../WhoreMasterRenewal/Resources/Data/config.xml` |
| Rooms XML | `../WhoreMasterRenewal/Resources/Data/Rooms.roomsx` |
| Screen XML | `../WhoreMasterRenewal/Resources/Interface/bank_screen.xml` (simplest example) |
| Traits format | `../WhoreMasterRenewal/Resources/Data/CoreTraits.traits` |
