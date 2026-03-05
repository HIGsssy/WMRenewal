use rand::Rng;
use wm_core::enums::{JobType, Skill, Stat};
use wm_core::girl::Girl;

use crate::brothel::Brothel;
use crate::girls::GirlManager;

use super::{Job, JobResult};

pub struct JobCollectDonations;
impl Job for JobCollectDonations {
    fn job_type(&self) -> JobType {
        JobType::CollectDonations
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let charisma = GirlManager::get_stat(girl, Stat::Charisma);
        let beauty = GirlManager::get_stat(girl, Stat::Beauty);
        result.gold_earned = 5 + (charisma + beauty) / 6;
        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 2);
        if rng.gen_range(0..100) < 25 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result.events.push("She collected donations.".to_string());
        result
    }
}

pub struct JobFeedPoor;
impl Job for JobFeedPoor {
    fn job_type(&self) -> JobType {
        JobType::FeedPoor
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        // Feeding poor costs gold but improves disposition
        result.gold_earned = -10; // Cost
        let tiredness = (7 - GirlManager::get_stat(girl, Stat::Constitution) / 14).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Happiness, 3);
        GirlManager::update_stat(girl, Stat::Exp, 2);
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result.events.push("She fed the poor.".to_string());
        result
    }
}

pub struct JobMakeItems;
impl Job for JobMakeItems {
    fn job_type(&self) -> JobType {
        JobType::MakeItems
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let intelligence = GirlManager::get_stat(girl, Stat::Intelligence);
        result.gold_earned = 10 + intelligence / 4;
        let tiredness = (7 - GirlManager::get_stat(girl, Stat::Constitution) / 14).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        if rng.gen_range(0..100) < 30 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result.events.push("She made items.".to_string());
        result
    }
}

pub struct JobSellItems;
impl Job for JobSellItems {
    fn job_type(&self) -> JobType {
        JobType::SellItems
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let charisma = GirlManager::get_stat(girl, Stat::Charisma);
        let service = GirlManager::get_skill(girl, Skill::Service);
        result.gold_earned = 15 + (charisma + service) / 4;
        let tiredness = (5 - GirlManager::get_stat(girl, Stat::Constitution) / 20).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        if rng.gen_range(0..100) < 30 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result.events.push("She sold items.".to_string());
        result
    }
}

pub struct JobCommunityService;
impl Job for JobCommunityService {
    fn job_type(&self) -> JobType {
        JobType::CommunityService
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        // No direct income but improves disposition
        let tiredness = (7 - GirlManager::get_stat(girl, Stat::Constitution) / 14).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Happiness, 2);
        GirlManager::update_stat(girl, Stat::Exp, 2);
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result
            .events
            .push("She performed community service.".to_string());
        result
    }
}
