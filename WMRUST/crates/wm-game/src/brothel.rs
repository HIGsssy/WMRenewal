use wm_core::girl::Girl;
use wm_core::room::Room;

/// A single brothel with its girls, rooms, and settings.
#[derive(Debug, Clone)]
pub struct Brothel {
    pub name: String,
    pub num_rooms: i32,
    pub max_num_rooms: i32,
    pub rooms: Vec<Room>,
    pub girls: Vec<usize>,
    pub bar: bool,
    pub gambling_hall: bool,
    pub has_matron: bool,
    pub advertised: f64,
    pub filthiness: i32,
    pub security_level: i32,
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
            filthiness: 0,
            security_level: 0,
        }
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

    pub fn add_brothel(&mut self, _name: &str) -> usize {
        todo!()
    }

    pub fn get_num_girls(&self, _brothel_id: usize) -> usize {
        todo!()
    }

    pub fn get_girls_on_job(
        &self,
        _brothel_id: usize,
        _job: wm_core::enums::JobType,
        _shift: wm_core::enums::Shift,
    ) -> Vec<&Girl> {
        todo!()
    }
}
