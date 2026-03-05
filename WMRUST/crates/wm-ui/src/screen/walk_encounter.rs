use rand::Rng;
use wm_core::enums::Stat;
use wm_game::girls::GirlManager;
use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::button::ButtonWidget;
use crate::widget::text_item::TextItemWidget;
use crate::widget::{Widget, WidgetBase, WidgetId, WidgetStore};

/// No-luck flavor messages when no girl is encountered.
const NO_LUCK_MESSAGES: &[&str] = &[
    "You spend the day wandering the streets of Crossgate, but find no one of interest.",
    "Despite your best efforts, your search turns up nothing today.",
    "The streets are quiet today. You find no one looking for work.",
    "You search through the markets and taverns but have no luck finding any girls.",
    "A fruitless walk through town. Perhaps the slave market would be more productive.",
    "You wander for hours but the only women you meet are not interested in your offer.",
    "The city seems empty today. Better luck next week.",
    "You explore the back alleys and plazas of Crossgate, but return empty-handed.",
];

/// State of the walk encounter interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EncounterState {
    /// No girl found — showing a no-luck message
    NoLuck,
    /// Met a girl — showing her description and recruit/leave choices
    Met,
    /// Girl accepted recruitment
    Accepted,
    /// Girl declined or player walked away
    Declined,
}

#[derive(Debug)]
pub struct WalkEncounterScreen {
    state: EncounterState,
    girl_id: Option<usize>,
    text_id: WidgetId,
    recruit_id: WidgetId,
    leave_id: WidgetId,
    ok_id: WidgetId,
}

impl WalkEncounterScreen {
    pub fn new() -> Self {
        Self {
            state: EncounterState::NoLuck,
            girl_id: None,
            text_id: 0,
            recruit_id: 0,
            leave_id: 0,
            ok_id: 0,
        }
    }
}

impl Screen for WalkEncounterScreen {
    fn id(&self) -> ScreenId {
        "walk_encounter"
    }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        widgets.clear();

        let mut rng = rand::thread_rng();

        // Mark walk as used for this week
        state.walk_around = true;

        // Roll for encounter: girl_meet% chance (default 30%)
        let meet_chance = state.config.initial.girl_meet;
        let roll = rng.gen_range(0..100);

        // Find an unassigned girl
        let mut candidate: Option<usize> = None;
        if roll < meet_chance {
            let total = state.girls.count();
            if total > 0 {
                // Collect unassigned girl IDs
                let unassigned: Vec<usize> = (0..total)
                    .filter(|&gid| state.brothels.find_girl_brothel(gid).is_none())
                    .collect();
                if !unassigned.is_empty() {
                    let idx = rng.gen_range(0..unassigned.len());
                    candidate = Some(unassigned[idx]);
                }
            }
        }

        // Main text area
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, "EncounterText", 40, 40, 720, 420);
        self.text_id = widgets.add(
            "EncounterText",
            Widget::TextItem(TextItemWidget {
                base,
                text: String::new(),
                font_size: 14,
                scroll_offset: 0,
                total_height: 0,
            }),
        );

        if let Some(gid) = candidate {
            // Met a girl!
            self.state = EncounterState::Met;
            self.girl_id = Some(gid);

            let girl = state.girls.get_girl(gid).unwrap();
            let desc = format!(
                "You go out searching around town for any new girls...\n\n\
                 You come across a young woman named {}.\n\n\
                 \"Hi, how are you?\"\n\n\
                 \"I'm not so good, I don't have any money and \
                 don't know where I am.\"\n\n\
                 Age: {}  |  Beauty: {}  |  Charisma: {}\n\n\
                 Would you like to offer her a place at your establishment?",
                girl.name,
                GirlManager::get_stat(girl, Stat::Age),
                GirlManager::get_stat(girl, Stat::Beauty),
                GirlManager::get_stat(girl, Stat::Charisma),
            );

            if let Some(Widget::TextItem(t)) = widgets.get_mut(self.text_id) {
                t.text = desc;
            }

            // Recruit button
            let id = widgets.allocate_id();
            let base = WidgetBase::new(id, "RecruitButton", 200, 500, 160, 50);
            self.recruit_id = widgets.add(
                "RecruitButton",
                Widget::Button(ButtonWidget {
                    base,
                    image_off: "BuyOff.png".into(),
                    image_on: "BuyOn.png".into(),
                    image_disabled: "BuyDisabled.png".into(),
                    transparency: true,
                    scale: true,
                    pressed: false,
                }),
            );

            // Leave button
            let id = widgets.allocate_id();
            let base = WidgetBase::new(id, "LeaveButton", 440, 500, 160, 50);
            self.leave_id = widgets.add(
                "LeaveButton",
                Widget::Button(ButtonWidget {
                    base,
                    image_off: "CancelOff.png".into(),
                    image_on: "CancelOn.png".into(),
                    image_disabled: "CancelOff.png".into(),
                    transparency: true,
                    scale: true,
                    pressed: false,
                }),
            );
        } else {
            // No luck
            self.state = EncounterState::NoLuck;
            let msg_idx = rng.gen_range(0..NO_LUCK_MESSAGES.len());
            if let Some(Widget::TextItem(t)) = widgets.get_mut(self.text_id) {
                t.text = NO_LUCK_MESSAGES[msg_idx].to_string();
            }

            // OK button to go back
            let id = widgets.allocate_id();
            let base = WidgetBase::new(id, "OkButton", 320, 500, 160, 50);
            self.ok_id = widgets.add(
                "OkButton",
                Widget::Button(ButtonWidget {
                    base,
                    image_off: "OkOff.png".into(),
                    image_on: "OkOn.png".into(),
                    image_disabled: "OkOff.png".into(),
                    transparency: true,
                    scale: true,
                    pressed: false,
                }),
            );
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
        if let UiEvent::MouseClick { x, y } = event {
            match self.state {
                EncounterState::NoLuck => {
                    if let Some(Widget::Button(b)) = widgets.get(self.ok_id) {
                        if b.base.is_over(x, y) {
                            return ScreenAction::Pop;
                        }
                    }
                }
                EncounterState::Met => {
                    if let Some(Widget::Button(b)) = widgets.get(self.recruit_id) {
                        if b.base.is_over(x, y) {
                            // Recruit the girl
                            if let Some(gid) = self.girl_id {
                                let cur = state.brothels.current_index();
                                state.brothels.assign_girl(cur, gid);
                                self.state = EncounterState::Accepted;

                                let name = state
                                    .girls
                                    .get_girl(gid)
                                    .map(|g| g.name.clone())
                                    .unwrap_or_default();
                                if let Some(Widget::TextItem(t)) = widgets.get_mut(self.text_id) {
                                    t.text =
                                        format!("{} agrees to work at your brothel!", name);
                                }
                                // Hide recruit/leave, show OK
                                if let Some(Widget::Button(b)) =
                                    widgets.get_mut(self.recruit_id)
                                {
                                    b.base.hidden = true;
                                }
                                if let Some(Widget::Button(b)) =
                                    widgets.get_mut(self.leave_id)
                                {
                                    b.base.hidden = true;
                                }
                                let id = widgets.allocate_id();
                                let base =
                                    WidgetBase::new(id, "OkButton", 320, 500, 160, 50);
                                self.ok_id = widgets.add(
                                    "OkButton",
                                    Widget::Button(ButtonWidget {
                                        base,
                                        image_off: "OkOff.png".into(),
                                        image_on: "OkOn.png".into(),
                                        image_disabled: "OkOff.png".into(),
                                        transparency: true,
                                        scale: true,
                                        pressed: false,
                                    }),
                                );
                            }
                            return ScreenAction::None;
                        }
                    }
                    if let Some(Widget::Button(b)) = widgets.get(self.leave_id) {
                        if b.base.is_over(x, y) {
                            self.state = EncounterState::Declined;
                            if let Some(Widget::TextItem(t)) = widgets.get_mut(self.text_id) {
                                t.text = "You decide to move on. \"Ok, bye.\"".to_string();
                            }
                            if let Some(Widget::Button(b)) = widgets.get_mut(self.recruit_id)
                            {
                                b.base.hidden = true;
                            }
                            if let Some(Widget::Button(b)) = widgets.get_mut(self.leave_id)
                            {
                                b.base.hidden = true;
                            }
                            let id = widgets.allocate_id();
                            let base = WidgetBase::new(id, "OkButton", 320, 500, 160, 50);
                            self.ok_id = widgets.add(
                                "OkButton",
                                Widget::Button(ButtonWidget {
                                    base,
                                    image_off: "OkOff.png".into(),
                                    image_on: "OkOn.png".into(),
                                    image_disabled: "OkOff.png".into(),
                                    transparency: true,
                                    scale: true,
                                    pressed: false,
                                }),
                            );
                            return ScreenAction::None;
                        }
                    }
                }
                EncounterState::Accepted | EncounterState::Declined => {
                    if let Some(Widget::Button(b)) = widgets.get(self.ok_id) {
                        if b.base.is_over(x, y) {
                            return ScreenAction::Pop;
                        }
                    }
                }
            }
        }
        ScreenAction::None
    }
}
