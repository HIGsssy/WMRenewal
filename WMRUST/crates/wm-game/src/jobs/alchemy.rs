use rand::Rng;
use wm_core::enums::{JobType, Skill, Stat};
use wm_core::girl::Girl;

use crate::brothel::Brothel;
use crate::girls::GirlManager;

use super::{Job, JobResult};

pub struct JobFindRegents;
impl Job for JobFindRegents {
    fn job_type(&self) -> JobType {
        JobType::FindRegents
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let intelligence = GirlManager::get_stat(girl, Stat::Intelligence);
        let magic = GirlManager::get_skill(girl, Skill::Magic);
        // Finding reagents: based on INT + Magic
        result.gold_earned = 10 + (intelligence + magic) / 5;
        let tiredness = (8 - GirlManager::get_stat(girl, Stat::Constitution) / 12).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        // 10% chance of injury while foraging
        if rng.gen_range(0..100) < 10 {
            let dmg = rng.gen_range(2..8);
            GirlManager::update_stat(girl, Stat::Health, -dmg);
            result
                .events
                .push(format!("She was injured while foraging (-{dmg} health)."));
        } else {
            result
                .events
                .push("She found reagents for the lab.".to_string());
        }
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Magic, 1);
        }
        result
    }
}

pub struct JobBrewPotions;
impl Job for JobBrewPotions {
    fn job_type(&self) -> JobType {
        JobType::BrewPotions
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let intelligence = GirlManager::get_stat(girl, Stat::Intelligence);
        let magic = GirlManager::get_skill(girl, Skill::Magic);
        result.gold_earned = 20 + (intelligence + magic) / 4;
        GirlManager::update_stat(girl, Stat::Mana, -15);
        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 4);
        if rng.gen_range(0..100) < 25 {
            GirlManager::update_skill(girl, Skill::Magic, 1);
        }
        result.events.push("She brewed potions.".to_string());
        result
    }
}

pub struct JobPotionTester;
impl Job for JobPotionTester {
    fn job_type(&self) -> JobType {
        JobType::PotionTester
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        result.gold_earned = 15;
        let tiredness = (5 - GirlManager::get_stat(girl, Stat::Constitution) / 20).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 2);
        // 15% chance of bad reaction
        if rng.gen_range(0..100) < 15 {
            let dmg = rng.gen_range(3..10);
            GirlManager::update_stat(girl, Stat::Health, -dmg);
            result.events.push(format!(
                "She had a bad reaction to a potion (-{dmg} health)."
            ));
        } else {
            // 20% chance of stat boost from good potion
            if rng.gen_range(0..100) < 20 {
                let stat = [
                    Stat::Charisma,
                    Stat::Beauty,
                    Stat::Constitution,
                    Stat::Intelligence,
                ][rng.gen_range(0..4)];
                GirlManager::update_stat(girl, stat, rng.gen_range(1..4));
                result
                    .events
                    .push(format!("A potion boosted her {:?}!", stat));
            } else {
                result
                    .events
                    .push("She tested potions without incident.".to_string());
            }
        }
        result
    }
}
