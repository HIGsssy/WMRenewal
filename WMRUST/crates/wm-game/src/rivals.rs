/// A rival gang/organization.
#[derive(Debug, Clone)]
pub struct Rival {
    pub name: String,
    pub power: i32,
    pub gold: i64,
    pub num_gangs: i32,
    pub num_brothels: i32,
    pub num_girls: i32,
    pub num_bars: i32,
    pub num_gambling_halls: i32,
}

impl Default for Rival {
    fn default() -> Self {
        Self {
            name: String::new(),
            power: 50,
            gold: 5000,
            num_gangs: 2,
            num_brothels: 1,
            num_girls: 5,
            num_bars: 0,
            num_gambling_halls: 0,
        }
    }
}

/// Manages all rival organizations.
#[derive(Debug)]
pub struct RivalManager {
    pub rivals: Vec<Rival>,
}

impl Default for RivalManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RivalManager {
    pub fn new() -> Self {
        Self {
            rivals: Vec::new(),
        }
    }

    pub fn add_rival(&mut self, _rival: Rival) {
        todo!()
    }

    pub fn remove_rival(&mut self, _index: usize) {
        todo!()
    }

    pub fn process_rivals(&mut self) {
        todo!()
    }

    pub fn check_takeover(&self) -> bool {
        todo!()
    }
}
