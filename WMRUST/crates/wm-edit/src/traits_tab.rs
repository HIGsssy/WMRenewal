use std::path::PathBuf;

use wm_core::traits::TraitDef;

#[derive(Default)]
pub struct TraitsTab {
    file_path: Option<PathBuf>,
    traits_list: Vec<TraitDef>,
    selected: Option<usize>,
    dirty: bool,
    status_msg: String,
}

enum Action {
    None,
    Load,
    Save,
    SaveAs,
    AddTrait,
    RemoveTrait,
}

impl TraitsTab {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        let mut action = Action::None;

        // Toolbar
        ui.horizontal(|ui| {
            if let Some(path) = &self.file_path {
                ui.label(format!(
                    "File: {}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ));
            } else {
                ui.label("No file loaded");
            }
            if ui.button("\u{1F4C2} Load").clicked() {
                action = Action::Load;
            }
            if ui.button("\u{1F4BE} Save").clicked() {
                action = Action::Save;
            }
            if ui.button("Save As...").clicked() {
                action = Action::SaveAs;
            }
            ui.separator();
            if ui.button("+ Add Trait").clicked() {
                action = Action::AddTrait;
            }
            if self.selected.is_some() && ui.button("- Remove").clicked() {
                action = Action::RemoveTrait;
            }
            if self.dirty {
                ui.colored_label(egui::Color32::YELLOW, "* Unsaved");
            }
        });

        match action {
            Action::Load => self.load_file(),
            Action::Save => self.save_file(false),
            Action::SaveAs => self.save_file(true),
            Action::AddTrait => self.add_trait(),
            Action::RemoveTrait => self.remove_trait(),
            Action::None => {}
        }

        if !self.status_msg.is_empty() {
            ui.label(&self.status_msg);
        }
        ui.separator();

        // Build list data
        let trait_names: Vec<(usize, String)> = self
            .traits_list
            .iter()
            .enumerate()
            .map(|(i, t)| (i, t.name.clone()))
            .collect();

        let mut new_selection = self.selected;

        // Left panel: trait list
        egui::SidePanel::left("traits_list_panel")
            .min_width(150.0)
            .default_width(200.0)
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.heading(format!("{} Traits", trait_names.len()));
                egui::ScrollArea::vertical()
                    .id_salt("traits_list_scroll")
                    .show(ui, |ui| {
                        for (i, name) in &trait_names {
                            let is_sel = new_selection == Some(*i);
                            if ui.selectable_label(is_sel, name).clicked() {
                                new_selection = Some(*i);
                            }
                        }
                    });
            });

        self.selected = new_selection;

        // Right panel: trait editor
        if let Some(idx) = self.selected {
            if idx < self.traits_list.len() {
                let TraitsTab {
                    traits_list, dirty, ..
                } = self;
                let trait_def = &mut traits_list[idx];

                egui::Grid::new("trait_edit_grid")
                    .num_columns(2)
                    .spacing([8.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Name:");
                        if ui.text_edit_singleline(&mut trait_def.name).changed() {
                            *dirty = true;
                        }
                        ui.end_row();
                    });

                ui.add_space(8.0);
                ui.label("Description:");
                if ui
                    .add(
                        egui::TextEdit::multiline(&mut trait_def.description)
                            .desired_rows(4)
                            .desired_width(f32::INFINITY),
                    )
                    .changed()
                {
                    *dirty = true;
                }
            }
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label("Select a trait to edit");
            });
        }
    }

    fn load_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Traits", &["traits"])
            .pick_file()
        {
            match wm_core::xml::load_traits(&path) {
                Ok(traits) => {
                    self.status_msg =
                        format!("Loaded {} traits from {}", traits.len(), path.display());
                    self.traits_list = traits;
                    self.file_path = Some(path);
                    self.selected = if self.traits_list.is_empty() {
                        None
                    } else {
                        Some(0)
                    };
                    self.dirty = false;
                }
                Err(e) => {
                    self.status_msg = format!("Error loading: {}", e);
                }
            }
        }
    }

    fn save_file(&mut self, save_as: bool) {
        let path = if save_as || self.file_path.is_none() {
            rfd::FileDialog::new()
                .add_filter("Traits", &["traits"])
                .save_file()
        } else {
            self.file_path.clone()
        };

        if let Some(path) = path {
            match wm_core::xml::save_traits(&path, &self.traits_list) {
                Ok(()) => {
                    self.status_msg = format!(
                        "Saved {} traits to {}",
                        self.traits_list.len(),
                        path.display()
                    );
                    self.file_path = Some(path);
                    self.dirty = false;
                }
                Err(e) => {
                    self.status_msg = format!("Error saving: {}", e);
                }
            }
        }
    }

    fn add_trait(&mut self) {
        self.traits_list.push(TraitDef {
            name: format!("New Trait {}", self.traits_list.len() + 1),
            description: String::new(),
        });
        self.selected = Some(self.traits_list.len() - 1);
        self.dirty = true;
    }

    fn remove_trait(&mut self) {
        if let Some(idx) = self.selected {
            if idx < self.traits_list.len() {
                self.traits_list.remove(idx);
                self.selected = if self.traits_list.is_empty() {
                    None
                } else {
                    Some(idx.min(self.traits_list.len() - 1))
                };
                self.dirty = true;
            }
        }
    }
}
