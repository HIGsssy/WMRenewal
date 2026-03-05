use rand::Rng;
use wm_core::enums::{JobType, Skill, Stat, Status};
use wm_core::girl::Girl;

use crate::brothel::Brothel;
use crate::girls::GirlManager;

use super::{Job, JobResult};

pub struct JobDoctor;
impl Job for JobDoctor {
    fn job_type(&self) -> JobType {
        JobType::Doctor
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
        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 4);
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Magic, 1);
        }
        result
            .events
            .push("She worked as the clinic doctor.".to_string());
        result
    }
}

pub struct JobGetAbort;
impl Job for JobGetAbort {
    fn job_type(&self) -> JobType {
        JobType::GetAbort
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        _rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        // Remove pregnancy status
        GirlManager::remove_status(girl, Status::Pregnant);
        GirlManager::remove_status(girl, Status::PregnantByPlayer);
        GirlManager::remove_status(girl, Status::Inseminated);
        girl.weeks_pregnant = 0;
        GirlManager::update_stat(girl, Stat::Health, -10);
        GirlManager::update_stat(girl, Stat::Happiness, -15);
        GirlManager::update_stat(girl, Stat::Spirit, -5);
        result.gold_earned = -50; // Cost of procedure
        result.events.push("She had an abortion.".to_string());
        result
    }
}

pub struct JobPhysicalSurgery;
impl Job for JobPhysicalSurgery {
    fn job_type(&self) -> JobType {
        JobType::PhysicalSurgery
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        // Cosmetic surgery: boost beauty at cost of gold and health
        let beauty_gain = rng.gen_range(1..5);
        GirlManager::update_stat(girl, Stat::Beauty, beauty_gain);
        GirlManager::update_stat(girl, Stat::Health, -15);
        GirlManager::update_stat(girl, Stat::Tiredness, 20);
        result.gold_earned = -100; // Surgery cost
        result
            .events
            .push(format!("She had cosmetic surgery (+{beauty_gain} beauty)."));
        result
    }
}

pub struct JobHealing;
impl Job for JobHealing {
    fn job_type(&self) -> JobType {
        JobType::Healing
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        // Girl provides healing services to other girls (or is being healed)
        let magic = GirlManager::get_skill(girl, Skill::Magic);
        let heal = 10 + magic / 5;
        GirlManager::update_stat(girl, Stat::Health, heal);
        GirlManager::update_stat(girl, Stat::Tiredness, -10);
        GirlManager::update_stat(girl, Stat::Mana, -10);
        GirlManager::update_stat(girl, Stat::Exp, 2);
        // Cure diseases
        if magic > 50 && rng.gen_range(0..100) < magic / 2 {
            GirlManager::remove_trait(girl, "AIDS");
            GirlManager::remove_trait(girl, "Chlamydia");
            GirlManager::remove_trait(girl, "Syphilis");
            GirlManager::remove_status(girl, Status::Poisoned);
            GirlManager::remove_status(girl, Status::BadlyPoisoned);
            result
                .events
                .push("She was cured of her ailments!".to_string());
        } else {
            result.events.push("She received healing.".to_string());
        }
        result
    }
}

pub struct JobRepairShop;
impl Job for JobRepairShop {
    fn job_type(&self) -> JobType {
        JobType::RepairShop
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let intelligence = GirlManager::get_stat(girl, Stat::Intelligence);
        result.gold_earned = 15 + intelligence / 4;
        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result.events.push("She repaired equipment.".to_string());
        result
    }
}
