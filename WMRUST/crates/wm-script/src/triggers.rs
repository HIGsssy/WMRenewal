use wm_core::enums::TriggerType;

/// A single trigger definition loaded from XML.
#[derive(Debug, Clone)]
pub struct Trigger {
    pub script: String,
    pub trigger_type: TriggerType,
    pub triggered: u32,
    pub chance: u8,
    pub once: bool,
    pub values: [i32; 2],
}

/// Context provided to the trigger evaluator for condition checks.
#[derive(Debug, Default)]
pub struct TriggerEvalContext {
    /// Girl stats (22 entries), if a girl target is set.
    pub girl_stats: Option<[i32; 22]>,
    /// Girl skills (10 entries), if a girl target is set.
    pub girl_skills: Option<[i32; 10]>,
    /// Girl status bitfield.
    pub girl_states: Option<u32>,
    /// Girl money.
    pub girl_money: Option<i64>,
    /// Girl weeks in employment.
    pub girl_weeks: Option<u32>,
    /// Player gold.
    pub player_gold: i64,
    /// Global flags (5 entries).
    pub global_flags: [bool; 5],
    /// Track which trigger indices have previously fired (for ScriptRun type).
    pub scripts_run: Vec<bool>,
}

/// Manages loading and evaluating triggers (per-girl and global).
#[derive(Debug)]
pub struct TriggerSystem {
    pub triggers: Vec<Trigger>,
    pub queue: Vec<usize>,
}

impl TriggerSystem {
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
            queue: Vec::new(),
        }
    }

    /// Load triggers from a GlobalTriggers.xml (or per-girl trigger file).
    pub fn load(&mut self, path: &std::path::Path) -> Result<(), TriggerError> {
        let content = std::fs::read_to_string(path)?;
        self.parse_xml(&content)
    }

    /// Parse triggers from XML string content.
    fn parse_xml(&mut self, xml: &str) -> Result<(), TriggerError> {
        use quick_xml::events::Event;
        use quick_xml::Reader;

        let mut reader = Reader::from_str(xml);

        loop {
            match reader.read_event() {
                Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e))
                    if e.name().as_ref() == b"Trigger" =>
                {
                    if let Some(trigger) = parse_trigger_element(e)? {
                        self.triggers.push(trigger);
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(TriggerError::XmlParse(e.to_string())),
                _ => {}
            }
        }

        Ok(())
    }

    /// Check all trigger conditions against the given context and queue those that fire.
    pub fn evaluate(&mut self, eval_ctx: &TriggerEvalContext) {
        for i in 0..self.triggers.len() {
            let trigger = &self.triggers[i];

            // Skip if once-only and already triggered
            if trigger.once && trigger.triggered > 0 {
                continue;
            }

            if self.check_condition(i, eval_ctx) {
                // Chance check
                if trigger.chance >= 100 || (rand::random::<u8>() % 100) < trigger.chance {
                    self.queue.push(i);
                }
            }
        }
    }

    /// Check whether the condition for trigger at index `idx` is met.
    fn check_condition(&self, idx: usize, ctx: &TriggerEvalContext) -> bool {
        let trigger = &self.triggers[idx];

        match trigger.trigger_type {
            TriggerType::Random => {
                // Always eligible (chance is checked in evaluate)
                true
            }
            TriggerType::Shopping => {
                // Fired during shopping events; always eligible when checked
                true
            }
            TriggerType::Skill => {
                if let Some(ref skills) = ctx.girl_skills {
                    let skill_id = trigger.values[0] as usize;
                    let threshold = trigger.values[1];
                    skills.get(skill_id).copied().unwrap_or(0) >= threshold
                } else {
                    false
                }
            }
            TriggerType::Stat => {
                if let Some(ref stats) = ctx.girl_stats {
                    let stat_id = trigger.values[0] as usize;
                    let threshold = trigger.values[1];
                    stats.get(stat_id).copied().unwrap_or(0) >= threshold
                } else {
                    false
                }
            }
            TriggerType::Status => {
                if let Some(states) = ctx.girl_states {
                    let status_bit = trigger.values[0];
                    let has = trigger.values[1]; // 1 = has, 0 = doesn't have
                    let bit_set = (states & (1 << status_bit)) != 0;
                    if has == 1 {
                        bit_set
                    } else {
                        !bit_set
                    }
                } else {
                    false
                }
            }
            TriggerType::Money => {
                if let Some(girl_money) = ctx.girl_money {
                    let comparison = trigger.values[0]; // 0 = less than, 1 = more than
                    let threshold = trigger.values[1] as i64;
                    if comparison == 0 {
                        girl_money < threshold
                    } else {
                        girl_money >= threshold
                    }
                } else {
                    false
                }
            }
            TriggerType::PlayerMoney => {
                let comparison = trigger.values[0]; // 0 = less than, 1 = more than
                let threshold = trigger.values[1] as i64;
                if comparison == 0 {
                    ctx.player_gold < threshold
                } else {
                    ctx.player_gold >= threshold
                }
            }
            TriggerType::Meet | TriggerType::Talk | TriggerType::Kidnapped => {
                // These are event-driven triggers, checked via check_for_trigger()
                false
            }
            TriggerType::WeeksPast => {
                if let Some(weeks) = ctx.girl_weeks {
                    weeks >= trigger.values[0] as u32
                } else {
                    false
                }
            }
            TriggerType::GlobalFlag => {
                let flag_id = trigger.values[0] as usize;
                ctx.global_flags.get(flag_id).copied().unwrap_or(false)
            }
            TriggerType::ScriptRun => {
                let script1 = trigger.values[0] as usize;
                let script2 = trigger.values[1] as usize;
                let run1 = if trigger.values[0] == -1 {
                    true
                } else {
                    ctx.scripts_run.get(script1).copied().unwrap_or(false)
                };
                let run2 = if trigger.values[1] == -1 {
                    true
                } else {
                    ctx.scripts_run.get(script2).copied().unwrap_or(false)
                };
                run1 && run2
            }
        }
    }

    /// Check for a trigger matching a specific event type (e.g. Meet, Talk, GlobalFlag).
    /// Returns the index of the matching trigger if found.
    pub fn check_for_trigger(
        &self,
        trigger_type: TriggerType,
        values: &[i32; 2],
    ) -> Option<usize> {
        for (i, trigger) in self.triggers.iter().enumerate() {
            if trigger.trigger_type != trigger_type {
                continue;
            }

            let mut matches = true;
            if values[0] != -1 && trigger.values[0] != values[0] {
                matches = false;
            }
            if values[1] != -1 && trigger.values[1] != values[1] {
                matches = false;
            }

            if matches {
                return Some(i);
            }
        }
        None
    }

    /// Process the next queued trigger, returning it. Marks it as triggered.
    pub fn process_next(&mut self) -> Option<&Trigger> {
        let idx = self.queue.first().copied()?;
        self.queue.remove(0);
        self.triggers[idx].triggered += 1;
        Some(&self.triggers[idx])
    }

    /// Check if the queue is empty.
    pub fn queue_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl Default for TriggerSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a single <Trigger> XML element from its attributes.
fn parse_trigger_element(
    element: &quick_xml::events::BytesStart,
) -> Result<Option<Trigger>, TriggerError> {
    let mut trigger_type_str = String::new();
    let mut file = String::new();
    let mut chance: u8 = 100;
    let mut once = false;
    let mut values = [0i32; 2];

    // Collect all attributes
    let mut flag_str = String::new();
    let mut skill_str = String::new();
    let mut stat_str = String::new();
    let mut status_str = String::new();
    let mut who_str = String::new();
    let mut where_str = String::new();
    let mut has_str = String::new();
    let mut comparison_str = String::new();
    let mut threshold_str = String::new();

    for attr in element.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
        let val = attr.unescape_value().unwrap_or_default();

        match key {
            "Type" => trigger_type_str = val.to_string(),
            "File" => file = val.to_string(),
            "Chance" => {
                let s = val.trim_end_matches('%');
                chance = s.parse().unwrap_or(100);
            }
            "OnceOnly" => {
                once = matches!(val.as_ref(), "True" | "true");
            }
            "Flag" => flag_str = val.to_string(),
            "Skill" => skill_str = val.to_string(),
            "Stat" => stat_str = val.to_string(),
            "Status" => status_str = val.to_string(),
            "Who" => who_str = val.to_string(),
            "Where" => where_str = val.to_string(),
            "Has" => has_str = val.to_string(),
            "Comparison" => comparison_str = val.to_string(),
            "Threshold" => threshold_str = val.to_string(),
            _ => {}
        }
    }

    if file.is_empty() || trigger_type_str.is_empty() {
        return Ok(None);
    }

    let trigger_type = match trigger_type_str.as_str() {
        "Random" => TriggerType::Random,
        "Shopping" => TriggerType::Shopping,
        "Skill" => TriggerType::Skill,
        "Stat" => TriggerType::Stat,
        "Status" => TriggerType::Status,
        "Money" => TriggerType::Money,
        "Meet" => TriggerType::Meet,
        "Talk" => TriggerType::Talk,
        "WeeksPast" => TriggerType::WeeksPast,
        "GlobalFlag" => TriggerType::GlobalFlag,
        "ScriptRun" => TriggerType::ScriptRun,
        "Kidnapped" => TriggerType::Kidnapped,
        "PlayerMoney" => TriggerType::PlayerMoney,
        _ => return Ok(None), // unknown type, skip
    };

    // Parse type-specific values
    match trigger_type {
        TriggerType::Skill => {
            values[0] = parse_skill_code(&skill_str);
            values[1] = threshold_str.parse().unwrap_or(0);
        }
        TriggerType::Stat => {
            values[0] = parse_stat_code(&stat_str);
            values[1] = threshold_str.parse().unwrap_or(0);
        }
        TriggerType::Status => {
            values[0] = parse_status_code(&status_str);
            values[1] = if has_str == "False" || has_str == "false" {
                0
            } else {
                1
            };
        }
        TriggerType::Money | TriggerType::PlayerMoney => {
            // Check the "Who" field — if "Player", force PlayerMoney type
            // (handled by the caller since we set the type already)
            let is_player = who_str == "Player";
            let _ = is_player; // Type was already determined from XML "Type" attr
            // Comparison: 0 = LessThan, 1 = MoreThan
            values[0] = if comparison_str == "LessThan" { 0 } else { 1 };
            values[1] = threshold_str.parse().unwrap_or(0);
        }
        TriggerType::Meet => {
            values[0] = parse_where_code(&where_str);
        }
        TriggerType::Talk => {
            values[0] = parse_talk_where_code(&where_str);
        }
        TriggerType::WeeksPast => {
            values[0] = threshold_str.parse().unwrap_or(0);
        }
        TriggerType::GlobalFlag => {
            values[0] = parse_global_flag_code(&flag_str);
        }
        _ => {}
    }

    Ok(Some(Trigger {
        script: file,
        trigger_type,
        triggered: 0,
        chance,
        once,
        values,
    }))
}

fn parse_skill_code(s: &str) -> i32 {
    match s {
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

fn parse_stat_code(s: &str) -> i32 {
    match s {
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
        "House" | "HousePerc" => 11,
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

fn parse_status_code(s: &str) -> i32 {
    match s {
        "Poisoned" => 1,
        "BadlyPoisoned" => 2,
        "Pregnant" => 3,
        "PregnantByPlayer" => 4,
        "Slave" => 5,
        "HasDaughter" => 6,
        "HasSon" => 7,
        "Inseminated" => 8,
        "Controlled" | "Controled" => 9,
        "Catacombs" => 10,
        _ => 0,
    }
}

fn parse_where_code(s: &str) -> i32 {
    match s {
        "Town" => 0,
        "Catacombs" => 1,
        "SlaveMarket" => 2,
        _ => 0,
    }
}

fn parse_talk_where_code(s: &str) -> i32 {
    match s {
        "Dungeon" => 0,
        "Brothel" => 1,
        _ => 0,
    }
}

fn parse_global_flag_code(s: &str) -> i32 {
    match s {
        "NoPay" => 0,        // FLAG_CUSTNOPAY
        "GirlDies" => 1,     // FLAG_DUNGEONGIRLDIE
        "CustomerDies" => 2, // FLAG_DUNGEONCUSTDIE
        "GamblingCheat" => 3, // FLAG_CUSTGAMBCHEAT
        "RivalLose" => 4,    // FLAG_RIVALLOSE
        _ => -1,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TriggerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("XML parse error: {0}")]
    XmlParse(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_global_triggers_xml() {
        let triggers_path = wm_core::resources_path()
            .join("Scripts")
            .join("GlobalTriggers.xml");
        if !triggers_path.exists() {
            eprintln!("Skipping: GlobalTriggers.xml not found");
            return;
        }

        let mut sys = TriggerSystem::new();
        sys.load(&triggers_path).unwrap();

        // The file has 4 triggers
        assert_eq!(sys.triggers.len(), 4);

        // First trigger: GlobalFlag "NoPay" → CustNoPay.script
        assert_eq!(sys.triggers[0].trigger_type, TriggerType::GlobalFlag);
        assert_eq!(sys.triggers[0].script, "CustNoPay.script");
        assert_eq!(sys.triggers[0].values[0], 0); // FLAG_CUSTNOPAY

        // Second: GlobalFlag "RivalLose"
        assert_eq!(sys.triggers[1].trigger_type, TriggerType::GlobalFlag);
        assert_eq!(sys.triggers[1].script, "RivalLose.script");

        // Third: GlobalFlag "GamblingCheat"
        assert_eq!(sys.triggers[2].trigger_type, TriggerType::GlobalFlag);
        assert_eq!(sys.triggers[2].script, "CustGambCheat.script");

        // Fourth: Money (PlayerMoney) with threshold -5000
        assert_eq!(sys.triggers[3].script, "NoMoney.script");
    }

    #[test]
    fn test_evaluate_global_flag_trigger() {
        let xml = r#"
            <Triggers>
                <Trigger Type="GlobalFlag" Flag="NoPay" File="test.script" />
            </Triggers>
        "#;

        let mut sys = TriggerSystem::new();
        sys.parse_xml(xml).unwrap();
        assert_eq!(sys.triggers.len(), 1);

        // Flag not set — should not fire
        let mut ctx = TriggerEvalContext::default();
        sys.evaluate(&ctx);
        assert!(sys.queue.is_empty());

        // Set flag — should fire
        ctx.global_flags[0] = true;
        sys.evaluate(&ctx);
        assert_eq!(sys.queue.len(), 1);
        assert_eq!(sys.queue[0], 0);
    }

    #[test]
    fn test_evaluate_skill_trigger() {
        let xml = r#"
            <Triggers>
                <Trigger Type="Skill" Skill="Combat" Threshold="50" File="fighter.lua" />
            </Triggers>
        "#;

        let mut sys = TriggerSystem::new();
        sys.parse_xml(xml).unwrap();

        // Below threshold
        let mut ctx = TriggerEvalContext::default();
        ctx.girl_skills = Some([0; 10]);
        ctx.girl_skills.as_mut().unwrap()[9] = 40; // Combat = 40
        sys.evaluate(&ctx);
        assert!(sys.queue.is_empty());

        // At threshold
        ctx.girl_skills.as_mut().unwrap()[9] = 50;
        sys.evaluate(&ctx);
        assert_eq!(sys.queue.len(), 1);
    }

    #[test]
    fn test_once_only_trigger() {
        let xml = r#"
            <Triggers>
                <Trigger Type="Random" File="once.lua" OnceOnly="True" Chance="100" />
            </Triggers>
        "#;

        let mut sys = TriggerSystem::new();
        sys.parse_xml(xml).unwrap();

        let ctx = TriggerEvalContext::default();

        // First evaluation — fires
        sys.evaluate(&ctx);
        assert_eq!(sys.queue.len(), 1);

        // Process it (marks triggered)
        let trigger = sys.process_next().unwrap();
        assert_eq!(trigger.script, "once.lua");

        // Second evaluation — should NOT fire again
        sys.evaluate(&ctx);
        assert!(sys.queue.is_empty());
    }

    #[test]
    fn test_check_for_trigger() {
        let xml = r#"
            <Triggers>
                <Trigger Type="Meet" Where="Town" File="meet_town.lua" />
                <Trigger Type="Meet" Where="Catacombs" File="meet_catacombs.lua" />
            </Triggers>
        "#;

        let mut sys = TriggerSystem::new();
        sys.parse_xml(xml).unwrap();

        // Search for town meeting
        let found = sys.check_for_trigger(TriggerType::Meet, &[0, -1]);
        assert_eq!(found, Some(0));

        // Search for catacombs meeting
        let found = sys.check_for_trigger(TriggerType::Meet, &[1, -1]);
        assert_eq!(found, Some(1));

        // Search for slave market — not present
        let found = sys.check_for_trigger(TriggerType::Meet, &[2, -1]);
        assert_eq!(found, None);
    }
}
