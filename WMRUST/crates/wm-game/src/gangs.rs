use rand::Rng;
use serde::{Deserialize, Serialize};
use wm_core::enums::GangMission;

/// A single gang.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gang {
    pub name: String,
    pub num_members: i32,
    pub max_members: i32,
    pub mission: GangMission,
    pub combat_skill: i32,
    pub magic_skill: i32,
    pub intelligence: i32,
    pub agility: i32,
    pub constitution: i32,
    pub charisma: i32,
    pub hired: bool,
    pub net_worth: i32,
    pub auto_recruit: bool,
    pub saw_combat: bool,
    pub prev_mission: Option<GangMission>,
}

impl Default for Gang {
    fn default() -> Self {
        Self {
            name: String::new(),
            num_members: 5,
            max_members: 15,
            mission: GangMission::Guarding,
            combat_skill: 50,
            magic_skill: 10,
            intelligence: 30,
            agility: 30,
            constitution: 30,
            charisma: 30,
            hired: false,
            net_worth: 0,
            auto_recruit: true,
            saw_combat: false,
            prev_mission: None,
        }
    }
}

impl Gang {
    /// Average attack skill (prefers magic if higher).
    pub fn attack_skill(&self) -> i32 {
        self.combat_skill.max(self.magic_skill)
    }

    /// Overall power rating.
    pub fn power(&self) -> i32 {
        (self.combat_skill + self.magic_skill + self.intelligence) * self.num_members
    }
}

/// Result of a gang mission for weekly event reporting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GangMissionResult {
    pub gold_earned: i64,
    pub gold_lost: i64,
    pub members_lost: i32,
    pub events: Vec<String>,
    pub territories_gained: i32,
}

/// Manages hired and recruitable gangs.
#[derive(Debug, Serialize, Deserialize)]
pub struct GangManager {
    pub hired_gangs: Vec<Gang>,
    pub recruit_list: Vec<Gang>,
    pub healing_potions: i32,
    pub nets: i32,
    pub weapon_level: i32,
}

impl Default for GangManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GangManager {
    pub fn new() -> Self {
        Self {
            hired_gangs: Vec::new(),
            recruit_list: Vec::new(),
            healing_potions: 10,
            nets: 0,
            weapon_level: 1,
        }
    }

    /// Generate a new random gang matching C++ AddNewGang formula.
    pub fn generate_gang(
        name: String,
        init_member_min: i32,
        init_member_max: i32,
        boosted: bool,
        rng: &mut dyn rand::RngCore,
    ) -> Gang {
        let range = (init_member_max + 1 - init_member_min).max(1);
        let mut members = init_member_min + rng.gen_range(0..range);
        if boosted {
            members = (members + 5).min(15);
        }

        let gen_skill = |rng: &mut dyn rand::RngCore, boosted: bool| -> i32 {
            let mut val = (1 + rng.gen_range(0..30)) + 20;
            // 20% chance of bonus
            if rng.gen_range(0..5) == 0 {
                val += 1 + rng.gen_range(0..10);
            }
            if boosted {
                val += 10 + rng.gen_range(0..11);
            }
            val.min(100)
        };

        Gang {
            name,
            num_members: members.clamp(1, 15),
            max_members: 15,
            combat_skill: gen_skill(rng, boosted),
            magic_skill: gen_skill(rng, boosted),
            intelligence: gen_skill(rng, boosted),
            agility: gen_skill(rng, boosted),
            constitution: gen_skill(rng, boosted),
            charisma: gen_skill(rng, boosted),
            ..Gang::default()
        }
    }

    /// Boost a gang skill using the C++ formula: median = (70/x)^2, capped at 5.
    pub fn boost_gang_skill(current: i32, rng: &mut dyn rand::RngCore) -> i32 {
        if current <= 0 {
            return current + 3;
        }
        let x = current as f64;
        let median = ((70.0 / x) * (70.0 / x)).min(5.0);
        let low = median * 0.5;
        let high = median * 1.5;
        let boost = low + rng.gen::<f64>() * (high - low);
        let int_part = boost as i32;
        let frac = boost - int_part as f64;
        // Fractional part = probability of +1
        let extra = if rng.gen::<f64>() < frac { 1 } else { 0 };
        (current + int_part + extra).min(100)
    }

    /// Populate the recruit list. Matches C++ weekly_recruit_update.
    #[allow(clippy::too_many_arguments)]
    pub fn weekly_recruit_update(
        &mut self,
        max_recruit: i32,
        start_random: i32,
        start_boosted: i32,
        init_member_min: i32,
        init_member_max: i32,
        add_new_min: i32,
        add_new_max: i32,
        chance_remove: i32,
        gang_names: &[String],
        rng: &mut dyn rand::RngCore,
    ) {
        // Remove unwanted gangs from recruit list
        self.recruit_list
            .retain(|_| rng.gen_range(0..100) >= chance_remove);

        // Add new random gangs
        let add_count = if add_new_max > add_new_min {
            rng.gen_range(add_new_min..=add_new_max)
        } else {
            add_new_min
        };
        for _ in 0..add_count {
            if self.recruit_list.len() >= max_recruit as usize {
                break;
            }
            let name = if gang_names.is_empty() {
                format!("Gang #{}", rng.gen_range(100..999))
            } else {
                gang_names[rng.gen_range(0..gang_names.len())].clone()
            };
            let gang = Self::generate_gang(name, init_member_min, init_member_max, false, rng);
            self.recruit_list.push(gang);
        }

        // Fill initial recruit list if empty
        if self.recruit_list.is_empty() {
            for i in 0..(start_random + start_boosted) {
                if self.recruit_list.len() >= max_recruit as usize {
                    break;
                }
                let name = if gang_names.is_empty() {
                    format!("Gang #{}", rng.gen_range(100..999))
                } else {
                    gang_names[rng.gen_range(0..gang_names.len())].clone()
                };
                let boosted = i >= start_random;
                let gang =
                    Self::generate_gang(name, init_member_min, init_member_max, boosted, rng);
                self.recruit_list.push(gang);
            }
        }
    }

    /// Hire a gang from the recruit list. Returns false if invalid index.
    pub fn hire_gang(&mut self, index: usize) -> bool {
        if index >= self.recruit_list.len() {
            return false;
        }
        let mut gang = self.recruit_list.remove(index);
        gang.hired = true;
        self.hired_gangs.push(gang);
        true
    }

    /// Fire a hired gang.
    pub fn fire_gang(&mut self, index: usize) {
        if index < self.hired_gangs.len() {
            self.hired_gangs.remove(index);
        }
    }

    /// Set a gang's mission.
    pub fn set_mission(&mut self, gang_index: usize, mission: GangMission) {
        if let Some(gang) = self.hired_gangs.get_mut(gang_index) {
            gang.mission = mission;
        }
    }

    /// Total weekly wages for all hired gangs. C++: 90 per gang.
    pub fn total_goon_wages(&self) -> f64 {
        self.hired_gangs.len() as f64 * 90.0
    }

    /// Get count of gangs on a specific mission.
    pub fn gangs_on_mission(&self, mission: GangMission) -> usize {
        self.hired_gangs
            .iter()
            .filter(|g| g.mission == mission)
            .count()
    }

    /// Get guard power (sum of guarding gang power).
    pub fn guard_power(&self) -> i32 {
        self.hired_gangs
            .iter()
            .filter(|g| g.mission == GangMission::Guarding)
            .map(|g| g.power())
            .sum()
    }

    /// Process a recruiting gang: +3 members/week.
    fn process_recruit(gang: &mut Gang) {
        gang.num_members = (gang.num_members + 3).min(gang.max_members);
    }

    /// Process auto-recruit: if ≤5 members, switch to recruit; return to prev at 15.
    fn check_auto_recruit(gang: &mut Gang) {
        if gang.auto_recruit && gang.num_members <= 5 && gang.mission != GangMission::Recruit {
            gang.prev_mission = Some(gang.mission);
            gang.mission = GangMission::Recruit;
        }
        if gang.mission == GangMission::Recruit
            && gang.num_members >= gang.max_members
            && gang.prev_mission.is_some()
        {
            gang.mission = gang.prev_mission.take().unwrap_or(GangMission::Guarding);
        }
    }

    /// Passive recruitment: +1 if no combat this week.
    fn passive_recruit(gang: &mut Gang) {
        if !gang.saw_combat && gang.num_members < gang.max_members {
            gang.num_members += 1;
        }
    }

    /// Process training mission for a gang.
    fn process_training(gang: &mut Gang, rng: &mut dyn rand::RngCore) {
        let skills_to_boost = rng.gen_range(2..=4);
        for _ in 0..skills_to_boost {
            let boosts = rng.gen_range(1..=3);
            // Pick skill weighted toward existing strengths
            let skill_idx = Self::pick_weighted_skill(gang, rng);
            for _ in 0..boosts {
                match skill_idx {
                    0 => gang.combat_skill = Self::boost_gang_skill(gang.combat_skill, rng),
                    1 => gang.magic_skill = Self::boost_gang_skill(gang.magic_skill, rng),
                    2 => gang.intelligence = Self::boost_gang_skill(gang.intelligence, rng),
                    3 => gang.agility = Self::boost_gang_skill(gang.agility, rng),
                    4 => gang.constitution = Self::boost_gang_skill(gang.constitution, rng),
                    _ => gang.charisma = Self::boost_gang_skill(gang.charisma, rng),
                }
            }
        }
    }

    /// Weighted random skill selection based on current values (C++ BoostGangRandomSkill).
    fn pick_weighted_skill(gang: &Gang, rng: &mut dyn rand::RngCore) -> usize {
        let skills = [
            gang.combat_skill as u64,
            gang.magic_skill as u64,
            gang.intelligence as u64,
            gang.agility as u64,
            gang.constitution as u64,
            gang.charisma as u64,
        ];
        let weights: Vec<u64> = skills.iter().map(|&s| s * s).collect();
        let total: u64 = weights.iter().sum();
        if total == 0 {
            return rng.gen_range(0..6);
        }
        let mut roll = rng.gen_range(0..total);
        for (i, &w) in weights.iter().enumerate() {
            if roll < w {
                return i;
            }
            roll -= w;
        }
        5
    }

    /// Process all gang missions for the week.
    pub fn process_missions(
        &mut self,
        player: &mut crate::player::Player,
        rng: &mut dyn rand::RngCore,
    ) -> Vec<GangMissionResult> {
        let mut results = Vec::new();

        // Reset combat flags
        for gang in &mut self.hired_gangs {
            gang.saw_combat = false;
        }

        for i in 0..self.hired_gangs.len() {
            Self::check_auto_recruit(&mut self.hired_gangs[i]);

            let result = match self.hired_gangs[i].mission {
                GangMission::Recruit => {
                    Self::process_recruit(&mut self.hired_gangs[i]);
                    GangMissionResult::default()
                }
                GangMission::Training => {
                    Self::process_training(&mut self.hired_gangs[i], rng);
                    GangMissionResult::default()
                }
                GangMission::Guarding | GangMission::SpyGirls => {
                    // Passive — handled during girl processing
                    GangMissionResult::default()
                }
                GangMission::Extortion => self.process_extortion(i, player, rng),
                GangMission::PettyTheft => self.process_petty_theft(i, player, rng),
                GangMission::GrandTheft => self.process_grand_theft(i, player, rng),
                GangMission::Kidnap => self.process_kidnap(i, rng),
                GangMission::Catacombs => self.process_catacombs(i, rng),
                GangMission::Sabotage => self.process_sabotage(i, player, rng),
                GangMission::CaptureGirl => {
                    // Handled during runaway processing
                    GangMissionResult::default()
                }
            };
            results.push(result);

            // Passive recruitment for non-combat gangs
            Self::passive_recruit(&mut self.hired_gangs[i]);
        }

        results
    }

    fn process_extortion(
        &mut self,
        _gang_idx: usize,
        player: &mut crate::player::Player,
        rng: &mut dyn rand::RngCore,
    ) -> GangMissionResult {
        let mut result = GangMissionResult::default();
        player.adjust_disposition(-1);
        player.adjust_customer_fear(1);
        player.adjust_suspicion(1);

        // Gain territories
        let territories = rng.gen_range(0..3);
        result.territories_gained = territories;
        player.businesses_extort += territories;
        result
            .events
            .push(format!("Gang extorted {} businesses", territories));
        result
    }

    fn process_petty_theft(
        &mut self,
        gang_idx: usize,
        player: &mut crate::player::Player,
        rng: &mut dyn rand::RngCore,
    ) -> GangMissionResult {
        let mut result = GangMissionResult::default();
        player.adjust_disposition(-1);
        player.adjust_customer_fear(1);
        player.adjust_suspicion(1);

        let ev = rng.gen_range(0..100);
        if ev < 60 {
            // Successful theft
            let targets = [130, 160, 500, 600, 800];
            let target = targets[rng.gen_range(0..targets.len())];
            let gold = rng.gen_range(1..=target) as i64;
            result.gold_earned = gold;
            result.events.push(format!("Gang stole {} gold", gold));
        } else if ev < 90 {
            // Losses
            let gang = &mut self.hired_gangs[gang_idx];
            if rng.gen_range(0..100) > gang.combat_skill {
                gang.num_members = (gang.num_members - 1).max(1);
                result.members_lost = 1;
                result
                    .events
                    .push("Lost a gang member during theft".to_string());
            }
            gang.saw_combat = true;
        } else {
            // Rival fight
            self.hired_gangs[gang_idx].saw_combat = true;
            result
                .events
                .push("Ran into rival gang during theft".to_string());
        }
        result
    }

    fn process_grand_theft(
        &mut self,
        gang_idx: usize,
        player: &mut crate::player::Player,
        rng: &mut dyn rand::RngCore,
    ) -> GangMissionResult {
        let mut result = GangMissionResult::default();
        player.adjust_disposition(-1);
        player.adjust_customer_fear(1);
        player.adjust_suspicion(1);

        let ev = rng.gen_range(0..100);
        if ev < 40 {
            // Successful robbery
            let targets = [400, 600, 800, 1000, 2000];
            let target = targets[rng.gen_range(0..targets.len())];
            let gold = rng.gen_range(1..=target) as i64;
            result.gold_earned = gold;
            result.events.push(format!("Gang robbed {} gold", gold));
        } else if ev < 90 {
            // Losses
            let gang = &mut self.hired_gangs[gang_idx];
            if rng.gen_range(0..100) > gang.combat_skill {
                gang.num_members = (gang.num_members - 1).max(1);
                result.members_lost = 1;
                result
                    .events
                    .push("Lost a gang member during robbery".to_string());
            }
            gang.saw_combat = true;
        } else {
            self.hired_gangs[gang_idx].saw_combat = true;
            result
                .events
                .push("Ran into rival gang during robbery".to_string());
        }
        result
    }

    fn process_kidnap(
        &mut self,
        gang_idx: usize,
        rng: &mut dyn rand::RngCore,
    ) -> GangMissionResult {
        let mut result = GangMissionResult::default();

        // 25% chance to find a girl
        if rng.gen_range(0..100) < 25 {
            let gang = &self.hired_gangs[gang_idx];
            // Attempt to convince: charisma check
            if rng.gen_range(0..100) < gang.charisma {
                result
                    .events
                    .push("Gang convinced a girl to join".to_string());
            } else if self.nets > 0 {
                self.nets -= 1;
                result
                    .events
                    .push("Gang captured a girl with a net".to_string());
            } else {
                // Combat attempt
                self.hired_gangs[gang_idx].saw_combat = true;
                if rng.gen_range(0..100) < self.hired_gangs[gang_idx].combat_skill {
                    result
                        .events
                        .push("Gang captured a girl by force".to_string());
                } else {
                    result.events.push("Girl escaped the gang".to_string());
                }
            }
        } else {
            result
                .events
                .push("Gang found no suitable targets".to_string());
        }
        result
    }

    fn process_catacombs(
        &mut self,
        gang_idx: usize,
        rng: &mut dyn rand::RngCore,
    ) -> GangMissionResult {
        let mut result = GangMissionResult::default();
        let gang = &mut self.hired_gangs[gang_idx];
        gang.saw_combat = true;

        // Casualties: each member rolls combat check
        let mut casualties = 0;
        for _ in 0..gang.num_members {
            if rng.gen_range(0..100) >= gang.combat_skill {
                casualties += 1;
            }
        }
        // Healing potions can save
        let potions_available = self.healing_potions.min(casualties);
        casualties -= potions_available;
        self.healing_potions -= potions_available;
        gang.num_members = (gang.num_members - casualties).max(1);
        result.members_lost = casualties;

        // Loot
        let gold = (rng.gen_range(0..700) + 300) as i64;
        result.gold_earned = gold;

        // Items (random chance)
        let mut item_count = 0;
        while rng.gen_range(0..100) < 60 {
            item_count += 1;
        }
        if item_count > 0 {
            result
                .events
                .push(format!("Found {} items in the catacombs", item_count));
        }

        // 40% chance monster girl
        if rng.gen_range(0..100) < 40 {
            result
                .events
                .push("Found a monster girl in the catacombs".to_string());
        }

        result.events.push(format!(
            "Catacombs: earned {} gold, lost {} members",
            gold, casualties
        ));
        result
    }

    fn process_sabotage(
        &mut self,
        gang_idx: usize,
        player: &mut crate::player::Player,
        rng: &mut dyn rand::RngCore,
    ) -> GangMissionResult {
        let mut result = GangMissionResult::default();
        self.hired_gangs[gang_idx].saw_combat = true;

        // 30% chance of finding nothing
        if rng.gen_range(0..100) < 30 {
            result
                .events
                .push("Gang found no targets to sabotage".to_string());
            return result;
        }

        let gang = &self.hired_gangs[gang_idx];
        // Destroy businesses: 1 + random(intel/4)
        let intel = gang.intelligence;
        let destroyed = 1 + rng.gen_range(0..=(intel / 4).max(1));
        result
            .events
            .push(format!("Destroyed {} rival businesses", destroyed));

        // Steal gold
        let max_gold = ((2 + intel / 4) * 400).max(44);
        let gold = (rng.gen_range(0..max_gold) + 44) as i64;
        result.gold_earned = gold;

        player.adjust_suspicion(2);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_gang() {
        let mut rng = rand::thread_rng();
        let gang = GangManager::generate_gang("Test".to_string(), 1, 10, false, &mut rng);
        assert!(gang.num_members >= 1 && gang.num_members <= 15);
        assert!(gang.combat_skill >= 21 && gang.combat_skill <= 100);
    }

    #[test]
    fn test_hire_fire() {
        let mut mgr = GangManager::new();
        mgr.recruit_list.push(Gang {
            name: "Bloods".to_string(),
            ..Gang::default()
        });
        assert!(mgr.hire_gang(0));
        assert_eq!(mgr.hired_gangs.len(), 1);
        assert_eq!(mgr.recruit_list.len(), 0);
        mgr.fire_gang(0);
        assert_eq!(mgr.hired_gangs.len(), 0);
    }

    #[test]
    fn test_boost_skill() {
        let mut rng = rand::thread_rng();
        let boosted = GangManager::boost_gang_skill(30, &mut rng);
        assert!(boosted > 30);
        // High skill should still improve (slowly)
        let high_boosted = GangManager::boost_gang_skill(90, &mut rng);
        assert!(high_boosted >= 90);
    }

    #[test]
    fn test_goon_wages() {
        let mut mgr = GangManager::new();
        mgr.hired_gangs.push(Gang::default());
        mgr.hired_gangs.push(Gang::default());
        assert_eq!(mgr.total_goon_wages(), 180.0);
    }
}
