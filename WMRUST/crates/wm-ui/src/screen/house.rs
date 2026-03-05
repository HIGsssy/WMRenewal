use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetId, WidgetStore};
use crate::xml_loader::load_screen_xml;

#[derive(Debug)]
pub struct HouseScreen {
    back_id: WidgetId,
    details_id: WidgetId,
}

impl HouseScreen {
    pub fn new() -> Self {
        Self {
            back_id: 0,
            details_id: 0,
        }
    }
}

impl Screen for HouseScreen {
    fn id(&self) -> ScreenId {
        "house"
    }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/house_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.back_id = widgets.get_id("BackButton").unwrap_or(0);
        self.details_id = widgets.get_id("HouseDetails").unwrap_or(0);

        let details = format!(
            "Player: {}\nDisposition: {}\nSuspicion: {}\n\nGold on hand: {}\nBank balance: {}\n\nTotal girls: {}\nBrothels: {}\nWeek: {}",
            state.player.name,
            state.player.disposition,
            state.player.suspicion,
            state.gold.cash_on_hand as i64,
            state.gold.bank_balance as i64,
            state.brothels.total_girls(),
            state.brothels.num_brothels(),
            state.week,
        );
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.details_id) {
            t.text = details;
        }
    }

    fn process(&mut self, _widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction {
        ScreenAction::None
    }

    fn on_event(
        &mut self,
        event: UiEvent,
        widgets: &mut WidgetStore,
        _state: &mut GameState,
    ) -> ScreenAction {
        if let UiEvent::MouseClick { x, y } = event {
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Pop;
                }
            }
        }
        ScreenAction::None
    }
}
