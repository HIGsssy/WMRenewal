use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::WidgetStore;

#[derive(Debug)]
pub struct GalleryScreen;

impl Screen for GalleryScreen {
    fn id(&self) -> ScreenId { "gallery" }
    fn init(&mut self, _widgets: &mut WidgetStore, _state: &mut GameState) {}
    fn process(&mut self, _widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction { ScreenAction::None }
    fn on_event(&mut self, _event: UiEvent, _widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction { ScreenAction::None }
}
