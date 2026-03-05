//! TextInputScreen — modal screen with an EditBox for entering text.
//!
//! Writes the result to `state.pending_text_result` on OK/Enter.
//! Sets it to `None` on Cancel/Escape.

use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::editbox::EditBoxWidget;
use crate::widget::text_item::TextItemWidget;
use crate::widget::{Widget, WidgetBase, WidgetId, WidgetStore};

#[derive(Debug)]
pub struct TextInputScreen {
    prompt: String,
    initial: String,
    prompt_id: WidgetId,
    edit_id: WidgetId,
}

impl TextInputScreen {
    pub fn new(prompt: &str, initial: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            initial: initial.to_string(),
            prompt_id: 0,
            edit_id: 0,
        }
    }

    fn get_text(&self, widgets: &WidgetStore) -> String {
        if let Some(Widget::EditBox(eb)) = widgets.get(self.edit_id) {
            eb.get_text().to_string()
        } else {
            String::new()
        }
    }

    fn confirm(&self, widgets: &WidgetStore, state: &mut GameState) -> ScreenAction {
        let text = self.get_text(widgets);
        if !text.is_empty() {
            state.pending_text_result = Some(text);
        } else {
            state.pending_text_result = None;
        }
        ScreenAction::Pop
    }
}

impl Screen for TextInputScreen {
    fn id(&self) -> ScreenId {
        "text_input"
    }

    fn init(&mut self, widgets: &mut WidgetStore, _state: &mut GameState) {
        // Prompt label
        let id1 = widgets.allocate_id();
        let base1 = WidgetBase::new(id1, "Prompt", 200, 220, 400, 30);
        self.prompt_id = widgets.add(
            "Prompt",
            Widget::TextItem(TextItemWidget {
                base: base1,
                text: self.prompt.clone(),
                font_size: 14,
                scroll_offset: 0,
                total_height: 0,
            }),
        );

        // Edit box
        let id2 = widgets.allocate_id();
        let base2 = WidgetBase::new(id2, "EditBox", 200, 260, 400, 28);
        self.edit_id = widgets.add(
            "EditBox",
            Widget::EditBox(EditBoxWidget {
                base: base2,
                text: self.initial.clone(),
                max_length: 40,
                focused: true,
            }),
        );
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
        match event {
            UiEvent::KeyPress { key, .. } => {
                if let Some(Widget::EditBox(eb)) = widgets.get_mut(self.edit_id) {
                    eb.handle_key(key);
                }
            }
            UiEvent::Enter => {
                return self.confirm(widgets, state);
            }
            UiEvent::Escape => {
                state.pending_text_result = None;
                return ScreenAction::Pop;
            }
            UiEvent::MouseClick { x, y } => {
                if let Some(Widget::EditBox(eb)) = widgets.get_mut(self.edit_id) {
                    eb.handle_click(x, y);
                }
            }
            _ => {}
        }
        ScreenAction::None
    }
}
