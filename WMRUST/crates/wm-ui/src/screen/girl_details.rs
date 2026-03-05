use wm_core::enums::{Skill, Stat};
use wm_game::girls::GirlManager;
use wm_game::state::GameState;
use wm_script::script_runner::ScriptRunner;

use crate::events::UiEvent;
use crate::screen::dialog::DialogScreen;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetId, WidgetStore};
use crate::xml_loader::load_screen_xml;

#[derive(Debug)]
pub struct GirlDetailsScreen {
    girl_id: usize,
    stat_list_id: WidgetId,
    skill_list_id: WidgetId,
    trait_list_id: WidgetId,
    job_type_list_id: WidgetId,
    job_list_id: WidgetId,
    girl_desc_id: WidgetId,
    girl_name_id: WidgetId,
    house_slider_id: WidgetId,
    back_id: WidgetId,
    prev_id: WidgetId,
    next_id: WidgetId,
    gallery_id: WidgetId,
    interact_id: WidgetId,
    send_dungeon_id: WidgetId,
    inventory_id: WidgetId,
}

impl GirlDetailsScreen {
    pub fn with_girl(girl_id: usize) -> Self {
        Self {
            girl_id,
            stat_list_id: 0,
            skill_list_id: 0,
            trait_list_id: 0,
            job_type_list_id: 0,
            job_list_id: 0,
            girl_desc_id: 0,
            girl_name_id: 0,
            house_slider_id: 0,
            back_id: 0,
            prev_id: 0,
            next_id: 0,
            gallery_id: 0,
            interact_id: 0,
            send_dungeon_id: 0,
            inventory_id: 0,
        }
    }

    fn populate_details(&self, widgets: &mut WidgetStore, state: &GameState) {
        let girl = match state.girls.get_girl(self.girl_id) {
            Some(g) => g,
            None => return,
        };

        // Name text
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.girl_name_id) {
            t.text = girl.name.clone();
        }

        // Stats list
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.stat_list_id) {
            lb.clear();
            for (i, &stat) in Stat::ALL.iter().enumerate() {
                let val = GirlManager::get_stat(girl, stat);
                lb.add_element(i as i32, &format!("{:?}|{}", stat, val));
            }
        }

        // Skills list
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.skill_list_id) {
            lb.clear();
            for (i, &skill) in Skill::ALL.iter().enumerate() {
                let val = GirlManager::get_skill(girl, skill);
                lb.add_element(i as i32, &format!("{:?}|{}", skill, val));
            }
        }

        // Trait list
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.trait_list_id) {
            lb.clear();
            for (i, t) in girl.traits.iter().enumerate() {
                lb.add_element(i as i32, t);
            }
        }

        // Description
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.girl_desc_id) {
            let job_day = girl
                .job_day
                .map(|j| format!("{:?}", j))
                .unwrap_or_else(|| "None".into());
            let job_night = girl
                .job_night
                .map(|j| format!("{:?}", j))
                .unwrap_or_else(|| "None".into());
            t.text = format!(
                "{}\n\n{}\n\nDay Job: {}\nNight Job: {}\nVirgin: {}\nPregnant: {}",
                girl.name,
                girl.desc,
                job_day,
                job_night,
                if girl.virgin { "Yes" } else { "No" },
                if girl.weeks_pregnant > 0 {
                    format!("{} weeks", girl.weeks_pregnant)
                } else {
                    "No".into()
                },
            );
        }

        // House % slider
        if let Some(Widget::Slider(s)) = widgets.get_mut(self.house_slider_id) {
            s.value = GirlManager::get_stat(girl, Stat::HousePerc);
        }
    }

    fn make_interact_dialog(&self, state: &GameState) -> Option<DialogScreen> {
        let girl = state.girls.get_girl(self.girl_id)?;
        let scripts = wm_core::resources_path().join("Scripts");
        let code = load_script_code(&scripts, "DefaultInteractDetails")?;
        let runner = ScriptRunner::new(&code).ok()?;
        {
            let mut ctx = runner.context().lock().unwrap();
            ctx.populate_from_girl(girl);
        }
        Some(DialogScreen::from_runner(runner).with_target_girl(self.girl_id))
    }
}

/// Load a script by name — tries .lua first, then converts .script.
fn load_script_code(scripts_dir: &std::path::Path, name: &str) -> Option<String> {
    let lua_path = scripts_dir.join(format!("{}.lua", name));
    if lua_path.exists() {
        return std::fs::read_to_string(&lua_path).ok();
    }
    let script_path = scripts_dir.join(format!("{}.script", name));
    if script_path.exists() {
        return wm_script::script_converter::convert_script_to_lua(&script_path).ok();
    }
    None
}

impl Screen for GirlDetailsScreen {
    fn id(&self) -> ScreenId {
        "girl_details"
    }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/girl_details_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.stat_list_id = widgets.get_id("StatList").unwrap_or(0);
        self.skill_list_id = widgets.get_id("SkillList").unwrap_or(0);
        self.trait_list_id = widgets.get_id("TraitList").unwrap_or(0);
        self.job_type_list_id = widgets.get_id("JobTypeList").unwrap_or(0);
        self.job_list_id = widgets.get_id("JobList").unwrap_or(0);
        self.girl_desc_id = widgets.get_id("GirlDescription").unwrap_or(0);
        self.girl_name_id = widgets.get_id("GirlName").unwrap_or(0);
        self.house_slider_id = widgets.get_id("HousePercSlider").unwrap_or(0);
        self.back_id = widgets.get_id("BackButton").unwrap_or(0);
        self.prev_id = widgets.get_id("PrevButton").unwrap_or(0);
        self.next_id = widgets.get_id("NextButton").unwrap_or(0);
        self.gallery_id = widgets.get_id("GalleryButton").unwrap_or(0);
        self.interact_id = widgets.get_id("InteractButton").unwrap_or(0);
        self.send_dungeon_id = widgets.get_id("SendDungeonButton").unwrap_or(0);
        self.inventory_id = widgets.get_id("InventoryButton").unwrap_or(0);

        self.populate_details(widgets, state);
    }

    fn process(&mut self, _widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction {
        ScreenAction::None
    }

    fn on_event(
        &mut self,
        event: UiEvent,
        widgets: &mut WidgetStore,
        state: &mut GameState,
    ) -> ScreenAction {
        if let UiEvent::MouseClick { x, y } = event {
            // Back
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Pop;
                }
            }
            // Prev girl
            if let Some(Widget::Button(b)) = widgets.get(self.prev_id) {
                if b.base.is_over(x, y) {
                    let brothel = state.brothels.current_brothel();
                    if let Some(pos) = brothel.girls.iter().position(|&g| g == self.girl_id) {
                        if pos > 0 {
                            self.girl_id = brothel.girls[pos - 1];
                            self.populate_details(widgets, state);
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Next girl
            if let Some(Widget::Button(b)) = widgets.get(self.next_id) {
                if b.base.is_over(x, y) {
                    let brothel = state.brothels.current_brothel();
                    if let Some(pos) = brothel.girls.iter().position(|&g| g == self.girl_id) {
                        if pos + 1 < brothel.girls.len() {
                            self.girl_id = brothel.girls[pos + 1];
                            self.populate_details(widgets, state);
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Send to dungeon
            if let Some(Widget::Button(b)) = widgets.get(self.send_dungeon_id) {
                if b.base.is_over(x, y) {
                    let cur = state.brothels.current_index();
                    state.brothels.unassign_girl(cur, self.girl_id);
                    if let Some(girl) = state.girls.remove_girl(self.girl_id) {
                        state
                            .dungeon
                            .add_girl(girl, wm_core::enums::DungeonReason::GirlWhim);
                    }
                    return ScreenAction::Pop;
                }
            }
            // House % slider interaction
            if let Some(Widget::Slider(s)) = widgets.get_mut(self.house_slider_id) {
                if s.base.is_over(x, y) {
                    s.handle_click(x, y);
                    let new_val = s.value;
                    if let Some(girl) = state.girls.get_girl_mut(self.girl_id) {
                        girl.stats[Stat::HousePerc as usize] = new_val;
                    }
                    return ScreenAction::None;
                }
            }
            // List clicks (stat, skill, trait)
            for list_id in [self.stat_list_id, self.skill_list_id, self.trait_list_id] {
                if let Some(Widget::ListBox(lb)) = widgets.get_mut(list_id) {
                    if lb.base.is_over(x, y) {
                        lb.handle_click(x, y);
                        return ScreenAction::None;
                    }
                }
            }
            // Interact button — run DefaultInteractDetails script
            if let Some(Widget::Button(b)) = widgets.get(self.interact_id) {
                if b.base.is_over(x, y) {
                    if let Some(screen) = self.make_interact_dialog(state) {
                        return ScreenAction::Push(Box::new(screen));
                    }
                }
            }
        }
        ScreenAction::None
    }
}
