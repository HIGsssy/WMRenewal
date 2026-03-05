//! DialogScreen — drives a ScriptRunner coroutine, showing messages
//! one at a time and choices via a ListBox.

use wm_game::state::GameState;
use wm_script::api::ScriptMessage;
use wm_script::script_runner::{RunnerStatus, ScriptRunner};

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::listbox::{ColumnDef, ListBoxWidget};
use crate::widget::text_item::TextItemWidget;
use crate::widget::{Widget, WidgetBase, WidgetId, WidgetStore};

/// Phase of the dialog UI.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Phase {
    /// Showing messages one at a time (click to advance).
    Messages,
    /// Showing a choice ListBox (click an item to choose).
    Choice {
        box_id: i32,
    },
    /// Script finished — next click pops the screen.
    Done,
}

#[derive(Debug)]
pub struct DialogScreen {
    runner: ScriptRunner,
    /// Buffered messages from the current script segment.
    messages: Vec<ScriptMessage>,
    /// Index into `messages` for currently displayed message.
    msg_index: usize,
    phase: Phase,
    text_id: WidgetId,
    hint_id: WidgetId,
    list_id: WidgetId,
    /// Girl that was the target of this script (for applying effects).
    target_girl_id: Option<usize>,
    /// Whether effects have already been applied.
    effects_applied: bool,
}

impl DialogScreen {
    /// Create from a ScriptRunner (already constructed with code loaded).
    pub fn from_runner(runner: ScriptRunner) -> Self {
        Self {
            runner,
            messages: Vec::new(),
            msg_index: 0,
            phase: Phase::Messages,
            text_id: 0,
            hint_id: 0,
            list_id: 0,
            target_girl_id: None,
            effects_applied: false,
        }
    }

    /// Set the target girl for applying script effects.
    pub fn with_target_girl(mut self, girl_id: usize) -> Self {
        self.target_girl_id = Some(girl_id);
        self
    }

    /// Convenience: create from a script file path.
    pub fn from_file(path: &std::path::Path) -> Option<Self> {
        let code = std::fs::read_to_string(path).ok()?;
        let runner = ScriptRunner::new(&code).ok()?;
        Some(Self::from_runner(runner))
    }

    /// Resume the coroutine, drain messages/choices, and transition phase.
    fn step(&mut self) {
        let status = self.runner.resume().unwrap_or(RunnerStatus::Done);
        self.drain_context();

        match status {
            RunnerStatus::Yielded => {
                // Check if there are pending choices
                let has_choices = {
                    let ctx = self.runner.context().lock().unwrap();
                    !ctx.pending_choice_options.is_empty()
                };
                if has_choices && self.messages.is_empty() {
                    // Jump straight to choice phase
                    self.enter_choice_phase();
                } else {
                    self.phase = Phase::Messages;
                    self.msg_index = 0;
                }
            }
            RunnerStatus::Done => {
                if self.messages.is_empty() {
                    self.phase = Phase::Done;
                } else {
                    self.phase = Phase::Messages;
                    self.msg_index = 0;
                }
            }
        }
    }

    /// Move messages out of the ScriptContext into our local buffer.
    fn drain_context(&mut self) {
        let mut ctx = self.runner.context().lock().unwrap();
        self.messages = ctx.messages.drain(..).collect();
        self.msg_index = 0;
    }

    /// Transition to the choice phase using the first pending choice box.
    fn enter_choice_phase(&mut self) {
        let ctx = self.runner.context().lock().unwrap();
        if let Some((box_id, _options)) = ctx.pending_choice_options.first() {
            self.phase = Phase::Choice { box_id: *box_id };
        }
    }

    /// Update the visible text widget with the current message.
    fn show_current_message(&self, widgets: &mut WidgetStore) {
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.text_id) {
            if self.msg_index < self.messages.len() {
                t.set_text(&self.messages[self.msg_index].text);
            } else {
                t.set_text("");
            }
        }
        // Update hint
        if let Some(Widget::TextItem(h)) = widgets.get_mut(self.hint_id) {
            match &self.phase {
                Phase::Messages => {
                    let remaining = self.messages.len().saturating_sub(self.msg_index + 1);
                    if remaining > 0 {
                        h.text = format!("Click to continue... ({} more)", remaining);
                    } else {
                        h.text = "Click to continue...".into();
                    }
                    h.base.hidden = false;
                }
                Phase::Choice { .. } => {
                    h.text = "Select a choice:".into();
                    h.base.hidden = false;
                }
                Phase::Done => {
                    h.text = "Click to close.".into();
                    h.base.hidden = false;
                }
            }
        }
    }

    /// Populate the ListBox with choice options.
    fn show_choices(&self, widgets: &mut WidgetStore) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.list_id) {
            lb.clear();
            let ctx = self.runner.context().lock().unwrap();
            if let Some((_box_id, options)) = ctx.pending_choice_options.first() {
                for (i, opt) in options.iter().enumerate() {
                    lb.add_element(i as i32, opt);
                }
            }
            lb.base.hidden = false;
        }
    }

    /// Hide the ListBox.
    fn hide_choices(&self, widgets: &mut WidgetStore) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.list_id) {
            lb.base.hidden = true;
            lb.clear();
        }
    }

    /// Apply script context effects to game state.
    fn apply_effects(&mut self, state: &mut GameState) {
        if self.effects_applied {
            return;
        }
        self.effects_applied = true;

        let ctx = self.runner.context().lock().unwrap();

        // Gold
        if ctx.gold_delta != 0 {
            state.gold.cash_on_hand += ctx.gold_delta as f64;
        }

        // Add target girl to current brothel
        if ctx.add_target_girl {
            if let Some(girl_id) = self.target_girl_id {
                let cur = state.brothels.current_index();
                state.brothels.assign_girl(cur, girl_id);
            }
        }

        // Write back girl stat/skill/trait/flag changes
        if let Some(girl_id) = self.target_girl_id {
            if let Some(girl) = state.girls.get_girl_mut(girl_id) {
                for i in 0..22.min(girl.stats.len()) {
                    girl.stats[i] = ctx.girl_stats[i];
                }
                for i in 0..10.min(girl.skills.len()) {
                    girl.skills[i] = ctx.girl_skills[i];
                }
                girl.traits = ctx.girl_traits.iter().cloned().collect();
                for i in 0..30.min(girl.flags.len()) {
                    girl.flags[i] = ctx.girl_flags[i] != 0;
                }
            }
        }

        // Suspicion / disposition (use scaled adjust methods)
        if ctx.suspicion_delta != 0 {
            state.player.adjust_suspicion(ctx.suspicion_delta);
        }
        if ctx.disposition_delta != 0 {
            state.player.adjust_disposition(ctx.disposition_delta);
        }
    }
}

impl Screen for DialogScreen {
    fn id(&self) -> ScreenId {
        "dialog"
    }

    fn init(&mut self, widgets: &mut WidgetStore, _state: &mut GameState) {
        // Main text area (upper portion)
        let id1 = widgets.allocate_id();
        let base1 = WidgetBase::new(id1, "DialogText", 40, 20, 720, 400);
        self.text_id = widgets.add(
            "DialogText",
            Widget::TextItem(TextItemWidget {
                base: base1,
                text: String::new(),
                font_size: 14,
                scroll_offset: 0,
                total_height: 0,
            }),
        );

        // Hint text (bottom)
        let id2 = widgets.allocate_id();
        let base2 = WidgetBase::new(id2, "DialogHint", 40, 555, 720, 25);
        self.hint_id = widgets.add(
            "DialogHint",
            Widget::TextItem(TextItemWidget {
                base: base2,
                text: String::new(),
                font_size: 12,
                scroll_offset: 0,
                total_height: 0,
            }),
        );

        // Choice ListBox (center, hidden until needed)
        let id3 = widgets.allocate_id();
        let base3 = WidgetBase::new(id3, "DialogChoices", 150, 430, 500, 120);
        self.list_id = widgets.add(
            "DialogChoices",
            Widget::ListBox(ListBoxWidget {
                base: base3,
                items: Vec::new(),
                columns: vec![ColumnDef {
                    name: "Choice".into(),
                    header: "".into(),
                    offset: 0,
                    skip: false,
                }],
                multi_select: false,
                show_headers: false,
                header_dividers: false,
                header_clicks_sort: false,
                scroll_position: 0,
                sorted_column: String::new(),
                sorted_descending: false,
                border_size: 1,
                element_height: 20,
            }),
        );

        // Hide choice list initially
        self.hide_choices(widgets);

        // Do the first resume to get initial messages
        self.step();
        self.show_current_message(widgets);
        if self.phase == Phase::Messages && self.messages.is_empty() && !self.runner.is_done() {
            // Script yielded with no messages — might have choices
            self.enter_choice_phase();
        }
        if let Phase::Choice { .. } = &self.phase {
            self.show_choices(widgets);
        }
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
            UiEvent::MouseClick { x, y } => {
                match &self.phase {
                    Phase::Messages => {
                        // Advance to next message
                        self.msg_index += 1;
                        if self.msg_index >= self.messages.len() {
                            // All messages shown for this segment
                            if self.runner.is_done() {
                                self.phase = Phase::Done;
                                self.show_current_message(widgets);
                            } else {
                                // Check for pending choices before resuming
                                let has_choices = {
                                    let ctx = self.runner.context().lock().unwrap();
                                    !ctx.pending_choice_options.is_empty()
                                };
                                if has_choices {
                                    self.enter_choice_phase();
                                    self.show_choices(widgets);
                                    self.show_current_message(widgets);
                                } else {
                                    // Resume the script for the next segment
                                    self.step();
                                    self.show_current_message(widgets);
                                    if let Phase::Choice { .. } = &self.phase {
                                        self.show_choices(widgets);
                                    }
                                }
                            }
                        } else {
                            self.show_current_message(widgets);
                        }
                    }
                    Phase::Choice { box_id } => {
                        let box_id = *box_id;
                        // Forward click to ListBox
                        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.list_id) {
                            if lb.base.is_over(x, y) {
                                lb.handle_click(x, y);
                                if let Some(selected) = lb.get_selected() {
                                    // Write choice back to context
                                    {
                                        let mut ctx =
                                            self.runner.context().lock().unwrap();
                                        ctx.choices.insert(box_id, selected);
                                        ctx.pending_choice_options.clear();
                                    }
                                    // Hide choices, resume script
                                    self.hide_choices(widgets);
                                    self.step();
                                    self.show_current_message(widgets);
                                    if let Phase::Choice { .. } = &self.phase {
                                        self.show_choices(widgets);
                                    }
                                }
                            }
                        }
                    }
                    Phase::Done => {
                        self.apply_effects(state);
                        return ScreenAction::Pop;
                    }
                }
            }
            UiEvent::Escape => {
                self.apply_effects(state);
                return ScreenAction::Pop;
            }
            _ => {}
        }
        ScreenAction::None
    }
}
