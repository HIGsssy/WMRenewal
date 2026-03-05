use rand::Rng;
use wm_core::enums::{Skill, Stat};
use wm_core::girl::Girl;

use crate::gangs::Gang;
use crate::girls::GirlManager;

/// Result of a combat encounter.
#[derive(Debug, Clone)]
pub struct CombatResult {
    pub attacker_won: bool,
    pub attacker_casualties: i32,
    pub defender_casualties: i32,
    pub events: Vec<String>,
}

/// Gang vs Gang brawl. Matches C++ GangBrawl.
/// Returns (attacker_won, attacker_casualties, defender_casualties).
pub fn gang_brawl(
    attacker: &mut Gang,
    defender: &mut Gang,
    attacker_potions: &mut i32,
    defender_potions: &mut i32,
    weapon_level: i32,
    rng: &mut dyn rand::RngCore,
) -> CombatResult {
    let mut result = CombatResult {
        attacker_won: false,
        attacker_casualties: 0,
        defender_casualties: 0,
        events: Vec::new(),
    };

    let mut atk_alive = attacker.num_members;
    let mut def_alive = defender.num_members;
    let atk_start = atk_alive;
    let def_start = def_alive;

    // 1v1 duels until one side is eliminated or flees
    let max_rounds = (atk_alive + def_alive) * 3;
    for _ in 0..max_rounds {
        if atk_alive <= 0 || def_alive <= 0 {
            break;
        }

        // Morale break: when half lost, 40% chance to flee
        if atk_alive <= atk_start / 2 && rng.gen_range(0..100) < 40 {
            result.events.push("Attackers fled!".to_string());
            break;
        }
        if def_alive <= def_start / 2 && rng.gen_range(0..100) < 40 {
            result.attacker_won = true;
            result.events.push("Defenders fled!".to_string());
            break;
        }

        // Each goon gets health=100 for the duel
        let mut atk_hp = 100;
        let mut def_hp = 100;

        for _ in 0..20 {
            // Attacker strikes
            let atk_skill = attacker.attack_skill();
            if rng.gen_range(0..100) < atk_skill {
                let mut dmg = (weapon_level + 1) * 5;
                if attacker.magic_skill > attacker.combat_skill {
                    dmg += 10;
                }
                dmg -= defender.constitution / 15;
                dmg = dmg.max(1);
                def_hp -= dmg;
            }

            if def_hp <= 0 {
                break;
            }

            // Defender strikes
            let def_skill = defender.attack_skill();
            if rng.gen_range(0..100) < def_skill {
                let mut dmg = (weapon_level + 1) * 5;
                if defender.magic_skill > defender.combat_skill {
                    dmg += 10;
                }
                dmg -= attacker.constitution / 15;
                dmg = dmg.max(1);
                atk_hp -= dmg;
            }

            // Healing potions at HP <= 40
            if def_hp <= 40 && *defender_potions > 0 {
                def_hp += 30;
                *defender_potions -= 1;
            }
            if atk_hp <= 40 && *attacker_potions > 0 {
                atk_hp += 30;
                *attacker_potions -= 1;
            }

            if atk_hp <= 0 {
                break;
            }
        }

        if def_hp <= 0 {
            def_alive -= 1;
        }
        if atk_hp <= 0 {
            atk_alive -= 1;
        }
    }

    result.attacker_casualties = atk_start - atk_alive;
    result.defender_casualties = def_start - def_alive;
    result.attacker_won = def_alive <= 0 || (atk_alive > 0 && atk_alive >= def_alive);

    attacker.num_members = atk_alive.max(0);
    defender.num_members = def_alive.max(0);

    // Winner gets combat skill boost
    if result.attacker_won {
        attacker.combat_skill = (attacker.combat_skill + 2).min(100);
    } else {
        defender.combat_skill = (defender.combat_skill + 2).min(100);
    }

    attacker.saw_combat = true;
    defender.saw_combat = true;

    result
}

/// Girl vs player gang combat. Matches C++ GangCombat (girl vs player's gang).
/// Returns true if girl escapes.
pub fn girl_vs_gang(
    girl: &mut Girl,
    gang: &mut Gang,
    potions: &mut i32,
    weapon_level: i32,
    rng: &mut dyn rand::RngCore,
) -> CombatResult {
    let mut result = CombatResult {
        attacker_won: false, // girl is "attacker" trying to escape
        attacker_casualties: 0,
        defender_casualties: 0,
        events: Vec::new(),
    };

    // Incorporeal trait: auto-win
    if GirlManager::has_trait(girl, "Incorporial") {
        result.attacker_won = true;
        gang.num_members /= 2;
        // 40% per remaining member killed
        let mut extra_kills = 0;
        for _ in 0..gang.num_members {
            if rng.gen_range(0..100) < 40 {
                extra_kills += 1;
            }
        }
        gang.num_members = (gang.num_members - extra_kills).max(0);
        result
            .events
            .push("Girl is incorporeal - phased through attackers!".to_string());
        return result;
    }

    let max_goons = (4 + 1).min(gang.num_members); // 4 + num_guard_gangs (simplified to 1)
    let mut girl_hp = GirlManager::get_stat(girl, Stat::Health);
    let mut goons_alive = max_goons;
    let girl_combat = GirlManager::get_skill(girl, Skill::Combat);
    let girl_magic = GirlManager::get_skill(girl, Skill::Magic);
    let mut girl_agility = GirlManager::get_stat(girl, Stat::Agility);

    for _ in 0..50 {
        if girl_hp <= 20 || goons_alive <= 0 {
            break;
        }

        // Girl attacks
        let girl_attack = if girl_magic > girl_combat {
            girl_magic
        } else {
            girl_combat
        };
        if rng.gen_range(0..100) < girl_attack {
            let dmg = if girl_magic > girl_combat {
                girl_magic / 5 + 2 + 5
            } else {
                girl_combat / 10 + 5
            };
            // Hit a goon
            if dmg > 30 {
                // counts as a kill
                goons_alive -= 1;
            }
            // Skill gain
            if rng.gen_range(0..2) == 0 {
                GirlManager::update_skill(girl, Skill::Combat, 1);
            }
        }

        // Goon attacks
        let goon_attack = gang.attack_skill();
        if rng.gen_range(0..100) < goon_attack {
            let mut dmg = (weapon_level + 1) * 5;
            if gang.magic_skill > gang.combat_skill {
                dmg += 10;
            }
            // Girl dodge
            let dodge = girl_agility.min(90);
            if rng.gen_range(0..100) >= dodge {
                dmg -= GirlManager::get_stat(girl, Stat::Constitution) / 10;
                dmg = dmg.max(1);
                girl_hp -= dmg;
            }
            girl_agility = (girl_agility - 2).max(0);
        }

        // Healing potions for goons
        if *potions > 0 && goons_alive > 0 {
            // Simplified: use potion if any goon took damage
            *potions -= 1;
        }

        // Half goons lost: 40% to flee
        if goons_alive <= max_goons / 2 && rng.gen_range(0..100) < 40 {
            result.attacker_won = true;
            result.events.push("Gang fled from the girl!".to_string());
            break;
        }
    }

    if girl_hp <= 20 {
        // Girl lost
        result.attacker_won = false;
        // Apply damage to girl
        let actual_hp = GirlManager::get_stat(girl, Stat::Health);
        let damage = actual_hp - girl_hp.max(1);
        GirlManager::update_stat(girl, Stat::Health, -damage);
    } else if goons_alive <= 0 {
        result.attacker_won = true;
    }

    result.defender_casualties = max_goons - goons_alive;
    gang.num_members = (gang.num_members - result.defender_casualties).max(0);
    gang.saw_combat = true;

    result
}

/// Girl escape attempt odds calculation. Matches C++ cGirlGangFight.
pub fn girl_escape_odds(girl: &Girl, guard_gangs: &[&Gang], weapon_level: i32) -> f64 {
    if guard_gangs.is_empty() {
        return 1.0; // No guards, auto-escape
    }

    let girl_stats = GirlManager::get_skill(girl, Skill::Combat) as f64
        + GirlManager::get_skill(girl, Skill::Magic) as f64
        + GirlManager::get_stat(girl, Stat::Intelligence) as f64;

    let max_goons: i32 = guard_gangs
        .iter()
        .map(|g| (4 + 1).min(g.num_members)) // simplified
        .sum();

    let goon_stats = weapon_level as f64 * 5.0 * max_goons as f64
        + guard_gangs
            .iter()
            .map(|g| (g.combat_skill + g.magic_skill + g.intelligence) as f64)
            .sum::<f64>();

    let mut odds = girl_stats / (goon_stats + girl_stats);

    // Trait modifiers
    if GirlManager::has_trait(girl, "Clumsy") {
        odds -= 0.05;
    }
    if GirlManager::has_trait(girl, "Broken Will") {
        odds -= 0.10;
    }
    if GirlManager::has_trait(girl, "Meek") {
        odds -= 0.05;
    }
    if GirlManager::has_trait(girl, "Dependant") {
        odds -= 0.10;
    }
    if GirlManager::has_trait(girl, "Fearless") {
        odds += 0.10;
    }
    if GirlManager::has_trait(girl, "Fleet of Foot") {
        odds += 0.10;
    }

    odds.clamp(0.0, 1.0)
}

/// Player vs girl combat. Returns true if player wins.
pub fn player_combat(girl: &mut Girl, weapon_level: i32, rng: &mut dyn rand::RngCore) -> bool {
    // Incorporeal: player auto-wins
    if GirlManager::has_trait(girl, "Incorporial") {
        return true;
    }

    let mut player_hp = 100;
    let girl_hp_start = GirlManager::get_stat(girl, Stat::Health);
    let mut girl_hp = girl_hp_start;
    let girl_combat = GirlManager::get_skill(girl, Skill::Combat);
    let girl_magic = GirlManager::get_skill(girl, Skill::Magic);
    let girl_agility = GirlManager::get_stat(girl, Stat::Agility);
    let mut girl_mana = GirlManager::get_stat(girl, Stat::Mana);
    let player_combat = 60; // Base player combat
    let player_magic = 40;
    let mut player_agility = 50;

    for _ in 0..30 {
        if girl_hp <= 20 || player_hp <= 20 {
            break;
        }

        // Girl attacks
        let girl_attack = if girl_magic > girl_combat && girl_mana >= 7 {
            girl_mana -= 7;
            girl_magic
        } else {
            girl_combat
        };
        if rng.gen_range(0..100) < girl_attack {
            let dmg = if girl_magic > girl_combat {
                2 + girl_magic / 5
            } else {
                5 + girl_combat / 10
            };
            // Player dodge
            if rng.gen_range(0..100) >= player_agility {
                player_hp -= dmg;
            }
            player_agility = (player_agility - 2).max(0);
        }

        // Player attacks
        let player_attack = player_combat.max(player_magic);
        if rng.gen_range(0..100) < player_attack {
            let dmg = (weapon_level + 1) * 5;
            // Girl dodge
            if rng.gen_range(0..100) >= girl_agility {
                girl_hp -= dmg;
            }
        }
    }

    let player_won = girl_hp <= 20;
    // Apply actual damage to girl
    let damage = girl_hp_start - girl_hp.max(1);
    GirlManager::update_stat(girl, Stat::Health, -damage);

    player_won
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gang_brawl() {
        let mut rng = rand::thread_rng();
        let mut atk = Gang {
            num_members: 10,
            combat_skill: 60,
            ..Gang::default()
        };
        let mut def = Gang {
            num_members: 10,
            combat_skill: 40,
            ..Gang::default()
        };
        let mut atk_potions = 5;
        let mut def_potions = 5;

        let result = gang_brawl(
            &mut atk,
            &mut def,
            &mut atk_potions,
            &mut def_potions,
            1,
            &mut rng,
        );
        assert!(
            atk.num_members + def.num_members < 20,
            "Some casualties expected"
        );
        assert!(result.attacker_casualties >= 0);
        assert!(result.defender_casualties >= 0);
    }

    #[test]
    fn test_girl_escape_odds() {
        let mut girl = Girl::default();
        girl.skills[Skill::Combat as usize] = 80;
        girl.skills[Skill::Magic as usize] = 50;
        girl.stats[Stat::Intelligence as usize] = 60;

        let gang = Gang {
            num_members: 5,
            combat_skill: 40,
            magic_skill: 20,
            intelligence: 30,
            ..Gang::default()
        };

        let odds = girl_escape_odds(&girl, &[&gang], 1);
        assert!(odds > 0.0 && odds < 1.0);
    }
}
