use serde::{Deserialize, Serialize};

use crate::enums::{EffectTarget, ItemType, Rarity};

/// A game item with stat/skill/trait effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub item_type: ItemType,
    pub badness: i32,
    pub special: String,
    pub cost: i32,
    pub rarity: Rarity,
    pub infinite: bool,
    pub girl_buy_chance: i32,
    pub effects: Vec<Effect>,
}

/// A single effect applied when an item is used/equipped.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub target: EffectTarget,
    pub name: String,
    pub amount: i32,
}

// -- XML deserialization structs (quick-xml + serde) --

#[derive(Debug, Deserialize)]
pub struct ItemsXml {
    #[serde(rename = "Item", default)]
    pub items: Vec<ItemXml>,
}

#[derive(Debug, Deserialize)]
pub struct ItemXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Desc", default)]
    pub desc: String,
    #[serde(rename = "@Type")]
    pub item_type: String,
    #[serde(rename = "@Badness", default)]
    pub badness: i32,
    #[serde(rename = "@Special", default)]
    pub special: String,
    #[serde(rename = "@Cost", default)]
    pub cost: i32,
    #[serde(rename = "@Rarity", default)]
    pub rarity: String,
    #[serde(rename = "@Infinite", default)]
    pub infinite: String,
    #[serde(rename = "@GirlBuyChance", default)]
    pub girl_buy_chance: i32,
    #[serde(rename = "Effect", default)]
    pub effects: Vec<EffectXml>,
}

#[derive(Debug, Deserialize)]
pub struct EffectXml {
    #[serde(rename = "@What")]
    pub what: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Amount", default)]
    pub amount: i32,
}

impl ItemXml {
    /// Convert from XML representation to domain Item.
    pub fn into_item(self) -> Item {
        let item_type = match self.item_type.as_str() {
            "Food" => ItemType::Food,
            "Ring" => ItemType::Ring,
            "Necklace" => ItemType::Necklace,
            "Dress" => ItemType::Dress,
            "Underwear" => ItemType::Underwear,
            "Shoes" => ItemType::Shoes,
            "Hat" => ItemType::Hat,
            "Helmet" => ItemType::Helmet,
            "SmallWeapon" => ItemType::SmallWeapon,
            "LargeWeapon" => ItemType::LargeWeapon,
            "Armor" => ItemType::Armor,
            "Shield" => ItemType::Shield,
            "Consumable" => ItemType::Consumable,
            "Makeup" => ItemType::Makeup,
            _ => ItemType::Misc,
        };

        let rarity = match self.rarity.as_str() {
            "Common" => Rarity::Common,
            "Shop50" => Rarity::Shop50,
            "Shop25" => Rarity::Shop25,
            "Shop05" => Rarity::Shop05,
            "Catacomb15" => Rarity::Catacomb15,
            "ScriptOnly" => Rarity::ScriptOnly,
            "Reward" => Rarity::Reward,
            _ => Rarity::Common,
        };

        let infinite = matches!(self.infinite.to_lowercase().as_str(), "true" | "1");

        let effects = self
            .effects
            .into_iter()
            .map(|e| {
                let target = match e.what.as_str() {
                    "Stat" => EffectTarget::Stat,
                    "Skill" => EffectTarget::Skill,
                    "Trait" => EffectTarget::Trait,
                    _ => EffectTarget::Stat,
                };
                Effect {
                    target,
                    name: e.name,
                    amount: e.amount,
                }
            })
            .collect();

        Item {
            name: self.name,
            desc: self.desc,
            item_type,
            badness: self.badness,
            special: self.special,
            cost: self.cost,
            rarity,
            infinite,
            girl_buy_chance: self.girl_buy_chance,
            effects,
        }
    }
}
