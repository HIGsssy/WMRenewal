use wm_core::enums::Stat;
use wm_game::girls::GirlManager;
use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetStore, WidgetId};
use crate::xml_loader::load_screen_xml;

#[derive(Debug)]
pub struct PrisonScreen {
    prison_list_id: WidgetId,
    desc_id: WidgetId,
    release_id: WidgetId,
    show_more_id: WidgetId,
    back_id: WidgetId,
}

impl PrisonScreen {
    pub fn new() -> Self {
        Self {
            prison_list_id: 0, desc_id: 0, release_id: 0,
            show_more_id: 0, back_id: 0,
        }
    }

    fn populate_list(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.prison_list_id) {
            lb.clear();
            // Prison shows dungeon inmates that are customers or captured
            for (i, inmate) in state.dungeon.inmates.iter().enumerate() {
                lb.add_element(i as i32, &inmate.girl.name);
            }
        }
    }
}

impl Screen for PrisonScreen {
    fn id(&self) -> ScreenId { "prison" }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/prison_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.prison_list_id = widgets.get_id("PrisonList").unwrap_or(0);
        self.desc_id = widgets.get_id("GirlDescription").unwrap_or(0);
        self.release_id = widgets.get_id("ReleaseButton").unwrap_or(0);
        self.show_more_id = widgets.get_id("ShowMoreButton").unwrap_or(0);
        self.back_id = widgets.get_id("BackButton").unwrap_or(0);

        self.populate_list(widgets, state);
    }

    fn process(&mut self, _widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction {
        ScreenAction::None
    }

    fn on_event(&mut self, event: UiEvent, widgets: &mut WidgetStore, state: &mut GameState) -> ScreenAction {
        if let UiEvent::MouseClick { x, y } = event {
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) { return ScreenAction::Pop; }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.release_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.prison_list_id) {
                        if let Some(sel) = lb.get_selected() {
                            let _ = state.dungeon.release(sel as usize);
                            self.populate_list(widgets, state);
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Prison list click
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.prison_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(sel) = lb.get_selected() {
                        if let Some(inmate) = state.dungeon.inmates.get(sel as usize) {
                            let health = GirlManager::get_stat(&inmate.girl, Stat::Health);
                            let desc = format!(
                                "Name: {}\nHealth: {}\nWeeks: {}\nReason: {:?}\nFed: {}",
                                inmate.girl.name, health, inmate.weeks,
                                inmate.reason, if inmate.fed { "Yes" } else { "No" },
                            );
                            if let Some(Widget::TextItem(t)) = widgets.get_mut(self.desc_id) {
                                t.text = desc;
                            }
                        }
                    }
                    return ScreenAction::None;
                }
            }
        }
        ScreenAction::None
    }
}
