use wm_core::config::GameConfig;
use wm_core::enums::GameFlag;
use wm_core::gold::Gold;

use crate::brothel::BrothelManager;
use crate::customers::CustomerGenerator;
use crate::dungeon::DungeonManager;
use crate::gangs::GangManager;
use crate::girls::GirlManager;
use crate::player::Player;
use crate::rivals::RivalManager;

/// Top-level game state holding all managers and data.
#[derive(Debug)]
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
    pub global_flags: [bool; GameFlag::COUNT],
    pub week: u32,
    pub walk_around: bool,
    pub cheats: bool,
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
            global_flags: [false; GameFlag::COUNT],
            week: 1,
            walk_around: false,
            cheats: false,
        }
    }
}
