use wm_core::enums::{EffectTarget, ItemType, Skill, Stat};
use wm_core::girl::{Girl, MAX_INVENTORY};
use wm_core::item::Item;

use crate::girls::GirlManager;

/// Maximum count per item type a girl can equip.
fn max_equip_slots(item_type: ItemType) -> usize {
    match item_type {
        ItemType::Ring => 8,
        ItemType::Dress
        | ItemType::Underwear
        | ItemType::Shoes
        | ItemType::Hat
        | ItemType::Helmet
        | ItemType::Necklace
        | ItemType::Armor => 1,
        ItemType::SmallWeapon | ItemType::LargeWeapon | ItemType::Shield => 2,
        // Consumables: Food, Makeup, Consumable — no equip limit (consumed immediately)
        ItemType::Food | ItemType::Makeup | ItemType::Consumable => 0,
        ItemType::Misc => 0,
    }
}

/// Returns true if the item type is consumed on use (removed from inventory).
fn is_consumable(item_type: ItemType) -> bool {
    matches!(
        item_type,
        ItemType::Food | ItemType::Makeup | ItemType::Consumable
    )
}

/// Count how many items of a given type a girl has equipped.
fn count_equipped_of_type(girl: &Girl, items: &[Item], item_type: ItemType) -> usize {
    girl.inventory
        .iter()
        .enumerate()
        .filter(|(i, &item_id)| {
            *i < girl.equipped.len()
                && girl.equipped[*i]
                && item_id < items.len()
                && items[item_id].item_type == item_type
        })
        .count()
}

/// Check if a girl can equip an item of the given type.
/// If `force` is true and slots are full, returns the inventory index to unequip first.
pub fn can_equip(girl: &Girl, items: &[Item], item_type: ItemType, force: bool) -> EquipCheck {
    let max = max_equip_slots(item_type);
    if max == 0 {
        // Consumable — always "equippable" (use immediately)
        if is_consumable(item_type) {
            return EquipCheck::Consumable;
        }
        return EquipCheck::CannotEquip;
    }

    let count = count_equipped_of_type(girl, items, item_type);
    if count < max {
        return EquipCheck::Ok;
    }

    if force {
        // Find the last equipped item of this type to auto-unequip
        for (i, &item_id) in girl.inventory.iter().enumerate().rev() {
            if i < girl.equipped.len()
                && girl.equipped[i]
                && item_id < items.len()
                && items[item_id].item_type == item_type
            {
                return EquipCheck::ForceUnequip(i);
            }
        }
    }

    EquipCheck::SlotsFull
}

#[derive(Debug, PartialEq)]
pub enum EquipCheck {
    Ok,
    Consumable,
    CannotEquip,
    SlotsFull,
    ForceUnequip(usize),
}

/// Add an item to a girl's inventory. Returns false if inventory full.
pub fn add_item(girl: &mut Girl, item_id: usize) -> bool {
    if girl.inventory.len() >= MAX_INVENTORY {
        return false;
    }
    girl.inventory.push(item_id);
    girl.equipped.push(false);
    true
}

/// Remove an item from a girl's inventory by inventory slot index.
/// Unequips first if equipped.
pub fn remove_item(girl: &mut Girl, items: &[Item], slot: usize) -> bool {
    if slot >= girl.inventory.len() {
        return false;
    }
    if slot < girl.equipped.len() && girl.equipped[slot] {
        unequip(girl, items, slot);
    }
    girl.inventory.remove(slot);
    if slot < girl.equipped.len() {
        girl.equipped.remove(slot);
    }
    true
}

/// Equip an item at the given inventory slot.
/// Applies stat/skill/trait effects to the girl.
pub fn equip(girl: &mut Girl, items: &[Item], slot: usize, force: bool) -> bool {
    if slot >= girl.inventory.len() {
        return false;
    }
    // Ensure equipped vec is long enough
    while girl.equipped.len() <= slot {
        girl.equipped.push(false);
    }
    if girl.equipped[slot] {
        return false; // Already equipped
    }

    let item_id = girl.inventory[slot];
    if item_id >= items.len() {
        return false;
    }

    let item_type = items[item_id].item_type;
    let check = can_equip(girl, items, item_type, force);

    match check {
        EquipCheck::Ok => {}
        EquipCheck::Consumable => {
            // Apply effects then remove from inventory
            apply_effects(girl, &items[item_id], false);
            girl.inventory.remove(slot);
            girl.equipped.remove(slot);
            return true;
        }
        EquipCheck::ForceUnequip(unequip_slot) => {
            unequip(girl, items, unequip_slot);
        }
        EquipCheck::SlotsFull | EquipCheck::CannotEquip => {
            return false;
        }
    }

    // Apply permanent effects
    let is_temporary = items[item_id].special.eq_ignore_ascii_case("temporary");
    apply_effects(girl, &items[item_id], is_temporary);
    girl.equipped[slot] = true;
    true
}

/// Unequip an item at the given inventory slot.
/// Reverses stat/skill effects.
pub fn unequip(girl: &mut Girl, items: &[Item], slot: usize) -> bool {
    if slot >= girl.equipped.len() || !girl.equipped[slot] {
        return false;
    }
    let item_id = girl.inventory[slot];
    if item_id >= items.len() {
        return false;
    }

    let is_temporary = items[item_id].special.eq_ignore_ascii_case("temporary");
    reverse_effects(girl, &items[item_id], is_temporary);
    girl.equipped[slot] = false;
    true
}

/// Apply item effects to a girl.
fn apply_effects(girl: &mut Girl, item: &Item, temporary: bool) {
    for effect in &item.effects {
        match effect.target {
            EffectTarget::Stat => {
                if let Some(stat) = stat_from_name(&effect.name) {
                    if temporary {
                        GirlManager::update_temp_stat(girl, stat, effect.amount);
                    } else {
                        GirlManager::update_stat(girl, stat, effect.amount);
                    }
                }
            }
            EffectTarget::Skill => {
                if let Some(skill) = skill_from_name(&effect.name) {
                    if temporary {
                        GirlManager::update_temp_skill(girl, skill, effect.amount);
                    } else {
                        GirlManager::update_skill(girl, skill, effect.amount);
                    }
                }
            }
            EffectTarget::Trait => {
                if effect.amount > 0 {
                    GirlManager::add_trait(girl, &effect.name);
                } else {
                    GirlManager::remove_trait(girl, &effect.name);
                }
            }
        }
    }
}

/// Reverse item effects (on unequip).
fn reverse_effects(girl: &mut Girl, item: &Item, temporary: bool) {
    for effect in &item.effects {
        match effect.target {
            EffectTarget::Stat => {
                if let Some(stat) = stat_from_name(&effect.name) {
                    if temporary {
                        GirlManager::update_temp_stat(girl, stat, -effect.amount);
                    } else {
                        GirlManager::update_stat(girl, stat, -effect.amount);
                    }
                }
            }
            EffectTarget::Skill => {
                if let Some(skill) = skill_from_name(&effect.name) {
                    if temporary {
                        GirlManager::update_temp_skill(girl, skill, -effect.amount);
                    } else {
                        GirlManager::update_skill(girl, skill, -effect.amount);
                    }
                }
            }
            EffectTarget::Trait => {
                // Reverse: if we added a trait on equip, remove it on unequip (and vice versa)
                if effect.amount > 0 {
                    GirlManager::remove_trait(girl, &effect.name);
                } else {
                    GirlManager::add_trait(girl, &effect.name);
                }
            }
        }
    }
}

/// Girl auto-buys an item. Matches C++ GirlBuyItem.
/// Returns true if the girl bought and equipped the item.
pub fn girl_buy_item(girl: &mut Girl, items: &[Item], item_id: usize, girl_gold: &mut i32) -> bool {
    if item_id >= items.len() {
        return false;
    }
    let item = &items[item_id];
    if *girl_gold < item.cost {
        return false;
    }

    let max = max_equip_slots(item.item_type);
    if max > 0 {
        let count = count_equipped_of_type(girl, items, item.item_type);
        if count >= max {
            // Find worst equipped item of same type (lowest cost) and sell it
            let mut worst_slot: Option<usize> = None;
            let mut worst_cost = i32::MAX;
            for (i, &inv_id) in girl.inventory.iter().enumerate() {
                if i < girl.equipped.len()
                    && girl.equipped[i]
                    && inv_id < items.len()
                    && items[inv_id].item_type == item.item_type
                    && items[inv_id].cost < worst_cost
                {
                    worst_cost = items[inv_id].cost;
                    worst_slot = Some(i);
                }
            }
            if let Some(slot) = worst_slot {
                if worst_cost < item.cost {
                    // Sell old item (half price)
                    *girl_gold += worst_cost / 2;
                    remove_item(girl, items, slot);
                } else {
                    return false; // Not worth buying
                }
            }
        }
    }

    *girl_gold -= item.cost;
    if !add_item(girl, item_id) {
        *girl_gold += item.cost; // Refund — inventory full
        return false;
    }
    let slot = girl.inventory.len() - 1;
    equip(girl, items, slot, true);
    true
}

/// Calculate happiness from receiving an item. Matches C++ HappinessFromItem.
pub fn happiness_from_item(item: &Item) -> i32 {
    let mut happy = item.cost / 20;
    if item.badness > 0 {
        happy -= item.badness;
    }
    happy.clamp(-10, 15)
}

/// Resolve stat name to Stat enum.
fn stat_from_name(name: &str) -> Option<Stat> {
    let lower = name.to_lowercase();
    match lower.as_str() {
        "charisma" => Some(Stat::Charisma),
        "happiness" => Some(Stat::Happiness),
        "libido" => Some(Stat::Libido),
        "constitution" => Some(Stat::Constitution),
        "intelligence" => Some(Stat::Intelligence),
        "confidence" => Some(Stat::Confidence),
        "mana" => Some(Stat::Mana),
        "agility" => Some(Stat::Agility),
        "fame" => Some(Stat::Fame),
        "level" => Some(Stat::Level),
        "askprice" | "ask price" => Some(Stat::AskPrice),
        "houseperc" | "house percentage" => Some(Stat::HousePerc),
        "exp" | "experience" => Some(Stat::Exp),
        "age" => Some(Stat::Age),
        "obedience" => Some(Stat::Obedience),
        "spirit" => Some(Stat::Spirit),
        "beauty" => Some(Stat::Beauty),
        "tiredness" => Some(Stat::Tiredness),
        "health" => Some(Stat::Health),
        "pcfear" | "fear" => Some(Stat::PCFear),
        "pclove" | "love" => Some(Stat::PCLove),
        "pchate" | "hate" => Some(Stat::PCHate),
        _ => None,
    }
}

/// Resolve skill name to Skill enum.
fn skill_from_name(name: &str) -> Option<Skill> {
    let lower = name.to_lowercase();
    match lower.as_str() {
        "anal" => Some(Skill::Anal),
        "magic" => Some(Skill::Magic),
        "bdsm" => Some(Skill::BDSM),
        "normalsex" | "normal sex" | "sex" => Some(Skill::NormalSex),
        "beastiality" | "bestiality" => Some(Skill::Beastiality),
        "group" => Some(Skill::Group),
        "lesbian" => Some(Skill::Lesbian),
        "service" => Some(Skill::Service),
        "strip" | "stripping" => Some(Skill::Strip),
        "combat" => Some(Skill::Combat),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wm_core::item::Effect;

    fn make_ring(name: &str, stat: &str, amount: i32) -> Item {
        Item {
            name: name.to_string(),
            desc: String::new(),
            item_type: ItemType::Ring,
            badness: 0,
            special: String::new(),
            cost: 100,
            rarity: wm_core::enums::Rarity::Common,
            infinite: false,
            girl_buy_chance: 0,
            effects: vec![Effect {
                target: EffectTarget::Stat,
                name: stat.to_string(),
                amount,
            }],
        }
    }

    #[test]
    fn test_equip_unequip_ring() {
        let items = vec![make_ring("Ring of Power", "Charisma", 10)];
        let mut girl = Girl::default();
        add_item(&mut girl, 0);

        let base = GirlManager::get_stat(&girl, Stat::Charisma);
        equip(&mut girl, &items, 0, false);
        assert!(girl.equipped[0]);
        assert_eq!(GirlManager::get_stat(&girl, Stat::Charisma), base + 10);

        unequip(&mut girl, &items, 0);
        assert!(!girl.equipped[0]);
        assert_eq!(GirlManager::get_stat(&girl, Stat::Charisma), base);
    }

    #[test]
    fn test_ring_slot_limit() {
        let items: Vec<Item> = (0..9)
            .map(|i| make_ring(&format!("Ring {i}"), "Charisma", 1))
            .collect();
        let mut girl = Girl::default();
        // Add and equip 8 rings
        for id in 0..8 {
            add_item(&mut girl, id);
            assert!(equip(&mut girl, &items, id, false));
        }
        // 9th ring should fail without force
        add_item(&mut girl, 8);
        assert!(!equip(&mut girl, &items, 8, false));
    }

    #[test]
    fn test_consumable_removed() {
        let items = vec![Item {
            name: "Healing Food".to_string(),
            desc: String::new(),
            item_type: ItemType::Food,
            badness: 0,
            special: String::new(),
            cost: 10,
            rarity: wm_core::enums::Rarity::Common,
            infinite: false,
            girl_buy_chance: 0,
            effects: vec![Effect {
                target: EffectTarget::Stat,
                name: "Health".to_string(),
                amount: 20,
            }],
        }];
        let mut girl = Girl::default();
        let base_hp = GirlManager::get_stat(&girl, Stat::Health);
        add_item(&mut girl, 0);
        equip(&mut girl, &items, 0, false);
        // Item consumed — removed from inventory
        assert!(girl.inventory.is_empty());
        assert_eq!(GirlManager::get_stat(&girl, Stat::Health), base_hp + 20);
    }

    #[test]
    fn test_happiness_from_item() {
        let item = make_ring("Cheap Ring", "Charisma", 1);
        assert_eq!(happiness_from_item(&item), 5); // 100/20 = 5
    }
}
