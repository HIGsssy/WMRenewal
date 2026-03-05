use wm_game::state::GameState;
use wm_script::lua_engine::LuaEngine;

use crate::events::UiEvent;
use crate::screen::brothel_management::BrothelManagementScreen;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::text_item::TextItemWidget;
use crate::widget::{Widget, WidgetBase, WidgetId, WidgetStore};

/// Intro story screen — runs Intro.lua and shows messages one at a time.
/// Click anywhere to advance to the next message, then to brothel management.
#[derive(Debug)]
pub struct IntroScreen {
    messages: Vec<String>,
    current: usize,
    text_id: WidgetId,
    hint_id: WidgetId,
}

impl IntroScreen {
    pub fn new() -> Self {
        // Run Intro.lua to collect story messages
        let mut messages = Vec::new();
        let scripts_path = wm_core::resources_path().join("Scripts/Intro.lua");
        if scripts_path.exists() {
            if let Ok(engine) = LuaEngine::new() {
                engine.reset_context();
                if engine.exec_file(&scripts_path).is_ok() {
                    let ctx = engine.context().lock().unwrap();
                    messages = ctx.messages.iter().map(|m| m.text.clone()).collect();
                }
            }
        }
        if messages.is_empty() {
            messages.push("Welcome to WhoreMaster Renewal!\n\nClick to continue...".into());
        }
        Self {
            messages,
            current: 0,
            text_id: 0,
            hint_id: 0,
        }
    }

    fn show_current(&self, widgets: &mut WidgetStore) {
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.text_id) {
            if self.current < self.messages.len() {
                t.text = self.messages[self.current].clone();
            }
        }
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.hint_id) {
            let remaining = self.messages.len().saturating_sub(self.current + 1);
            t.text = if remaining > 0 {
                format!("Click to continue... ({} more)", remaining)
            } else {
                "Click to begin...".into()
            };
        }
    }
}

impl Screen for IntroScreen {
    fn id(&self) -> ScreenId {
        "intro"
    }

    fn init(&mut self, widgets: &mut WidgetStore, _state: &mut GameState) {
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, "IntroText", 40, 40, 720, 480);
        self.text_id = widgets.add(
            "IntroText",
            Widget::TextItem(TextItemWidget {
                base,
                text: String::new(),
                font_size: 14,
                scroll_offset: 0,
                total_height: 0,
            }),
        );

        let id2 = widgets.allocate_id();
        let base2 = WidgetBase::new(id2, "Hint", 40, 550, 720, 30);
        self.hint_id = widgets.add(
            "Hint",
            Widget::TextItem(TextItemWidget {
                base: base2,
                text: String::new(),
                font_size: 12,
                scroll_offset: 0,
                total_height: 0,
            }),
        );

        self.current = 0;
        self.show_current(widgets);
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
        if let UiEvent::MouseClick { .. } = event {
            self.current += 1;
            if self.current >= self.messages.len() {
                // Done with intro — go to brothel management
                return ScreenAction::Push(Box::new(BrothelManagementScreen::new()));
            }
            self.show_current(widgets);
        }
        ScreenAction::None
    }
}
