use rand::Rng;
use wm_core::enums::Shift;

use crate::brothel::Brothel;

/// Wealth class of a customer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomerClass {
    Poor,
    Middle,
    Rich,
}

/// A single customer visiting a brothel.
#[derive(Debug, Clone)]
pub struct Customer {
    pub class: CustomerClass,
    pub money: i32,
    pub is_group: bool,
    pub group_size: i32,
    pub is_female: bool,
    pub is_official: bool,
    pub fetish: i32,
    pub preferred_skill: i32, // Skill enum index for sex preference
    pub stats: [i32; 10],     // general stat values (combat, appearance, etc.)
}

impl Default for Customer {
    fn default() -> Self {
        Self {
            class: CustomerClass::Poor,
            money: 50,
            is_group: false,
            group_size: 1,
            is_female: false,
            is_official: false,
            fetish: 0,
            preferred_skill: 3, // NormalSex
            stats: [50; 10],
        }
    }
}

/// Customer generation for brothels.
#[derive(Debug)]
pub struct CustomerGenerator {
    pub num_customers_per_brothel: Vec<i32>,
}

impl Default for CustomerGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomerGenerator {
    pub fn new() -> Self {
        Self {
            num_customers_per_brothel: Vec::new(),
        }
    }

    /// Generate customer count for a brothel. Matches C++ GenerateCustomers formula.
    pub fn generate_customers(
        &mut self,
        brothels: &[Brothel],
        shift: Shift,
        rng: &mut dyn rand::RngCore,
    ) {
        self.num_customers_per_brothel.clear();

        for brothel in brothels {
            let num_girls = brothel.num_girls() as f64;
            if num_girls == 0.0 {
                self.num_customers_per_brothel.push(0);
                continue;
            }

            // Base customers
            let base_mult = match shift {
                Shift::Day => 1.5,
                Shift::Night => 2.0,
            };
            let mut customers = (num_girls * base_mult) as i32;

            // Fame bonus: fame/4
            customers += brothel.fame / 4;

            // Advertising bonus
            let ad_bonus = (brothel.advertising_budget * brothel.advertised * 0.06) as i32;
            // Randomize 50%-150%
            let ad_rand = rng.gen_range(50..=150);
            customers += ad_bonus * ad_rand / 100;

            // Filthiness penalty
            customers -= brothel.filthiness / 10;

            customers = customers.max(0);
            self.num_customers_per_brothel.push(customers);
        }
    }

    /// Get the customer count for a specific brothel.
    pub fn get_customers(&self, brothel_id: usize) -> i32 {
        self.num_customers_per_brothel
            .get(brothel_id)
            .copied()
            .unwrap_or(0)
    }

    /// Generate a single customer. Matches C++ GetCustomer formula.
    pub fn generate_customer(rng: &mut dyn rand::RngCore) -> Customer {
        let mut customer = Customer::default();

        // 4% chance of group
        if rng.gen_range(0..100) < 4 {
            customer.is_group = true;
            customer.group_size = rng.gen_range(1..=3);
        }

        // 15% chance female
        customer.is_female = rng.gen_range(0..100) < 15;

        // General stats: (dice%81)+20 → range 20-100
        for stat in customer.stats.iter_mut() {
            *stat = rng.gen_range(0..81) + 20;
        }

        // Customer class and money
        let class_roll = rng.gen_range(0..100);
        // Poor: 30-79%, Middle: 5-65%, Rich: remainder
        // Simplified: Poor < 50, Middle < 85, Rich >= 85
        if class_roll < 50 {
            customer.class = CustomerClass::Poor;
            customer.money = rng.gen_range(20..=120);
        } else if class_roll < 85 {
            customer.class = CustomerClass::Middle;
            customer.money = rng.gen_range(60..=260);
        } else {
            customer.class = CustomerClass::Rich;
            customer.money = rng.gen_range(600..=2600);
        }

        // 1% chance of being a town official
        customer.is_official = rng.gen_range(0..100) == 0;

        // Random fetish (0-14)
        customer.fetish = rng.gen_range(0..15);

        // Sex preference: random skill (Anal=0, BDSM=2, NormalSex=3, Beastiality=4, Group=5, Lesbian=6)
        // Exclude Magic(1), Service(7), Strip(8), Combat(9)
        let sex_skills = [0, 2, 3, 4, 5, 6];
        customer.preferred_skill = sex_skills[rng.gen_range(0..sex_skills.len())];

        // Female customers prefer lesbian or bestiality
        if customer.is_female {
            if rng.gen_range(0..2) == 0 {
                customer.preferred_skill = 6; // Lesbian
            } else {
                customer.preferred_skill = 4; // Bestiality
            }
        }

        customer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_customer() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let c = CustomerGenerator::generate_customer(&mut rng);
            assert!(c.money > 0);
            for &s in &c.stats {
                assert!(s >= 20 && s <= 100);
            }
        }
    }

    #[test]
    fn test_customer_counts() {
        let mut gen = CustomerGenerator::new();
        let mut rng = rand::thread_rng();
        let mut b = Brothel::default();
        b.girls = vec![0, 1, 2, 3, 4]; // 5 girls
        b.fame = 40;
        gen.generate_customers(&[b], Shift::Night, &mut rng);
        let count = gen.get_customers(0);
        assert!(
            count > 0,
            "Should generate customers for brothel with girls"
        );
    }
}
