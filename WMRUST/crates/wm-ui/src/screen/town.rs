use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::bank::BankScreen;
use crate::screen::house::HouseScreen;
use crate::screen::mayor::MayorScreen;
use crate::screen::prison::PrisonScreen;
use crate::screen::slave_market::SlaveMarketScreen;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetStore, WidgetId};
use crate::xml_loader::load_screen_xml;

#[derive(Debug)]
pub struct TownScreen {
    back_id: WidgetId,
    walk_id: WidgetId,
    slave_market_id: WidgetId,
    mayor_id: WidgetId,
    bank_id: WidgetId,
    house_id: WidgetId,
    prison_id: WidgetId,
    brothel_ids: [WidgetId; 6],
    current_brothel_id: WidgetId,
}

impl TownScreen {
    pub fn new() -> Self {
        Self {
            back_id: 0,
            walk_id: 0,
            slave_market_id: 0,
            mayor_id: 0,
            bank_id: 0,
            house_id: 0,
            prison_id: 0,
            brothel_ids: [0; 6],
            current_brothel_id: 0,
        }
    }
}

impl Screen for TownScreen {
    fn id(&self) -> ScreenId { "town" }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/town_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.back_id = widgets.get_id("BackButton").unwrap_or(0);
        self.walk_id = widgets.get_id("WalkButton").unwrap_or(0);
        self.slave_market_id = widgets.get_id("SlaveMarket").unwrap_or(0);
        self.mayor_id = widgets.get_id("MayorsOffice").unwrap_or(0);
        self.bank_id = widgets.get_id("Bank").unwrap_or(0);
        self.house_id = widgets.get_id("House").unwrap_or(0);
        self.prison_id = widgets.get_id("Prison").unwrap_or(0);
        for i in 0..6 {
            self.brothel_ids[i] = widgets.get_id(&format!("Brothel{}", i)).unwrap_or(0);
        }
        self.current_brothel_id = widgets.get_id("CurrentBrothel").unwrap_or(0);

        // Display current brothel name
        let name = state.brothels.current_brothel_name();
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.current_brothel_id) {
            t.text = format!("Current: {}", name);
        }
    }

    fn process(&mut self, _widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction {
        ScreenAction::None
    }

    fn on_event(&mut self, event: UiEvent, widgets: &mut WidgetStore, state: &mut GameState) -> ScreenAction {
        if let UiEvent::MouseClick { x, y } = event {
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) { return ScreenAction::Pop; }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.slave_market_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(SlaveMarketScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.bank_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(BankScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.mayor_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(MayorScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.house_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(HouseScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.prison_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(PrisonScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.walk_id) {
                if b.base.is_over(x, y) {
                    state.walk_around = true;
                    return ScreenAction::None;
                }
            }
            // Brothel selection
            for i in 0..6 {
                if let Some(Widget::Button(b)) = widgets.get(self.brothel_ids[i]) {
                    if b.base.is_over(x, y) {
                        state.brothels.set_current(i);
                        return ScreenAction::Pop;
                    }
                }
            }
        }
        ScreenAction::None
    }
}
