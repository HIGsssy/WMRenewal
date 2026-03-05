use std::io::{Cursor, Read};
use std::path::Path;

// Entry types matching C++ enum Types { _NONE=0, _TEXT=1, _BOOL=2, _INT=3, _FLOAT=4, _CHOICE=5 }
const ENTRY_TYPE_TEXT: i32 = 1;

/// A single entry in a script action.
#[derive(Debug)]
struct ScriptEntry {
    _entry_type: i32,
    value: i32, // union: length for TEXT, lValue/Selection/bValue for others
    var: u8,    // 1 = use as variable index into vars[]
    text: Option<String>,
}

/// A single script action (node in the linked list).
#[derive(Debug)]
struct ScriptAction {
    action_type: i32,
    entries: Vec<ScriptEntry>,
}

/// Read a little-endian i32 from a cursor.
fn read_i32(cursor: &mut Cursor<&[u8]>) -> Result<i32, ConvertError> {
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

/// Read a single byte from a cursor.
fn read_u8(cursor: &mut Cursor<&[u8]>) -> Result<u8, ConvertError> {
    let mut buf = [0u8; 1];
    cursor.read_exact(&mut buf)?;
    Ok(buf[0])
}

/// Parse the binary .script format into a list of actions.
fn parse_script(data: &[u8]) -> Result<Vec<ScriptAction>, ConvertError> {
    let mut cursor = Cursor::new(data);
    let num_actions = read_i32(&mut cursor)?;

    let mut actions = Vec::with_capacity(num_actions as usize);

    for _ in 0..num_actions {
        let action_type = read_i32(&mut cursor)?;
        let num_entries = read_i32(&mut cursor)?;

        let mut entries = Vec::with_capacity(num_entries as usize);

        for _ in 0..num_entries {
            let entry_type = read_i32(&mut cursor)?;
            let value = read_i32(&mut cursor)?;
            let var = read_u8(&mut cursor)?;

            let text = if entry_type == ENTRY_TYPE_TEXT && value > 0 {
                let len = value as usize;
                let mut buf = vec![0u8; len];
                cursor.read_exact(&mut buf)?;
                // Text may include null terminator
                let s = String::from_utf8_lossy(&buf);
                Some(s.trim_end_matches('\0').to_string())
            } else {
                None
            };

            entries.push(ScriptEntry {
                _entry_type: entry_type,
                value,
                var,
                text,
            });
        }

        actions.push(ScriptAction {
            action_type,
            entries,
        });
    }

    Ok(actions)
}

/// Get the stat name for a given index (0-21).
fn stat_name(idx: i32) -> &'static str {
    match idx {
        0 => "Charisma",
        1 => "Happiness",
        2 => "Libido",
        3 => "Constitution",
        4 => "Intelligence",
        5 => "Confidence",
        6 => "Mana",
        7 => "Agility",
        8 => "Fame",
        9 => "Level",
        10 => "AskPrice",
        11 => "HousePerc",
        12 => "Exp",
        13 => "Age",
        14 => "Obedience",
        15 => "Spirit",
        16 => "Beauty",
        17 => "Tiredness",
        18 => "Health",
        19 => "PCFear",
        20 => "PCLove",
        21 => "PCHate",
        _ => "Unknown",
    }
}

/// Get the skill name for a given index (0-9).
fn skill_name(idx: i32) -> &'static str {
    match idx {
        0 => "Anal",
        1 => "Magic",
        2 => "BDSM",
        3 => "NormalSex",
        4 => "Beastiality",
        5 => "Group",
        6 => "Lesbian",
        7 => "Service",
        8 => "Strip",
        9 => "Combat",
        _ => "Unknown",
    }
}

/// Get the stat-or-skill name for AdjustTargetGirlStat (indices 0-21 = stat, 22-31 = skill).
fn stat_or_skill_name(idx: i32) -> &'static str {
    if idx >= 22 {
        skill_name(idx - 22)
    } else {
        stat_name(idx)
    }
}

/// Get the comparison operator string.
fn comparison_op(sel: i32) -> &'static str {
    match sel {
        0 => "==",
        1 => "<",
        2 => "<=",
        3 => ">",
        4 => ">=",
        5 => "~=",
        _ => "==",
    }
}

/// Resolve an entry value, accounting for the var flag.
fn val_expr(entry: &ScriptEntry) -> String {
    if entry.var == 1 {
        format!("vars[{}]", entry.value)
    } else {
        format!("{}", entry.value)
    }
}

/// Dungeon reason name from CHOICE index.
fn dungeon_cust_reason(sel: i32) -> &'static str {
    match sel {
        0 => "\"NoPay\"",
        1 => "\"BeatGirl\"",
        _ => "\"Unknown\"",
    }
}

/// Dungeon girl reason name from CHOICE index.
fn dungeon_girl_reason(sel: i32) -> &'static str {
    match sel {
        0 => "\"Kidnapped\"",
        1 => "\"Captured\"",
        _ => "\"Unknown\"",
    }
}

/// Escape a string for Lua (double-bracket safe).
fn lua_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

/// Convert parsed script actions into Lua source code.
fn actions_to_lua(actions: &[ScriptAction]) -> Result<String, ConvertError> {
    let mut out = String::new();
    out.push_str("-- Auto-converted from .script binary format\n");
    out.push_str("local vars = {}\n");
    out.push_str("for i = 0, 19 do vars[i] = 0 end\n\n");

    let mut i = 0;
    while i < actions.len() {
        let action = &actions[i];
        let indent = "";

        match action.action_type {
            // 0: Dialog
            0 => {
                let text = action
                    .entries
                    .first()
                    .and_then(|e| e.text.as_deref())
                    .unwrap_or("");
                out.push_str(&format!(
                    "{}wm.message(\"{}\", 0)\n",
                    indent,
                    lua_escape(text)
                ));
            }
            // 1: Init
            1 => {
                out.push_str("-- init block\n");
            }
            // 2: EndInit
            2 => {
                out.push_str("-- end init\n\n");
            }
            // 3: EndScript
            3 => {
                out.push_str("return -- end script\n");
            }
            // 4: Restart
            4 => {
                out.push_str("coroutine.yield(\"restart\")\n");
            }
            // 5: ChoiceBox
            5 => {
                let id = val_expr(&action.entries[0]);
                let num: i32 = action.entries[1].value;
                out.push_str(&format!("{}wm.choice_box({}, {{\n", indent, id));
                // The next `num` actions are TEXT entries
                for j in 0..num {
                    let text_idx = i + 1 + j as usize;
                    if text_idx < actions.len() {
                        let text = actions[text_idx]
                            .entries
                            .first()
                            .and_then(|e| e.text.as_deref())
                            .unwrap_or("");
                        out.push_str(&format!("    \"{}\",\n", lua_escape(text)));
                    }
                }
                out.push_str("})\n");
                i += num as usize; // skip the TEXT actions
            }
            // 6: TEXT (handled by ChoiceBox)
            6 => {
                // standalone TEXT shouldn't execute; skip
            }
            // 7: SetVar
            7 => {
                let var_idx = val_expr(&action.entries[0]);
                let value = val_expr(&action.entries[1]);
                out.push_str(&format!("{}vars[{}] = {}\n", indent, var_idx, value));
            }
            // 8: SetVarRandom
            8 => {
                let var_idx = val_expr(&action.entries[0]);
                let min_val = val_expr(&action.entries[1]);
                let max_val = val_expr(&action.entries[2]);
                out.push_str(&format!(
                    "{}vars[{}] = math.random({}, {})\n",
                    indent, var_idx, min_val, max_val
                ));
            }
            // 9: IfVar
            9 => {
                let var_idx = val_expr(&action.entries[0]);
                let op = comparison_op(action.entries[1].value);
                let cmp_val = val_expr(&action.entries[2]);
                out.push_str(&format!(
                    "{}if vars[{}] {} {} then\n",
                    indent, var_idx, op, cmp_val
                ));
            }
            // 10: Else
            10 => {
                out.push_str(&format!("{}else\n", indent));
            }
            // 11: EndIf
            11 => {
                out.push_str(&format!("{}end\n", indent));
            }
            // 12: ActivateChoice
            12 => {
                let id = val_expr(&action.entries[0]);
                out.push_str(&format!("{}wm.activate_choice({})\n", indent, id));
            }
            // 13: IfChoice
            13 => {
                let id = val_expr(&action.entries[0]);
                let choice = val_expr(&action.entries[1]);
                out.push_str(&format!(
                    "{}if wm.get_choice({}) == {} then\n",
                    indent, id, choice
                ));
            }
            // 14: SetPlayerSuspicion
            14 => {
                let value = val_expr(&action.entries[0]);
                out.push_str(&format!("{}wm.player.set_suspicion({})\n", indent, value));
            }
            // 15: SetPlayerDisposition
            15 => {
                let value = val_expr(&action.entries[0]);
                out.push_str(&format!("{}wm.player.set_disposition({})\n", indent, value));
            }
            // 16: ClearGlobalFlag
            16 => {
                let flag = val_expr(&action.entries[0]);
                out.push_str(&format!("{}wm.global.set_flag({}, false)\n", indent, flag));
            }
            // 17: AddCustToDungeon
            17 => {
                let reason = dungeon_cust_reason(action.entries[0].value);
                let num = val_expr(&action.entries[1]);
                let wife = val_expr(&action.entries[2]);
                out.push_str(&format!(
                    "{}wm.dungeon.add_customer({}, {}, {})\n",
                    indent, reason, num, wife
                ));
            }
            // 18: AddRandomGirlToDungeon
            18 => {
                let reason = dungeon_girl_reason(action.entries[0].value);
                let min_age = val_expr(&action.entries[1]);
                let max_age = val_expr(&action.entries[2]);
                let slave = val_expr(&action.entries[3]);
                let non_human = val_expr(&action.entries[4]);
                out.push_str(&format!(
                    "{}wm.dungeon.add_random_girl({}, {}, {}, {}, {})\n",
                    indent, reason, min_age, max_age, slave, non_human
                ));
            }
            // 19: SetGlobal
            19 => {
                let flag = action.entries[0].value;
                let value = action.entries[1].value;
                out.push_str(&format!(
                    "{}wm.global.set_flag({}, {})\n",
                    indent, flag, value
                ));
            }
            // 20: SetGirlFlag
            20 => {
                let flag = val_expr(&action.entries[0]);
                let value = val_expr(&action.entries[1]);
                out.push_str(&format!(
                    "{}wm.girl.set_flag({}, {})\n",
                    indent, flag, value
                ));
            }
            // 21: AddRandomValueToGold
            21 => {
                let min_val = val_expr(&action.entries[0]);
                let max_val = val_expr(&action.entries[1]);
                out.push_str(&format!(
                    "{}wm.add_gold(math.random({}, {}))\n",
                    indent, min_val, max_val
                ));
            }
            // 22: AddManyRandomGirlsToDungeon
            22 => {
                let count = val_expr(&action.entries[0]);
                let reason = dungeon_girl_reason(action.entries[1].value);
                let min_age = val_expr(&action.entries[2]);
                let max_age = val_expr(&action.entries[3]);
                let slave = val_expr(&action.entries[4]);
                let non_human = val_expr(&action.entries[5]);
                out.push_str(&format!(
                    "{}wm.dungeon.add_random_girls({}, {}, {}, {}, {}, {})\n",
                    indent, count, reason, min_age, max_age, slave, non_human
                ));
            }
            // 23: AddTargetGirl
            23 => {
                out.push_str(&format!("{}wm.girl.add_to_brothel()\n", indent));
            }
            // 24: AdjustTargetGirlStat
            24 => {
                // Entry 0 is CHOICE (stat/skill index), Entry 1 is INT (delta)
                let stat_skill_idx = action.entries[0].value;
                let delta = val_expr(&action.entries[1]);
                let name = stat_or_skill_name(stat_skill_idx);
                if stat_skill_idx >= 22 {
                    out.push_str(&format!(
                        "{}wm.girl.set_skill(\"{}\", {})\n",
                        indent, name, delta
                    ));
                } else {
                    out.push_str(&format!(
                        "{}wm.girl.set_stat(\"{}\", {})\n",
                        indent, name, delta
                    ));
                }
            }
            // 25: PlayerRapeTargetGirl
            25 => {
                out.push_str(&format!("{}wm.girl.player_rape()\n", indent));
            }
            // 26: GivePlayerRandomSpecialItem
            26 => {
                out.push_str(&format!("{}wm.give_random_special_item()\n", indent));
            }
            // 27: IfPassSkillCheck
            27 => {
                let skill_idx = action.entries[0].value;
                let name = skill_name(skill_idx);
                out.push_str(&format!(
                    "{}if wm.girl.pass_skill_check(\"{}\") then\n",
                    indent, name
                ));
            }
            // 28: IfPassStatCheck
            28 => {
                let stat_idx = action.entries[0].value;
                let name = stat_name(stat_idx);
                out.push_str(&format!(
                    "{}if wm.girl.pass_stat_check(\"{}\") then\n",
                    indent, name
                ));
            }
            // 29: IfGirlFlag
            29 => {
                let flag = val_expr(&action.entries[0]);
                let op = comparison_op(action.entries[1].value);
                let value = val_expr(&action.entries[2]);
                out.push_str(&format!(
                    "{}if wm.girl.get_flag({}) {} {} then\n",
                    indent, flag, op, value
                ));
            }
            // 30: GameOver
            30 => {
                out.push_str(&format!("{}wm.game_over()\n", indent));
            }
            // 31: IfGirlStat
            31 => {
                let stat_idx = action.entries[0].value;
                let op = comparison_op(action.entries[1].value);
                let value = val_expr(&action.entries[2]);
                let name = stat_name(stat_idx);
                out.push_str(&format!(
                    "{}if wm.girl.get_stat(\"{}\") {} {} then\n",
                    indent, name, op, value
                ));
            }
            // 32: IfGirlSkill
            32 => {
                let skill_idx = action.entries[0].value;
                let op = comparison_op(action.entries[1].value);
                let value = val_expr(&action.entries[2]);
                let name = skill_name(skill_idx);
                out.push_str(&format!(
                    "{}if wm.girl.get_skill(\"{}\") {} {} then\n",
                    indent, name, op, value
                ));
            }
            // 33: IfHasTrait
            33 => {
                let trait_name = action
                    .entries
                    .first()
                    .and_then(|e| e.text.as_deref())
                    .unwrap_or("Unknown");
                out.push_str(&format!(
                    "{}if wm.girl.has_trait(\"{}\") then\n",
                    indent,
                    lua_escape(trait_name)
                ));
            }
            // 34: TortureTarget
            34 => {
                out.push_str(&format!("{}wm.girl.torture()\n", indent));
            }
            // 35: ScoldTarget
            35 => {
                out.push_str(&format!("{}wm.girl.scold()\n", indent));
            }
            // 36: NormalSexTarget
            36 => {
                out.push_str(&format!("{}wm.girl.normal_sex()\n", indent));
            }
            // 37: BeastSexTarget
            37 => {
                out.push_str(&format!("{}wm.girl.beast_sex()\n", indent));
            }
            // 38: AnalSexTarget
            38 => {
                out.push_str(&format!("{}wm.girl.anal_sex()\n", indent));
            }
            // 39: BDSMSexTarget
            39 => {
                out.push_str(&format!("{}wm.girl.bdsm_sex()\n", indent));
            }
            // 40: IfNotDisobey
            40 => {
                out.push_str(&format!("{}if not wm.girl.disobey_check() then\n", indent));
            }
            _ => {
                return Err(ConvertError::UnknownOpcode(action.action_type as u8));
            }
        }

        i += 1;
    }

    Ok(out)
}

/// Convert a legacy binary .script file to Lua source.
pub fn convert_script_to_lua(path: &Path) -> Result<String, ConvertError> {
    let data = std::fs::read(path)?;
    let actions = parse_script(&data)?;
    actions_to_lua(&actions)
}

/// Convert all .script files in a directory; returns a vec of (filename, lua_source).
pub fn convert_all_scripts(scripts_dir: &Path) -> Result<Vec<(String, String)>, ConvertError> {
    let mut results = Vec::new();

    if !scripts_dir.is_dir() {
        return Ok(results);
    }

    for entry in std::fs::read_dir(scripts_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("script") {
            let lua = convert_script_to_lua(&path)?;
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            results.push((format!("{}.lua", stem), lua));
        }
    }

    Ok(results)
}

#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown opcode: {0}")]
    UnknownOpcode(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_script() {
        // A script with 0 actions: just a 4-byte i32 = 0
        let data = 0i32.to_le_bytes();
        let actions = parse_script(&data).unwrap();
        assert!(actions.is_empty());
    }

    #[test]
    fn test_convert_scripts_from_resources() {
        let scripts_dir = wm_core::resources_path().join("Scripts");
        if !scripts_dir.exists() {
            eprintln!("Skipping: resources/Scripts not found");
            return;
        }

        let results = convert_all_scripts(&scripts_dir).unwrap();
        assert!(
            !results.is_empty(),
            "Expected at least one .script file converted"
        );

        for (name, lua) in &results {
            assert!(
                !lua.is_empty(),
                "Converted Lua for {} should not be empty",
                name
            );
            // Verify it starts with the header comment
            assert!(
                lua.starts_with("-- Auto-converted"),
                "Missing header in {}",
                name
            );
            println!("Converted {}: {} bytes", name, lua.len());
        }
    }
}
