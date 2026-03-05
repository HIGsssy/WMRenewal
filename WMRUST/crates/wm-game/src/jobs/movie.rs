use rand::Rng;
use wm_core::enums::{JobType, Skill, Stat};
use wm_core::girl::Girl;

use crate::brothel::Brothel;
use crate::girls::GirlManager;

use super::{Job, JobResult};

/// Shared film job logic. Returns base pay scaled by skill and beauty.
fn film_common(
    girl: &mut Girl,
    skill: Skill,
    base_pay: i32,
    rng: &mut dyn rand::RngCore,
) -> JobResult {
    let mut result = JobResult::default();
    let skill_val = GirlManager::get_skill(girl, skill);
    let beauty = GirlManager::get_stat(girl, Stat::Beauty);
    let fame = GirlManager::get_stat(girl, Stat::Fame);

    // Pay: base + skill/2 + beauty/4 + fame/5
    result.gold_earned = base_pay + skill_val / 2 + beauty / 4 + fame / 5;

    let tiredness = (8 - GirlManager::get_stat(girl, Stat::Constitution) / 12).max(1);
    GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
    GirlManager::update_temp_stat(girl, Stat::Libido, -3);

    // Skill gain
    let gain = if GirlManager::has_trait(girl, "Quick Learner") {
        rng.gen_range(1..5)
    } else if GirlManager::has_trait(girl, "Slow Learner") {
        rng.gen_range(0..2)
    } else {
        rng.gen_range(1..4)
    };
    GirlManager::update_skill(girl, skill, gain);
    GirlManager::update_stat(girl, Stat::Exp, rng.gen_range(3..8));

    // Fame from filming
    if rng.gen_range(0..100) < 40 {
        GirlManager::update_stat(girl, Stat::Fame, 1);
    }

    result
}

pub struct JobFilmBeast;
impl Job for JobFilmBeast {
    fn job_type(&self) -> JobType { JobType::FilmBeast }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = film_common(girl, Skill::Beastiality, 40, rng);
        GirlManager::update_stat(girl, Stat::Spirit, -1);
        result.events.push("She filmed a beast scene.".to_string());
        result
    }
}

pub struct JobFilmSex;
impl Job for JobFilmSex {
    fn job_type(&self) -> JobType { JobType::FilmSex }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = film_common(girl, Skill::NormalSex, 35, rng);
        result.events.push("She filmed a sex scene.".to_string());
        result
    }
}

pub struct JobFilmAnal;
impl Job for JobFilmAnal {
    fn job_type(&self) -> JobType { JobType::FilmAnal }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = film_common(girl, Skill::Anal, 40, rng);
        GirlManager::update_stat(girl, Stat::Spirit, -1);
        result.events.push("She filmed an anal scene.".to_string());
        result
    }
}

pub struct JobFilmLesbian;
impl Job for JobFilmLesbian {
    fn job_type(&self) -> JobType { JobType::FilmLesbian }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = film_common(girl, Skill::Lesbian, 30, rng);
        result.events.push("She filmed a lesbian scene.".to_string());
        result
    }
}

pub struct JobFilmBondage;
impl Job for JobFilmBondage {
    fn job_type(&self) -> JobType { JobType::FilmBondage }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = film_common(girl, Skill::BDSM, 40, rng);
        GirlManager::update_stat(girl, Stat::Spirit, -1);
        result.events.push("She filmed a bondage scene.".to_string());
        result
    }
}

pub struct JobFluffer;
impl Job for JobFluffer {
    fn job_type(&self) -> JobType { JobType::Fluffer }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let service = GirlManager::get_skill(girl, Skill::Service);
        result.gold_earned = 15 + service / 3;
        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 2);
        GirlManager::update_temp_stat(girl, Stat::Libido, -2);
        if rng.gen_range(0..100) < 30 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }
        result.events.push("She worked as a fluffer.".to_string());
        result
    }
}

pub struct JobCameraMage;
impl Job for JobCameraMage {
    fn job_type(&self) -> JobType { JobType::CameraMage }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let magic = GirlManager::get_skill(girl, Skill::Magic);
        result.gold_earned = 20 + magic / 3;
        GirlManager::update_stat(girl, Stat::Mana, -15);
        let tiredness = (5 - GirlManager::get_stat(girl, Stat::Constitution) / 20).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        if rng.gen_range(0..100) < 25 {
            GirlManager::update_skill(girl, Skill::Magic, 1);
        }
        result.events.push("She operated the crystal camera.".to_string());
        result
    }
}

pub struct JobCrystalPurifier;
impl Job for JobCrystalPurifier {
    fn job_type(&self) -> JobType { JobType::CrystalPurifier }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let magic = GirlManager::get_skill(girl, Skill::Magic);
        let intelligence = GirlManager::get_stat(girl, Stat::Intelligence);
        result.gold_earned = 15 + (magic + intelligence) / 4;
        GirlManager::update_stat(girl, Stat::Mana, -10);
        let tiredness = (5 - GirlManager::get_stat(girl, Stat::Constitution) / 20).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Magic, 1);
        }
        result.events.push("She purified crystals for filming.".to_string());
        result
    }
}
