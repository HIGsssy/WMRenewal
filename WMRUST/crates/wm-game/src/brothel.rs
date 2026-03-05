use wm_core::enums::{JobType, Shift};
use wm_core::girl::Girl;
use wm_core::room::Room;

/// A single brothel with its girls, rooms, and settings.
#[derive(Debug, Clone)]
pub struct Brothel {
    pub name: String,
    pub num_rooms: i32,
    pub max_num_rooms: i32,
    pub rooms: Vec<Room>,
    pub girls: Vec<usize>, // indices into GirlManager
    pub bar: bool,
    pub gambling_hall: bool,
    pub has_matron: bool,
    pub advertised: f64,
    pub advertising_budget: f64,
    pub filthiness: i32,
    pub security_level: i32,
    pub fame: i32,
    pub happiness: i32,
    pub total_customers: i32,
    pub restrict_anal: bool,
    pub restrict_bdsm: bool,
    pub restrict_beast: bool,
    pub restrict_group: bool,
    pub restrict_normal: bool,
    pub restrict_lesbian: bool,
}

impl Default for Brothel {
    fn default() -> Self {
        Self {
            name: "Brothel".to_string(),
            num_rooms: 20,
            max_num_rooms: 200,
            rooms: Vec::new(),
            girls: Vec::new(),
            bar: false,
            gambling_hall: false,
            has_matron: false,
            advertised: 1.0,
            advertising_budget: 0.0,
            filthiness: 0,
            security_level: 0,
            fame: 0,
            happiness: 0,
            total_customers: 0,
            restrict_anal: false,
            restrict_bdsm: false,
            restrict_beast: false,
            restrict_group: false,
            restrict_normal: false,
            restrict_lesbian: false,
        }
    }
}

impl Brothel {
    pub fn num_girls(&self) -> usize {
        self.girls.len()
    }

    pub fn add_girl(&mut self, girl_id: usize) {
        if !self.girls.contains(&girl_id) {
            self.girls.push(girl_id);
        }
    }

    pub fn remove_girl(&mut self, girl_id: usize) {
        self.girls.retain(|&id| id != girl_id);
    }

    pub fn has_room(&self) -> bool {
        (self.girls.len() as i32) < self.num_rooms
    }

    /// Reset weekly counters at start of turn.
    pub fn reset_weekly(&mut self) {
        self.total_customers = 0;
        self.happiness = 0;
    }

    /// Get girl IDs assigned to a specific job on a given shift.
    pub fn get_girls_on_job(&self, all_girls: &[Girl], job: JobType, shift: Shift) -> Vec<usize> {
        self.girls
            .iter()
            .copied()
            .filter(|&gid| {
                if let Some(girl) = all_girls.get(gid) {
                    match shift {
                        Shift::Day => girl.job_day == Some(job),
                        Shift::Night => girl.job_night == Some(job),
                    }
                } else {
                    false
                }
            })
            .collect()
    }
}

/// Manages all brothels.
#[derive(Debug)]
pub struct BrothelManager {
    pub brothels: Vec<Brothel>,
    pub current: usize,
}

impl Default for BrothelManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BrothelManager {
    pub fn new() -> Self {
        Self {
            brothels: vec![Brothel::default()],
            current: 0,
        }
    }

    pub fn current_brothel(&self) -> &Brothel {
        &self.brothels[self.current]
    }

    pub fn current_brothel_mut(&mut self) -> &mut Brothel {
        &mut self.brothels[self.current]
    }

    pub fn add_brothel(&mut self, name: &str) -> usize {
        let id = self.brothels.len();
        let mut b = Brothel::default();
        b.name = name.to_string();
        self.brothels.push(b);
        id
    }

    pub fn get_brothel(&self, id: usize) -> Option<&Brothel> {
        self.brothels.get(id)
    }

    pub fn get_brothel_mut(&mut self, id: usize) -> Option<&mut Brothel> {
        self.brothels.get_mut(id)
    }

    pub fn num_brothels(&self) -> usize {
        self.brothels.len()
    }

    pub fn get_num_girls(&self, brothel_id: usize) -> usize {
        self.brothels
            .get(brothel_id)
            .map(|b| b.num_girls())
            .unwrap_or(0)
    }

    /// Assign a girl to a brothel. Returns false if no room.
    pub fn assign_girl(&mut self, brothel_id: usize, girl_id: usize) -> bool {
        if let Some(brothel) = self.brothels.get_mut(brothel_id) {
            if brothel.has_room() {
                brothel.add_girl(girl_id);
                return true;
            }
        }
        false
    }

    /// Unassign a girl from a brothel.
    pub fn unassign_girl(&mut self, brothel_id: usize, girl_id: usize) {
        if let Some(brothel) = self.brothels.get_mut(brothel_id) {
            brothel.remove_girl(girl_id);
        }
    }

    /// Find which brothel a girl is assigned to.
    pub fn find_girl_brothel(&self, girl_id: usize) -> Option<usize> {
        self.brothels
            .iter()
            .position(|b| b.girls.contains(&girl_id))
    }

    /// Transfer a girl between brothels. Returns false if target has no room.
    pub fn transfer_girl(
        &mut self,
        girl_id: usize,
        from_brothel: usize,
        to_brothel: usize,
    ) -> bool {
        if from_brothel == to_brothel {
            return true;
        }
        if let Some(target) = self.brothels.get(to_brothel) {
            if !target.has_room() {
                return false;
            }
        } else {
            return false;
        }
        self.brothels[from_brothel].remove_girl(girl_id);
        self.brothels[to_brothel].add_girl(girl_id);
        true
    }

    /// Total girls across all brothels.
    pub fn total_girls(&self) -> usize {
        self.brothels.iter().map(|b| b.num_girls()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brothel_management() {
        let mut mgr = BrothelManager::new();
        assert_eq!(mgr.num_brothels(), 1);

        let id2 = mgr.add_brothel("Second");
        assert_eq!(mgr.num_brothels(), 2);
        assert_eq!(mgr.get_brothel(id2).unwrap().name, "Second");

        assert!(mgr.assign_girl(0, 42));
        assert_eq!(mgr.get_num_girls(0), 1);
        assert_eq!(mgr.find_girl_brothel(42), Some(0));

        assert!(mgr.transfer_girl(42, 0, id2));
        assert_eq!(mgr.get_num_girls(0), 0);
        assert_eq!(mgr.get_num_girls(id2), 1);
    }

    #[test]
    fn test_brothel_room_limit() {
        let mut b = Brothel::default();
        b.num_rooms = 2;
        b.add_girl(0);
        b.add_girl(1);
        assert!(!b.has_room());
    }
}
