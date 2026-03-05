use serde::{Deserialize, Serialize};
use wm_core::config::GameConfig;
use wm_core::enums::GameFlag;
use wm_core::gold::Gold;
use wm_core::item::Item;

use crate::brothel::BrothelManager;
use crate::customers::CustomerGenerator;
use crate::dungeon::DungeonManager;
use crate::gangs::GangManager;
use crate::girls::GirlManager;
use crate::jobs::JobDispatcher;
use crate::player::Player;
use crate::rivals::RivalManager;

/// Top-level game state holding all managers and data.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub config: GameConfig,
    pub player: Player,
    pub gold: Gold,
    pub brothels: BrothelManager,
    pub girls: GirlManager,
    pub gangs: GangManager,
    pub customers: CustomerGenerator,
    pub dungeon: DungeonManager,
    pub rivals: RivalManager,
    #[serde(skip)]
    pub job_dispatcher: JobDispatcher,
    pub items: Vec<Item>,
    pub global_flags: [bool; GameFlag::COUNT],
    pub week: u32,
    pub beasts: i32,
    pub healing_potions: i32,
    pub weapon_level: i32,
    pub walk_around: bool,
    pub cheats: bool,
    /// Indices into `items` that are available in the shop this week.
    pub shop_stock: Vec<usize>,
    /// Result from the last TextInputScreen (consumed by the caller after pop).
    #[serde(skip)]
    pub pending_text_result: Option<String>,
}

impl GameState {
    /// Create a new game state with default/initial values.
    pub fn new(config: GameConfig) -> Self {
        let initial_gold = config.initial.gold as f64;
        Self {
            config,
            player: Player::default(),
            gold: Gold {
                cash_on_hand: initial_gold,
                ..Gold::default()
            },
            brothels: BrothelManager::new(),
            girls: GirlManager::new(),
            gangs: GangManager::new(),
            customers: CustomerGenerator::new(),
            dungeon: DungeonManager::new(),
            rivals: RivalManager::new(),
            job_dispatcher: JobDispatcher::new(),
            items: Vec::new(),
            global_flags: [false; GameFlag::COUNT],
            week: 1,
            beasts: 0,
            healing_potions: 5,
            weapon_level: 0,
            walk_around: false,
            cheats: false,
            shop_stock: Vec::new(),
            pending_text_result: None,
        }
    }

    /// Rebuild the shop stock list based on item rarities.
    /// Common items always appear; Shop50/Shop25/Shop05 appear by random chance.
    pub fn refresh_shop_stock(&mut self) {
        use rand::Rng;
        use wm_core::enums::Rarity;

        let mut rng = rand::thread_rng();
        self.shop_stock.clear();
        for (idx, item) in self.items.iter().enumerate() {
            let include = match item.rarity {
                Rarity::Common => true,
                Rarity::Shop50 => rng.gen_range(0..100) < 50,
                Rarity::Shop25 => rng.gen_range(0..100) < 25,
                Rarity::Shop05 => rng.gen_range(0..100) < 5,
                Rarity::Catacomb15 | Rarity::ScriptOnly | Rarity::Reward => false,
            };
            if include {
                self.shop_stock.push(idx);
            }
        }
    }
}
