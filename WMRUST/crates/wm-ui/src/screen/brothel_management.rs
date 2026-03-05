use wm_core::enums::Stat;
use wm_game::girls::GirlManager;
use wm_game::state::GameState;
use wm_game::turn::TurnProcessor;
use wm_script::lua_engine::LuaEngine;
use wm_script::script_converter;
use wm_script::triggers::{TriggerEvalContext, TriggerSystem};

use crate::events::UiEvent;
use crate::screen::building_setup::BuildingSetupScreen;
use crate::screen::dungeon::DungeonScreen;
use crate::screen::girl_management::GirlManagementScreen;
use crate::screen::town::TownScreen;
use crate::screen::turn_summary::TurnSummaryScreen;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::button::ButtonWidget;
use crate::widget::listbox::ListBoxWidget;
use crate::widget::text_item::TextItemWidget;
use crate::widget::{Widget, WidgetBase, WidgetId, WidgetStore};

#[derive(Debug)]
pub struct BrothelManagementScreen {
    girl_mgmt_id: WidgetId,
    setup_id: WidgetId,
    dungeon_id: WidgetId,
    town_id: WidgetId,
    next_week_id: WidgetId,
    turn_summary_id: WidgetId,
    quit_id: WidgetId,
    prev_id: WidgetId,
    next_id: WidgetId,
    details_id: WidgetId,
    name_id: WidgetId,
    girl_list_id: WidgetId,
    girl_desc_id: WidgetId,
    last_events: Option<Vec<String>>,
}

impl BrothelManagementScreen {
    pub fn new() -> Self {
        Self {
            girl_mgmt_id: 0,
            setup_id: 0,
            dungeon_id: 0,
            town_id: 0,
            next_week_id: 0,
            turn_summary_id: 0,
            quit_id: 0,
            prev_id: 0,
            next_id: 0,
            details_id: 0,
            name_id: 0,
            girl_list_id: 0,
            girl_desc_id: 0,
            last_events: None,
        }
    }

    fn make_button(
        widgets: &mut WidgetStore,
        name: &str,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> WidgetId {
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, name, x, y, w, h);
        widgets.add(
            name,
            Widget::Button(ButtonWidget {
                base,
                image_off: format!("{}Off.png", name),
                image_on: format!("{}On.png", name),
                image_disabled: format!("{}Disabled.png", name),
                transparency: true,
                scale: true,
                pressed: false,
            }),
        )
    }

    fn update_details(&self, widgets: &mut WidgetStore, state: &GameState) {
        let brothel = state.brothels.current_brothel();
        let details = format!(
            "Girls: {}/{}\nRooms: {}\nFame: {}\nHappiness: {}\nFilth: {}\nWeek: {}\nGold: {:.0}",
            brothel.num_girls(),
            brothel.num_rooms,
            brothel.num_rooms,
            brothel.fame,
            brothel.happiness,
            brothel.filthiness,
            state.week,
            state.gold.cash_on_hand,
        );
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.details_id) {
            t.text = details;
        }
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.name_id) {
            t.text = brothel.name.clone();
        }
    }

    fn populate_girl_list(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.girl_list_id) {
            lb.clear();
            let brothel = state.brothels.current_brothel();
            for &girl_id in &brothel.girls {
                if let Some(girl) = state.girls.get_girl(girl_id) {
                    let job_day = girl
                        .job_day
                        .map(|j| format!("{:?}", j))
                        .unwrap_or_else(|| "None".into());
                    let health = GirlManager::get_stat(girl, Stat::Health);
                    let happiness = GirlManager::get_stat(girl, Stat::Happiness);
                    let data = format!("{}|{}|{}|{}", girl.name, health, happiness, job_day);
                    lb.add_element(girl_id as i32, &data);
                }
            }
        }
    }
}

impl Default for BrothelManagementScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl Screen for BrothelManagementScreen {
    fn id(&self) -> ScreenId {
        "brothel_management"
    }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        // Layout from BrothelScreen.txt (programmatic)
        // Brothel Name text at 8,8 584x40
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, "BrothelName", 8, 8, 584, 40);
        self.name_id = widgets.add(
            "BrothelName",
            Widget::TextItem(TextItemWidget {
                base,
                text: String::new(),
                font_size: 20,
                scroll_offset: 0,
                total_height: 0,
            }),
        );

        // Girl list at 8,48 584x480 (main content area from BrothelScreen.txt)
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, "BrothelGirlList", 8, 48, 584, 480);
        self.girl_list_id = widgets.add(
            "BrothelGirlList",
            Widget::ListBox(ListBoxWidget {
                base,
                items: Vec::new(),
                columns: vec![
                    crate::widget::listbox::ColumnDef {
                        name: "Name".into(),
                        header: "Name".into(),
                        offset: 0,
                        skip: false,
                    },
                    crate::widget::listbox::ColumnDef {
                        name: "Health".into(),
                        header: "HP".into(),
                        offset: 200,
                        skip: false,
                    },
                    crate::widget::listbox::ColumnDef {
                        name: "Happiness".into(),
                        header: "Happy".into(),
                        offset: 300,
                        skip: false,
                    },
                    crate::widget::listbox::ColumnDef {
                        name: "Job".into(),
                        header: "Day Job".into(),
                        offset: 400,
                        skip: false,
                    },
                ],
                multi_select: false,
                show_headers: true,
                header_dividers: true,
                header_clicks_sort: true,
                scroll_position: 0,
                sorted_column: String::new(),
                sorted_descending: false,
                border_size: 1,
                element_height: 20,
            }),
        );

        // Girl description text below list
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, "GirlDesc", 8, 530, 400, 20);
        self.girl_desc_id = widgets.add(
            "GirlDesc",
            Widget::TextItem(TextItemWidget {
                base,
                text: "Click a girl to select. Visit Town > Slave Market to buy girls.".into(),
                font_size: 10,
                scroll_offset: 0,
                total_height: 0,
            }),
        );

        // Brothel Details text at 600,20 160x170
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, "BrothelDetails", 600, 20, 160, 170);
        self.details_id = widgets.add(
            "BrothelDetails",
            Widget::TextItem(TextItemWidget {
                base,
                text: String::new(),
                font_size: 10,
                scroll_offset: 0,
                total_height: 0,
            }),
        );

        // Buttons from BrothelScreen.txt
        self.girl_mgmt_id = Self::make_button(widgets, "Girls", 600, 258, 160, 32);
        self.setup_id = Self::make_button(widgets, "Building", 600, 338, 160, 32);
        self.dungeon_id = Self::make_button(widgets, "Dungeon", 600, 398, 160, 32);
        self.town_id = Self::make_button(widgets, "VisitTown", 600, 458, 160, 32);
        self.next_week_id = Self::make_button(widgets, "NextWeek", 600, 536, 160, 32);
        self.turn_summary_id = Self::make_button(widgets, "TurnSum", 432, 536, 160, 32);
        self.quit_id = Self::make_button(widgets, "Quit", 8, 536, 160, 32);
        self.prev_id = Self::make_button(widgets, "Prev", 600, 208, 72, 32);
        self.next_id = Self::make_button(widgets, "Next", 688, 208, 72, 32);

        self.update_details(widgets, state);
        self.populate_girl_list(widgets, state);
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
            if let Some(Widget::Button(b)) = widgets.get(self.girl_mgmt_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(GirlManagementScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.setup_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(BuildingSetupScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.dungeon_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(DungeonScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.town_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(TownScreen::new()));
                }
            }
            // Girl list click
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.girl_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(sel) = lb.get_selected() {
                        let gid = sel as usize;
                        if let Some(girl) = state.girls.get_girl(gid) {
                            let desc = format!(
                                "{} - HP:{} Happy:{}",
                                girl.name,
                                GirlManager::get_stat(girl, Stat::Health),
                                GirlManager::get_stat(girl, Stat::Happiness),
                            );
                            if let Some(Widget::TextItem(t)) = widgets.get_mut(self.girl_desc_id) {
                                t.text = desc;
                            }
                        }
                    }
                    return ScreenAction::None;
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.next_week_id) {
                if b.base.is_over(x, y) {
                    let events = TurnProcessor::process_week(state);
                    let mut all_events = events.events;

                    // Evaluate global triggers and run fired scripts
                    let scripts_dir = wm_core::resources_path().join("Scripts");
                    let triggers_path = scripts_dir.join("GlobalTriggers.xml");
                    if triggers_path.exists() {
                        let mut trigger_sys = TriggerSystem::new();
                        if trigger_sys.load(&triggers_path).is_ok() {
                            let eval_ctx = TriggerEvalContext {
                                player_gold: state.gold.cash_on_hand as i64,
                                global_flags: state.global_flags,
                                ..Default::default()
                            };
                            trigger_sys.evaluate(&eval_ctx);

                            if let Ok(engine) = LuaEngine::new() {
                                while let Some(trigger) = trigger_sys.process_next() {
                                    let script_file = &trigger.script;
                                    engine.reset_context();

                                    // Try .lua first, then convert .script
                                    let lua_path = scripts_dir.join(
                                        script_file.replace(".script", ".lua"),
                                    );
                                    let ran = if lua_path.exists() {
                                        engine.exec_file(&lua_path).is_ok()
                                    } else {
                                        let script_path = scripts_dir.join(script_file);
                                        if script_path.exists() {
                                            if let Ok(lua_code) =
                                                script_converter::convert_script_to_lua(
                                                    &script_path,
                                                )
                                            {
                                                engine.exec_str(&lua_code).is_ok()
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        }
                                    };
                                    if ran {
                                        let ctx = engine.context().lock().unwrap();
                                        for msg in &ctx.messages {
                                            all_events.push(msg.text.clone());
                                        }
                                        // Apply gold changes
                                        state.gold.cash_on_hand += ctx.gold_delta as f64;
                                    }
                                }
                            }
                        }
                    }

                    self.last_events = Some(all_events.clone());
                    self.update_details(widgets, state);
                    return ScreenAction::Push(Box::new(TurnSummaryScreen::with_events(
                        all_events,
                    )));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.turn_summary_id) {
                if b.base.is_over(x, y) {
                    let evts = self.last_events.clone().unwrap_or_default();
                    return ScreenAction::Push(Box::new(TurnSummaryScreen::with_events(evts)));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.quit_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Quit;
                }
            }
            // Prev/Next brothel
            if let Some(Widget::Button(b)) = widgets.get(self.prev_id) {
                if b.base.is_over(x, y) {
                    let cur = state.brothels.current_index();
                    if cur > 0 {
                        state.brothels.set_current(cur - 1);
                        self.update_details(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.next_id) {
                if b.base.is_over(x, y) {
                    let cur = state.brothels.current_index();
                    if cur + 1 < state.brothels.num_brothels() {
                        state.brothels.set_current(cur + 1);
                        self.update_details(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
        }
        ScreenAction::None
    }
}
