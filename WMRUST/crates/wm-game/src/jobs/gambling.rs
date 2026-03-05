use rand::Rng;
use wm_core::enums::{JobType, Skill, Stat};
use wm_core::girl::Girl;

use crate::brothel::Brothel;
use crate::girls::GirlManager;

use super::{Job, JobResult};

pub struct JobCustomerService;
impl Job for JobCustomerService {
    fn job_type(&self) -> JobType { JobType::CustomerService }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let service = GirlManager::get_skill(girl, Skill::Service);
        let charisma = GirlManager::get_stat(girl, Stat::Charisma);
        result.gold_earned = 10 + (service + charisma) / 5;
        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        if rng.gen_range(0..100) < 30 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result.events.push("She provided customer service.".to_string());
        result
    }
}

pub struct JobWhoreGambHall;
impl Job for JobWhoreGambHall {
    fn job_type(&self) -> JobType { JobType::WhoreGambHall }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let ask_price = GirlManager::get_stat(girl, Stat::AskPrice).max(1);
        let num = rng.gen_range(1..4);
        result.gold_earned = ask_price * num + 30; // +30 hall bonus
        GirlManager::update_skill(girl, Skill::NormalSex, rng.gen_range(1..4));
        GirlManager::update_temp_stat(girl, Stat::Libido, -4 * num);
        let tiredness = (10 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 4);
        result.events.push(format!("She slept with {num} gambling hall customers."));
        result
    }
}

pub struct JobDealer;
impl Job for JobDealer {
    fn job_type(&self) -> JobType { JobType::Dealer }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let intelligence = GirlManager::get_stat(girl, Stat::Intelligence);
        result.gold_earned = 15 + intelligence / 3;
        let tiredness = (5 - GirlManager::get_stat(girl, Stat::Constitution) / 20).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        if rng.gen_range(0..100) < 25 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result.events.push("She dealt cards at the gambling hall.".to_string());
        result
    }
}

pub struct JobEntertainment;
impl Job for JobEntertainment {
    fn job_type(&self) -> JobType { JobType::Entertainment }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let charisma = GirlManager::get_stat(girl, Stat::Charisma);
        let beauty = GirlManager::get_stat(girl, Stat::Beauty);
        result.gold_earned = 15 + (charisma + beauty) / 5;
        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_stat(girl, Stat::Fame, 1);
        }
        result.events.push("She entertained the gambling hall patrons.".to_string());
        result
    }
}

pub struct JobXXXEntertainment;
impl Job for JobXXXEntertainment {
    fn job_type(&self) -> JobType { JobType::XXXEntertainment }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let charisma = GirlManager::get_stat(girl, Stat::Charisma);
        let beauty = GirlManager::get_stat(girl, Stat::Beauty);
        result.gold_earned = 25 + (charisma + beauty) / 4;
        GirlManager::update_temp_stat(girl, Stat::Libido, -3);
        let tiredness = (8 - GirlManager::get_stat(girl, Stat::Constitution) / 12).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 4);
        if rng.gen_range(0..100) < 35 {
            GirlManager::update_skill(girl, Skill::Strip, 1);
        }
        result.events.push("She performed XXX entertainment.".to_string());
        result
    }
}
