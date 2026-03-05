use wm_core::enums::GangMission;
use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetId, WidgetStore};
use crate::xml_loader::load_screen_xml;

#[derive(Debug)]
pub struct GangManagementScreen {
    gang_list_id: WidgetId,
    recruit_list_id: WidgetId,
    mission_list_id: WidgetId,
    mission_desc_id: WidgetId,
    hire_id: WidgetId,
    fire_id: WidgetId,
    weapon_up_id: WidgetId,
    weapon_desc_id: WidgetId,
    heal_pot_desc_id: WidgetId,
    buy_heal_id: WidgetId,
    net_desc_id: WidgetId,
    buy_nets_id: WidgetId,
    total_cost_id: WidgetId,
    back_id: WidgetId,
}

impl GangManagementScreen {
    pub fn new() -> Self {
        Self {
            gang_list_id: 0,
            recruit_list_id: 0,
            mission_list_id: 0,
            mission_desc_id: 0,
            hire_id: 0,
            fire_id: 0,
            weapon_up_id: 0,
            weapon_desc_id: 0,
            heal_pot_desc_id: 0,
            buy_heal_id: 0,
            net_desc_id: 0,
            buy_nets_id: 0,
            total_cost_id: 0,
            back_id: 0,
        }
    }

    fn populate_gangs(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.gang_list_id) {
            lb.clear();
            for (i, gang) in state.gangs.hired_gangs.iter().enumerate() {
                let data = format!(
                    "{}|{:?}|{}|{}|{}|{}|{}|{}|{}",
                    gang.name,
                    gang.mission,
                    gang.num_members,
                    gang.combat_skill,
                    gang.magic_skill,
                    gang.intelligence,
                    gang.agility,
                    gang.constitution,
                    gang.charisma,
                );
                lb.add_element(i as i32, &data);
            }
        }
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.recruit_list_id) {
            lb.clear();
            for (i, gang) in state.gangs.recruit_list.iter().enumerate() {
                let data = format!(
                    "{}|{}|{}|{}|{}|{}|{}|{}",
                    gang.name,
                    gang.num_members,
                    gang.combat_skill,
                    gang.magic_skill,
                    gang.intelligence,
                    gang.agility,
                    gang.constitution,
                    gang.charisma,
                );
                lb.add_element(i as i32, &data);
            }
        }
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.total_cost_id) {
            let cost = state.gangs.total_goon_wages() as i64;
            t.text = format!("Weekly Cost: {}", cost);
        }
        self.update_equipment(widgets, state);
    }

    fn populate_missions(&self, widgets: &mut WidgetStore) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.mission_list_id) {
            lb.clear();
            let missions = [
                (0, "Guarding"),
                (1, "Sabotage"),
                (2, "Spy on Girls"),
                (3, "Capture Girl"),
                (4, "Extortion"),
                (5, "Petty Theft"),
                (6, "Grand Theft"),
                (7, "Kidnap"),
                (8, "Catacombs"),
                (9, "Training"),
                (10, "Recruit"),
            ];
            for (id, name) in missions {
                lb.add_element(id, name);
            }
        }
    }

    fn update_equipment(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.weapon_desc_id) {
            t.text = format!("Weapon Level: {}", state.gangs.weapon_level);
        }
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.heal_pot_desc_id) {
            t.text = format!("Heal Potions (10g each): {}", state.gangs.healing_potions);
        }
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.net_desc_id) {
            t.text = format!("Nets (5g each): {}", state.gangs.nets);
        }
    }
}

impl Screen for GangManagementScreen {
    fn id(&self) -> ScreenId {
        "gang_management"
    }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/gangs_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.gang_list_id = widgets.get_id("GangList").unwrap_or(0);
        self.recruit_list_id = widgets.get_id("RecruitList").unwrap_or(0);
        self.mission_list_id = widgets.get_id("MissionList").unwrap_or(0);
        self.mission_desc_id = widgets.get_id("MissionDescription").unwrap_or(0);
        self.hire_id = widgets.get_id("GangHireButton").unwrap_or(0);
        self.fire_id = widgets.get_id("GangFireButton").unwrap_or(0);
        self.weapon_up_id = widgets.get_id("WeaponUpButton").unwrap_or(0);
        self.weapon_desc_id = widgets.get_id("WeaponDescription").unwrap_or(0);
        self.heal_pot_desc_id = widgets.get_id("HealPotDescription").unwrap_or(0);
        self.buy_heal_id = widgets.get_id("BuyHealPotButton").unwrap_or(0);
        self.net_desc_id = widgets.get_id("NetDescription").unwrap_or(0);
        self.buy_nets_id = widgets.get_id("BuyNetsButton").unwrap_or(0);
        self.total_cost_id = widgets.get_id("TotalCost").unwrap_or(0);
        self.back_id = widgets.get_id("BackButton").unwrap_or(0);

        self.populate_gangs(widgets, state);
        self.populate_missions(widgets);
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
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Pop;
                }
            }
            // Hire gang
            if let Some(Widget::Button(b)) = widgets.get(self.hire_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.recruit_list_id) {
                        if let Some(sel) = lb.get_selected() {
                            state.gangs.hire_gang(sel as usize);
                            self.populate_gangs(widgets, state);
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Fire gang
            if let Some(Widget::Button(b)) = widgets.get(self.fire_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.gang_list_id) {
                        if let Some(sel) = lb.get_selected() {
                            state.gangs.fire_gang(sel as usize);
                            self.populate_gangs(widgets, state);
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Weapon upgrade
            if let Some(Widget::Button(b)) = widgets.get(self.weapon_up_id) {
                if b.base.is_over(x, y) {
                    let cost = (state.gangs.weapon_level + 1) as f64 * 150.0;
                    if state.gold.cash_on_hand >= cost {
                        state.gold.cash_on_hand -= cost;
                        state.gangs.weapon_level += 1;
                        self.update_equipment(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
            // Buy heal potions
            if let Some(Widget::Button(b)) = widgets.get(self.buy_heal_id) {
                if b.base.is_over(x, y) {
                    let cost = 200.0; // 20 potions * 10g each
                    if state.gold.cash_on_hand >= cost {
                        state.gold.cash_on_hand -= cost;
                        state.gangs.healing_potions += 20;
                        self.update_equipment(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
            // Buy nets
            if let Some(Widget::Button(b)) = widgets.get(self.buy_nets_id) {
                if b.base.is_over(x, y) {
                    let cost = 100.0; // 20 nets * 5g each
                    if state.gold.cash_on_hand >= cost {
                        state.gold.cash_on_hand -= cost;
                        state.gangs.nets += 20;
                        self.update_equipment(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
            // Mission assignment
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.mission_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(mission_id) = lb.get_selected() {
                        // Assign to selected gang
                        if let Some(Widget::ListBox(gl)) = widgets.get(self.gang_list_id) {
                            if let Some(gang_sel) = gl.get_selected() {
                                let mission = match mission_id {
                                    0 => GangMission::Guarding,
                                    1 => GangMission::Sabotage,
                                    2 => GangMission::SpyGirls,
                                    3 => GangMission::CaptureGirl,
                                    4 => GangMission::Extortion,
                                    5 => GangMission::PettyTheft,
                                    6 => GangMission::GrandTheft,
                                    7 => GangMission::Kidnap,
                                    8 => GangMission::Catacombs,
                                    9 => GangMission::Training,
                                    10 => GangMission::Recruit,
                                    _ => GangMission::Guarding,
                                };
                                state.gangs.set_mission(gang_sel as usize, mission);
                                self.populate_gangs(widgets, state);
                            }
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Gang list click
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.gang_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    return ScreenAction::None;
                }
            }
            // Recruit list click
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.recruit_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    return ScreenAction::None;
                }
            }
        }
        ScreenAction::None
    }
}
