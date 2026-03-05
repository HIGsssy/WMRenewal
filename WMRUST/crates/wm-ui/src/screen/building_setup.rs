use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetStore, WidgetId};
use crate::xml_loader::load_screen_xml;

#[derive(Debug)]
pub struct BuildingSetupScreen {
    back_id: WidgetId,
    buy10_id: WidgetId,
    buy20_id: WidgetId,
    bar_hire_id: WidgetId,
    bar_fire_id: WidgetId,
    casino_hire_id: WidgetId,
    casino_fire_id: WidgetId,
    build_rooms_id: WidgetId,
    ad_slider_id: WidgetId,
    ad_value_id: WidgetId,
    potions_text_id: WidgetId,
    current_brothel_id: WidgetId,
    prohibit_anal_id: WidgetId,
    prohibit_bdsm_id: WidgetId,
    prohibit_beast_id: WidgetId,
    prohibit_group_id: WidgetId,
    prohibit_normal_id: WidgetId,
    prohibit_lesbian_id: WidgetId,
}

impl BuildingSetupScreen {
    pub fn new() -> Self {
        Self {
            back_id: 0, buy10_id: 0, buy20_id: 0,
            bar_hire_id: 0, bar_fire_id: 0,
            casino_hire_id: 0, casino_fire_id: 0,
            build_rooms_id: 0, ad_slider_id: 0, ad_value_id: 0,
            potions_text_id: 0, current_brothel_id: 0,
            prohibit_anal_id: 0, prohibit_bdsm_id: 0,
            prohibit_beast_id: 0, prohibit_group_id: 0,
            prohibit_normal_id: 0, prohibit_lesbian_id: 0,
        }
    }

    fn update_display(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.current_brothel_id) {
            t.text = state.brothels.current_brothel_name().to_string();
        }
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.potions_text_id) {
            t.text = format!("You have: {}", state.healing_potions);
        }
        let brothel = state.brothels.current_brothel();
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.ad_value_id) {
            t.text = format!("{} gold / week", brothel.advertising_budget as i64);
        }
        // Sync restriction checkboxes
        if let Some(Widget::CheckBox(cb)) = widgets.get_mut(self.prohibit_anal_id) { cb.checked = brothel.restrict_anal; }
        if let Some(Widget::CheckBox(cb)) = widgets.get_mut(self.prohibit_bdsm_id) { cb.checked = brothel.restrict_bdsm; }
        if let Some(Widget::CheckBox(cb)) = widgets.get_mut(self.prohibit_beast_id) { cb.checked = brothel.restrict_beast; }
        if let Some(Widget::CheckBox(cb)) = widgets.get_mut(self.prohibit_group_id) { cb.checked = brothel.restrict_group; }
        if let Some(Widget::CheckBox(cb)) = widgets.get_mut(self.prohibit_normal_id) { cb.checked = brothel.restrict_normal; }
        if let Some(Widget::CheckBox(cb)) = widgets.get_mut(self.prohibit_lesbian_id) { cb.checked = brothel.restrict_lesbian; }
    }
}

impl Screen for BuildingSetupScreen {
    fn id(&self) -> ScreenId { "building_setup" }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/building_setup_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.back_id = widgets.get_id("BackButton").unwrap_or(0);
        self.buy10_id = widgets.get_id("10PotionsButton").unwrap_or(0);
        self.buy20_id = widgets.get_id("20PotionsButton").unwrap_or(0);
        self.bar_hire_id = widgets.get_id("BarHireButton").unwrap_or(0);
        self.bar_fire_id = widgets.get_id("BarFireButton").unwrap_or(0);
        self.casino_hire_id = widgets.get_id("CasinoHireButton").unwrap_or(0);
        self.casino_fire_id = widgets.get_id("CasinoFireButton").unwrap_or(0);
        self.build_rooms_id = widgets.get_id("BuildRoomsButton").unwrap_or(0);
        self.ad_slider_id = widgets.get_id("AdvertisingSlider").unwrap_or(0);
        self.ad_value_id = widgets.get_id("AdvertisingValue").unwrap_or(0);
        self.potions_text_id = widgets.get_id("AvailablePotions").unwrap_or(0);
        self.current_brothel_id = widgets.get_id("CurrentBrothel").unwrap_or(0);
        self.prohibit_anal_id = widgets.get_id("ProhibitAnalToggle").unwrap_or(0);
        self.prohibit_bdsm_id = widgets.get_id("ProhibitBDSMToggle").unwrap_or(0);
        self.prohibit_beast_id = widgets.get_id("ProhibitBeastToggle").unwrap_or(0);
        self.prohibit_group_id = widgets.get_id("ProhibitGroupToggle").unwrap_or(0);
        self.prohibit_normal_id = widgets.get_id("ProhibitNormalToggle").unwrap_or(0);
        self.prohibit_lesbian_id = widgets.get_id("ProhibitLesbianToggle").unwrap_or(0);

        // Set slider to current advertising budget
        let budget = state.brothels.current_brothel().advertising_budget as i32;
        if let Some(Widget::Slider(sl)) = widgets.get_mut(self.ad_slider_id) {
            sl.value = budget;
        }
        self.update_display(widgets, state);
    }

    fn process(&mut self, widgets: &mut WidgetStore, state: &mut GameState) -> ScreenAction {
        // Live-update advertising from slider
        if let Some(Widget::Slider(sl)) = widgets.get(self.ad_slider_id) {
            let val = sl.value as f64;
            if (state.brothels.current_brothel().advertising_budget - val).abs() > 0.5 {
                state.brothels.current_brothel_mut().advertising_budget = val;
                self.update_display(widgets, state);
            }
        }
        ScreenAction::None
    }

    fn on_event(&mut self, event: UiEvent, widgets: &mut WidgetStore, state: &mut GameState) -> ScreenAction {
        if let UiEvent::MouseClick { x, y } = event {
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) { return ScreenAction::Pop; }
            }
            // Buy potions
            if let Some(Widget::Button(b)) = widgets.get(self.buy10_id) {
                if b.base.is_over(x, y) {
                    let cost = 20.0;
                    if state.gold.cash_on_hand >= cost {
                        state.gold.cash_on_hand -= cost;
                        state.healing_potions += 10;
                        self.update_display(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.buy20_id) {
                if b.base.is_over(x, y) {
                    let cost = 40.0;
                    if state.gold.cash_on_hand >= cost {
                        state.gold.cash_on_hand -= cost;
                        state.healing_potions += 20;
                        self.update_display(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
            // Build rooms (5000 gold)
            if let Some(Widget::Button(b)) = widgets.get(self.build_rooms_id) {
                if b.base.is_over(x, y) {
                    let cost = 5000.0;
                    if state.gold.cash_on_hand >= cost {
                        state.gold.cash_on_hand -= cost;
                        state.brothels.current_brothel_mut().num_rooms += 20;
                        self.update_display(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
            // Bar hire/fire
            if let Some(Widget::Button(b)) = widgets.get(self.bar_hire_id) {
                if b.base.is_over(x, y) {
                    state.brothels.current_brothel_mut().bar = true;
                    self.update_display(widgets, state);
                    return ScreenAction::None;
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.bar_fire_id) {
                if b.base.is_over(x, y) {
                    state.brothels.current_brothel_mut().bar = false;
                    self.update_display(widgets, state);
                    return ScreenAction::None;
                }
            }
            // Casino hire/fire
            if let Some(Widget::Button(b)) = widgets.get(self.casino_hire_id) {
                if b.base.is_over(x, y) {
                    state.brothels.current_brothel_mut().gambling_hall = true;
                    self.update_display(widgets, state);
                    return ScreenAction::None;
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.casino_fire_id) {
                if b.base.is_over(x, y) {
                    state.brothels.current_brothel_mut().gambling_hall = false;
                    self.update_display(widgets, state);
                    return ScreenAction::None;
                }
            }
            // Restriction toggles
            let toggle_ids = [
                (self.prohibit_anal_id, "anal"),
                (self.prohibit_bdsm_id, "bdsm"),
                (self.prohibit_beast_id, "beast"),
                (self.prohibit_group_id, "group"),
                (self.prohibit_normal_id, "normal"),
                (self.prohibit_lesbian_id, "lesbian"),
            ];
            for (wid, kind) in toggle_ids {
                if let Some(Widget::CheckBox(cb)) = widgets.get_mut(wid) {
                    if cb.base.is_over(x, y) {
                        cb.checked = !cb.checked;
                        let val = cb.checked;
                        let b = state.brothels.current_brothel_mut();
                        match kind {
                            "anal" => b.restrict_anal = val,
                            "bdsm" => b.restrict_bdsm = val,
                            "beast" => b.restrict_beast = val,
                            "group" => b.restrict_group = val,
                            "normal" => b.restrict_normal = val,
                            "lesbian" => b.restrict_lesbian = val,
                            _ => {}
                        }
                        return ScreenAction::None;
                    }
                }
            }
        }
        ScreenAction::None
    }
}
