pub mod bank;
pub mod brothel_management;
pub mod building_setup;
pub mod dungeon;
pub mod gallery;
pub mod gang_management;
pub mod girl_details;
pub mod girl_management;
pub mod house;
pub mod item_management;
pub mod load_game;
pub mod main_menu;
pub mod mayor;
pub mod slave_market;
pub mod town;
pub mod turn_summary;

use crate::events::UiEvent;
use crate::widget::WidgetStore;

/// Identifies a screen for `PopTo` navigation.
pub type ScreenId = &'static str;

/// Actions returned by screen processing to control navigation.
#[derive(Debug)]
pub enum ScreenAction {
    None,
    Push(Box<dyn Screen>),
    Pop,
    PopTo(ScreenId),
    Quit,
}

/// Trait implemented by all game screens.
pub trait Screen: std::fmt::Debug {
    /// Unique identifier for this screen type.
    fn id(&self) -> ScreenId;

    /// Initialize/populate widgets for this screen.
    fn init(&mut self, widgets: &mut WidgetStore);

    /// Per-frame processing (update widget state from game state, etc.).
    fn process(&mut self, widgets: &mut WidgetStore) -> ScreenAction;

    /// Handle a UI event (click, key press, etc.).
    fn on_event(&mut self, event: UiEvent, widgets: &mut WidgetStore) -> ScreenAction;
}

/// Manages the screen stack (push/pop navigation).
#[derive(Debug)]
pub struct ScreenManager {
    stack: Vec<Box<dyn Screen>>,
    pub widgets: WidgetStore,
}

impl ScreenManager {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            widgets: WidgetStore::new(),
        }
    }

    pub fn push(&mut self, mut screen: Box<dyn Screen>) {
        self.widgets.clear();
        screen.init(&mut self.widgets);
        self.stack.push(screen);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        if let Some(screen) = self.stack.last_mut() {
            self.widgets.clear();
            screen.init(&mut self.widgets);
        }
    }

    pub fn pop_to(&mut self, id: ScreenId) {
        while let Some(top) = self.stack.last() {
            if top.id() == id {
                break;
            }
            self.stack.pop();
        }
        if let Some(screen) = self.stack.last_mut() {
            self.widgets.clear();
            screen.init(&mut self.widgets);
        }
    }

    pub fn current(&self) -> Option<&dyn Screen> {
        self.stack.last().map(|s| s.as_ref())
    }

    pub fn process(&mut self) -> ScreenAction {
        if let Some(screen) = self.stack.last_mut() {
            screen.process(&mut self.widgets)
        } else {
            ScreenAction::Quit
        }
    }

    pub fn on_event(&mut self, event: UiEvent) -> ScreenAction {
        if let Some(screen) = self.stack.last_mut() {
            screen.on_event(event, &mut self.widgets)
        } else {
            ScreenAction::None
        }
    }

    pub fn handle_action(&mut self, action: ScreenAction) {
        match action {
            ScreenAction::None => {}
            ScreenAction::Push(screen) => self.push(screen),
            ScreenAction::Pop => self.pop(),
            ScreenAction::PopTo(id) => self.pop_to(id),
            ScreenAction::Quit => {} // handled by caller
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl Default for ScreenManager {
    fn default() -> Self {
        Self::new()
    }
}
