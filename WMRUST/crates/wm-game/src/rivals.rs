use rand::Rng;
use serde::{Deserialize, Serialize};

/// A rival gang/organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rival {
    pub name: String,
    pub power: i32,
    pub gold: i64,
    pub num_gangs: i32,
    pub num_brothels: i32,
    pub num_girls: i32,
    pub num_bars: i32,
    pub num_gambling_halls: i32,
    pub businesses_extort: i32,
    pub bribe_rate: i32,
    pub influence: i32,
}

impl Default for Rival {
    fn default() -> Self {
        Self {
            name: String::new(),
            power: 50,
            gold: 5000,
            num_gangs: 3,
            num_brothels: 1,
            num_girls: 8,
            num_bars: 0,
            num_gambling_halls: 0,
            businesses_extort: 0,
            bribe_rate: 0,
            influence: 0,
        }
    }
}

impl Rival {
    /// Maximum girls = 20 * num_brothels.
    pub fn max_girls(&self) -> i32 {
        20 * self.num_brothels.max(1)
    }

    /// Check if rival is dead (0 brothels OR 0 girls OR 0 gold).
    pub fn is_dead(&self) -> bool {
        self.num_brothels <= 0 || self.num_girls <= 0 || self.gold <= 0
    }

    /// Calculate weekly income. Matches C++ rival update formula.
    pub fn calculate_income(&self, rng: &mut dyn rand::RngCore) -> i64 {
        let mut income: i64 = 0;

        // Girl income
        let max_gi = self.num_girls as i64 * 80;
        let min_gi = self.max_girls() as i64;
        if max_gi > min_gi && min_gi > 0 {
            income += rng.gen_range(min_gi..=max_gi) * rng.gen_range(1..=2);
        }

        // Customers estimate
        let customers = if self.num_girls > 0 {
            (rng.gen_range(0..self.num_girls as i64) + self.num_girls as i64) * rng.gen_range(1..=2)
        } else {
            0
        };

        // Gambling income
        if self.num_gambling_halls > 0 && customers > 0 {
            let min_g = customers * 5;
            let max_g = customers * 100;
            income += self.num_gambling_halls as i64 * rng.gen_range(min_g..=max_g);
        }

        // Bar income
        if self.num_bars > 0 && customers > 0 {
            let min_b = customers * 10;
            let max_b = customers * 20;
            income += self.num_bars as i64 * rng.gen_range(min_b..=max_b);
        }

        // Extortion
        income += self.businesses_extort as i64 * 20; // INCOME_BUSINESS

        // Theft
        income += rng.gen_range(0..600) as i64;

        income
    }

    /// Calculate weekly upkeep. Matches C++ rival upkeep formula.
    pub fn calculate_upkeep(&self, tax_rate: f64, income: i64, rng: &mut dyn rand::RngCore) -> i64 {
        let mut upkeep: i64 = 0;

        // Girl rooms
        upkeep += self.max_girls() as i64;
        // Girl upkeep
        upkeep += self.num_girls as i64 * 5;
        // Bribes
        upkeep += self.bribe_rate as i64;
        // Bars
        upkeep += self.num_bars as i64 * 20;
        // Gambling halls
        upkeep += self.num_gambling_halls as i64 * 80;

        // Gambling losses
        if self.num_gambling_halls > 0 {
            let min_loss = 10;
            let max_loss = 500;
            upkeep += self.num_gambling_halls as i64 * rng.gen_range(min_loss..=max_loss);
        }

        // Bar supplies
        if self.num_bars > 0 {
            upkeep += self.num_bars as i64 * (rng.gen_range(0..100) + 30) as i64;
        }

        // Gangs
        upkeep += self.num_gangs as i64 * 90;

        // Tax (influenced by influence)
        let effective_rate = (tax_rate - self.influence as f64 / 2000.0).max(0.01);
        let laundered = income as f64 * 0.25 * rng.gen::<f64>();
        let taxable = (income as f64 - laundered).max(0.0);
        upkeep += (taxable * effective_rate) as i64;

        upkeep
    }

    /// Process weekly AI spending. Returns events.
    pub fn process_spending(&mut self, rng: &mut dyn rand::RngCore) -> Vec<String> {
        let mut events = Vec::new();

        // 1. New brothel at 20 girls per brothel, max 6
        if self.num_girls >= self.max_girls() && self.num_brothels < 6 && self.gold >= 20_000 {
            self.num_brothels += 1;
            self.gold -= 20_000;
            events.push(format!("{} opened a new brothel", self.name));
        }

        // 2. New girls: 1-6 per week at 550 each
        let girls_to_buy = rng.gen_range(1..=6).min(self.max_girls() - self.num_girls);
        if girls_to_buy > 0 {
            let cost = girls_to_buy as i64 * 550;
            if self.gold >= cost {
                self.num_girls += girls_to_buy;
                self.gold -= cost;
            }
        }

        // 3. Hire gangs: up to 8 total
        if self.num_gangs < 8 {
            let new_gangs = rng.gen_range(0..=5).min(8 - self.num_gangs);
            self.num_gangs += new_gangs;
        }

        // 4. Buy bars or gambling halls
        if self.gold >= 2500 && self.num_bars < 3 && rng.gen_range(0..100) < 30 {
            self.num_bars += 1;
            self.gold -= 2500;
        }
        if self.gold >= 15000 && self.num_gambling_halls < 2 && rng.gen_range(0..100) < 20 {
            self.num_gambling_halls += 1;
            self.gold -= 15000;
        }

        // 5. Increase bribe rate
        let profit = self.gold; // simplified
        if profit > 1000 {
            self.bribe_rate += 50;
        }

        events
    }

    /// Process rival attack action. Returns (gold_stolen, events).
    pub fn process_attack(&mut self, rng: &mut dyn rand::RngCore) -> (i64, Vec<String>) {
        let mut events = Vec::new();
        let mut gold_stolen: i64 = 0;

        if self.num_gangs <= 0 {
            return (0, events);
        }

        // 70% chance to attack
        if rng.gen_range(0..100) >= 70 {
            return (0, events);
        }

        // Simplified: attack player
        // 50% chance to win
        if rng.gen_range(0..2) == 0 {
            // Destroy player territories
            let destroyed = rng.gen_range(1..=3);
            events.push(format!(
                "{} attacked and destroyed {} territories",
                self.name, destroyed
            ));

            // Plunder gold
            gold_stolen = (rng.gen_range(0..2000) + 45) as i64;
            events.push(format!("{} plundered {} gold", self.name, gold_stolen));
        } else {
            events.push(format!("{} attacked but was repelled", self.name));
            // Rival losses
            self.num_gangs = (self.num_gangs - 1).max(0);
        }

        (gold_stolen, events)
    }
}

/// Manages all rival organizations.
#[derive(Debug, Serialize, Deserialize)]
pub struct RivalManager {
    pub rivals: Vec<Rival>,
}

impl Default for RivalManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RivalManager {
    pub fn new() -> Self {
        Self { rivals: Vec::new() }
    }

    pub fn add_rival(&mut self, rival: Rival) {
        self.rivals.push(rival);
    }

    pub fn remove_rival(&mut self, index: usize) {
        if index < self.rivals.len() {
            self.rivals.remove(index);
        }
    }

    /// Process all rivals for the week. Returns all events.
    pub fn process_rivals(
        &mut self,
        tax_rate: f64,
        rng: &mut dyn rand::RngCore,
    ) -> (i64, Vec<String>) {
        let mut total_plunder: i64 = 0;
        let mut all_events = Vec::new();

        for rival in &mut self.rivals {
            // Calculate finances
            let income = rival.calculate_income(rng);
            let upkeep = rival.calculate_upkeep(tax_rate, income, rng);
            rival.gold += income - upkeep;

            // Super profit cap
            if income - upkeep > 10_000 {
                rival.gold -= 10_000;
            }

            // AI spending
            let spend_events = rival.process_spending(rng);
            all_events.extend(spend_events);

            // Attack
            let (plunder, attack_events) = rival.process_attack(rng);
            total_plunder += plunder;
            all_events.extend(attack_events);
        }

        // Remove dead rivals
        self.rivals.retain(|r| !r.is_dead());

        (total_plunder, all_events)
    }

    /// Check if player has been taken over (all rivals combined too powerful).
    pub fn check_takeover(&self) -> bool {
        let total_power: i32 = self.rivals.iter().map(|r| r.power).sum();
        total_power > 500 // threshold for game-over
    }

    /// 30% chance per week to spawn a new rival post-game-win.
    pub fn maybe_spawn_rival(&mut self, rival_names: &[String], rng: &mut dyn rand::RngCore) {
        if rng.gen_range(0..100) < 30 {
            let name = if rival_names.is_empty() {
                format!("Rival #{}", rng.gen_range(100..999))
            } else {
                let first = &rival_names[rng.gen_range(0..rival_names.len())];
                first.clone()
            };
            self.add_rival(Rival {
                name,
                ..Rival::default()
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rival_income() {
        let mut rng = rand::thread_rng();
        let rival = Rival::default();
        let income = rival.calculate_income(&mut rng);
        assert!(income > 0);
    }

    #[test]
    fn test_rival_death() {
        let mut rival = Rival::default();
        rival.num_brothels = 0;
        assert!(rival.is_dead());

        let mut rival2 = Rival::default();
        rival2.gold = 0;
        assert!(rival2.is_dead());
    }

    #[test]
    fn test_process_rivals() {
        let mut mgr = RivalManager::new();
        mgr.add_rival(Rival {
            name: "Evil Corp".to_string(),
            ..Rival::default()
        });
        let mut rng = rand::thread_rng();
        let (_plunder, _events) = mgr.process_rivals(0.06, &mut rng);
        // Should have processed without error
        assert!(mgr.rivals.len() <= 1);
    }
}
