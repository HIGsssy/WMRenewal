use rand::Rng;
use wm_core::enums::{JobType, Skill, Stat};
use wm_core::girl::Girl;

use crate::brothel::Brothel;
use crate::girls::GirlManager;

use super::{Job, JobResult};

/// Shared sex logic matching C++ GirlFucks.
/// Returns (customer_happiness_delta, gold_tip, events).
fn girl_fucks(
    girl: &mut Girl,
    skill: Skill,
    rng: &mut dyn rand::RngCore,
) -> (i32, i32, Vec<String>) {
    let mut events = Vec::new();
    let mut cust_happy: i32 = 0;
    let skill_val = GirlManager::get_skill(girl, skill);

    // Skill contribution to customer happiness
    if skill_val < 50 {
        cust_happy -= (100 - skill_val) / 5;
    } else {
        cust_happy += skill_val / 5;
    }

    // Fame bonus
    cust_happy += GirlManager::get_stat(girl, Stat::Fame) / 5;
    // Service bonus
    cust_happy += GirlManager::get_skill(girl, Skill::Service) / 10;

    // Trait modifiers
    if GirlManager::has_trait(girl, "Fast Orgasms") {
        cust_happy += 15;
    }
    if GirlManager::has_trait(girl, "Slow Orgasms") {
        cust_happy -= 10;
    }
    if GirlManager::has_trait(girl, "Psychic") {
        cust_happy += 10;
    }
    if GirlManager::has_trait(girl, "Fake Orgasms") {
        cust_happy += 15;
    }
    if GirlManager::has_trait(girl, "Abnormally Large Boobs") {
        cust_happy += 15;
    }
    if GirlManager::has_trait(girl, "Big Boobs") {
        cust_happy += 10;
    }

    // Disease penalties
    if GirlManager::has_trait(girl, "AIDS") {
        cust_happy -= 10;
    }
    if GirlManager::has_trait(girl, "Chlamydia") {
        cust_happy -= 20;
    }
    if GirlManager::has_trait(girl, "Syphilis") {
        cust_happy -= 10;
    }

    // Magic bonus if mana available
    let mana = GirlManager::get_stat(girl, Stat::Mana);
    if mana > 20 {
        cust_happy += GirlManager::get_skill(girl, Skill::Magic) / 10;
        GirlManager::update_stat(girl, Stat::Mana, -20);
    }

    // Inexperience penalties
    match skill {
        Skill::Anal if skill_val <= 20 => {
            GirlManager::update_stat(girl, Stat::Happiness, -3);
            GirlManager::update_stat(girl, Stat::Confidence, -1);
            GirlManager::update_stat(girl, Stat::Spirit, -3);
            GirlManager::update_stat(girl, Stat::Health, -3);
        }
        Skill::BDSM if skill_val <= 30 => {
            GirlManager::update_stat(girl, Stat::Happiness, -2);
            GirlManager::update_stat(girl, Stat::Spirit, -3);
            GirlManager::update_stat(girl, Stat::Confidence, -1);
            GirlManager::update_stat(girl, Stat::Health, -3);
        }
        Skill::NormalSex if skill_val < 10 => {
            GirlManager::update_stat(girl, Stat::Happiness, -2);
            GirlManager::update_stat(girl, Stat::Spirit, -3);
            GirlManager::update_stat(girl, Stat::Confidence, -1);
            GirlManager::update_stat(girl, Stat::Health, -3);
        }
        Skill::Beastiality if skill_val <= 30 => {
            GirlManager::update_stat(girl, Stat::Happiness, -2);
            GirlManager::update_stat(girl, Stat::Spirit, -3);
            GirlManager::update_stat(girl, Stat::Confidence, -1);
            GirlManager::update_stat(girl, Stat::Health, -3);
        }
        Skill::Group if skill_val <= 30 => {
            GirlManager::update_stat(girl, Stat::Happiness, -2);
            GirlManager::update_stat(girl, Stat::Spirit, -3);
            GirlManager::update_stat(girl, Stat::Confidence, -1);
            GirlManager::update_stat(girl, Stat::Health, -3);
        }
        _ => {}
    }

    // Spirit drain for certain acts
    match skill {
        Skill::Anal | Skill::BDSM | Skill::Beastiality => {
            GirlManager::update_stat(girl, Stat::Spirit, -1);
        }
        _ => {}
    }

    // Tiredness
    let tired = (10 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(0);
    GirlManager::update_stat(girl, Stat::Tiredness, tired);

    // Libido effect on girl happiness
    let libido = GirlManager::get_stat(girl, Stat::Libido);
    if libido > 5 {
        GirlManager::update_stat(girl, Stat::Happiness, libido / 5);
    } else {
        GirlManager::update_stat(girl, Stat::Happiness, -2);
    }

    // Skill gains per fuck
    let (skill_gain, xp_gain) = if GirlManager::has_trait(girl, "Quick Learner") {
        (rng.gen_range(1..6), rng.gen_range(1..8) * 3)
    } else if GirlManager::has_trait(girl, "Slow Learner") {
        (rng.gen_range(1..3), rng.gen_range(1..4) * 3)
    } else {
        (rng.gen_range(1..4), rng.gen_range(1..6) * 3)
    };

    GirlManager::update_skill(girl, skill, skill_gain);
    GirlManager::update_skill(girl, Skill::Service, rng.gen_range(1..4));
    GirlManager::update_stat(girl, Stat::Exp, xp_gain);

    // Libido decrease per customer
    GirlManager::update_temp_stat(girl, Stat::Libido, -4);

    // Fame gain: (cust_happy - 1) / 33
    let fame_gain = (cust_happy.max(0) - 1).max(0) / 33;
    if fame_gain > 0 {
        GirlManager::update_stat(girl, Stat::Fame, fame_gain);
    }

    // Tips: if customer very happy
    let tip = if cust_happy > 50 {
        rng.gen_range(5..21)
    } else {
        0
    };

    events.push(format!("Customer satisfaction: {cust_happy}"));
    (cust_happy, tip, events)
}

// ===== Whore (Brothel) =====
// Matches C++ WorkWhore.cpp
pub struct JobWhoreBrothel;
impl Job for JobWhoreBrothel {
    fn job_type(&self) -> JobType {
        JobType::WhoreBrothel
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        let beauty = GirlManager::get_stat(girl, Stat::Beauty).max(1);
        let charisma = GirlManager::get_stat(girl, Stat::Charisma).max(1);
        let fame = GirlManager::get_stat(girl, Stat::Fame).max(1);

        // Max customers: Beauty/50 + Charisma/50 + Fame/25, cap 10
        let num_custs = ((beauty / 50).max(1) + (charisma / 50).max(1) + fame / 25).min(10);
        let ask_price = GirlManager::get_stat(girl, Stat::AskPrice).max(1);

        let mut total_gold = 0;
        let mut total_tips = 0;

        for _ in 0..num_custs {
            // Pick sex type
            let sex_skills = [
                Skill::NormalSex,
                Skill::Anal,
                Skill::BDSM,
                Skill::Group,
                Skill::Lesbian,
            ];
            let skill = sex_skills[rng.gen_range(0..sex_skills.len())];

            let (cust_happy, tip, events) = girl_fucks(girl, skill, rng);
            result.events.extend(events);

            // Did customer pay?
            if cust_happy > 0 {
                total_gold += ask_price;
                total_tips += tip;
            } else {
                // Unhappy customer: 50% chance doesn't pay
                if rng.gen_range(0..100) < 50 {
                    result.events.push("A customer refused to pay!".to_string());
                } else {
                    total_gold += ask_price;
                }
            }
        }

        result.gold_earned = total_gold + total_tips;
        result.events.push(format!(
            "She serviced {num_custs} customers for {} gold.",
            result.gold_earned
        ));
        result
    }
}

// ===== Whore (Streets) =====
pub struct JobWhoreStreets;
impl Job for JobWhoreStreets {
    fn job_type(&self) -> JobType {
        JobType::WhoreStreets
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        let beauty = GirlManager::get_stat(girl, Stat::Beauty).max(1);
        let charisma = GirlManager::get_stat(girl, Stat::Charisma).max(1);
        let fame = GirlManager::get_stat(girl, Stat::Fame).max(1);

        // Street: 2/3 of normal customers and pay
        let num_custs =
            (((beauty / 50).max(1) + (charisma / 50).max(1) + fame / 25) * 2 / 3).min(7);
        let ask_price = (GirlManager::get_stat(girl, Stat::AskPrice).max(1) * 2) / 3;

        let mut total_gold = 0;

        for _ in 0..num_custs.max(1) {
            let skill = if rng.gen_range(0..100) < 60 {
                Skill::NormalSex
            } else {
                Skill::Anal
            };
            let (cust_happy, tip, events) = girl_fucks(girl, skill, rng);
            result.events.extend(events);

            if cust_happy > 0 {
                total_gold += ask_price + tip;
            }
        }

        // 5% chance of rival gang attack
        if rng.gen_range(0..100) < 5 {
            let dmg = rng.gen_range(0..20).max(0);
            GirlManager::update_stat(girl, Stat::Health, -dmg);
            GirlManager::update_stat(girl, Stat::Tiredness, rng.gen_range(5..15));
            result
                .events
                .push(format!("She was attacked by a rival gang! (-{dmg} health)"));
        }

        result.gold_earned = total_gold;
        result
    }
}

// ===== Brothel Stripper =====
// Matches C++ WorkStripper: 50/50 strip+sex or strip only
pub struct JobBrothelStripper;
impl Job for JobBrothelStripper {
    fn job_type(&self) -> JobType {
        JobType::BrothelStripper
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();
        let ask_price = GirlManager::get_stat(girl, Stat::AskPrice).max(1);

        if rng.gen_range(0..2) == 0 {
            // Strip + Sex
            let (_, tip, events) = girl_fucks(girl, Skill::NormalSex, rng);
            result.events.extend(events);
            result.gold_earned = ask_price + 30 + tip;
            GirlManager::update_temp_stat(girl, Stat::Libido, -4);
            result
                .events
                .push("She stripped and had sex with a customer.".to_string());
        } else {
            // Strip only
            result.gold_earned = ask_price + 10;
            let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
            GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
            result
                .events
                .push("She performed a strip show.".to_string());
        }

        // Strip skill gain
        if rng.gen_range(0..100) < 40 {
            GirlManager::update_skill(girl, Skill::Strip, 1);
        }
        GirlManager::update_stat(girl, Stat::Exp, rng.gen_range(2..6));
        result
    }
}

// ===== Masseuse =====
pub struct JobMasseuse;
impl Job for JobMasseuse {
    fn job_type(&self) -> JobType {
        JobType::Masseuse
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        let service = GirlManager::get_skill(girl, Skill::Service);
        let beauty = GirlManager::get_stat(girl, Stat::Beauty);
        let base_pay = 15 + (service + beauty) / 4;

        result.gold_earned = base_pay;

        let tiredness = (5 - GirlManager::get_stat(girl, Stat::Constitution) / 20).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);

        // Service skill gain
        if rng.gen_range(0..100) < 35 {
            GirlManager::update_skill(girl, Skill::Service, 1);
            result.skill_changes.push((Skill::Service, 1));
        }

        result
            .events
            .push(format!("She gave massages and earned {base_pay} gold."));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whore_brothel_earns_gold() {
        let mut girl = Girl::default();
        girl.stats[Stat::Beauty as usize] = 60;
        girl.stats[Stat::Charisma as usize] = 60;
        girl.stats[Stat::AskPrice as usize] = 20;
        girl.skills[Skill::NormalSex as usize] = 50;
        let brothel = Brothel::default();
        let mut rng = rand::thread_rng();

        let result = JobWhoreBrothel.process(&mut girl, &brothel, &mut rng);
        assert!(result.gold_earned > 0, "Whore should earn gold");
    }

    #[test]
    fn test_stripper_earns_gold() {
        let mut girl = Girl::default();
        girl.stats[Stat::AskPrice as usize] = 15;
        girl.skills[Skill::Strip as usize] = 40;
        girl.skills[Skill::NormalSex as usize] = 40;
        let brothel = Brothel::default();
        let mut rng = rand::thread_rng();

        let result = JobBrothelStripper.process(&mut girl, &brothel, &mut rng);
        assert!(result.gold_earned > 0);
    }
}
