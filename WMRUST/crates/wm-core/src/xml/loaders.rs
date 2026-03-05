use std::path::Path;

use crate::config::{ConfigXml, GameConfig};
use crate::enums::{EffectTarget, ItemType, Rarity, Skill, Stat};
use crate::girl::{Girl, GirlXml, GirlsXml};
use crate::item::{Item, ItemXml, ItemsXml};
use crate::room::{FacilitiesXml, FacilityXml, Room};
use crate::screen::{ScreenLayout, ScreenXml};
use crate::traits::{parse_traits, TraitDef};

/// Error type for resource loading operations.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("XML deserialization error: {0}")]
    Xml(#[from] quick_xml::DeError),
}

/// Load items from an Items.itemsx XML file.
pub fn load_items(path: &Path) -> Result<Vec<Item>, LoadError> {
    let xml_str = std::fs::read_to_string(path)?;
    let items_xml: ItemsXml = quick_xml::de::from_str(&xml_str)?;
    Ok(items_xml
        .items
        .into_iter()
        .map(ItemXml::into_item)
        .collect())
}

/// Load rooms/facilities from a Rooms.roomsx XML file.
pub fn load_rooms(path: &Path) -> Result<Vec<Room>, LoadError> {
    let xml_str = std::fs::read_to_string(path)?;
    let facilities: FacilitiesXml = quick_xml::de::from_str(&xml_str)?;
    Ok(facilities
        .facilities
        .into_iter()
        .map(FacilityXml::into_room)
        .collect())
}

/// Load game configuration from config.xml.
pub fn load_config(path: &Path) -> Result<GameConfig, LoadError> {
    let xml_str = std::fs::read_to_string(path)?;
    let config_xml: ConfigXml = quick_xml::de::from_str(&xml_str)?;
    Ok(config_xml.into_config())
}

/// Load girl definitions from a Girls.girlsx XML file.
pub fn load_girls(path: &Path) -> Result<Vec<Girl>, LoadError> {
    let xml_str = std::fs::read_to_string(path)?;
    let girls_xml: GirlsXml = quick_xml::de::from_str(&xml_str)?;
    Ok(girls_xml
        .girls
        .into_iter()
        .map(GirlXml::into_girl)
        .collect())
}

/// Load trait definitions from CoreTraits.traits (plain text format).
pub fn load_traits(path: &Path) -> Result<Vec<TraitDef>, LoadError> {
    let text = std::fs::read_to_string(path)?;
    Ok(parse_traits(&text))
}

/// Load a screen layout from an Interface/*.xml file.
pub fn load_screen(path: &Path) -> Result<ScreenLayout, LoadError> {
    let xml_str = std::fs::read_to_string(path)?;
    let screen_xml: ScreenXml = quick_xml::de::from_str(&xml_str)?;
    Ok(screen_xml.into_layout())
}

// ---------------------------------------------------------------------------
// Save / write functions
// ---------------------------------------------------------------------------

/// Escape special XML characters in attribute values.
fn escape_xml_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Convert ItemType to its XML attribute string.
pub fn item_type_to_str(t: ItemType) -> &'static str {
    match t {
        ItemType::Food => "Food",
        ItemType::Ring => "Ring",
        ItemType::Necklace => "Necklace",
        ItemType::Dress => "Dress",
        ItemType::Underwear => "Underwear",
        ItemType::Shoes => "Shoes",
        ItemType::Hat => "Hat",
        ItemType::Helmet => "Helmet",
        ItemType::SmallWeapon => "SmallWeapon",
        ItemType::LargeWeapon => "LargeWeapon",
        ItemType::Armor => "Armor",
        ItemType::Shield => "Shield",
        ItemType::Consumable => "Consumable",
        ItemType::Makeup => "Makeup",
        ItemType::Misc => "Misc",
    }
}

/// Convert Rarity to its XML attribute string.
pub fn rarity_to_str(r: Rarity) -> &'static str {
    match r {
        Rarity::Common => "Common",
        Rarity::Shop50 => "Shop50",
        Rarity::Shop25 => "Shop25",
        Rarity::Shop05 => "Shop05",
        Rarity::Catacomb15 => "Catacomb15",
        Rarity::ScriptOnly => "ScriptOnly",
        Rarity::Reward => "Reward",
    }
}

/// Convert EffectTarget to its XML attribute string.
pub fn effect_target_to_str(t: EffectTarget) -> &'static str {
    match t {
        EffectTarget::Stat => "Stat",
        EffectTarget::Skill => "Skill",
        EffectTarget::Trait => "Trait",
    }
}

/// Save girl definitions to a .girlsx XML file.
pub fn save_girls(path: &Path, girls: &[Girl]) -> Result<(), LoadError> {
    use std::fmt::Write;
    let mut xml = String::from("<Girls>\n");
    for girl in girls {
        write!(xml, "  <Girl Name=\"{}\"", escape_xml_attr(&girl.name)).unwrap();
        if !girl.desc.is_empty() {
            write!(xml, " Desc=\"{}\"", escape_xml_attr(&girl.desc)).unwrap();
        }
        if girl.money != 0 {
            write!(xml, " Gold=\"{}\"", girl.money).unwrap();
        }
        for &(attr, stat) in &[
            ("Charisma", Stat::Charisma),
            ("Happiness", Stat::Happiness),
            ("Libido", Stat::Libido),
            ("Constitution", Stat::Constitution),
            ("Intelligence", Stat::Intelligence),
            ("Confidence", Stat::Confidence),
            ("Mana", Stat::Mana),
            ("Agility", Stat::Agility),
            ("Fame", Stat::Fame),
            ("Level", Stat::Level),
            ("AskPrice", Stat::AskPrice),
            ("House", Stat::HousePerc),
            ("Exp", Stat::Exp),
            ("Age", Stat::Age),
            ("Obedience", Stat::Obedience),
            ("Spirit", Stat::Spirit),
            ("Beauty", Stat::Beauty),
            ("Tiredness", Stat::Tiredness),
            ("Health", Stat::Health),
            ("PCFear", Stat::PCFear),
            ("PCLove", Stat::PCLove),
            ("PCHate", Stat::PCHate),
        ] {
            write!(xml, " {}=\"{}\"", attr, girl.stats[stat as usize]).unwrap();
        }
        for &(attr, skill) in &[
            ("Anal", Skill::Anal),
            ("Magic", Skill::Magic),
            ("BDSM", Skill::BDSM),
            ("NormalSex", Skill::NormalSex),
            ("Beastiality", Skill::Beastiality),
            ("Group", Skill::Group),
            ("Lesbian", Skill::Lesbian),
            ("Service", Skill::Service),
            ("Strip", Skill::Strip),
            ("Combat", Skill::Combat),
        ] {
            write!(xml, " {}=\"{}\"", attr, girl.skills[skill as usize]).unwrap();
        }
        if girl.traits.is_empty() {
            writeln!(xml, " />").unwrap();
        } else {
            writeln!(xml, ">").unwrap();
            for t in &girl.traits {
                writeln!(xml, "    <Trait Name=\"{}\"/>", escape_xml_attr(t)).unwrap();
            }
            writeln!(xml, "  </Girl>").unwrap();
        }
    }
    xml.push_str("</Girls>\n");
    std::fs::write(path, xml)?;
    Ok(())
}

/// Save items to a .itemsx XML file.
pub fn save_items(path: &Path, items: &[Item]) -> Result<(), LoadError> {
    use std::fmt::Write;
    let mut xml = String::from("<Items>\n");
    for item in items {
        write!(xml, "  <Item Name=\"{}\"", escape_xml_attr(&item.name)).unwrap();
        if !item.desc.is_empty() {
            write!(xml, " Desc=\"{}\"", escape_xml_attr(&item.desc)).unwrap();
        }
        write!(xml, " Type=\"{}\"", item_type_to_str(item.item_type)).unwrap();
        if item.badness != 0 {
            write!(xml, " Badness=\"{}\"", item.badness).unwrap();
        }
        if !item.special.is_empty() {
            write!(xml, " Special=\"{}\"", escape_xml_attr(&item.special)).unwrap();
        }
        write!(xml, " Cost=\"{}\"", item.cost).unwrap();
        write!(xml, " Rarity=\"{}\"", rarity_to_str(item.rarity)).unwrap();
        if item.infinite {
            write!(xml, " Infinite=\"true\"").unwrap();
        }
        if item.girl_buy_chance != 0 {
            write!(xml, " GirlBuyChance=\"{}\"", item.girl_buy_chance).unwrap();
        }
        if item.effects.is_empty() {
            writeln!(xml, " />").unwrap();
        } else {
            writeln!(xml, ">").unwrap();
            for effect in &item.effects {
                writeln!(
                    xml,
                    "    <Effect What=\"{}\" Name=\"{}\" Amount=\"{}\"/>",
                    effect_target_to_str(effect.target),
                    escape_xml_attr(&effect.name),
                    effect.amount
                )
                .unwrap();
            }
            writeln!(xml, "  </Item>").unwrap();
        }
    }
    xml.push_str("</Items>\n");
    std::fs::write(path, xml)?;
    Ok(())
}

/// Save trait definitions to a .traits plain-text file.
pub fn save_traits(path: &Path, traits_list: &[TraitDef]) -> Result<(), LoadError> {
    use std::fmt::Write;
    let mut text = String::new();
    for t in traits_list {
        writeln!(text, "{}", t.name).unwrap();
        writeln!(text, "{}", t.description).unwrap();
    }
    std::fs::write(path, text)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn resources_dir() -> PathBuf {
        // Try the symlink/junction in WMRUST/resources first, then fall back
        // to the relative path to the original WhoreMasterRenewal resources.
        let from_workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../resources");
        if from_workspace.join("Data").exists() {
            return from_workspace;
        }
        let from_original =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../WhoreMasterRenewal/Resources");
        if from_original.join("Data").exists() {
            return from_original;
        }
        panic!("Could not locate game Resources directory for tests");
    }

    #[test]
    fn test_load_items() {
        let path = resources_dir().join("Data/Items.itemsx");
        if !path.exists() {
            eprintln!("Skipping test_load_items: {:?} not found", path);
            return;
        }
        let items = load_items(&path).expect("Failed to load Items.itemsx");
        assert!(
            items.len() > 50,
            "Should have 50+ items, got {}",
            items.len()
        );

        // Verify a known item
        let aids_cure = items.iter().find(|i| i.name == "AIDS Cure");
        assert!(aids_cure.is_some(), "Should contain 'AIDS Cure' item");
        let aids_cure = aids_cure.unwrap();
        assert_eq!(aids_cure.cost, 3500);
        assert!(
            !aids_cure.effects.is_empty(),
            "AIDS Cure should have effects"
        );
    }

    #[test]
    fn test_load_rooms() {
        let path = resources_dir().join("Data/Rooms.roomsx");
        if !path.exists() {
            eprintln!("Skipping test_load_rooms: {:?} not found", path);
            return;
        }
        let rooms = load_rooms(&path).expect("Failed to load Rooms.roomsx");
        assert!(!rooms.is_empty(), "Should load at least one room");

        // Verify first room (Bedroom)
        let bedroom = rooms.iter().find(|r| r.name == "Bedroom");
        assert!(bedroom.is_some(), "Should contain 'Bedroom' facility");
        let bedroom = bedroom.unwrap();
        assert_eq!(bedroom.space, 4);
        assert_eq!(bedroom.provides, 4);
        assert_eq!(bedroom.price, 100);
        assert!(
            !bedroom.functions.is_empty(),
            "Bedroom should have functions"
        );

        // Verify percentage Success value parsed correctly
        let dorm = rooms.iter().find(|r| r.name == "Dormitory Unit");
        assert!(dorm.is_some(), "Should contain 'Dormitory Unit'");
        let dorm = dorm.unwrap();
        let whoring_fn = dorm.functions.iter().find(|f| f.name == "Whoring");
        assert!(
            whoring_fn.is_some(),
            "Dormitory should have Whoring function"
        );
        let whoring_fn = whoring_fn.unwrap();
        assert!(
            whoring_fn.success.is_some(),
            "Whoring should have Success value"
        );
        assert!(
            (whoring_fn.success.unwrap() - 30.0).abs() < 0.01,
            "Success should be 30%"
        );
    }

    #[test]
    fn test_load_config() {
        let path = resources_dir().join("Data/config.xml");
        if !path.exists() {
            eprintln!("Skipping test_load_config: {:?} not found", path);
            return;
        }
        let config = load_config(&path).expect("Failed to load config.xml");
        assert_eq!(config.initial.gold, 4000);
        assert_eq!(config.initial.girl_meet, 30);
        assert_eq!(config.gambling.odds, 49);
        // Tax rate parsed from "6%" → 0.06
        assert!(
            (config.tax.rate - 0.06).abs() < 0.001,
            "Tax rate should be 0.06, got {}",
            config.tax.rate
        );
    }

    #[test]
    fn test_load_traits() {
        let path = resources_dir().join("Data/CoreTraits.traits");
        if !path.exists() {
            eprintln!("Skipping test_load_traits: {:?} not found", path);
            return;
        }
        let traits = load_traits(&path).expect("Failed to load CoreTraits.traits");
        assert!(!traits.is_empty(), "Should load at least one trait");
        assert!(!traits[0].name.is_empty());

        // Check for known traits
        let big_boobs = traits.iter().find(|t| t.name == "Big Boobs");
        assert!(big_boobs.is_some(), "Should contain 'Big Boobs' trait");
        assert!(!big_boobs.unwrap().description.is_empty());
    }

    #[test]
    fn test_load_girls() {
        let path = resources_dir().join("Characters/Girls.girlsx");
        if !path.exists() {
            eprintln!("Skipping test_load_girls: {:?} not found", path);
            return;
        }
        let girls = load_girls(&path).expect("Failed to load Girls.girlsx");
        assert!(!girls.is_empty(), "Should load at least one girl");

        // Verify a known girl
        let meiling = girls.iter().find(|g| g.name == "Hong Meiling");
        assert!(meiling.is_some(), "Should contain 'Hong Meiling'");
        let meiling = meiling.unwrap();
        assert_eq!(meiling.stats[crate::enums::Stat::Charisma as usize], 10);
        assert_eq!(meiling.stats[crate::enums::Stat::Health as usize], 100);
        assert_eq!(meiling.stats[crate::enums::Stat::Agility as usize], 60);
        assert_eq!(meiling.skills[crate::enums::Skill::Combat as usize], 30);
        assert!(meiling.traits.contains(&"Big Boobs".to_string()));
        assert!(meiling.traits.contains(&"Strong".to_string()));
    }

    #[test]
    fn test_load_screen_bank() {
        let path = resources_dir().join("Interface/bank_screen.xml");
        if !path.exists() {
            eprintln!("Skipping test_load_screen_bank: {:?} not found", path);
            return;
        }
        let layout = load_screen(&path).expect("Failed to load bank_screen.xml");
        assert!(
            !layout.widgets.is_empty(),
            "Bank screen should have widgets"
        );

        // Count widget types
        let mut buttons = 0;
        let mut texts = 0;
        let mut images = 0;
        let mut windows = 0;
        for w in &layout.widgets {
            match w {
                crate::screen::WidgetDef::Button(_) => buttons += 1,
                crate::screen::WidgetDef::Text(_) => texts += 1,
                crate::screen::WidgetDef::Image(_) => images += 1,
                crate::screen::WidgetDef::Window(_) => windows += 1,
                _ => {}
            }
        }
        assert_eq!(windows, 1, "Bank screen should have 1 window");
        assert!(buttons >= 3, "Bank screen should have at least 3 buttons");
        assert!(texts >= 1, "Bank screen should have at least 1 text");
        assert!(images >= 1, "Bank screen should have at least 1 image");
    }

    #[test]
    fn test_load_screen_dungeon() {
        let path = resources_dir().join("Interface/dungeon_screen.xml");
        if !path.exists() {
            eprintln!("Skipping test_load_screen_dungeon: {:?} not found", path);
            return;
        }
        let layout = load_screen(&path).expect("Failed to load dungeon_screen.xml");

        // Should have a ListBox with columns
        let has_listbox = layout
            .widgets
            .iter()
            .any(|w| matches!(w, crate::screen::WidgetDef::ListBox(lb) if !lb.columns.is_empty()));
        assert!(
            has_listbox,
            "Dungeon screen should have a ListBox with columns"
        );
    }
}

#[cfg(test)]
mod gold_tests {
    use crate::gold::Gold;

    #[test]
    fn test_deposit_withdraw() {
        let mut gold = Gold::new(4000);
        assert_eq!(gold.cash_on_hand, 4000.0);

        gold.deposit(1000.0).unwrap();
        assert_eq!(gold.cash_on_hand, 3000.0);
        assert_eq!(gold.bank_balance, 1000.0);

        gold.withdraw(500.0).unwrap();
        assert_eq!(gold.cash_on_hand, 3500.0);
        assert_eq!(gold.bank_balance, 500.0);

        // Overdraft should fail
        assert!(gold.deposit(5000.0).is_err());
        assert!(gold.withdraw(1000.0).is_err());
    }

    #[test]
    fn test_income_tracking() {
        let mut gold = Gold::new(1000);
        gold.add_brothel_work(500.0);
        gold.add_bar_income(200.0);

        assert_eq!(gold.total_income(), 700.0);
        assert_eq!(gold.cash_on_hand, 1700.0);
    }

    #[test]
    fn test_expense_tracking() {
        let mut gold = Gold::new(5000);
        assert!(gold.pay_slave_cost(2000.0));
        gold.charge_goon_wages(300.0);

        assert_eq!(gold.total_expenses(), 2300.0);
        assert_eq!(gold.cash_on_hand, 2700.0);
        assert_eq!(gold.net_income(), -2300.0);
    }

    #[test]
    fn test_instant_expense_fails_when_broke() {
        let mut gold = Gold::new(100);
        assert!(!gold.pay_slave_cost(500.0));
        assert_eq!(gold.cash_on_hand, 100.0); // unchanged
    }

    #[test]
    fn test_forced_expense_allows_debt() {
        let mut gold = Gold::new(100);
        gold.charge_goon_wages(500.0);
        assert_eq!(gold.cash_on_hand, -400.0); // went into debt
    }

    #[test]
    fn test_reset_weekly() {
        let mut gold = Gold::new(1000);
        gold.add_brothel_work(500.0);
        gold.charge_goon_wages(200.0);
        assert_eq!(gold.total_income(), 500.0);
        assert_eq!(gold.total_expenses(), 200.0);

        gold.reset_weekly();
        assert_eq!(gold.total_income(), 0.0);
        assert_eq!(gold.total_expenses(), 0.0);
        assert_eq!(gold.cash_on_hand, 1300.0); // cash preserved
    }

    #[test]
    fn test_bank_interest_goes_to_bank() {
        let mut gold = Gold::new(1000);
        gold.deposit(500.0).unwrap();
        gold.add_bank_interest(25.0);
        assert_eq!(gold.bank_balance, 525.0);
        assert_eq!(gold.cash_on_hand, 500.0); // unchanged
    }
}

#[cfg(test)]
mod save_tests {
    use super::*;
    use std::path::PathBuf;

    fn resources_dir() -> PathBuf {
        let from_workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../resources");
        if from_workspace.join("Data").exists() {
            return from_workspace;
        }
        let from_original =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../WhoreMasterRenewal/Resources");
        if from_original.join("Data").exists() {
            return from_original;
        }
        panic!("Could not locate game Resources directory for tests");
    }

    #[test]
    fn test_girls_round_trip() {
        let path = resources_dir().join("Characters/Girls.girlsx");
        if !path.exists() {
            eprintln!("Skipping: {:?} not found", path);
            return;
        }
        let girls = load_girls(&path).expect("Failed to load");
        assert!(!girls.is_empty());

        let tmp = std::env::temp_dir().join("wm_test_girls.girlsx");
        save_girls(&tmp, &girls).expect("Failed to save");

        let reloaded = load_girls(&tmp).expect("Failed to reload");
        assert_eq!(girls.len(), reloaded.len());

        // Verify first girl matches
        let orig = &girls[0];
        let copy = &reloaded[0];
        assert_eq!(orig.name, copy.name);
        assert_eq!(orig.stats, copy.stats);
        assert_eq!(orig.skills, copy.skills);
        assert_eq!(orig.traits, copy.traits);

        let _ = std::fs::remove_file(&tmp);
    }

    #[test]
    fn test_items_round_trip() {
        let path = resources_dir().join("Data/Items.itemsx");
        if !path.exists() {
            eprintln!("Skipping: {:?} not found", path);
            return;
        }
        let items = load_items(&path).expect("Failed to load");
        assert!(!items.is_empty());

        let tmp = std::env::temp_dir().join("wm_test_items.itemsx");
        save_items(&tmp, &items).expect("Failed to save");

        let reloaded = load_items(&tmp).expect("Failed to reload");
        assert_eq!(items.len(), reloaded.len());

        // Verify a known item round-trips correctly
        let orig = items.iter().find(|i| i.name == "AIDS Cure").unwrap();
        let copy = reloaded.iter().find(|i| i.name == "AIDS Cure").unwrap();
        assert_eq!(orig.cost, copy.cost);
        assert_eq!(orig.item_type, copy.item_type);
        assert_eq!(orig.rarity, copy.rarity);
        assert_eq!(orig.effects.len(), copy.effects.len());

        let _ = std::fs::remove_file(&tmp);
    }

    #[test]
    fn test_traits_round_trip() {
        let path = resources_dir().join("Data/CoreTraits.traits");
        if !path.exists() {
            eprintln!("Skipping: {:?} not found", path);
            return;
        }
        let traits = load_traits(&path).expect("Failed to load");
        assert!(!traits.is_empty());

        let tmp = std::env::temp_dir().join("wm_test_core.traits");
        save_traits(&tmp, &traits).expect("Failed to save");

        let reloaded = load_traits(&tmp).expect("Failed to reload");
        assert_eq!(traits.len(), reloaded.len());
        assert_eq!(traits[0].name, reloaded[0].name);
        assert_eq!(traits[0].description, reloaded[0].description);

        let _ = std::fs::remove_file(&tmp);
    }
}
