use std::path::Path;

use crate::config::{ConfigXml, GameConfig};
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
    Ok(items_xml.items.into_iter().map(ItemXml::into_item).collect())
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
    Ok(girls_xml.girls.into_iter().map(GirlXml::into_girl).collect())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn resources_dir() -> PathBuf {
        // Try the symlink/junction in WMRUST/resources first, then fall back
        // to the relative path to the original WhoreMasterRenewal resources.
        let from_workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../resources");
        if from_workspace.join("Data").exists() {
            return from_workspace;
        }
        let from_original = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../WhoreMasterRenewal/Resources");
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
        assert!(items.len() > 50, "Should have 50+ items, got {}", items.len());

        // Verify a known item
        let aids_cure = items.iter().find(|i| i.name == "AIDS Cure");
        assert!(aids_cure.is_some(), "Should contain 'AIDS Cure' item");
        let aids_cure = aids_cure.unwrap();
        assert_eq!(aids_cure.cost, 3500);
        assert!(!aids_cure.effects.is_empty(), "AIDS Cure should have effects");
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
        assert!(!bedroom.functions.is_empty(), "Bedroom should have functions");

        // Verify percentage Success value parsed correctly
        let dorm = rooms.iter().find(|r| r.name == "Dormitory Unit");
        assert!(dorm.is_some(), "Should contain 'Dormitory Unit'");
        let dorm = dorm.unwrap();
        let whoring_fn = dorm.functions.iter().find(|f| f.name == "Whoring");
        assert!(whoring_fn.is_some(), "Dormitory should have Whoring function");
        let whoring_fn = whoring_fn.unwrap();
        assert!(whoring_fn.success.is_some(), "Whoring should have Success value");
        assert!((whoring_fn.success.unwrap() - 30.0).abs() < 0.01, "Success should be 30%");
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
        assert!((config.tax.rate - 0.06).abs() < 0.001, "Tax rate should be 0.06, got {}", config.tax.rate);
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
        assert!(!layout.widgets.is_empty(), "Bank screen should have widgets");

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
        let has_listbox = layout.widgets.iter().any(|w| {
            matches!(w, crate::screen::WidgetDef::ListBox(lb) if !lb.columns.is_empty())
        });
        assert!(has_listbox, "Dungeon screen should have a ListBox with columns");
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
