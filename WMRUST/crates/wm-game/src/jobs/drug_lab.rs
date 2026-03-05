use rand::Rng;
use wm_core::enums::{JobType, Skill, Stat};
use wm_core::girl::Girl;

use crate::brothel::Brothel;
use crate::girls::GirlManager;

use super::{Job, JobResult};

pub struct JobVirasPlantFucker;
impl Job for JobVirasPlantFucker {
    fn job_type(&self) -> JobType { JobType::VirasPlantFucker }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        // Produces Viras Blood drug ingredient
        let beastiality = GirlManager::get_skill(girl, Skill::Beastiality);
        result.gold_earned = 20 + beastiality / 3;
        GirlManager::update_stat(girl, Stat::Spirit, -2);
        GirlManager::update_temp_stat(girl, Stat::Libido, -5);
        let tiredness = (10 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(2);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Beastiality, 1);
        }
        result.events.push("She worked with the Viras plants.".to_string());
        result
    }
}

pub struct JobShroudGrower;
impl Job for JobShroudGrower {
    fn job_type(&self) -> JobType { JobType::ShroudGrower }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let intelligence = GirlManager::get_stat(girl, Stat::Intelligence);
        result.gold_earned = 20 + intelligence / 4;
        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 2);
        if rng.gen_range(0..100) < 15 {
            GirlManager::update_skill(girl, Skill::Magic, 1);
        }
        result.events.push("She grew Shroud mushrooms.".to_string());
        result
    }
}

pub struct JobFairyDuster;
impl Job for JobFairyDuster {
    fn job_type(&self) -> JobType { JobType::FairyDuster }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let intelligence = GirlManager::get_stat(girl, Stat::Intelligence);
        let magic = GirlManager::get_skill(girl, Skill::Magic);
        result.gold_earned = 25 + (intelligence + magic) / 5;
        GirlManager::update_stat(girl, Stat::Mana, -10);
        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Magic, 1);
        }
        result.events.push("She processed Fairy Dust.".to_string());
        result
    }
}

pub struct JobDrugDealer;
impl Job for JobDrugDealer {
    fn job_type(&self) -> JobType { JobType::DrugDealer }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let charisma = GirlManager::get_stat(girl, Stat::Charisma);
        let intelligence = GirlManager::get_stat(girl, Stat::Intelligence);
        result.gold_earned = 30 + (charisma + intelligence) / 4;
        let tiredness = (5 - GirlManager::get_stat(girl, Stat::Constitution) / 20).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        // Risk: 5% chance of arrest (lose gold)
        if rng.gen_range(0..100) < 5 {
            result.gold_earned = 0;
            result.events.push("She was nearly caught dealing drugs!".to_string());
        } else {
            result.events.push("She dealt drugs.".to_string());
        }
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result
    }
}
