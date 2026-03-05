use rand::Rng;
use wm_core::enums::{JobType, Skill, Stat};
use wm_core::girl::Girl;

use crate::brothel::Brothel;
use crate::girls::GirlManager;

use super::{Job, JobResult};

// ===== Resting / Free Time =====
// Matches C++ WorkFreetime.cpp
pub struct JobResting;
impl Job for JobResting {
    fn job_type(&self) -> JobType {
        JobType::Resting
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        GirlManager::update_stat(girl, Stat::Tiredness, -20);
        GirlManager::update_stat(girl, Stat::Happiness, 15);
        GirlManager::update_stat(girl, Stat::Health, 10);
        GirlManager::update_stat(girl, Stat::Mana, 10);
        GirlManager::update_temp_stat(girl, Stat::Libido, 5);
        GirlManager::update_stat(girl, Stat::Exp, 1);

        result.stat_changes.push((Stat::Tiredness, -20));
        result.stat_changes.push((Stat::Happiness, 15));
        result.stat_changes.push((Stat::Health, 10));
        result.events.push("She rested and recovered.".to_string());

        // 50% chance of shopping (simplified: just happiness boost)
        if rng.gen_range(0..100) < 50 {
            GirlManager::update_stat(girl, Stat::Happiness, 3);
            result.events.push("She went shopping.".to_string());
        }

        result
    }
}

// ===== Training =====
// Matches C++ WorkTraining: solo = 50% nothing, otherwise +1..4 to random skill/stat
pub struct JobTraining;
impl Job for JobTraining {
    fn job_type(&self) -> JobType {
        JobType::Training
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        // Tiredness from training
        let tiredness = (10 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(0);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_temp_stat(girl, Stat::Libido, 2);

        // Solo training: 50% chance nothing happens
        if rng.gen_range(0..100) < 50 {
            result.events.push("Training was uneventful.".to_string());
            return result;
        }

        // Pick random trainable attribute
        let trainable_stats = [Stat::Charisma, Stat::Constitution, Stat::Libido];
        let trainable_skills = [
            Skill::Anal,
            Skill::Magic,
            Skill::BDSM,
            Skill::NormalSex,
            Skill::Beastiality,
            Skill::Group,
            Skill::Lesbian,
            Skill::Service,
            Skill::Strip,
            Skill::Combat,
        ];

        let total = trainable_stats.len() + trainable_skills.len();
        let pick = rng.gen_range(0..total);
        let gain = 1 + rng.gen_range(0..3); // 1-3

        if pick < trainable_stats.len() {
            let stat = trainable_stats[pick];
            GirlManager::update_stat(girl, stat, gain);
            result.stat_changes.push((stat, gain));
            result.events.push(format!("Trained {:?} +{gain}.", stat));
        } else {
            let skill = trainable_skills[pick - trainable_stats.len()];
            GirlManager::update_skill(girl, skill, gain);
            result.skill_changes.push((skill, gain));
            result.events.push(format!("Trained {:?} +{gain}.", skill));
        }

        GirlManager::update_stat(girl, Stat::Exp, rng.gen_range(3..8));
        result
    }
}

// ===== Cleaning =====
pub struct JobCleaning;
impl Job for JobCleaning {
    fn job_type(&self) -> JobType {
        JobType::Cleaning
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        let tiredness = (10 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 2);

        // Cleaning reduces filthiness (handled by turn processor via brothel)
        // Gold: small wage
        result.gold_earned = 5;
        result.events.push("She cleaned the building.".to_string());

        // Service skill gain
        if rng.gen_range(0..100) < 30 {
            GirlManager::update_skill(girl, Skill::Service, 1);
            result.skill_changes.push((Skill::Service, 1));
        }

        result
    }
}

// ===== Security =====
pub struct JobSecurity;
impl Job for JobSecurity {
    fn job_type(&self) -> JobType {
        JobType::Security
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        let tiredness = (8 - GirlManager::get_stat(girl, Stat::Constitution) / 12).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);

        // Security effectiveness based on combat skill
        let combat = GirlManager::get_skill(girl, Skill::Combat);
        if combat > 50 {
            result
                .events
                .push("She kept the building secure.".to_string());
        } else {
            result
                .events
                .push("She did her best to keep watch.".to_string());
        }

        // Combat skill gain
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Combat, 1);
            result.skill_changes.push((Skill::Combat, 1));
        }

        result.gold_earned = 10;
        result
    }
}

// ===== Advertising =====
pub struct JobAdvertising;
impl Job for JobAdvertising {
    fn job_type(&self) -> JobType {
        JobType::Advertising
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 2);

        // Effectiveness scales with charisma + beauty
        let charisma = GirlManager::get_stat(girl, Stat::Charisma);
        let beauty = GirlManager::get_stat(girl, Stat::Beauty);
        let effectiveness = (charisma + beauty) / 2;
        result
            .events
            .push(format!("She advertised (effectiveness: {effectiveness})."));

        // Fame very small gain
        if effectiveness > 50 && rng.gen_range(0..100) < 20 {
            GirlManager::update_stat(girl, Stat::Fame, 1);
        }

        // Service skill gain
        if rng.gen_range(0..100) < 25 {
            GirlManager::update_skill(girl, Skill::Service, 1);
        }

        result
    }
}

// ===== Matron =====
pub struct JobMatron;
impl Job for JobMatron {
    fn job_type(&self) -> JobType {
        JobType::Matron
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        _rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        let tiredness = (5 - GirlManager::get_stat(girl, Stat::Constitution) / 20).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 3);

        // Matron boosts other girls (handled by turn processor)
        result
            .events
            .push("She supervised the other girls.".to_string());
        result.gold_earned = 15;
        result
    }
}

// ===== Torturer =====
pub struct JobTorturer;
impl Job for JobTorturer {
    fn job_type(&self) -> JobType {
        JobType::Torturer
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        _rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        let tiredness = (8 - GirlManager::get_stat(girl, Stat::Constitution) / 12).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);
        GirlManager::update_stat(girl, Stat::Exp, 2);

        // Actual torture logic is in DungeonManager
        result.events.push("She worked as torturer.".to_string());
        result
    }
}

// ===== Explore Catacombs =====
pub struct JobExploreCatacombs;
impl Job for JobExploreCatacombs {
    fn job_type(&self) -> JobType {
        JobType::ExploreCatacombs
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        let tiredness = (12 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(2);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);

        let combat = GirlManager::get_skill(girl, Skill::Combat);
        let magic = GirlManager::get_skill(girl, Skill::Magic);
        let best = combat.max(magic);

        // Find items/gold based on skill
        let gold = rng.gen_range(10..30) + best / 2;
        result.gold_earned = gold;

        // Danger roll: 20% chance of injury
        if rng.gen_range(0..100) < 20 {
            let dmg = rng.gen_range(5..20);
            GirlManager::update_stat(girl, Stat::Health, -dmg);
            result
                .events
                .push(format!("She was injured in the catacombs (-{dmg} health)."));
        } else {
            result
                .events
                .push(format!("She explored the catacombs and found {gold} gold."));
        }

        // Skill gains
        if rng.gen_range(0..100) < 30 {
            GirlManager::update_skill(girl, Skill::Combat, 1);
        }
        GirlManager::update_stat(girl, Stat::Exp, rng.gen_range(3..8));
        result
    }
}

// ===== Beast Capture =====
pub struct JobBeastCapture;
impl Job for JobBeastCapture {
    fn job_type(&self) -> JobType {
        JobType::BeastCapture
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        let tiredness = (10 - GirlManager::get_stat(girl, Stat::Constitution) / 10).max(2);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);

        let combat = GirlManager::get_skill(girl, Skill::Combat);
        // Capture chance based on combat skill
        if rng.gen_range(0..100) < 30 + combat / 3 {
            result.events.push("She captured a beast!".to_string());
            // Beast count managed by state
        } else {
            result
                .events
                .push("She failed to capture any beasts.".to_string());
        }

        if rng.gen_range(0..100) < 15 {
            let dmg = rng.gen_range(3..12);
            GirlManager::update_stat(girl, Stat::Health, -dmg);
            result
                .events
                .push(format!("A beast injured her (-{dmg} health)."));
        }

        GirlManager::update_stat(girl, Stat::Exp, 3);
        result
    }
}

// ===== Beast Carer =====
pub struct JobBeastCarer;
impl Job for JobBeastCarer {
    fn job_type(&self) -> JobType {
        JobType::BeastCarer
    }
    fn process(
        &self,
        girl: &mut Girl,
        _brothel: &Brothel,
        rng: &mut dyn rand::RngCore,
    ) -> JobResult {
        let mut result = JobResult::default();

        let tiredness = (6 - GirlManager::get_stat(girl, Stat::Constitution) / 15).max(1);
        GirlManager::update_stat(girl, Stat::Tiredness, tiredness);

        result.events.push("She cared for the beasts.".to_string());

        // Small beastiality skill gain
        if rng.gen_range(0..100) < 20 {
            GirlManager::update_skill(girl, Skill::Beastiality, 1);
        }

        GirlManager::update_stat(girl, Stat::Exp, 2);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resting_heals() {
        let mut girl = Girl::default();
        girl.stats[Stat::Health as usize] = 50;
        girl.stats[Stat::Tiredness as usize] = 80;
        let brothel = Brothel::default();
        let mut rng = rand::thread_rng();

        let _result = JobResting.process(&mut girl, &brothel, &mut rng);
        assert!(girl.stats[Stat::Health as usize] >= 60);
        assert!(girl.stats[Stat::Tiredness as usize] <= 60);
    }

    #[test]
    fn test_training_gives_xp() {
        let mut girl = Girl::default();
        let initial_exp = girl.stats[Stat::Exp as usize];
        let brothel = Brothel::default();
        let mut rng = rand::thread_rng();

        // Run many times to ensure at least some XP gained
        for _ in 0..20 {
            let _ = JobTraining.process(&mut girl, &brothel, &mut rng);
        }
        assert!(girl.stats[Stat::Exp as usize] > initial_exp);
    }
}
