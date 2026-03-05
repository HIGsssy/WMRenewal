use wm_core::girl::Girl;

/// Manages the collection of all girls in the game.
#[derive(Debug)]
pub struct GirlManager {
    pub girls: Vec<Girl>,
}

impl Default for GirlManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GirlManager {
    pub fn new() -> Self {
        Self { girls: Vec::new() }
    }

    pub fn add_girl(&mut self, girl: Girl) -> usize {
        let id = self.girls.len();
        self.girls.push(girl);
        id
    }

    pub fn get_girl(&self, _id: usize) -> Option<&Girl> {
        todo!()
    }

    pub fn get_girl_mut(&mut self, _id: usize) -> Option<&mut Girl> {
        todo!()
    }

    pub fn remove_girl(&mut self, _id: usize) -> Option<Girl> {
        todo!()
    }

    pub fn load_girls_from_xml(&mut self, _path: &std::path::Path) {
        todo!()
    }

    pub fn stat_check(
        &self,
        _girl: &Girl,
        _stat: wm_core::enums::Stat,
        _threshold: i32,
    ) -> bool {
        todo!()
    }

    pub fn update_stat(
        &self,
        _girl: &mut Girl,
        _stat: wm_core::enums::Stat,
        _amount: i32,
    ) {
        todo!()
    }

    pub fn update_skill(
        &self,
        _girl: &mut Girl,
        _skill: wm_core::enums::Skill,
        _amount: i32,
    ) {
        todo!()
    }

    pub fn has_trait(&self, _girl: &Girl, _trait_name: &str) -> bool {
        todo!()
    }

    pub fn add_trait(&self, _girl: &mut Girl, _trait_name: &str) {
        todo!()
    }

    pub fn remove_trait(&self, _girl: &mut Girl, _trait_name: &str) {
        todo!()
    }
}
