use rand::Rng;
use wm_core::enums::{JobType, Skill, Stat};
use wm_core::girl::Girl;

use crate::brothel::Brothel;
use crate::girls::GirlManager;

use super::{Job, JobResult};

pub struct JobFightBeasts;
impl Job for JobFightBeasts {
    fn job_type(&self) -> JobType {
        JobType::FightBeasts
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let combat = GirlManager::get_skill(girl, Skill::Combat);
        let magic = GirlManager::get_skill(girl, Skill::Magic);
        let best = combat.max(magic);

        // Win chance: combat-based
        let win = rng.gen_range(0..100) < 30 + best / 2;
        if win {
            result.gold_earned = 30 + best / 2;
            result.events.push("She defeated the beast!".to_string());
            if rng.gen_range(0..100) < 30 {
                GirlManager::update_stat(girl, Stat::Fame, 1);
            }
        } else {
            let dmg = rng.gen_range(5..20);
            GirlManager::update_stat(girl, Stat::Health, -dmg);
            result
                .events
                .push(format!("She lost the fight (-{dmg} health)."));
        }

        let tiredness = (12 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(2);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, rng.gen_range(3..8));
        if rng.gen_range(0..100) < 30 {
            GirlManager::update_skill(girl, Skill::Combat, 1);
        }
        result
    }
}

pub struct JobWrestle;
impl Job for JobWrestle {
    fn job_type(&self) -> JobType {
        JobType::Wrestle
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let combat = GirlManager::get_skill(girl, Skill::Combat);
        let agility = GirlManager::get_stat(girl, Stat::Agility);
        let best = (combat + agility) / 2;

        let win = rng.gen_range(0..100) < 30 + best / 2;
        if win {
            result.gold_earned = 25 + best / 3;
            result
                .events
                .push("She won the wrestling match!".to_string());
        } else {
            let dmg = rng.gen_range(2..10);
            GirlManager::update_stat(girl, Stat::Health, -dmg);
            result
                .events
                .push("She lost the wrestling match.".to_string());
        }

        let tiredness = (10 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(2);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, rng.gen_range(3..6));
        if rng.gen_range(0..100) < 25 {
            GirlManager::update_skill(girl, Skill::Combat, 1);
        }
        result
    }
}

pub struct JobFightToDeath;
impl Job for JobFightToDeath {
    fn job_type(&self) -> JobType {
        JobType::FightToDeath
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let combat = GirlManager::get_skill(girl, Skill::Combat);
        let magic = GirlManager::get_skill(girl, Skill::Magic);
        let best = combat.max(magic);

        // Higher stakes: more gold, more danger
        let win = rng.gen_range(0..100) < 20 + best / 2;
        if win {
            result.gold_earned = 50 + best;
            GirlManager::update_stat(girl, Stat::Fame, 2);
            result
                .events
                .push("She killed her opponent in the arena!".to_string());
        } else {
            // Severe injury
            let dmg = rng.gen_range(15..40);
            GirlManager::update_stat(girl, Stat::Health, -dmg);
            result.events.push(format!(
                "She was badly injured in the arena (-{dmg} health)."
            ));
            // Could die if health drops to 0
        }

        let tiredness = (15 - GirlManager::get_stat(girl, Stat::Constitution) / 8).max(3);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, rng.gen_range(5..12));
        GirlManager::update_skill(girl, Skill::Combat, rng.gen_range(1..4));
        result
    }
}

pub struct JobFightVolunteers;
impl Job for JobFightVolunteers {
    fn job_type(&self) -> JobType {
        JobType::FightVolunteers
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let combat = GirlManager::get_skill(girl, Skill::Combat);

        let win = rng.gen_range(0..100) < 35 + combat / 2;
        if win {
            result.gold_earned = 20 + combat / 3;
            result.events.push("She beat the volunteer!".to_string());
        } else {
            let dmg = rng.gen_range(3..12);
            GirlManager::update_stat(girl, Stat::Health, -dmg);
            result.events.push("She lost to the volunteer.".to_string());
        }

        let tiredness = (10 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(2);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, rng.gen_range(3..7));
        if rng.gen_range(0..100) < 30 {
            GirlManager::update_skill(girl, Skill::Combat, 1);
        }
        result
    }
}

pub struct JobCollectBets;
impl Job for JobCollectBets {
    fn job_type(&self) -> JobType {
        JobType::CollectBets
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let charisma = GirlManager::get_stat(girl, Stat::Charisma);
        let intelligence = GirlManager::get_stat(girl, Stat::Intelligence);
        result.gold_earned = 15 + (charisma + intelligence) / 4;
        let tiredness = (5 - GirlManager::get_stat(girl, Stat::Constitution) / 20).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 2);
        if rng.gen_range(0..100) < 25 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result
            .events
            .push("She collected bets at the arena.".to_string());
        result
    }
}
