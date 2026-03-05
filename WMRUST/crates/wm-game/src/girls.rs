use rand::Rng;
use wm_core::enums::{ActionType, Skill, Stat, Status};
use wm_core::girl::Girl;
use wm_core::xml::loaders;

/// Manages the collection of all girls in the game.
#[derive(Debug)]
pub struct GirlManager {
    pub girls: Vec<Girl>,
}

impl Default for GirlManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Clamp a stat value to its valid range.
fn stat_clamp(stat: Stat, value: i32) -> i32 {
    match stat {
        // PCFear, PCLove, PCHate: 0-100
        Stat::PCFear | Stat::PCLove | Stat::PCHate => value.clamp(0, 100),
        // Health, Mana, Tiredness: 0-100
        Stat::Health | Stat::Mana | Stat::Tiredness => value.clamp(0, 100),
        // Happiness, Obedience: 0-100
        Stat::Happiness | Stat::Obedience => value.clamp(0, 100),
        // Exp: 0-255 (level up at 255)
        Stat::Exp => value.clamp(0, 255),
        // Age: 18+ (no upper cap enforced)
        Stat::Age => value.max(18),
        // Level: 0-255
        Stat::Level => value.clamp(0, 255),
        // HousePerc: 0-100
        Stat::HousePerc => value.clamp(0, 100),
        // AskPrice: 0+
        Stat::AskPrice => value.max(0),
        // Fame: 0-100
        Stat::Fame => value.clamp(0, 100),
        // All other stats: 0-100
        _ => value.clamp(0, 100),
    }
}

fn skill_clamp(value: i32) -> i32 {
    value.clamp(0, 100)
}

impl GirlManager {
    pub fn new() -> Self {
        Self { girls: Vec::new() }
    }

    pub fn add_girl(&mut self, girl: Girl) -> usize {
        let id = self.girls.len();
        self.girls.push(girl);
        id
    }

    pub fn get_girl(&self, id: usize) -> Option<&Girl> {
        self.girls.get(id)
    }

    pub fn get_girl_mut(&mut self, id: usize) -> Option<&mut Girl> {
        self.girls.get_mut(id)
    }

    pub fn remove_girl(&mut self, id: usize) -> Option<Girl> {
        if id < self.girls.len() {
            Some(self.girls.remove(id))
        } else {
            None
        }
    }

    pub fn find_by_name(&self, name: &str) -> Option<usize> {
        self.girls.iter().position(|g| g.name == name)
    }

    pub fn all_girls(&self) -> &[Girl] {
        &self.girls
    }

    pub fn count(&self) -> usize {
        self.girls.len()
    }

    pub fn load_girls_from_xml(&mut self, path: &std::path::Path) {
        match loaders::load_girls(path) {
            Ok(girls) => {
                for g in girls {
                    self.add_girl(g);
                }
            }
            Err(e) => {
                eprintln!("Failed to load girls from {}: {}", path.display(), e);
            }
        }
    }

    /// Get the effective stat value (base + permanent mods + temp mods).
    pub fn get_stat(girl: &Girl, stat: Stat) -> i32 {
        let idx = stat as usize;
        stat_clamp(
            stat,
            girl.stats[idx] + girl.stat_mods[idx] + girl.temp_stats[idx],
        )
    }

    /// Get the effective skill value (base + permanent mods + temp mods).
    pub fn get_skill(girl: &Girl, skill: Skill) -> i32 {
        let idx = skill as usize;
        skill_clamp(girl.skills[idx] + girl.skill_mods[idx] + girl.temp_skills[idx])
    }

    /// Check if a stat meets or exceeds a threshold.
    pub fn stat_check(girl: &Girl, stat: Stat, threshold: i32) -> bool {
        Self::get_stat(girl, stat) >= threshold
    }

    /// Update a girl's base stat by an amount, clamping to valid range.
    pub fn update_stat(girl: &mut Girl, stat: Stat, amount: i32) {
        let idx = stat as usize;
        girl.stats[idx] = stat_clamp(stat, girl.stats[idx] + amount);
    }

    /// Update a girl's permanent stat modifier.
    pub fn update_stat_mod(girl: &mut Girl, stat: Stat, amount: i32) {
        let idx = stat as usize;
        girl.stat_mods[idx] += amount;
    }

    /// Update a girl's temporary stat. Temp stats decay 30% per week.
    pub fn update_temp_stat(girl: &mut Girl, stat: Stat, amount: i32) {
        let idx = stat as usize;
        girl.temp_stats[idx] += amount;
    }

    /// Update a girl's base skill by an amount, clamping to 0-100.
    pub fn update_skill(girl: &mut Girl, skill: Skill, amount: i32) {
        let idx = skill as usize;
        girl.skills[idx] = skill_clamp(girl.skills[idx] + amount);
    }

    /// Update a girl's permanent skill modifier.
    pub fn update_skill_mod(girl: &mut Girl, skill: Skill, amount: i32) {
        let idx = skill as usize;
        girl.skill_mods[idx] += amount;
    }

    /// Update a girl's temporary skill. Temp skills decay 30% per week.
    pub fn update_temp_skill(girl: &mut Girl, skill: Skill, amount: i32) {
        let idx = skill as usize;
        girl.temp_skills[idx] += amount;
    }

    /// Check if girl has a named trait.
    pub fn has_trait(girl: &Girl, trait_name: &str) -> bool {
        girl.traits
            .iter()
            .any(|t| t.eq_ignore_ascii_case(trait_name))
    }

    /// Add a trait if not already present.
    pub fn add_trait(girl: &mut Girl, trait_name: &str) {
        if !Self::has_trait(girl, trait_name) {
            girl.traits.push(trait_name.to_string());
        }
    }

    /// Remove a trait by name.
    pub fn remove_trait(girl: &mut Girl, trait_name: &str) {
        girl.traits.retain(|t| !t.eq_ignore_ascii_case(trait_name));
    }

    /// Check if girl has a specific status.
    pub fn has_status(girl: &Girl, status: Status) -> bool {
        girl.status.statuses.contains(&status)
    }

    /// Add a status if not already present.
    pub fn add_status(girl: &mut Girl, status: Status) {
        if !girl.status.statuses.contains(&status) {
            girl.status.statuses.push(status);
        }
    }

    /// Remove a status.
    pub fn remove_status(girl: &mut Girl, status: Status) {
        girl.status.statuses.retain(|&s| s != status);
    }

    /// Get enjoyment for an action type.
    pub fn get_enjoyment(girl: &Girl, action: ActionType) -> i32 {
        girl.enjoyment[action as usize].clamp(-100, 100)
    }

    /// Update enjoyment for an action type.
    pub fn update_enjoyment(girl: &mut Girl, action: ActionType, amount: i32) {
        let idx = action as usize;
        girl.enjoyment[idx] = (girl.enjoyment[idx] + amount).clamp(-100, 100);
    }

    /// Calculate the rebel value (chance of disobeying). Based on C++ GetRebelValue.
    pub fn get_rebel_value(girl: &Girl) -> i32 {
        let spirit = Self::get_stat(girl, Stat::Spirit);
        let obedience = Self::get_stat(girl, Stat::Obedience);
        let happiness = Self::get_stat(girl, Stat::Happiness);
        let pcfear = Self::get_stat(girl, Stat::PCFear);
        let pchate = Self::get_stat(girl, Stat::PCHate);

        let mut rebel = spirit - obedience;
        rebel += (100 - happiness) / 5; // unhappier = more rebellious
        rebel -= pcfear / 5; // fear suppresses rebellion
        rebel += pchate / 5; // hatred fuels rebellion

        if Self::has_trait(girl, "Broken Will") {
            rebel -= 30;
        }
        if Self::has_trait(girl, "Mind Fucked") {
            rebel -= 50;
        }
        if Self::has_trait(girl, "Dependant") {
            rebel -= 20;
        }
        if Self::has_trait(girl, "Fearless") {
            rebel += 10;
        }
        if Self::has_status(girl, Status::Slave) {
            rebel -= 15;
        }

        rebel.clamp(0, 100)
    }

    /// Check if girl disobeys an action. Returns true if she refuses.
    pub fn disobey_check(girl: &Girl, _action: ActionType, rng: &mut dyn rand::RngCore) -> bool {
        let rebel = Self::get_rebel_value(girl);
        rng.gen_range(0..100) < rebel
    }

    /// Decay temporary stats and skills by 30% per week.
    pub fn decay_temp_stats(girl: &mut Girl) {
        for t in girl.temp_stats.iter_mut() {
            *t = (*t as f64 * 0.7) as i32;
        }
        for t in girl.temp_skills.iter_mut() {
            *t = (*t as f64 * 0.7) as i32;
        }
    }

    /// Level up when experience reaches 255.
    pub fn check_level_up(girl: &mut Girl, rng: &mut dyn rand::RngCore) {
        if girl.stats[Stat::Exp as usize] >= 255 {
            girl.stats[Stat::Exp as usize] = 0;
            girl.stats[Stat::Level as usize] += 1;
            // Gain small random stat boosts on level up
            Self::level_up_stats(girl, rng);
        }
    }

    /// Apply random stat boosts on level-up (matching C++ LevelUpStats).
    fn level_up_stats(girl: &mut Girl, rng: &mut dyn rand::RngCore) {
        let stat_picks = [
            Stat::Constitution,
            Stat::Intelligence,
            Stat::Confidence,
            Stat::Agility,
            Stat::Charisma,
            Stat::Spirit,
            Stat::Beauty,
        ];
        // Pick 2-4 stats to boost
        let count = rng.gen_range(2..=4);
        for _ in 0..count {
            let stat = stat_picks[rng.gen_range(0..stat_picks.len())];
            let boost = rng.gen_range(1..=3);
            Self::update_stat(girl, stat, boost);
        }
        // Small skill boost too
        let skill_picks = Skill::ALL;
        let skill = skill_picks[rng.gen_range(0..skill_picks.len())];
        Self::update_skill(girl, skill, rng.gen_range(1..=2));
    }

    /// Calculate girl's asking price based on stats/skills. Matches C++ CalculateAskPrice.
    pub fn calculate_ask_price(girl: &Girl) -> i32 {
        let beauty = Self::get_stat(girl, Stat::Beauty);
        let charisma = Self::get_stat(girl, Stat::Charisma);
        let fame = Self::get_stat(girl, Stat::Fame);
        let intelligence = Self::get_stat(girl, Stat::Intelligence);
        let confidence = Self::get_stat(girl, Stat::Confidence);
        let sex = Self::get_skill(girl, Skill::NormalSex);
        let service = Self::get_skill(girl, Skill::Service);
        let level = Self::get_stat(girl, Stat::Level);

        let mut price = (beauty + charisma) / 2;
        price += fame / 4;
        price += intelligence / 10;
        price += confidence / 10;
        price += sex / 5;
        price += service / 5;
        price += level;

        // Trait modifiers
        if Self::has_trait(girl, "Big Boobs") {
            price += 5;
        }
        if Self::has_trait(girl, "Sexy Air") {
            price += 5;
        }
        if Self::has_trait(girl, "Cool Person") {
            price += 3;
        }
        if Self::has_trait(girl, "Cute") {
            price += 5;
        }
        if Self::has_trait(girl, "Small Boobs") {
            price -= 3;
        }
        if Self::has_trait(girl, "Fast orgasms") {
            price -= 5;
        }
        if Self::has_trait(girl, "Slow orgasms") {
            price += 3;
        }

        price.max(1)
    }

    /// Per-week stat updates common to all girls in a brothel. Matches C++ updateGirlTurnStats.
    pub fn update_girl_turn_stats(girl: &mut Girl) {
        // Natural tiredness recovery
        Self::update_stat(girl, Stat::Tiredness, -5);
        // Natural health recovery
        Self::update_stat(girl, Stat::Health, 2);
        // Mana recovery
        Self::update_stat(girl, Stat::Mana, 5);
        // Age tracking
        girl.weeks_past += 1;
    }

    /// Per-week happy trait effects. Matches C++ updateHappyTraits.
    pub fn update_happy_traits(girl: &mut Girl) {
        if Self::has_trait(girl, "Optimist") {
            Self::update_stat(girl, Stat::Happiness, 2);
        }
        if Self::has_trait(girl, "Pessimist") {
            Self::update_stat(girl, Stat::Happiness, -2);
        }
        if Self::has_trait(girl, "Nymphomaniac") {
            Self::update_stat(girl, Stat::Libido, 5);
        }
    }

    /// Handle STD damage per week. Matches C++ updateSTD.
    pub fn update_std(girl: &mut Girl) {
        if Self::has_status(girl, Status::Poisoned) {
            Self::update_stat(girl, Stat::Health, -2);
            Self::update_stat(girl, Stat::Happiness, -1);
        }
        if Self::has_status(girl, Status::BadlyPoisoned) {
            Self::update_stat(girl, Stat::Health, -5);
            Self::update_stat(girl, Stat::Happiness, -3);
        }
    }

    /// Apply house percentage effects on stats. Matches C++ updateGirlTurnBrothelStats.
    pub fn update_house_stats(girl: &mut Girl, is_slave: bool) {
        let house = Self::get_stat(girl, Stat::HousePerc);
        // bonus = (60 - house) / divisor
        let diff = 60 - house;

        if is_slave {
            // Slaves only get positive bonuses
            if diff > 0 {
                Self::update_stat(girl, Stat::Obedience, diff / 30);
                Self::update_stat(girl, Stat::PCFear, -(diff / 30));
                Self::update_stat(girl, Stat::PCHate, -(diff / 30));
                Self::update_stat(girl, Stat::Happiness, diff / 15);
            }
        } else {
            // Free girls get full range
            Self::update_stat(girl, Stat::Obedience, diff / 30);
            if diff > 0 {
                Self::update_stat(girl, Stat::PCFear, -(diff / 30));
                Self::update_stat(girl, Stat::PCHate, -(diff / 30));
            }
            Self::update_stat(girl, Stat::Happiness, diff / 15);
        }
    }

    /// Calculate pregnancy chance. Returns true if girl becomes pregnant.
    pub fn calc_pregnancy(
        girl: &mut Girl,
        base_chance: i32,
        is_player: bool,
        rng: &mut dyn rand::RngCore,
    ) -> bool {
        if girl.preg_cooldown > 0 {
            return false;
        }
        if Self::has_status(girl, Status::Pregnant)
            || Self::has_status(girl, Status::PregnantByPlayer)
        {
            return false;
        }
        if girl.use_anti_preg {
            return false;
        }

        let mut chance = base_chance;
        if Self::has_trait(girl, "Fertile") {
            chance += 5;
        }
        if Self::has_trait(girl, "Sterile") {
            return false;
        }

        if rng.gen_range(0..100) < chance {
            if is_player {
                Self::add_status(girl, Status::PregnantByPlayer);
            } else {
                Self::add_status(girl, Status::Pregnant);
            }
            girl.weeks_pregnant = 0;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_girl(name: &str) -> Girl {
        let mut g = Girl::default();
        g.name = name.to_string();
        g.stats[Stat::Health as usize] = 80;
        g.stats[Stat::Happiness as usize] = 50;
        g.stats[Stat::Spirit as usize] = 50;
        g.stats[Stat::Obedience as usize] = 50;
        g.stats[Stat::Beauty as usize] = 50;
        g.stats[Stat::Charisma as usize] = 50;
        g
    }

    #[test]
    fn test_add_get_remove() {
        let mut mgr = GirlManager::new();
        let id = mgr.add_girl(make_girl("Alice"));
        assert_eq!(mgr.count(), 1);
        assert!(mgr.get_girl(id).is_some());
        assert_eq!(mgr.get_girl(id).unwrap().name, "Alice");
        let removed = mgr.remove_girl(id);
        assert!(removed.is_some());
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_stat_update_clamp() {
        let mut g = make_girl("Test");
        GirlManager::update_stat(&mut g, Stat::Health, 200);
        assert_eq!(GirlManager::get_stat(&g, Stat::Health), 100);
        GirlManager::update_stat(&mut g, Stat::Health, -500);
        assert_eq!(GirlManager::get_stat(&g, Stat::Health), 0);
    }

    #[test]
    fn test_traits() {
        let mut g = make_girl("Test");
        assert!(!GirlManager::has_trait(&g, "Big Boobs"));
        GirlManager::add_trait(&mut g, "Big Boobs");
        assert!(GirlManager::has_trait(&g, "Big Boobs"));
        GirlManager::add_trait(&mut g, "Big Boobs"); // duplicate
        assert_eq!(g.traits.len(), 1);
        GirlManager::remove_trait(&mut g, "Big Boobs");
        assert!(!GirlManager::has_trait(&g, "Big Boobs"));
    }

    #[test]
    fn test_rebel_value() {
        let mut g = make_girl("Rebel");
        // Spirit 50, Obedience 50 → base rebel ~ 0 + modifiers
        let rebel = GirlManager::get_rebel_value(&g);
        assert!(rebel >= 0 && rebel <= 100);

        // Broken will should reduce rebel
        GirlManager::add_trait(&mut g, "Broken Will");
        let rebel2 = GirlManager::get_rebel_value(&g);
        assert!(rebel2 < rebel);
    }

    #[test]
    fn test_decay_temp_stats() {
        let mut g = make_girl("Temp");
        g.temp_stats[Stat::Health as usize] = 10;
        g.temp_skills[Skill::Combat as usize] = 10;
        GirlManager::decay_temp_stats(&mut g);
        assert_eq!(g.temp_stats[Stat::Health as usize], 7);
        assert_eq!(g.temp_skills[Skill::Combat as usize], 7);
    }

    #[test]
    fn test_ask_price_positive() {
        let g = make_girl("Pretty");
        let price = GirlManager::calculate_ask_price(&g);
        assert!(price > 0);
    }
}
