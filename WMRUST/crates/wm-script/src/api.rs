//! Stub functions for wm.* Lua API calls that the game scripts use.
//! These will be registered into the Lua environment.

pub fn wm_get_stat(_girl_name: &str, _stat_name: &str) -> i32 {
    todo!()
}

pub fn wm_set_stat(_girl_name: &str, _stat_name: &str, _value: i32) {
    todo!()
}

pub fn wm_get_skill(_girl_name: &str, _skill_name: &str) -> i32 {
    todo!()
}

pub fn wm_set_skill(_girl_name: &str, _skill_name: &str, _value: i32) {
    todo!()
}

pub fn wm_has_trait(_girl_name: &str, _trait_name: &str) -> bool {
    todo!()
}

pub fn wm_add_trait(_girl_name: &str, _trait_name: &str) {
    todo!()
}

pub fn wm_remove_trait(_girl_name: &str, _trait_name: &str) {
    todo!()
}

pub fn wm_message(_text: &str) {
    todo!()
}

pub fn wm_choice(_prompt: &str, _options: &[&str]) -> usize {
    todo!()
}

pub fn wm_get_gold() -> i32 {
    todo!()
}

pub fn wm_add_gold(_amount: i32) {
    todo!()
}

pub fn wm_set_global_flag(_flag: usize, _value: bool) {
    todo!()
}

pub fn wm_get_global_flag(_flag: usize) -> bool {
    todo!()
}

pub fn wm_set_girl_flag(_girl_name: &str, _flag: usize, _value: bool) {
    todo!()
}

pub fn wm_get_girl_flag(_girl_name: &str, _flag: usize) -> bool {
    todo!()
}
