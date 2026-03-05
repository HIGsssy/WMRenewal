# Lua Modding Guide — WhoreMaster Renewal

This document covers the Lua scripting API available for modding WhoreMaster Renewal. Scripts are loaded from the `resources/Scripts/` directory and can be triggered by the game's trigger system or executed directly by character events.

## Table of Contents

1. [Script Locations](#script-locations)
2. [The `wm` API Reference](#the-wm-api-reference)
3. [Girl Stats & Skills](#girl-stats--skills)
4. [Trigger System](#trigger-system)
5. [Writing a Custom Script](#writing-a-custom-script)
6. [Converting Legacy .script Files](#converting-legacy-script-files)

---

## Script Locations

| Path | Purpose |
|------|---------|
| `resources/Scripts/*.lua` | Global game scripts (intro, events, interactions) |
| `resources/Scripts/GlobalTriggers.xml` | Trigger definitions that fire scripts conditionally |
| `resources/Characters/<Name>/triggers.xml` | Per-character trigger definitions |
| `resources/Characters/<Name>/*.lua` | Per-character interaction scripts |

Scripts are standard **Lua 5.4** files. The game provides a sandboxed environment with the `wm` global table pre-registered.

---

## The `wm` API Reference

### Messages & UI

| Function | Description |
|----------|-------------|
| `wm.message(text, color)` | Display a message to the player. `color` is an integer (0 = default). |
| `wm.choice_box(id, options)` | Present a choice dialog. `id` is an integer key, `options` is a table of strings. |
| `wm.get_choice(id)` | Returns the player's selected option index (0-based) for the given choice box `id`. |
| `wm.activate_choice(id)` | Show the choice box UI for the given `id` (call after `wm.choice_box`). |

### Economy

| Function | Description |
|----------|-------------|
| `wm.add_gold(amount)` | Add or remove gold. Use negative values to deduct. |
| `wm.give_random_special_item()` | Give the player a random special item. |

### Player

| Function | Description |
|----------|-------------|
| `wm.player.set_suspicion(delta)` | Adjust player suspicion by `delta` (can be negative). |
| `wm.player.set_disposition(delta)` | Adjust player disposition by `delta`. |

### Global Flags

The game has 5 global flag slots (indices 0–4) for tracking world state.

| Function | Description |
|----------|-------------|
| `wm.global.set_flag(id, value)` | Set global flag `id` to `value` (boolean or integer — nonzero = true). |
| `wm.global.get_flag(id)` | Returns `true` or `false` for the given flag index. |

### Girl — Stats

The target girl is set by the game before running a script. All `wm.girl.*` functions operate on her.

| Function | Description |
|----------|-------------|
| `wm.girl.get_stat(name)` | Get the target girl's stat value (0–100). |
| `wm.girl.set_stat(name, delta)` | Adjust stat by `delta` (clamped to 0–100). |
| `wm.girl.pass_stat_check(name)` | Random check: returns `true` if `rand(0..100) < stat_value`. |

**Available stat names (22 total):**

| Index | Name | Index | Name |
|-------|------|-------|------|
| 0 | Charisma | 11 | HousePerc |
| 1 | Happiness | 12 | Exp |
| 2 | Libido | 13 | Age |
| 3 | Constitution | 14 | Obedience |
| 4 | Intelligence | 15 | Spirit |
| 5 | Confidence | 16 | Beauty |
| 6 | Mana | 17 | Tiredness |
| 7 | Agility | 18 | Health |
| 8 | Fame | 19 | PCFear |
| 9 | Level | 20 | PCLove |
| 10 | AskPrice | 21 | PCHate |

### Girl — Skills

| Function | Description |
|----------|-------------|
| `wm.girl.get_skill(name)` | Get the target girl's skill value (0–100). |
| `wm.girl.set_skill(name, delta)` | Adjust skill by `delta` (clamped to 0–100). |
| `wm.girl.pass_skill_check(name)` | Random check: returns `true` if `rand(0..100) < skill_value`. |

**Available skill names (10 total):**

| Index | Name | Index | Name |
|-------|------|-------|------|
| 0 | Anal | 5 | Group |
| 1 | Magic | 6 | Lesbian |
| 2 | BDSM | 7 | Service |
| 3 | NormalSex | 8 | Strip |
| 4 | Beastiality | 9 | Combat |

### Girl — Traits

| Function | Description |
|----------|-------------|
| `wm.girl.has_trait(name)` | Returns `true` if the girl has the named trait. |
| `wm.girl.add_trait(name)` | Add a trait to the girl. |
| `wm.girl.remove_trait(name)` | Remove a trait from the girl. |

Trait names are free-form strings matching definitions in `CoreTraits.traits` (e.g., `"Aggressive"`, `"Nymphomaniac"`, `"Fragile"`).

### Girl — Flags

Each girl has 30 flag slots (indices 0–29) for per-character state tracking.

| Function | Description |
|----------|-------------|
| `wm.girl.get_flag(id)` | Get flag value (integer) at index `id`. |
| `wm.girl.set_flag(id, value)` | Set flag at index `id` to `value`. |

### Girl — Actions

| Function | Description |
|----------|-------------|
| `wm.girl.add_to_brothel()` | Add the target girl to the player's active brothel. |
| `wm.girl.player_rape()` | Trigger a player rape event. |
| `wm.girl.torture()` | Trigger a torture event. |
| `wm.girl.scold()` | Trigger a scold event. |
| `wm.girl.normal_sex()` | Trigger a normal sex event. |
| `wm.girl.beast_sex()` | Trigger a beast sex event. |
| `wm.girl.anal_sex()` | Trigger an anal sex event. |
| `wm.girl.bdsm_sex()` | Trigger a BDSM sex event. |
| `wm.girl.disobey_check()` | Returns `true` if the girl disobeys (random check vs. Obedience stat). |

### Dungeon

| Function | Description |
|----------|-------------|
| `wm.dungeon.add_customer(reason, num, wife)` | Add a customer to the dungeon. `reason` = string, `num`/`wife` = integers. |
| `wm.dungeon.add_random_girl(reason, min_age, max_age, slave, non_human)` | Add 1 random girl to dungeon. `slave`/`non_human`: 0 or 1. |
| `wm.dungeon.add_random_girls(count, reason, min_age, max_age, slave, non_human)` | Add `count` random girls to dungeon. |

### Game State

| Function | Description |
|----------|-------------|
| `wm.game_over()` | End the game (player loses). |

---

## Trigger System

Triggers are defined in XML files and cause scripts to execute when conditions are met.

### GlobalTriggers.xml Format

```xml
<Triggers>
  <Trigger Script="MyEvent.lua" Type="Random" Chance="5" Once="1" />
  <Trigger Script="RichEvent.lua" Type="PlayerMoney" Chance="100" Once="0"
           Value0="10000" Value1="0" />
  <Trigger Script="SkillEvent.lua" Type="Skill" Chance="50" Once="1"
           Value0="3" Value1="75" />
</Triggers>
```

### Trigger Types

| Type | Enum | Condition | Value0 | Value1 |
|------|------|-----------|--------|--------|
| `Random` | 0 | Fires randomly each week | — | — |
| `Shopping` | 1 | Player is shopping | — | — |
| `Skill` | 2 | Girl has skill ≥ threshold | Skill index | Threshold |
| `Stat` | 3 | Girl has stat ≥ threshold | Stat index | Threshold |
| `Status` | 4 | Girl has specific status flags | Status bitfield | — |
| `Money` | 5 | Girl has gold ≥ threshold | Threshold | — |
| `Meet` | 6 | Meeting a girl in town | — | — |
| `Talk` | 7 | Talking to a girl | — | — |
| `WeeksPast` | 8 | Weeks since game start ≥ N | Week count | — |
| `GlobalFlag` | 9 | Global flag is set | Flag index | — |
| `ScriptRun` | 10 | Another script has already run | Script index | — |
| `Kidnapped` | 11 | Girl was kidnapped | — | — |
| `PlayerMoney` | 12 | Player gold ≥ threshold | Threshold | — |

### Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `Script` | string | Lua filename to execute (relative to Scripts/) |
| `Type` | string | Trigger type name (see table above) |
| `Chance` | 0–100 | Percentage chance the trigger fires when conditions are met |
| `Once` | 0/1 | If 1, trigger fires only once per game |
| `Value0` | int | First parameter (meaning depends on Type) |
| `Value1` | int | Second parameter (meaning depends on Type) |

---

## Writing a Custom Script

### Basic Example: Town Encounter

Create `resources/Scripts/MyEncounter.lua`:

```lua
-- A simple town encounter
wm.message("You spot a mysterious figure in the alley.", 0)

wm.choice_box(1, {"Approach them", "Ignore and walk away"})
wm.activate_choice(1)

local choice = wm.get_choice(1)
if choice == 0 then
    wm.message("The figure offers you gold for your discretion.", 0)
    wm.add_gold(200)
    wm.player.set_suspicion(5)
elseif choice == 1 then
    wm.message("You walk away. Perhaps another day.", 0)
end
```

Then add a trigger in `GlobalTriggers.xml`:

```xml
<Trigger Script="MyEncounter.lua" Type="Random" Chance="10" Once="0" />
```

### Example: Character Meeting Script

Create `resources/Characters/My Character/MeetGirl.lua`:

```lua
-- First meeting with a custom character
wm.message("A young woman catches your eye at the market.", 0)

if wm.girl.get_stat("Beauty") > 70 then
    wm.message("She is strikingly beautiful.", 0)
end

wm.choice_box(1, {"Offer her a job", "Buy her freedom", "Walk away"})
wm.activate_choice(1)

local choice = wm.get_choice(1)
if choice == 0 then
    wm.girl.set_stat("Happiness", 10)
    wm.girl.set_stat("PCLove", 5)
    wm.girl.add_to_brothel()
    wm.message("She gratefully accepts your offer.", 0)
elseif choice == 1 then
    if wm.add_gold(-500) then
        wm.girl.add_to_brothel()
        wm.girl.set_stat("PCLove", 20)
        wm.message("You pay 500 gold and set her free. She is deeply grateful.", 0)
    end
else
    wm.message("You walk away.", 0)
end
```

### Example: Stat-Based Skill Check

```lua
-- Guard encounter during gang mission
wm.message("Your gang encounters a guard patrol!", 0)

if wm.girl.pass_skill_check("Combat") then
    wm.message("Your champion fighter drives them off.", 0)
    wm.girl.set_skill("Combat", 2)  -- small skill gain
else
    wm.message("The guards overwhelm your forces.", 0)
    wm.girl.set_stat("Health", -15)
    wm.player.set_suspicion(10)
end
```

---

## Converting Legacy .script Files

The game includes a built-in converter for the original binary `.script` format used by the C++ codebase. The converter is part of the `wm-script` crate:

- **Input:** Binary `.script` files (linked-list format with typed entries and opcodes)
- **Output:** Equivalent `.lua` files using the `wm.*` API
- **All 41 original opcodes** are mapped to their Lua equivalents

The following legacy scripts have been converted:

| Original | Converted | Purpose |
|----------|-----------|---------|
| `CustNoPay.script` | `CustNoPay.lua` | Customer refuses to pay |
| `CustGambCheat.script` | `CustGambCheat.lua` | Customer caught cheating at gambling |
| `RivalLose.script` | `RivalLose.lua` | Rival defeated |
| `NoMoney.script` | `NoMoney.lua` | Player out of money |
| `DefaultInteractDetails.script` | `DefaultInteractDetails.lua` | Default girl interaction (details) |
| `DefaultInteractDungeon.script` | `DefaultInteractDungeon.lua` | Default dungeon interaction |
| `MeetTownDefault.script` | `MeetTownDefault.lua` | Default town meeting |
| `TalkDetailsDefault.script` | `TalkDetailsDefault.lua` | Default talk interaction |
| `TalkDungeonDefault.script` | `TalkDungeonDefault.lua` | Default dungeon talk |

New modders should write Lua directly — the binary format is legacy-only.

---

## Tips for Modders

1. **Test scripts standalone:** You can test Lua syntax by running the file through a standard Lua 5.4 interpreter (the `wm.*` calls will error, but syntax errors will show).
2. **Use girl flags** (0–29) to track per-girl state across multiple script invocations.
3. **Use global flags** (0–4) sparingly — they're shared across the entire game.
4. **Stat/skill deltas** are additive. `wm.girl.set_stat("Health", -10)` reduces health by 10. Values are clamped to 0–100.
5. **Choice boxes** are synchronous in the script context — the game pre-sets choice results before script execution.
6. **Triggers re-evaluate** each week during turn processing. Use `Once="1"` to prevent repeated firing.
