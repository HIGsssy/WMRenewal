use wm_core::enums::GangMission;

/// A single gang.
#[derive(Debug, Clone)]
pub struct Gang {
    pub name: String,
    pub num_members: i32,
    pub max_members: i32,
    pub mission: GangMission,
    pub combat_skill: i32,
    pub magic_skill: i32,
    pub intelligence: i32,
    pub agility: i32,
    pub constitution: i32,
    pub charisma: i32,
    pub hired: bool,
    pub net_worth: i32,
}

impl Default for Gang {
    fn default() -> Self {
        Self {
            name: String::new(),
            num_members: 5,
            max_members: 15,
            mission: GangMission::Guarding,
            combat_skill: 50,
            magic_skill: 10,
            intelligence: 30,
            agility: 30,
            constitution: 30,
            charisma: 30,
            hired: false,
            net_worth: 0,
        }
    }
}

/// Manages hired and recruitable gangs.
#[derive(Debug)]
pub struct GangManager {
    pub hired_gangs: Vec<Gang>,
    pub recruit_list: Vec<Gang>,
}

impl Default for GangManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GangManager {
    pub fn new() -> Self {
        Self {
            hired_gangs: Vec::new(),
            recruit_list: Vec::new(),
        }
    }

    pub fn hire_gang(&mut self, _index: usize) -> bool {
        todo!()
    }

    pub fn fire_gang(&mut self, _index: usize) {
        todo!()
    }

    pub fn set_mission(&mut self, _gang_index: usize, _mission: GangMission) {
        todo!()
    }

    pub fn process_missions(&mut self) {
        todo!()
    }

    pub fn weekly_recruit_update(&mut self) {
        todo!()
    }

    pub fn total_goon_wages(&self) -> f64 {
        todo!()
    }
}
