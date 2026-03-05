use rand::Rng;
use wm_core::enums::{DungeonReason, Skill, Stat, Status};
use wm_core::girl::Girl;

use crate::girls::GirlManager;

/// A prisoner in the dungeon.
#[derive(Debug, Clone)]
pub struct DungeonInmate {
    pub girl: Girl,
    pub reason: DungeonReason,
    pub weeks: u32,
    pub is_customer: bool,
    pub fed: bool,
}

/// Manages the dungeon/prison.
#[derive(Debug)]
pub struct DungeonManager {
    pub inmates: Vec<DungeonInmate>,
}

impl Default for DungeonManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DungeonManager {
    pub fn new() -> Self {
        Self {
            inmates: Vec::new(),
        }
    }

    /// Add a girl to the dungeon.
    pub fn add_girl(&mut self, girl: Girl, reason: DungeonReason) {
        self.inmates.push(DungeonInmate {
            girl,
            reason,
            weeks: 0,
            is_customer: false,
            fed: true,
        });
    }

    /// Add a customer prisoner.
    pub fn add_customer(&mut self, reason: DungeonReason) {
        let mut girl = Girl::default();
        girl.name = "Customer".to_string();
        girl.stats[Stat::Health as usize] = 100;
        self.inmates.push(DungeonInmate {
            girl,
            reason,
            weeks: 0,
            is_customer: true,
            fed: true,
        });
    }

    /// Release a prisoner, returning the girl if it's a girl.
    pub fn release(&mut self, index: usize) -> Option<Girl> {
        if index < self.inmates.len() {
            let inmate = self.inmates.remove(index);
            if !inmate.is_customer {
                Some(inmate.girl)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Total inmates count.
    pub fn num_inmates(&self) -> usize {
        self.inmates.len()
    }

    /// Get girl inmates only (not customers).
    pub fn girl_inmates(&self) -> Vec<(usize, &DungeonInmate)> {
        self.inmates
            .iter()
            .enumerate()
            .filter(|(_, i)| !i.is_customer)
            .collect()
    }

    /// Process weekly dungeon stat updates for all inmates.
    /// Matches C++ updateGirlTurnDungeonStats.
    pub fn process_week(&mut self) {
        // Remove dead bodies first
        self.inmates
            .retain(|inmate| GirlManager::get_stat(&inmate.girl, Stat::Health) > 0);

        for inmate in &mut self.inmates {
            inmate.weeks += 1;

            if inmate.is_customer {
                // Customers just lose HP if not fed
                if !inmate.fed {
                    GirlManager::update_stat(&mut inmate.girl, Stat::Health, -5);
                }
                continue;
            }

            let is_slave = GirlManager::has_status(&inmate.girl, Status::Slave);

            if inmate.fed {
                if is_slave {
                    // Slave, fed
                    GirlManager::update_stat(&mut inmate.girl, Stat::Confidence, -2);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Obedience, 2);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Spirit, -2);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCHate, 1);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCLove, -1);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCFear, 4);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Tiredness, -10);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Happiness, -1);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Health, 4);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Mana, 5);
                    GirlManager::update_skill(&mut inmate.girl, Skill::BDSM, 1);
                } else {
                    // Free, fed
                    GirlManager::update_stat(&mut inmate.girl, Stat::Confidence, -1);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Obedience, 1);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Spirit, -1);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCHate, 1);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCLove, -4);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCFear, 4);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Tiredness, -10);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Happiness, -5);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Health, 1);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Mana, 5);
                    GirlManager::update_skill(&mut inmate.girl, Skill::BDSM, 1);
                }
            } else {
                // Not fed
                if is_slave {
                    // Slave, starved
                    GirlManager::update_stat(&mut inmate.girl, Stat::Confidence, -2);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Obedience, 2);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Spirit, -2);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCHate, 1);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCLove, -2);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCFear, 4);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Tiredness, 1);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Happiness, -3);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Health, -5);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Mana, 1);
                    GirlManager::update_skill(&mut inmate.girl, Skill::BDSM, 2);
                } else {
                    // Free, starved
                    GirlManager::update_stat(&mut inmate.girl, Stat::Confidence, -2);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Obedience, 2);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Spirit, -2);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCHate, 4);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCLove, -5);
                    GirlManager::update_stat(&mut inmate.girl, Stat::PCFear, 6);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Tiredness, 2);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Happiness, -5);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Health, -5);
                    GirlManager::update_stat(&mut inmate.girl, Stat::Mana, 1);
                    GirlManager::update_skill(&mut inmate.girl, Skill::BDSM, 2);
                }
            }
        }
    }

    /// Torture a dungeon inmate. Matches C++ cGirlTorture logic.
    /// Returns events describing what happened.
    pub fn torture(
        &mut self,
        index: usize,
        torture_mod: i32,
        is_player_torturing: bool,
        rng: &mut dyn rand::RngCore,
    ) -> Vec<String> {
        let mut events = Vec::new();

        let inmate = match self.inmates.get_mut(index) {
            Some(i) => i,
            None => return events,
        };

        if inmate.is_customer || inmate.girl.tortured_today {
            return events;
        }
        inmate.girl.tortured_today = true;

        let is_slave = GirlManager::has_status(&inmate.girl, Status::Slave);
        let health = GirlManager::get_stat(&inmate.girl, Stat::Health);

        // Evil gained by player
        let _evil = if is_player_torturing {
            if is_slave {
                5
            } else {
                10
            }
        } else if is_slave {
            2
        } else {
            4
        };

        // Stat changes based on health threshold
        if health > 10 {
            // Heavy torture
            GirlManager::update_stat(&mut inmate.girl, Stat::Health, -5);
            GirlManager::update_stat(&mut inmate.girl, Stat::Happiness, -5);
            GirlManager::update_stat(&mut inmate.girl, Stat::Constitution, 1);
            GirlManager::update_stat(&mut inmate.girl, Stat::Confidence, -5);
            GirlManager::update_stat(&mut inmate.girl, Stat::Obedience, 10);
            GirlManager::update_stat(&mut inmate.girl, Stat::Spirit, -5);
            GirlManager::update_stat(&mut inmate.girl, Stat::Tiredness, -5);
            GirlManager::update_stat(&mut inmate.girl, Stat::PCHate, 3);
            GirlManager::update_stat(&mut inmate.girl, Stat::PCLove, -5);
            GirlManager::update_stat(&mut inmate.girl, Stat::PCFear, 7);
            GirlManager::update_skill(&mut inmate.girl, Skill::BDSM, 1);
            events.push("Heavy torture applied".to_string());
        } else {
            // Safer torture
            GirlManager::update_stat(&mut inmate.girl, Stat::Happiness, -2);
            GirlManager::update_stat(&mut inmate.girl, Stat::Confidence, -2);
            GirlManager::update_stat(&mut inmate.girl, Stat::Obedience, 4);
            GirlManager::update_stat(&mut inmate.girl, Stat::Spirit, -2);
            GirlManager::update_stat(&mut inmate.girl, Stat::Tiredness, -2);
            GirlManager::update_stat(&mut inmate.girl, Stat::PCHate, 1);
            GirlManager::update_stat(&mut inmate.girl, Stat::PCLove, -2);
            GirlManager::update_stat(&mut inmate.girl, Stat::PCFear, 3);
            events.push("Light torture applied (low health)".to_string());
        }

        // Injury check: base 3%, Fragile doubles, Tough halves
        let mut injury_chance = 3;
        if GirlManager::has_trait(&inmate.girl, "Fragile") {
            injury_chance *= 2;
        }
        if GirlManager::has_trait(&inmate.girl, "Tough") {
            injury_chance /= 2;
        }

        if rng.gen_range(0..100) < injury_chance {
            let hp_loss = rng.gen_range(5..=14);
            GirlManager::update_stat(&mut inmate.girl, Stat::Health, -hp_loss);

            // Scar types at 2× injury chance
            if rng.gen_range(0..100) < injury_chance * 2 {
                let scar_roll = rng.gen_range(0..3);
                match scar_roll {
                    0 => {
                        GirlManager::add_trait(&mut inmate.girl, "Small Scars");
                        events.push("Girl received small scars".to_string());
                    }
                    1 => {
                        GirlManager::add_trait(&mut inmate.girl, "Cool Scars");
                        events.push("Girl received cool scars".to_string());
                    }
                    _ => {
                        GirlManager::add_trait(&mut inmate.girl, "Horrific Scars");
                        events.push("Girl received horrific scars".to_string());
                    }
                }
            }

            // Miscarriage if pregnant
            if injury_chance * 2 > rng.gen_range(0..100)
                && (GirlManager::has_status(&inmate.girl, Status::Pregnant)
                    || GirlManager::has_status(&inmate.girl, Status::PregnantByPlayer))
            {
                GirlManager::remove_status(&mut inmate.girl, Status::Pregnant);
                GirlManager::remove_status(&mut inmate.girl, Status::PregnantByPlayer);
                inmate.girl.weeks_pregnant = 0;
                events.push("Girl suffered a miscarriage".to_string());
            }

            events.push(format!("Girl was injured: -{} HP", hp_loss));
        }

        // Trait gain chances (modified by weeks * torture_mod)
        let week_mod = (inmate.weeks as i32) * torture_mod;
        let is_torturer_girl = !is_player_torturing;

        // Broken Will: spirit < 20 AND health < 20
        let spirit = GirlManager::get_stat(&inmate.girl, Stat::Spirit);
        let cur_health = GirlManager::get_stat(&inmate.girl, Stat::Health);
        if spirit < 20 && cur_health < 20 {
            let mut chance = 5 + week_mod / 2;
            if is_torturer_girl {
                chance /= 2;
            }
            if rng.gen_range(0..100) < chance
                && !GirlManager::has_trait(&inmate.girl, "Broken Will")
            {
                GirlManager::add_trait(&mut inmate.girl, "Broken Will");
                events.push("Girl's will has been broken!".to_string());
            }
        }

        // Masochist: bdsm > 30
        let bdsm = GirlManager::get_skill(&inmate.girl, Skill::BDSM);
        if bdsm > 30 {
            let mut chance = 10 + week_mod;
            if is_torturer_girl {
                chance /= 2;
            }
            if rng.gen_range(0..100) < chance && !GirlManager::has_trait(&inmate.girl, "Masochist")
            {
                GirlManager::add_trait(&mut inmate.girl, "Masochist");
                events.push("Girl has become a masochist".to_string());
            }
        }

        // Mind Fucked: health < 10
        if cur_health < 10 {
            let mut chance = 10 + week_mod;
            if is_torturer_girl {
                chance /= 2;
            }
            if rng.gen_range(0..100) < chance
                && !GirlManager::has_trait(&inmate.girl, "Mind Fucked")
            {
                GirlManager::add_trait(&mut inmate.girl, "Mind Fucked");
                events.push("Girl has been mind fucked".to_string());
            }
        }

        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dungeon_girl() -> Girl {
        let mut g = Girl::default();
        g.name = "Prisoner".to_string();
        g.stats[Stat::Health as usize] = 80;
        g.stats[Stat::Happiness as usize] = 50;
        g.stats[Stat::Spirit as usize] = 50;
        g.stats[Stat::Obedience as usize] = 20;
        g.stats[Stat::Confidence as usize] = 50;
        g
    }

    #[test]
    fn test_add_release() {
        let mut dm = DungeonManager::new();
        dm.add_girl(make_dungeon_girl(), DungeonReason::GirlCaptured);
        assert_eq!(dm.num_inmates(), 1);
        let girl = dm.release(0);
        assert!(girl.is_some());
        assert_eq!(dm.num_inmates(), 0);
    }

    #[test]
    fn test_process_week_stat_changes() {
        let mut dm = DungeonManager::new();
        let girl = make_dungeon_girl();
        let initial_obed = girl.stats[Stat::Obedience as usize];
        dm.add_girl(girl, DungeonReason::GirlCaptured);
        dm.process_week();

        let g = &dm.inmates[0].girl;
        // Free, fed: obedience should increase
        assert!(g.stats[Stat::Obedience as usize] > initial_obed);
    }

    #[test]
    fn test_torture() {
        let mut dm = DungeonManager::new();
        dm.add_girl(make_dungeon_girl(), DungeonReason::GirlCaptured);
        let mut rng = rand::thread_rng();
        let events = dm.torture(0, 1, true, &mut rng);
        assert!(!events.is_empty());

        // Obedience should have increased from torture
        let obed = GirlManager::get_stat(&dm.inmates[0].girl, Stat::Obedience);
        assert!(obed > 20);
    }

    #[test]
    fn test_customer_health_loss() {
        let mut dm = DungeonManager::new();
        dm.add_customer(DungeonReason::CustomerNoPay);
        dm.inmates[0].fed = false;

        let pre_health = GirlManager::get_stat(&dm.inmates[0].girl, Stat::Health);
        dm.process_week();
        let post_health = GirlManager::get_stat(&dm.inmates[0].girl, Stat::Health);
        assert!(
            post_health < pre_health,
            "Starved customer should lose health"
        );
    }
}
