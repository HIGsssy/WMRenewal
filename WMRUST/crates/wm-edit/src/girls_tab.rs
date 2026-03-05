use std::path::PathBuf;

use wm_core::enums::{Skill, Stat};
use wm_core::girl::Girl;
use wm_core::traits::TraitDef;

#[derive(Default)]
pub struct GirlsTab {
    file_path: Option<PathBuf>,
    girls: Vec<Girl>,
    selected: Option<usize>,
    dirty: bool,
    status_msg: String,
    new_trait_name: String,
    trait_defs: Vec<TraitDef>,
}

enum Action {
    None,
    Load,
    Save,
    SaveAs,
    AddGirl,
    RemoveGirl,
}

const STAT_INFO: &[(&str, Stat, i32, i32)] = &[
    ("Charisma", Stat::Charisma, 0, 100),
    ("Happiness", Stat::Happiness, 0, 100),
    ("Libido", Stat::Libido, 0, 100),
    ("Constitution", Stat::Constitution, 0, 100),
    ("Intelligence", Stat::Intelligence, 0, 100),
    ("Confidence", Stat::Confidence, 0, 100),
    ("Mana", Stat::Mana, 0, 100),
    ("Agility", Stat::Agility, 0, 100),
    ("Fame", Stat::Fame, 0, 100),
    ("Level", Stat::Level, 0, 255),
    ("AskPrice", Stat::AskPrice, 0, 10000),
    ("House %", Stat::HousePerc, 0, 100),
    ("Exp", Stat::Exp, 0, 32000),
    ("Age", Stat::Age, 18, 99),
    ("Obedience", Stat::Obedience, 0, 100),
    ("Spirit", Stat::Spirit, 0, 100),
    ("Beauty", Stat::Beauty, 0, 100),
    ("Tiredness", Stat::Tiredness, 0, 100),
    ("Health", Stat::Health, 0, 100),
    ("PC Fear", Stat::PCFear, 0, 100),
    ("PC Love", Stat::PCLove, 0, 100),
    ("PC Hate", Stat::PCHate, 0, 100),
];

const SKILL_INFO: &[(&str, Skill)] = &[
    ("Anal", Skill::Anal),
    ("Magic", Skill::Magic),
    ("BDSM", Skill::BDSM),
    ("Normal Sex", Skill::NormalSex),
    ("Beastiality", Skill::Beastiality),
    ("Group", Skill::Group),
    ("Lesbian", Skill::Lesbian),
    ("Service", Skill::Service),
    ("Strip", Skill::Strip),
    ("Combat", Skill::Combat),
];

impl GirlsTab {
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
            if ui.button("+ Add Girl").clicked() {
                action = Action::AddGirl;
            }
            if self.selected.is_some() && ui.button("- Remove").clicked() {
                action = Action::RemoveGirl;
            }
            if self.dirty {
                ui.colored_label(egui::Color32::YELLOW, "* Unsaved");
            }
        });

        match action {
            Action::Load => self.load_file(),
            Action::Save => self.save_file(false),
            Action::SaveAs => self.save_file(true),
            Action::AddGirl => self.add_girl(),
            Action::RemoveGirl => self.remove_girl(),
            Action::None => {}
        }

        if !self.status_msg.is_empty() {
            ui.label(&self.status_msg);
        }
        ui.separator();

        // Build list data to avoid borrow conflicts
        let girl_names: Vec<(usize, String)> = self
            .girls
            .iter()
            .enumerate()
            .map(|(i, g)| (i, g.name.clone()))
            .collect();

        let mut new_selection = self.selected;

        // Left panel: girl list
        egui::SidePanel::left("girls_list_panel")
            .min_width(150.0)
            .default_width(200.0)
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.heading(format!("{} Girls", girl_names.len()));
                egui::ScrollArea::vertical()
                    .id_salt("girls_list_scroll")
                    .show(ui, |ui| {
                        for (i, name) in &girl_names {
                            let is_sel = new_selection == Some(*i);
                            if ui.selectable_label(is_sel, name).clicked() {
                                new_selection = Some(*i);
                            }
                        }
                    });
            });

        self.selected = new_selection;

        // Right panel: girl editor
        if let Some(idx) = self.selected {
            if idx < self.girls.len() {
                let GirlsTab {
                    girls,
                    trait_defs,
                    new_trait_name,
                    dirty,
                    ..
                } = self;
                egui::ScrollArea::vertical()
                    .id_salt("girl_editor_scroll")
                    .show(ui, |ui| {
                        Self::edit_girl(ui, &mut girls[idx], trait_defs, new_trait_name, dirty);
                    });
            }
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label("Select a girl to edit");
            });
        }
    }

    fn edit_girl(
        ui: &mut egui::Ui,
        girl: &mut Girl,
        trait_defs: &[TraitDef],
        new_trait_name: &mut String,
        dirty: &mut bool,
    ) {
        // Basic info
        ui.heading("Basic Info");
        egui::Grid::new("girl_basic_grid")
            .num_columns(2)
            .spacing([8.0, 4.0])
            .show(ui, |ui| {
                ui.label("Name:");
                if ui.text_edit_singleline(&mut girl.name).changed() {
                    *dirty = true;
                }
                ui.end_row();

                ui.label("Gold:");
                if ui.add(egui::DragValue::new(&mut girl.money)).changed() {
                    *dirty = true;
                }
                ui.end_row();
            });

        ui.label("Description:");
        if ui
            .add(
                egui::TextEdit::multiline(&mut girl.desc)
                    .desired_rows(3)
                    .desired_width(f32::INFINITY),
            )
            .changed()
        {
            *dirty = true;
        }

        ui.separator();

        // Stats
        egui::CollapsingHeader::new("Stats (22)")
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("stats_grid")
                    .num_columns(2)
                    .spacing([8.0, 2.0])
                    .show(ui, |ui| {
                        for &(name, stat, min, max) in STAT_INFO {
                            ui.label(name);
                            if ui
                                .add(egui::Slider::new(
                                    &mut girl.stats[stat as usize],
                                    min..=max,
                                ))
                                .changed()
                            {
                                *dirty = true;
                            }
                            ui.end_row();
                        }
                    });
            });

        ui.separator();

        // Skills
        egui::CollapsingHeader::new("Skills (10)")
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("skills_grid")
                    .num_columns(2)
                    .spacing([8.0, 2.0])
                    .show(ui, |ui| {
                        for &(name, skill) in SKILL_INFO {
                            ui.label(name);
                            if ui
                                .add(egui::Slider::new(
                                    &mut girl.skills[skill as usize],
                                    0..=100,
                                ))
                                .changed()
                            {
                                *dirty = true;
                            }
                            ui.end_row();
                        }
                    });
            });

        ui.separator();

        // Traits
        egui::CollapsingHeader::new(format!("Traits ({})", girl.traits.len()))
            .default_open(true)
            .show(ui, |ui| {
                let mut remove_idx = None;
                for (i, trait_name) in girl.traits.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(trait_name);
                        if ui.small_button("\u{2715}").clicked() {
                            remove_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = remove_idx {
                    girl.traits.remove(idx);
                    *dirty = true;
                }

                ui.horizontal(|ui| {
                    let display_text = if new_trait_name.is_empty() {
                        "Select trait...".to_string()
                    } else {
                        new_trait_name.clone()
                    };
                    egui::ComboBox::from_id_salt("add_trait_combo")
                        .selected_text(display_text)
                        .width(200.0)
                        .show_ui(ui, |ui| {
                            for td in trait_defs {
                                if !girl.traits.contains(&td.name)
                                    && ui
                                        .selectable_label(
                                            *new_trait_name == td.name,
                                            &td.name,
                                        )
                                        .clicked()
                                {
                                    *new_trait_name = td.name.clone();
                                }
                            }
                        });
                    if ui.button("Add Trait").clicked() && !new_trait_name.is_empty() {
                        if !girl.traits.contains(new_trait_name) {
                            girl.traits.push(new_trait_name.clone());
                            *dirty = true;
                        }
                        new_trait_name.clear();
                    }
                });
            });
    }

    fn load_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Girl XML", &["girlsx"])
            .pick_file()
        {
            match wm_core::xml::load_girls(&path) {
                Ok(girls) => {
                    self.status_msg =
                        format!("Loaded {} girls from {}", girls.len(), path.display());
                    self.girls = girls;
                    self.file_path = Some(path);
                    self.selected = if self.girls.is_empty() {
                        None
                    } else {
                        Some(0)
                    };
                    self.dirty = false;
                    self.load_trait_defs();
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
                .add_filter("Girl XML", &["girlsx"])
                .save_file()
        } else {
            self.file_path.clone()
        };

        if let Some(path) = path {
            match wm_core::xml::save_girls(&path, &self.girls) {
                Ok(()) => {
                    self.status_msg =
                        format!("Saved {} girls to {}", self.girls.len(), path.display());
                    self.file_path = Some(path);
                    self.dirty = false;
                }
                Err(e) => {
                    self.status_msg = format!("Error saving: {}", e);
                }
            }
        }
    }

    fn add_girl(&mut self) {
        let mut girl = Girl {
            name: format!("New Girl {}", self.girls.len() + 1),
            ..Girl::default()
        };
        girl.stats[Stat::Health as usize] = 100;
        girl.stats[Stat::Happiness as usize] = 50;
        girl.stats[Stat::Age as usize] = 18;
        self.girls.push(girl);
        self.selected = Some(self.girls.len() - 1);
        self.dirty = true;
    }

    fn remove_girl(&mut self) {
        if let Some(idx) = self.selected {
            if idx < self.girls.len() {
                self.girls.remove(idx);
                self.selected = if self.girls.is_empty() {
                    None
                } else {
                    Some(idx.min(self.girls.len() - 1))
                };
                self.dirty = true;
            }
        }
    }

    fn load_trait_defs(&mut self) {
        // Try to find CoreTraits.traits relative to the loaded girl file
        if let Some(file_path) = &self.file_path {
            if let Some(parent) = file_path.parent().and_then(|p| p.parent()) {
                let path = parent.join("Data").join("CoreTraits.traits");
                if path.exists() {
                    if let Ok(traits) = wm_core::xml::load_traits(&path) {
                        self.trait_defs = traits;
                        return;
                    }
                }
            }
        }
        // Fallback: try the resources path
        let path = wm_core::resources_path().join("Data/CoreTraits.traits");
        if path.exists() {
            if let Ok(traits) = wm_core::xml::load_traits(&path) {
                self.trait_defs = traits;
            }
        }
    }
}
