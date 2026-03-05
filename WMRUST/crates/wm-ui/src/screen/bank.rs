use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetId, WidgetStore};
use crate::xml_loader::load_screen_xml;

#[derive(Debug)]
pub struct BankScreen {
    back_id: WidgetId,
    deposit_id: WidgetId,
    deposit_all_id: WidgetId,
    withdraw_id: WidgetId,
    details_id: WidgetId,
}

impl BankScreen {
    pub fn new() -> Self {
        Self {
            back_id: 0,
            deposit_id: 0,
            deposit_all_id: 0,
            withdraw_id: 0,
            details_id: 0,
        }
    }

    fn update_details(&self, widgets: &mut WidgetStore, state: &GameState) {
        let bank = state.gold.bank_balance;
        let cash = state.gold.cash_on_hand as i64;
        let text = format!("Bank account: {} gold\nOn hand: {} gold", bank as i64, cash);
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.details_id) {
            t.text = text;
        }

        // Enable/disable buttons based on funds
        if let Some(Widget::Button(b)) = widgets.get_mut(self.deposit_id) {
            b.base.disabled = cash <= 0;
        }
        if let Some(Widget::Button(b)) = widgets.get_mut(self.deposit_all_id) {
            b.base.disabled = cash <= 0;
        }
        if let Some(Widget::Button(b)) = widgets.get_mut(self.withdraw_id) {
            b.base.disabled = bank <= 0.0;
        }
    }
}

impl Screen for BankScreen {
    fn id(&self) -> ScreenId {
        "bank"
    }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/bank_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.back_id = widgets.get_id("BackButton").unwrap_or(0);
        self.deposit_id = widgets.get_id("DepositButton").unwrap_or(0);
        self.deposit_all_id = widgets.get_id("DepositAllButton").unwrap_or(0);
        self.withdraw_id = widgets.get_id("WithdrawButton").unwrap_or(0);
        self.details_id = widgets.get_id("BankDetails").unwrap_or(0);

        self.update_details(widgets, state);
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
            if let Some(Widget::Button(b)) = widgets.get(self.deposit_all_id) {
                if b.base.is_over(x, y) && !b.base.disabled {
                    let amount = state.gold.cash_on_hand;
                    let _ = state.gold.deposit(amount);
                    self.update_details(widgets, state);
                    return ScreenAction::None;
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.deposit_id) {
                if b.base.is_over(x, y) && !b.base.disabled {
                    let amount = 100.0_f64.min(state.gold.cash_on_hand);
                    let _ = state.gold.deposit(amount);
                    self.update_details(widgets, state);
                    return ScreenAction::None;
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.withdraw_id) {
                if b.base.is_over(x, y) && !b.base.disabled {
                    let amount = 100.0_f64.min(state.gold.bank_balance);
                    let _ = state.gold.withdraw(amount);
                    self.update_details(widgets, state);
                    return ScreenAction::None;
                }
            }
        }
        ScreenAction::None
    }
}
