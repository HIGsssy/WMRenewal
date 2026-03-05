use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::WidgetStore;

#[derive(Debug)]
pub struct TownScreen;

impl Screen for TownScreen {
    fn id(&self) -> ScreenId { "town" }
    fn init(&mut self, _widgets: &mut WidgetStore) { todo!() }
    fn process(&mut self, _widgets: &mut WidgetStore) -> ScreenAction { ScreenAction::None }
    fn on_event(&mut self, _event: UiEvent, _widgets: &mut WidgetStore) -> ScreenAction { todo!() }
}
