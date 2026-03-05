use rand::Rng;
use wm_core::enums::{JobType, Skill, Stat};
use wm_core::girl::Girl;

use crate::brothel::Brothel;
use crate::girls::GirlManager;

use super::{Job, JobResult};

/// Shared bar work logic. Matches C++ WorkBar.
fn bar_work_common(
    girl: &mut Girl,
    rng: &mut dyn rand::RngCore,
    result: &mut JobResult,
) {
    let beauty = GirlManager::get_stat(girl, Stat::Beauty);
    let charisma = GirlManager::get_stat(girl, Stat::Charisma);
    let roll_max = ((beauty + charisma) / 4).max(1);

    result.gold_earned += 10 + rng.gen_range(0..roll_max);

    // Skill/XP gains
    let (skill_gain, xp_gain, libido_gain) = if GirlManager::has_trait(girl, "Quick Learner") {
        (4, 8, 1)
    } else if GirlManager::has_trait(girl, "Slow Learner") {
        (2, 2, 1)
    } else {
        (3, 5, 1)
    };

    let libido = if GirlManager::has_trait(girl, "Nymphomaniac") { libido_gain + 2 } else { libido_gain };

    GirlManager::update_stat(girl, Stat::Fame, 1);
    GirlManager::update_stat(girl, Stat::Exp, xp_gain);
    GirlManager::update_skill(girl, Skill::Service, skill_gain);
    GirlManager::update_temp_stat(girl, Stat::Libido, libido);

    // Enjoyment roll
    let roll = rng.gen_range(0..100);
    if roll < 6 {
        // Patron abuse
        GirlManager::update_stat(girl, Stat::Happiness, -3);
        result.events.push("A patron harassed her.".to_string());
    } else if roll < 26 {
        GirlManager::update_stat(girl, Stat::Happiness, 1);
    }
}

// ===== Barmaid =====
pub struct JobBarmaid;
impl Job for JobBarmaid {
    fn job_type(&self) -> JobType { JobType::Barmaid }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        result.gold_earned = 5; // Base barmaid pay
        bar_work_common(girl, rng, &mut result);
        let tiredness = (8 - GirlManager::get_stat(girl, Stat::Constitution) / 12).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        result.events.push("She worked as a barmaid.".to_string());
        result
    }
}

// ===== Waitress =====
pub struct JobWaitress;
impl Job for JobWaitress {
    fn job_type(&self) -> JobType { JobType::Waitress }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        result.gold_earned = 15; // Base waitress pay
        bar_work_common(girl, rng, &mut result);
        let tiredness = (8 - GirlManager::get_stat(girl, Stat::Constitution) / 12).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        result.events.push("She worked as a waitress.".to_string());
        result
    }
}

// ===== Stripper (Bar) =====
pub struct JobStripper;
impl Job for JobStripper {
    fn job_type(&self) -> JobType { JobType::Stripper }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        result.gold_earned = 30; // Base stripper pay
        bar_work_common(girl, rng, &mut result);
        let tiredness = (7 - GirlManager::get_stat(girl, Stat::Constitution) / 14).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        if rng.gen_range(0..100) < 40 {
            GirlManager::update_skill(girl, Skill::Strip, 1);
        }
        result.events.push("She stripped at the bar.".to_string());
        result
    }
}

// ===== Whore (Bar) =====
pub struct JobWhoreBar;
impl Job for JobWhoreBar {
    fn job_type(&self) -> JobType { JobType::WhoreBar }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let ask_price = GirlManager::get_stat(girl, Stat::AskPrice).max(1);
        let num = rng.gen_range(1..3); // 1-2 customers

        let mut total = 30; // Bar bonus
        for _ in 0..num {
            total += ask_price;
            // Tiredness and skill handled by sex logic
            GirlManager::update_temp_stat(girl, Stat::Libido, -4);
            let skill = if rng.gen_range(0..100) < 70 { Skill::NormalSex } else { Skill::Anal };
            GirlManager::update_skill(girl, skill, rng.gen_range(1..4));
        }

        bar_work_common(girl, rng, &mut result);
        result.gold_earned += total;
        let tiredness = (10 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        result.events.push(format!("She worked the bar and slept with {num} customers."));
        result
    }
}

// ===== Singer =====
pub struct JobSinger;
impl Job for JobSinger {
    fn job_type(&self) -> JobType { JobType::Singer }
    fn process(&self, girl: &mut Girl, _brothel: &Brothel, rng: &mut dyn rand::RngCore) -> JobResult {
        let mut result = JobResult::default();
        let charisma = GirlManager::get_stat(girl, Stat::Charisma);
        let beauty = GirlManager::get_stat(girl, Stat::Beauty);

        result.gold_earned = 20 + (charisma + beauty) / 5;
        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);

        // Fame gain if good
        if charisma > 50 && rng.gen_range(0..100) < 30 {
            GirlManager::update_stat(girl, Stat::Fame, 1);
        }

        result.events.push("She sang at the bar.".to_string());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barmaid_earns() {
        let mut girl = Girl::default();
        girl.stats[Stat::Beauty as usize] = 50;
        girl.stats[Stat::Charisma as usize] = 50;
        let brothel = Brothel::default();
        let mut rng = rand::thread_rng();

        let result = JobBarmaid.process(&mut girl, &brothel, &mut rng);
        assert!(result.gold_earned > 0);
    }
}
