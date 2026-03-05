//! The wm.* Lua API — functions registered into the Lua environment.
//!
//! These operate on a shared `ScriptContext` that bridges the Lua VM
//! with the game state. The context is stored as Lua app data (userdata)
//! so callbacks can access it.

use std::sync::{Arc, Mutex};

use mlua::prelude::*;

/// A queued message to display to the player.
#[derive(Debug, Clone)]
pub struct ScriptMessage {
    pub text: String,
    pub color: i32,
}

/// Shared mutable context that Lua API callbacks read/write.
///
/// The game populates this before running a script (e.g. setting the
/// target girl's stats) and reads results afterward (messages, flag
/// changes, etc.).
#[derive(Debug, Default)]
pub struct ScriptContext {
    // ---- message queue ----
    pub messages: Vec<ScriptMessage>,

    // ---- choice box ----
    /// Pending choice results keyed by box id.
    pub choices: std::collections::HashMap<i32, i32>,
    /// Pending choice options for the current box.
    pub pending_choice_options: Vec<(i32, Vec<String>)>,

    // ---- girl target ----
    /// Stats array (22 entries) for the target girl.
    pub girl_stats: [i32; 22],
    /// Skills array (10 entries) for the target girl.
    pub girl_skills: [i32; 10],
    /// Girl flags (up to 30).
    pub girl_flags: [i32; 30],
    /// Girl traits (set of trait names).
    pub girl_traits: std::collections::HashSet<String>,

    // ---- global flags ----
    pub global_flags: [bool; 5],

    // ---- player ----
    pub suspicion_delta: i32,
    pub disposition_delta: i32,

    // ---- game state signals ----
    pub gold_delta: i64,
    pub game_over: bool,
    pub add_target_girl: bool,
    pub player_rape: bool,
    pub torture: bool,
    pub scold: bool,
    pub normal_sex: bool,
    pub beast_sex: bool,
    pub anal_sex: bool,
    pub bdsm_sex: bool,
    pub give_random_item: bool,

    // ---- dungeon operations ----
    pub dungeon_add_customers: Vec<DungeonCustomerOp>,
    pub dungeon_add_girls: Vec<DungeonGirlOp>,
}

/// Parameters for adding a customer to the dungeon.
#[derive(Debug, Clone)]
pub struct DungeonCustomerOp {
    pub reason: String,
    pub num: i32,
    pub wife: i32,
}

/// Parameters for adding a random girl to the dungeon.
#[derive(Debug, Clone)]
pub struct DungeonGirlOp {
    pub count: i32,
    pub reason: String,
    pub min_age: i32,
    pub max_age: i32,
    pub slave: bool,
    pub non_human: bool,
}

/// Type alias for the shared, thread-safe context.
pub type SharedContext = Arc<Mutex<ScriptContext>>;

impl ScriptContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset transient state before running a new script.
    pub fn reset_transient(&mut self) {
        self.messages.clear();
        self.choices.clear();
        self.pending_choice_options.clear();
        self.suspicion_delta = 0;
        self.disposition_delta = 0;
        self.gold_delta = 0;
        self.game_over = false;
        self.add_target_girl = false;
        self.player_rape = false;
        self.torture = false;
        self.scold = false;
        self.normal_sex = false;
        self.beast_sex = false;
        self.anal_sex = false;
        self.bdsm_sex = false;
        self.give_random_item = false;
        self.dungeon_add_customers.clear();
        self.dungeon_add_girls.clear();
    }
}

/// Register all wm.* API functions into the Lua state.
pub fn register_api(lua: &Lua, ctx: SharedContext) -> LuaResult<()> {
    let wm = lua.create_table()?;

    // -- wm.message(text, color) --
    {
        let ctx = ctx.clone();
        wm.set(
            "message",
            lua.create_function(move |_, (text, color): (String, i32)| {
                let mut ctx = ctx.lock().unwrap();
                ctx.messages.push(ScriptMessage { text, color });
                Ok(())
            })?,
        )?;
    }

    // -- wm.choice_box(id, options_table) --
    {
        let ctx = ctx.clone();
        wm.set(
            "choice_box",
            lua.create_function(move |_, (id, options): (i32, Vec<String>)| {
                let mut ctx = ctx.lock().unwrap();
                ctx.pending_choice_options.push((id, options));
                // Default choice = 0 until the UI sets it
                ctx.choices.entry(id).or_insert(0);
                Ok(())
            })?,
        )?;
    }

    // -- wm.get_choice(id) -> int --
    {
        let ctx = ctx.clone();
        wm.set(
            "get_choice",
            lua.create_function(move |_, id: i32| {
                let ctx = ctx.lock().unwrap();
                Ok(ctx.choices.get(&id).copied().unwrap_or(0))
            })?,
        )?;
    }

    // -- wm.activate_choice(id) --
    {
        wm.set(
            "activate_choice",
            lua.create_function(move |_, _id: i32| {
                // In the full game this shows the choice box UI
                // For now it's a no-op as we set choices before script runs
                Ok(())
            })?,
        )?;
    }

    // -- wm.add_gold(amount) --
    {
        let ctx = ctx.clone();
        wm.set(
            "add_gold",
            lua.create_function(move |_, amount: i64| {
                let mut ctx = ctx.lock().unwrap();
                ctx.gold_delta += amount;
                Ok(())
            })?,
        )?;
    }

    // -- wm.game_over() --
    {
        let ctx = ctx.clone();
        wm.set(
            "game_over",
            lua.create_function(move |_, ()| {
                let mut ctx = ctx.lock().unwrap();
                ctx.game_over = true;
                Ok(())
            })?,
        )?;
    }

    // -- wm.give_random_special_item() --
    {
        let ctx = ctx.clone();
        wm.set(
            "give_random_special_item",
            lua.create_function(move |_, ()| {
                let mut ctx = ctx.lock().unwrap();
                ctx.give_random_item = true;
                Ok(())
            })?,
        )?;
    }

    // ---- wm.player sub-table ----
    let player = lua.create_table()?;
    {
        let ctx = ctx.clone();
        player.set(
            "set_suspicion",
            lua.create_function(move |_, delta: i32| {
                let mut ctx = ctx.lock().unwrap();
                ctx.suspicion_delta += delta;
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        player.set(
            "set_disposition",
            lua.create_function(move |_, delta: i32| {
                let mut ctx = ctx.lock().unwrap();
                ctx.disposition_delta += delta;
                Ok(())
            })?,
        )?;
    }
    wm.set("player", player)?;

    // ---- wm.global sub-table ----
    let global = lua.create_table()?;
    {
        let ctx = ctx.clone();
        global.set(
            "set_flag",
            lua.create_function(move |_, (id, value): (i32, LuaValue)| {
                let mut ctx = ctx.lock().unwrap();
                if let Some(slot) = ctx.global_flags.get_mut(id as usize) {
                    *slot = match value {
                        LuaValue::Boolean(b) => b,
                        LuaValue::Integer(i) => i != 0,
                        LuaValue::Number(f) => f != 0.0,
                        _ => true,
                    };
                }
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        global.set(
            "get_flag",
            lua.create_function(move |_, id: i32| {
                let ctx = ctx.lock().unwrap();
                Ok(ctx.global_flags.get(id as usize).copied().unwrap_or(false))
            })?,
        )?;
    }
    wm.set("global", global)?;

    // ---- wm.girl sub-table ----
    let girl = lua.create_table()?;
    {
        let ctx = ctx.clone();
        girl.set(
            "get_stat",
            lua.create_function(move |_, name: String| {
                let ctx = ctx.lock().unwrap();
                let idx = stat_index(&name);
                Ok(ctx.girl_stats.get(idx).copied().unwrap_or(0))
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "set_stat",
            lua.create_function(move |_, (name, delta): (String, i32)| {
                let mut ctx = ctx.lock().unwrap();
                let idx = stat_index(&name);
                if let Some(val) = ctx.girl_stats.get_mut(idx) {
                    *val = (*val + delta).clamp(0, 100);
                }
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "get_skill",
            lua.create_function(move |_, name: String| {
                let ctx = ctx.lock().unwrap();
                let idx = skill_index(&name);
                Ok(ctx.girl_skills.get(idx).copied().unwrap_or(0))
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "set_skill",
            lua.create_function(move |_, (name, delta): (String, i32)| {
                let mut ctx = ctx.lock().unwrap();
                let idx = skill_index(&name);
                if let Some(val) = ctx.girl_skills.get_mut(idx) {
                    *val = (*val + delta).clamp(0, 100);
                }
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "has_trait",
            lua.create_function(move |_, name: String| {
                let ctx = ctx.lock().unwrap();
                Ok(ctx.girl_traits.contains(&name))
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "add_trait",
            lua.create_function(move |_, name: String| {
                let mut ctx = ctx.lock().unwrap();
                ctx.girl_traits.insert(name);
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "remove_trait",
            lua.create_function(move |_, name: String| {
                let mut ctx = ctx.lock().unwrap();
                ctx.girl_traits.remove(&name);
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "get_flag",
            lua.create_function(move |_, id: i32| {
                let ctx = ctx.lock().unwrap();
                Ok(ctx.girl_flags.get(id as usize).copied().unwrap_or(0))
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "set_flag",
            lua.create_function(move |_, (id, value): (i32, i32)| {
                let mut ctx = ctx.lock().unwrap();
                if let Some(slot) = ctx.girl_flags.get_mut(id as usize) {
                    *slot = value;
                }
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "pass_skill_check",
            lua.create_function(move |_, name: String| {
                let ctx = ctx.lock().unwrap();
                let idx = skill_index(&name);
                let skill_val = ctx.girl_skills.get(idx).copied().unwrap_or(0);
                // Random check: rand(0..=100) < skill
                let roll = rand::random::<u32>() % 101;
                Ok(roll < skill_val as u32)
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "pass_stat_check",
            lua.create_function(move |_, name: String| {
                let ctx = ctx.lock().unwrap();
                let idx = stat_index(&name);
                let stat_val = ctx.girl_stats.get(idx).copied().unwrap_or(0);
                let roll = rand::random::<u32>() % 101;
                Ok(roll < stat_val as u32)
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "add_to_brothel",
            lua.create_function(move |_, ()| {
                let mut ctx = ctx.lock().unwrap();
                ctx.add_target_girl = true;
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "player_rape",
            lua.create_function(move |_, ()| {
                let mut ctx = ctx.lock().unwrap();
                ctx.player_rape = true;
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "torture",
            lua.create_function(move |_, ()| {
                let mut ctx = ctx.lock().unwrap();
                ctx.torture = true;
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "scold",
            lua.create_function(move |_, ()| {
                let mut ctx = ctx.lock().unwrap();
                ctx.scold = true;
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "normal_sex",
            lua.create_function(move |_, ()| {
                let mut ctx = ctx.lock().unwrap();
                ctx.normal_sex = true;
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "beast_sex",
            lua.create_function(move |_, ()| {
                let mut ctx = ctx.lock().unwrap();
                ctx.beast_sex = true;
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "anal_sex",
            lua.create_function(move |_, ()| {
                let mut ctx = ctx.lock().unwrap();
                ctx.anal_sex = true;
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "bdsm_sex",
            lua.create_function(move |_, ()| {
                let mut ctx = ctx.lock().unwrap();
                ctx.bdsm_sex = true;
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        girl.set(
            "disobey_check",
            lua.create_function(move |_, ()| {
                // Simplified: based on obedience stat
                let ctx = ctx.lock().unwrap();
                let obedience = ctx.girl_stats[14]; // Obedience index
                let roll = rand::random::<u32>() % 101;
                Ok(roll >= obedience as u32)
            })?,
        )?;
    }
    wm.set("girl", girl)?;

    // ---- wm.dungeon sub-table ----
    let dungeon = lua.create_table()?;
    {
        let ctx = ctx.clone();
        dungeon.set(
            "add_customer",
            lua.create_function(move |_, (reason, num, wife): (String, i32, i32)| {
                let mut ctx = ctx.lock().unwrap();
                ctx.dungeon_add_customers.push(DungeonCustomerOp {
                    reason,
                    num,
                    wife,
                });
                Ok(())
            })?,
        )?;
    }
    {
        let ctx = ctx.clone();
        dungeon.set(
            "add_random_girl",
            lua.create_function(
                move |_, (reason, min_age, max_age, slave, non_human): (
                    String,
                    i32,
                    i32,
                    i32,
                    i32,
                )| {
                    let mut ctx = ctx.lock().unwrap();
                    ctx.dungeon_add_girls.push(DungeonGirlOp {
                        count: 1,
                        reason,
                        min_age,
                        max_age,
                        slave: slave != 0,
                        non_human: non_human != 0,
                    });
                    Ok(())
                },
            )?,
        )?;
    }
    {
        let ctx = ctx.clone();
        dungeon.set(
            "add_random_girls",
            lua.create_function(
                move |_,
                      (count, reason, min_age, max_age, slave, non_human): (
                    i32,
                    String,
                    i32,
                    i32,
                    i32,
                    i32,
                )| {
                    let mut ctx = ctx.lock().unwrap();
                    ctx.dungeon_add_girls.push(DungeonGirlOp {
                        count,
                        reason,
                        min_age,
                        max_age,
                        slave: slave != 0,
                        non_human: non_human != 0,
                    });
                    Ok(())
                },
            )?,
        )?;
    }
    wm.set("dungeon", dungeon)?;

    // Register the wm table as a global
    lua.globals().set("wm", wm)?;

    Ok(())
}

/// Map a stat name string to its index (0-21).
fn stat_index(name: &str) -> usize {
    match name {
        "Charisma" => 0,
        "Happiness" => 1,
        "Libido" => 2,
        "Constitution" => 3,
        "Intelligence" => 4,
        "Confidence" => 5,
        "Mana" => 6,
        "Agility" => 7,
        "Fame" => 8,
        "Level" => 9,
        "AskPrice" => 10,
        "HousePerc" => 11,
        "Exp" => 12,
        "Age" => 13,
        "Obedience" => 14,
        "Spirit" => 15,
        "Beauty" => 16,
        "Tiredness" => 17,
        "Health" => 18,
        "PCFear" => 19,
        "PCLove" => 20,
        "PCHate" => 21,
        _ => 0,
    }
}

/// Map a skill name string to its index (0-9).
fn skill_index(name: &str) -> usize {
    match name {
        "Anal" => 0,
        "Magic" => 1,
        "BDSM" => 2,
        "NormalSex" => 3,
        "Beastiality" => 4,
        "Group" => 5,
        "Lesbian" => 6,
        "Service" => 7,
        "Strip" => 8,
        "Combat" => 9,
        _ => 0,
    }
}
