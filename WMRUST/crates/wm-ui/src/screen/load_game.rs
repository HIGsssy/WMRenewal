use wm_game::state::GameState;
use wm_game::jobs::JobDispatcher;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::button::ButtonWidget;
use crate::widget::listbox::ListBoxWidget;
use crate::widget::text_item::TextItemWidget;
use crate::widget::{Widget, WidgetBase, WidgetId, WidgetStore};

#[derive(Debug)]
pub struct LoadGameScreen {
    save_list_id: WidgetId,
    load_id: WidgetId,
    save_id: WidgetId,
    back_id: WidgetId,
    status_id: WidgetId,
    save_files: Vec<String>,
}

impl LoadGameScreen {
    pub fn new() -> Self {
        Self {
            save_list_id: 0,
            load_id: 0,
            save_id: 0,
            back_id: 0,
            status_id: 0,
            save_files: Vec::new(),
        }
    }

    fn saves_dir() -> std::path::PathBuf {
        let dir = std::path::PathBuf::from("saves");
        if !dir.exists() {
            let _ = std::fs::create_dir_all(&dir);
        }
        dir
    }

    fn scan_saves(&mut self) {
        self.save_files.clear();
        let save_dir = Self::saves_dir();
        if let Ok(entries) = std::fs::read_dir(&save_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                        self.save_files.push(name.to_string());
                    }
                }
            }
        }
        self.save_files.sort();
    }

    fn populate_list(&self, widgets: &mut WidgetStore) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.save_list_id) {
            lb.clear();
            for (i, name) in self.save_files.iter().enumerate() {
                lb.add_element(i as i32, name);
            }
        }
    }

    fn set_status(&self, widgets: &mut WidgetStore, msg: &str) {
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.status_id) {
            t.text = msg.to_string();
        }
    }
}

impl Screen for LoadGameScreen {
    fn id(&self) -> ScreenId {
        "load_game"
    }

    fn init(&mut self, widgets: &mut WidgetStore, _state: &mut GameState) {
        widgets.clear();

        // Save list
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, "SaveList", 50, 50, 700, 400);
        let lb = ListBoxWidget {
            base,
            items: Vec::new(),
            columns: Vec::new(),
            multi_select: false,
            show_headers: false,
            header_dividers: false,
            header_clicks_sort: false,
            scroll_position: 0,
            sorted_column: String::new(),
            sorted_descending: false,
            border_size: 1,
            element_height: 20,
        };
        self.save_list_id = widgets.add("SaveList", Widget::ListBox(lb));

        // Status text
        let id2 = widgets.allocate_id();
        let base2 = WidgetBase::new(id2, "Status", 50, 460, 700, 30);
        let ti = TextItemWidget {
            base: base2,
            text: "Select a save file or create a new save.".into(),
            font_size: 14,
            scroll_offset: 0,
            total_height: 0,
        };
        self.status_id = widgets.add("Status", Widget::TextItem(ti));

        // Load button
        let id3 = widgets.allocate_id();
        let base3 = WidgetBase::new(id3, "LoadButton", 200, 510, 100, 30);
        let btn = ButtonWidget {
            base: base3,
            image_off: "LoadButtonOff.png".into(),
            image_on: "LoadButtonOn.png".into(),
            image_disabled: "LoadButtonDisabled.png".into(),
            transparency: true,
            scale: true,
            pressed: false,
        };
        self.load_id = widgets.add("LoadButton", Widget::Button(btn));

        // Save button
        let id4 = widgets.allocate_id();
        let base4 = WidgetBase::new(id4, "SaveButton", 350, 510, 100, 30);
        let btn2 = ButtonWidget {
            base: base4,
            image_off: "SaveButtonOff.png".into(),
            image_on: "SaveButtonOn.png".into(),
            image_disabled: "SaveButtonDisabled.png".into(),
            transparency: true,
            scale: true,
            pressed: false,
        };
        self.save_id = widgets.add("SaveButton", Widget::Button(btn2));

        // Back button
        let id5 = widgets.allocate_id();
        let base5 = WidgetBase::new(id5, "BackButton", 500, 510, 100, 30);
        let btn3 = ButtonWidget {
            base: base5,
            image_off: "BackButtonOff.png".into(),
            image_on: "BackButtonOn.png".into(),
            image_disabled: "BackButtonDisabled.png".into(),
            transparency: true,
            scale: true,
            pressed: false,
        };
        self.back_id = widgets.add("BackButton", Widget::Button(btn3));

        self.scan_saves();
        self.populate_list(widgets);
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
            // Back
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Pop;
                }
            }
            // Save
            if let Some(Widget::Button(b)) = widgets.get(self.save_id) {
                if b.base.is_over(x, y) {
                    let save_name = format!("save_week_{}", state.week);
                    let path = Self::saves_dir().join(format!("{}.json", save_name));
                    match serde_json::to_string_pretty(state) {
                        Ok(json) => match std::fs::write(&path, json) {
                            Ok(()) => {
                                self.set_status(
                                    widgets,
                                    &format!("Saved to '{}'.", save_name),
                                );
                                self.scan_saves();
                                self.populate_list(widgets);
                            }
                            Err(e) => {
                                self.set_status(
                                    widgets,
                                    &format!("Save failed: {}", e),
                                );
                            }
                        },
                        Err(e) => {
                            self.set_status(
                                widgets,
                                &format!("Serialization failed: {}", e),
                            );
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Load
            if let Some(Widget::Button(b)) = widgets.get(self.load_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.save_list_id) {
                        if let Some(sel) = lb.get_selected() {
                            let sel = sel as usize;
                            if sel < self.save_files.len() {
                                let save_name = &self.save_files[sel];
                                let path = Self::saves_dir().join(format!("{}.json", save_name));
                                match std::fs::read_to_string(&path) {
                                    Ok(json) => match serde_json::from_str::<GameState>(&json) {
                                        Ok(mut loaded) => {
                                            // Reconstruct skipped fields
                                            loaded.job_dispatcher = JobDispatcher::new();
                                            *state = loaded;
                                            self.set_status(
                                                widgets,
                                                &format!("Loaded '{}'.", save_name),
                                            );
                                        }
                                        Err(e) => {
                                            self.set_status(
                                                widgets,
                                                &format!("Load parse error: {}", e),
                                            );
                                        }
                                    },
                                    Err(e) => {
                                        self.set_status(
                                            widgets,
                                            &format!("Load read error: {}", e),
                                        );
                                    }
                                }
                            }
                        } else {
                            self.set_status(widgets, "No save file selected.");
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // List click
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.save_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    return ScreenAction::None;
                }
            }
        }
        ScreenAction::None
    }
}
