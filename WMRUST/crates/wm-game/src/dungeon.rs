use wm_core::enums::DungeonReason;
use wm_core::girl::Girl;

/// A prisoner in the dungeon.
#[derive(Debug, Clone)]
pub struct DungeonInmate {
    pub girl: Girl,
    pub reason: DungeonReason,
    pub weeks: u32,
    pub is_customer: bool,
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

    pub fn add_girl(&mut self, _girl: Girl, _reason: DungeonReason) {
        todo!()
    }

    pub fn release(&mut self, _index: usize) -> Option<Girl> {
        todo!()
    }

    pub fn num_inmates(&self) -> usize {
        self.inmates.len()
    }

    pub fn process_week(&mut self) {
        todo!()
    }
}
