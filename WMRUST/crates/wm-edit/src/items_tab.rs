use std::path::PathBuf;

use wm_core::enums::{EffectTarget, ItemType, Rarity};
use wm_core::item::{Effect, Item};
use wm_core::xml::{effect_target_to_str, item_type_to_str, rarity_to_str};

const ALL_ITEM_TYPES: &[ItemType] = &[
    ItemType::Food,
    ItemType::Ring,
    ItemType::Necklace,
    ItemType::Dress,
    ItemType::Underwear,
    ItemType::Shoes,
    ItemType::Hat,
    ItemType::Helmet,
    ItemType::SmallWeapon,
    ItemType::LargeWeapon,
    ItemType::Armor,
    ItemType::Shield,
    ItemType::Consumable,
    ItemType::Makeup,
    ItemType::Misc,
];

const ALL_RARITIES: &[Rarity] = &[
    Rarity::Common,
    Rarity::Shop50,
    Rarity::Shop25,
    Rarity::Shop05,
    Rarity::Catacomb15,
    Rarity::ScriptOnly,
    Rarity::Reward,
];

const ALL_EFFECT_TARGETS: &[EffectTarget] =
    &[EffectTarget::Stat, EffectTarget::Skill, EffectTarget::Trait];

#[derive(Default)]
pub struct ItemsTab {
    file_path: Option<PathBuf>,
    items: Vec<Item>,
    selected: Option<usize>,
    dirty: bool,
    status_msg: String,
}

enum Action {
    None,
    Load,
    Save,
    SaveAs,
    AddItem,
    RemoveItem,
}

impl ItemsTab {
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
            if ui.button("+ Add Item").clicked() {
                action = Action::AddItem;
            }
            if self.selected.is_some() && ui.button("- Remove").clicked() {
                action = Action::RemoveItem;
            }
            if self.dirty {
                ui.colored_label(egui::Color32::YELLOW, "* Unsaved");
            }
        });

        match action {
            Action::Load => self.load_file(),
            Action::Save => self.save_file(false),
            Action::SaveAs => self.save_file(true),
            Action::AddItem => self.add_item(),
            Action::RemoveItem => self.remove_item(),
            Action::None => {}
        }

        if !self.status_msg.is_empty() {
            ui.label(&self.status_msg);
        }
        ui.separator();

        // Build list data
        let item_names: Vec<(usize, String)> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| (i, item.name.clone()))
            .collect();

        let mut new_selection = self.selected;

        // Left panel: item list
        egui::SidePanel::left("items_list_panel")
            .min_width(150.0)
            .default_width(200.0)
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.heading(format!("{} Items", item_names.len()));
                egui::ScrollArea::vertical()
                    .id_salt("items_list_scroll")
                    .show(ui, |ui| {
                        for (i, name) in &item_names {
                            let is_sel = new_selection == Some(*i);
                            if ui.selectable_label(is_sel, name).clicked() {
                                new_selection = Some(*i);
                            }
                        }
                    });
            });

        self.selected = new_selection;

        // Right panel: item editor
        if let Some(idx) = self.selected {
            if idx < self.items.len() {
                let ItemsTab { items, dirty, .. } = self;
                egui::ScrollArea::vertical()
                    .id_salt("item_editor_scroll")
                    .show(ui, |ui| {
                        Self::edit_item(ui, &mut items[idx], dirty);
                    });
            }
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label("Select an item to edit");
            });
        }
    }

    fn edit_item(ui: &mut egui::Ui, item: &mut Item, dirty: &mut bool) {
        ui.heading("Item Properties");

        egui::Grid::new("item_props_grid")
            .num_columns(2)
            .spacing([8.0, 4.0])
            .show(ui, |ui| {
                ui.label("Name:");
                if ui.text_edit_singleline(&mut item.name).changed() {
                    *dirty = true;
                }
                ui.end_row();

                ui.label("Type:");
                egui::ComboBox::from_id_salt("item_type_combo")
                    .selected_text(item_type_to_str(item.item_type))
                    .show_ui(ui, |ui| {
                        for &itype in ALL_ITEM_TYPES {
                            if ui
                                .selectable_label(item.item_type == itype, item_type_to_str(itype))
                                .clicked()
                            {
                                item.item_type = itype;
                                *dirty = true;
                            }
                        }
                    });
                ui.end_row();

                ui.label("Cost:");
                if ui.add(egui::DragValue::new(&mut item.cost)).changed() {
                    *dirty = true;
                }
                ui.end_row();

                ui.label("Rarity:");
                egui::ComboBox::from_id_salt("item_rarity_combo")
                    .selected_text(rarity_to_str(item.rarity))
                    .show_ui(ui, |ui| {
                        for &r in ALL_RARITIES {
                            if ui
                                .selectable_label(item.rarity == r, rarity_to_str(r))
                                .clicked()
                            {
                                item.rarity = r;
                                *dirty = true;
                            }
                        }
                    });
                ui.end_row();

                ui.label("Badness:");
                if ui.add(egui::DragValue::new(&mut item.badness)).changed() {
                    *dirty = true;
                }
                ui.end_row();

                ui.label("Special:");
                if ui.text_edit_singleline(&mut item.special).changed() {
                    *dirty = true;
                }
                ui.end_row();

                ui.label("Infinite:");
                if ui.checkbox(&mut item.infinite, "").changed() {
                    *dirty = true;
                }
                ui.end_row();

                ui.label("Girl Buy Chance:");
                if ui
                    .add(egui::DragValue::new(&mut item.girl_buy_chance).range(0..=100))
                    .changed()
                {
                    *dirty = true;
                }
                ui.end_row();
            });

        ui.label("Description:");
        if ui
            .add(
                egui::TextEdit::multiline(&mut item.desc)
                    .desired_rows(3)
                    .desired_width(f32::INFINITY),
            )
            .changed()
        {
            *dirty = true;
        }

        ui.separator();

        // Effects
        egui::CollapsingHeader::new(format!("Effects ({})", item.effects.len()))
            .default_open(true)
            .show(ui, |ui| {
                let mut remove_idx = None;
                let effects_len = item.effects.len();

                for i in 0..effects_len {
                    ui.horizontal(|ui| {
                        let effect = &mut item.effects[i];

                        // Target combo
                        egui::ComboBox::from_id_salt(format!("effect_target_{i}"))
                            .selected_text(effect_target_to_str(effect.target))
                            .width(60.0)
                            .show_ui(ui, |ui| {
                                for &et in ALL_EFFECT_TARGETS {
                                    if ui
                                        .selectable_label(
                                            effect.target == et,
                                            effect_target_to_str(et),
                                        )
                                        .clicked()
                                    {
                                        effect.target = et;
                                        *dirty = true;
                                    }
                                }
                            });

                        // Name
                        if ui
                            .add(egui::TextEdit::singleline(&mut effect.name).desired_width(120.0))
                            .changed()
                        {
                            *dirty = true;
                        }

                        // Amount
                        ui.label("=");
                        if ui.add(egui::DragValue::new(&mut effect.amount)).changed() {
                            *dirty = true;
                        }

                        if ui.small_button("\u{2715}").clicked() {
                            remove_idx = Some(i);
                        }
                    });
                }

                if let Some(idx) = remove_idx {
                    item.effects.remove(idx);
                    *dirty = true;
                }

                if ui.button("+ Add Effect").clicked() {
                    item.effects.push(Effect {
                        target: EffectTarget::Stat,
                        name: String::new(),
                        amount: 0,
                    });
                    *dirty = true;
                }
            });
    }

    fn load_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Items XML", &["itemsx"])
            .pick_file()
        {
            match wm_core::xml::load_items(&path) {
                Ok(items) => {
                    self.status_msg =
                        format!("Loaded {} items from {}", items.len(), path.display());
                    self.items = items;
                    self.file_path = Some(path);
                    self.selected = if self.items.is_empty() { None } else { Some(0) };
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
                .add_filter("Items XML", &["itemsx"])
                .save_file()
        } else {
            self.file_path.clone()
        };

        if let Some(path) = path {
            match wm_core::xml::save_items(&path, &self.items) {
                Ok(()) => {
                    self.status_msg =
                        format!("Saved {} items to {}", self.items.len(), path.display());
                    self.file_path = Some(path);
                    self.dirty = false;
                }
                Err(e) => {
                    self.status_msg = format!("Error saving: {}", e);
                }
            }
        }
    }

    fn add_item(&mut self) {
        let item = Item {
            name: format!("New Item {}", self.items.len() + 1),
            desc: String::new(),
            item_type: ItemType::Misc,
            badness: 0,
            special: String::new(),
            cost: 0,
            rarity: Rarity::Common,
            infinite: false,
            girl_buy_chance: 0,
            effects: Vec::new(),
        };
        self.items.push(item);
        self.selected = Some(self.items.len() - 1);
        self.dirty = true;
    }

    fn remove_item(&mut self) {
        if let Some(idx) = self.selected {
            if idx < self.items.len() {
                self.items.remove(idx);
                self.selected = if self.items.is_empty() {
                    None
                } else {
                    Some(idx.min(self.items.len() - 1))
                };
                self.dirty = true;
            }
        }
    }
}
