use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::brothel_management::BrothelManagementScreen;
use crate::screen::load_game::LoadGameScreen;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::button::ButtonWidget;
use crate::widget::{Widget, WidgetBase, WidgetStore, WidgetId};

#[derive(Debug)]
pub struct MainMenuScreen {
    new_game_id: WidgetId,
    load_game_id: WidgetId,
    quit_id: WidgetId,
}

impl MainMenuScreen {
    pub fn new() -> Self {
        Self {
            new_game_id: 0,
            load_game_id: 0,
            quit_id: 0,
        }
    }
}

impl Default for MainMenuScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl Screen for MainMenuScreen {
    fn id(&self) -> ScreenId { "main_menu" }

    fn init(&mut self, widgets: &mut WidgetStore, _state: &mut GameState) {
        // Layout from MainMenu.txt: Window at 254,144 288x320
        // Buttons: NewGame 16,32 256x64; LoadGame 16,128 256x64; Exit 16,224 256x64
        let ox = 254;
        let oy = 144;

        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, "NewGameButton", ox + 16, oy + 32, 256, 64);
        self.new_game_id = widgets.add("NewGameButton", Widget::Button(ButtonWidget {
            base,
            image_off: "NewGameOff.png".into(),
            image_on: "NewGameOn.png".into(),
            image_disabled: "NewGameDisabled.png".into(),
            transparency: true,
            scale: true,
            pressed: false,
        }));

        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, "LoadGameButton", ox + 16, oy + 128, 256, 64);
        self.load_game_id = widgets.add("LoadGameButton", Widget::Button(ButtonWidget {
            base,
            image_off: "LoadGameOff.png".into(),
            image_on: "LoadGameOn.png".into(),
            image_disabled: "LoadGameDisabled.png".into(),
            transparency: true,
            scale: true,
            pressed: false,
        }));

        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, "ExitGameButton", ox + 16, oy + 224, 256, 64);
        self.quit_id = widgets.add("ExitGameButton", Widget::Button(ButtonWidget {
            base,
            image_off: "ExitGameOff.png".into(),
            image_on: "ExitGameOn.png".into(),
            image_disabled: "ExitGameDisabled.png".into(),
            transparency: true,
            scale: true,
            pressed: false,
        }));
    }

    fn process(&mut self, _widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction {
        ScreenAction::None
    }

    fn on_event(&mut self, event: UiEvent, widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction {
        if let UiEvent::MouseClick { x, y } = event {
            if let Some(Widget::Button(b)) = widgets.get(self.new_game_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(BrothelManagementScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.load_game_id) {
                if b.base.is_over(x, y) {
                      return ScreenAction::Push(Box::new(LoadGameScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.quit_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Quit;
                }
            }
        }
        ScreenAction::None
    }
}
