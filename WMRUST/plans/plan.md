Plan: WhoreMaster Renewal — Full Recreation in Rust
Recreate the game from scratch in Rust using sdl2, quick-xml/serde, mlua, and a custom widget system. All existing XML data files, 300+ image assets, and Lua scripts reuse directly. The proprietary .script binary format gets reverse-engineered and converted to Lua. WMEdit becomes a standalone Rust + egui desktop app. No backward save compatibility needed.

Decisions (Confirmed)
#	Decision	Choice
1	Path	Full recreation
2	Language	Rust (plan accounts for no prior experience)
3	Save compat	No — fresh save format
4	.script files	Reverse-engineer — feasible: format is fully documented in source code + ScriptCommands.txt, only 9 small files, ~12hr effort
5	WMEdit	Standalone Rust + egui desktop app
6	Legal	Use existing assets during dev; clear before public release
Rust Stack
Need	Crate	Why
Rendering	sdl2 (rust-sdl2)	Closest to original SDL, mature 2D
Image/Font/GFX	sdl2::image, sdl2::ttf, sdl2::gfx	Match original capabilities
XML parsing	quick-xml + serde	Derives directly to Rust structs
Lua scripting	mlua	Most actively maintained; Lua 5.4
Serialization	serde + serde_json	Save/load, config
Editor UI	eframe + egui	Immediate-mode GUI for WMEdit
Error handling	anyhow + thiserror	Ergonomic errors
Phases
Phase 0 — Rust Setup & Scaffold (foundation)

Install toolchain, init cargo project, add SDL2 deps
Implement minimal game loop: open window, clear screen, poll events, render an image + text
Set up project module structure: graphics.rs, ui/, game/, data/, scripting/
Introduces: ownership, borrowing, Result, match, modules
Phase 1 — Data Model & XML Loading (parallel with Phase 2 after Phase 0)

Define enums: Stat (22), Skill (10), Trait, JobType (30+), ItemType
Define serde-derived structs: Girl, Item, Effect, Room, GameConfig, Gold
Write deserializers for all XML data file formats (.itemsx, .girlsx, .roomsx, .traits, config.xml)
Unit tests loading real data files
Reference: Girl.hpp, cGold.h, Items.itemsx, config.xml
Phase 2 — UI Widget System & Screen Framework (parallel with Phase 1)

8 widget types as enum variants: Button, TextItem, ListBox, EditBox, CheckBox, Slider, ScrollBar, ImageItem — reference cButton.h, cListBox.h, etc.
Screen trait with init(), process(), on_event() lifecycle
ScreenManager push/pop stack — reference cWindowManager.h
XML screen loader parsing Resources/Interface/*.xml
ListBox is the hardest widget (multi-column, sortable, multi-select) — budget extra time
Prove out with Main Menu + Bank screen
Phase 3 — Core Game Logic (depends on Phase 1)

Decompose the God Object from day one: GirlManager, BrothelManager, GangManager, CustomerGenerator, DungeonManager, RivalManager as separate structs
Job trait with process() → one struct per job type in src/game/jobs/ — reference 20+ Work*.cpp files
TurnProcessor orchestrating weekly cycle: jobs → gangs → rivals → customers → income → events
Gang combat — reference cGirlGangFight.h
Event/trigger system, pregnancy mechanics
Unit tests: simulate 10+ turns, assert stat deltas and gold balance
Phase 4 — Scripting: Lua + .script Conversion (depends on Phase 3)

Write binary parser for .script format using documented struct layout from cGameScript.h + opcode metadata from ScriptCommands.txt
Convert all 9 .script files → Lua
Embed Lua 5.4 via mlua, expose full wm.* API covering all 41 opcodes (the original Lua API only covered ~30% — extend it)
Implement TriggerSystem loading GlobalTriggers.xml with 11 trigger types
Phase 5 — All Game Screens (depends on Phase 2 + 3)

10+ screens: Town, Brothel Management, Girl Details, Slave Market, Dungeon, Gangs, Bank, House, Prison, Upgrade, Turn Summary, Main Menu, Load/Save
Girl Details is the most complex (stats, skills, traits, inventory, job assignment)
Turn Summary is the most gameplay-critical (color-coded event list)
Phase 6 — WMEdit Standalone (parallel after Phase 1)

Separate crate in cargo workspace, shares wm-data library crate
eframe + egui UI: Girls tab, Items tab, Traits viewer
Loads/saves all XML data formats
Phase 7 — Polish & Release Prep

Replace proprietary fonts, review character assets
Cross-platform CI (GitHub Actions), packaging
Modding documentation for Lua API
Verification
cargo build + cargo clippy — zero warnings
All XML data files deserialize correctly (unit tests)
All 10+ screens render with correct layout from XML definitions
10-turn simulation produces correct stat/gold/event outcomes (integration tests)
Lua scripts execute; triggers fire; all 9 converted scripts produce correct results
ListBox multi-select, sort, scroll all functional
Builds on Windows + Linux
WMEdit loads, edits, saves data files without loss
cargo test — full suite passes
Scope Estimate
Phase	~LOC	Complexity
0 — Scaffold	200	Low
1 — Data Model	1.5-2K	Low-Med
2 — UI Widgets	4-6K	High
3 — Game Logic	8-12K	High
4 — Scripting	2-3K	Medium
5 — All Screens	5-8K	Medium
6 — WMEdit	2-3K	Medium
7 — Polish	0.5-1K	Low
Total	~23-36K	
Further Considerations
Rust learning path: Complete chapters 1-10 of "The Rust Programming Language" (free, online) before starting Phase 0. The rustlings exercises are excellent for hands-on practice. This is a turn-based game with no async/threading — ideal for learning Rust.

Cargo workspace: Structure as a workspace with whoremaster-renewal (game), wmedit (editor), and wm-data (shared library) crates to keep WMEdit standalone while sharing data types.

egui as fallback for game UI: If the custom SDL2 widget system in Phase 2 proves too painful, egui (via eframe) could serve as both game and editor UI — it has buttons, lists, sliders natively. Trade-off: less pixel-perfect control, but dramatically faster to build. Worth keeping in mind as an escape hatch.