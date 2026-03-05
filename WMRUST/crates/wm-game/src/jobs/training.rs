use rand::Rng;
use wm_core::enums::{JobType, Skill, Stat};
use wm_core::girl::Girl;

use crate::brothel::Brothel;
use crate::girls::GirlManager;

use super::{Job, JobResult};

/// Shared teaching logic: girl teaches a specific skill.
fn teach_skill(
    girl: &mut Girl,
    skill: Skill,
    rng: &mut dyn rand::RngCore,
) -> JobResult {
    let mut result = JobResult::default();
    let skill_val = GirlManager::get_skill(girl, skill);
    // Teacher needs at least 50 skill to teach effectively
    let effectiveness = (skill_val - 30).max(0) / 10;
    result.gold_earned = 10 + effectiveness * 3;

    let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
    GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
    GirlManager::update_stat(girl, Stat::Exp, 3);

    // Small chance to improve own skill while teaching
    if rng.gen_range(0..100) < 15 {
        GirlManager::update_skill(girl, skill, 1);
    }
    // Service gain from teaching
    if rng.gen_range(0..100) < 20 {
        GirlManager::update_skill(girl, Skill::Service, 1);
    }

    result.events.push(format!("She taught {:?}.", skill));
    result
}

pub struct JobTeachBDSM;
impl Job for JobTeachBDSM {
    fn job_type(&self) -> JobType { JobType::TeachBDSM }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        teach_skill(girl, Skill::BDSM, rng)
    }
}

pub struct JobTeachSex;
impl Job for JobTeachSex {
    fn job_type(&self) -> JobType { JobType::TeachSex }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        teach_skill(girl, Skill::NormalSex, rng)
    }
}

pub struct JobTeachBeast;
impl Job for JobTeachBeast {
    fn job_type(&self) -> JobType { JobType::TeachBeast }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        teach_skill(girl, Skill::Beastiality, rng)
    }
}

pub struct JobTeachMagic;
impl Job for JobTeachMagic {
    fn job_type(&self) -> JobType { JobType::TeachMagic }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        teach_skill(girl, Skill::Magic, rng)
    }
}

pub struct JobTeachCombat;
impl Job for JobTeachCombat {
    fn job_type(&self) -> JobType { JobType::TeachCombat }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        teach_skill(girl, Skill::Combat, rng)
    }
}

pub struct JobDaycare;
impl Job for JobDaycare {
    fn job_type(&self) -> JobType { JobType::Daycare }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let tiredness = (4 - GirlManager::get_stat(girl, Stat::Constitution) / 25).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Happiness, 2);
        GirlManager::update_stat(girl, Stat::Exp, 1);
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result.events.push("She watched over the children.".to_string());
        result
    }
}

pub struct JobSchooling;
impl Job for JobSchooling {
    fn job_type(&self) -> JobType { JobType::Schooling }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let intel = GirlManager::get_stat(girl, Stat::Intelligence);
        let tiredness = (5 - GirlManager::get_stat(girl, Stat::Constitution) / 20).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        // Schooling improves intelligence
        if rng.gen_range(0..100) < 40 {
            GirlManager::update_stat(girl, Stat::Intelligence, rng.gen_range(1..3));
        }
        GirlManager::update_stat(girl, Stat::Exp, 4);
        result.gold_earned = 5 + intel / 10;
        result.events.push("She attended school.".to_string());
        result
    }
}

pub struct JobTeachDancing;
impl Job for JobTeachDancing {
    fn job_type(&self) -> JobType { JobType::TeachDancing }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        teach_skill(girl, Skill::Strip, rng)
    }
}

pub struct JobTeachService;
impl Job for JobTeachService {
    fn job_type(&self) -> JobType { JobType::TeachService }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        teach_skill(girl, Skill::Service, rng)
    }
}

pub struct JobTrain;
impl Job for JobTrain {
    fn job_type(&self) -> JobType { JobType::Train }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        // Same as general training (solo)
        let mut result = JobResult::default();
        let tiredness = (10 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_temp_stat(girl, Stat::Libido, 2);

        if rng.gen_range(0..100) < 50 {
            result.events.push("Training was uneventful.".to_string());
            return result;
        }

        let skills = Skill::ALL;
        let skill = skills[rng.gen_range(0..skills.len())];
        let gain = 1 + rng.gen_range(0..3);
        GirlManager::update_skill(girl, skill, gain);
        GirlManager::update_stat(girl, Stat::Exp, rng.gen_range(3..8));
        result.events.push(format!("Trained {:?} +{gain}.", skill));
        result
    }
}
