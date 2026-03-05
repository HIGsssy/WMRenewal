use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetStore, WidgetId};
use crate::xml_loader::load_screen_xml;

#[derive(Debug)]
pub struct MayorScreen {
    details_id: WidgetId,
    bribe_id: WidgetId,
    back_id: WidgetId,
}

impl MayorScreen {
    pub fn new() -> Self {
        Self { details_id: 0, bribe_id: 0, back_id: 0 }
    }

    fn update_details(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.details_id) {
            let disp = state.player.disposition;
            let moral = if disp > 30 { "Good" } else if disp < -30 { "Evil" } else { "Neutral" };
            t.text = format!(
                "Mayor's Office\n\nYour Disposition: {} ({})\nSuspicion: {}\nInfluence: {}\n\nBribe Cost: {} gold",
                disp, moral, state.player.suspicion, state.player.influence,
                bribe_cost(state.player.suspicion),
            );
        }
    }
}

fn bribe_cost(suspicion: i32) -> i64 {
    ((suspicion.max(1) as i64) * 50).max(100)
}

impl Screen for MayorScreen {
    fn id(&self) -> ScreenId { "mayor" }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/mayor_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.details_id = widgets.get_id("MayorDetails").unwrap_or(0);
        self.bribe_id = widgets.get_id("BribeButton").unwrap_or(0);
        self.back_id = widgets.get_id("BackButton").unwrap_or(0);

        self.update_details(widgets, state);
    }

    fn process(&mut self, _widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction {
        ScreenAction::None
    }

    fn on_event(&mut self, event: UiEvent, widgets: &mut WidgetStore, state: &mut GameState) -> ScreenAction {
        if let UiEvent::MouseClick { x, y } = event {
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) { return ScreenAction::Pop; }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.bribe_id) {
                if b.base.is_over(x, y) {
                    let cost = bribe_cost(state.player.suspicion) as f64;
                    if state.gold.cash_on_hand >= cost {
                        state.gold.cash_on_hand -= cost;
                        state.player.adjust_suspicion(-10);
                        self.update_details(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
        }
        ScreenAction::None
    }
}
