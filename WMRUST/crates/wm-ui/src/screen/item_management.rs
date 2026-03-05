use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetStore, WidgetId};
use crate::xml_loader::load_screen_xml;

#[derive(Debug)]
pub struct ItemManagementScreen {
    owners_left_id: WidgetId,
    owners_right_id: WidgetId,
    items_left_id: WidgetId,
    items_right_id: WidgetId,
    filter_list_id: WidgetId,
    shift_right_id: WidgetId,
    shift_left_id: WidgetId,
    equip_id: WidgetId,
    unequip_id: WidgetId,
    back_id: WidgetId,
    desc_id: WidgetId,
    left_owner: Owner,
    right_owner: Owner,
}

#[derive(Debug, Clone)]
enum Owner {
    Shop,
    Player,
    Girl(usize), // girl_id
}

impl ItemManagementScreen {
    pub fn new() -> Self {
        Self {
            owners_left_id: 0, owners_right_id: 0,
            items_left_id: 0, items_right_id: 0,
            filter_list_id: 0, shift_right_id: 0, shift_left_id: 0,
            equip_id: 0, unequip_id: 0, back_id: 0, desc_id: 0,
            left_owner: Owner::Player,
            right_owner: Owner::Shop,
        }
    }

    fn populate_owners(&self, widgets: &mut WidgetStore, state: &GameState) {
        // Left owners: Player + all girls in current brothel
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.owners_left_id) {
            lb.clear();
            lb.add_element(-1, "Player Inventory");
            let brothel = state.brothels.current_brothel();
            for &gid in &brothel.girls {
                if let Some(girl) = state.girls.get_girl(gid) {
                    lb.add_element(gid as i32, &girl.name);
                }
            }
        }
        // Right owners: Shop + all girls
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.owners_right_id) {
            lb.clear();
            lb.add_element(-2, "Shop");
            let brothel = state.brothels.current_brothel();
            for &gid in &brothel.girls {
                if let Some(girl) = state.girls.get_girl(gid) {
                    lb.add_element(gid as i32, &girl.name);
                }
            }
        }
    }

    fn populate_items_left(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.items_left_id) {
            lb.clear();
            match &self.left_owner {
                Owner::Player => {
                    // Show global items
                    for (i, item) in state.items.iter().enumerate() {
                        lb.add_element(i as i32, &format!("{}|{:?}|{}", item.name, item.item_type, item.cost));
                    }
                }
                Owner::Girl(gid) => {
                    if let Some(girl) = state.girls.get_girl(*gid) {
                        for (slot, &item_idx) in girl.inventory.iter().enumerate() {
                            if item_idx < state.items.len() {
                                let item = &state.items[item_idx];
                                let eq = if slot < girl.equipped.len() && girl.equipped[slot] { " [E]" } else { "" };
                                lb.add_element(slot as i32, &format!("{}{}", item.name, eq));
                            }
                        }
                    }
                }
                Owner::Shop => {}
            }
        }
    }

    fn populate_items_right(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.items_right_id) {
            lb.clear();
            match &self.right_owner {
                Owner::Shop => {
                    for (i, item) in state.items.iter().enumerate() {
                        if item.infinite || item.cost > 0 {
                            lb.add_element(i as i32, &format!("{}|{:?}|{}", item.name, item.item_type, item.cost));
                        }
                    }
                }
                Owner::Girl(gid) => {
                    if let Some(girl) = state.girls.get_girl(*gid) {
                        for (slot, &item_idx) in girl.inventory.iter().enumerate() {
                            if item_idx < state.items.len() {
                                let item = &state.items[item_idx];
                                let eq = if slot < girl.equipped.len() && girl.equipped[slot] { " [E]" } else { "" };
                                lb.add_element(slot as i32, &format!("{}{}", item.name, eq));
                            }
                        }
                    }
                }
                Owner::Player => {
                    for (i, item) in state.items.iter().enumerate() {
                        lb.add_element(i as i32, &format!("{}|{:?}|{}", item.name, item.item_type, item.cost));
                    }
                }
            }
        }
    }

    fn update_desc(&self, widgets: &mut WidgetStore, state: &GameState, item_idx: usize) {
        if item_idx < state.items.len() {
            let item = &state.items[item_idx];
            let effects: Vec<String> = item.effects.iter()
                .map(|e| format!("{:?} {} {:+}", e.target, e.name, e.amount))
                .collect();
            let desc = format!("{}\n\nType: {:?}\nCost: {}\nRarity: {:?}\n\nEffects:\n{}",
                item.desc, item.item_type, item.cost, item.rarity,
                if effects.is_empty() { "None".to_string() } else { effects.join("\n") },
            );
            if let Some(Widget::TextItem(t)) = widgets.get_mut(self.desc_id) {
                t.text = desc;
            }
        }
    }
}

impl Screen for ItemManagementScreen {
    fn id(&self) -> ScreenId { "item_management" }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/itemmanagement_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.owners_left_id = widgets.get_id("OwnersLeft").unwrap_or(0);
        self.owners_right_id = widgets.get_id("OwnersRight").unwrap_or(0);
        self.items_left_id = widgets.get_id("ItemsLeft").unwrap_or(0);
        self.items_right_id = widgets.get_id("ItemsRight").unwrap_or(0);
        self.filter_list_id = widgets.get_id("FilterList").unwrap_or(0);
        self.shift_right_id = widgets.get_id("ShiftRight").unwrap_or(0);
        self.shift_left_id = widgets.get_id("ShiftLeft").unwrap_or(0);
        self.equip_id = widgets.get_id("EquipButton").unwrap_or(0);
        self.unequip_id = widgets.get_id("UnequipButton").unwrap_or(0);
        self.back_id = widgets.get_id("BackButton").unwrap_or(0);
        self.desc_id = widgets.get_id("ItemDescription").unwrap_or(0);

        self.populate_owners(widgets, state);
        self.populate_items_left(widgets, state);
        self.populate_items_right(widgets, state);
    }

    fn process(&mut self, _widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction {
        ScreenAction::None
    }

    fn on_event(&mut self, event: UiEvent, widgets: &mut WidgetStore, state: &mut GameState) -> ScreenAction {
        if let UiEvent::MouseClick { x, y } = event {
            // Back
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) { return ScreenAction::Pop; }
            }
            // Equip button
            if let Some(Widget::Button(b)) = widgets.get(self.equip_id) {
                if b.base.is_over(x, y) {
                    if let Owner::Girl(gid) = &self.left_owner {
                        if let Some(Widget::ListBox(lb)) = widgets.get(self.items_left_id) {
                            if let Some(slot) = lb.get_selected() {
                                let slot = slot as usize;
                                if let Some(girl) = state.girls.get_girl_mut(*gid) {
                                    if slot < girl.inventory.len() {
                                        // Ensure equipped vec is large enough
                                        while girl.equipped.len() <= slot { girl.equipped.push(false); }
                                        girl.equipped[slot] = true;
                                        self.populate_items_left(widgets, state);
                                    }
                                }
                            }
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Unequip button
            if let Some(Widget::Button(b)) = widgets.get(self.unequip_id) {
                if b.base.is_over(x, y) {
                    if let Owner::Girl(gid) = &self.left_owner {
                        if let Some(Widget::ListBox(lb)) = widgets.get(self.items_left_id) {
                            if let Some(slot) = lb.get_selected() {
                                let slot = slot as usize;
                                if let Some(girl) = state.girls.get_girl_mut(*gid) {
                                    if slot < girl.equipped.len() {
                                        girl.equipped[slot] = false;
                                        self.populate_items_left(widgets, state);
                                    }
                                }
                            }
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Transfer right (left → right owner)
            if let Some(Widget::Button(b)) = widgets.get(self.shift_right_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.items_left_id) {
                        if let Some(sel) = lb.get_selected() {
                            let sel = sel as usize;
                            if let Owner::Girl(src_gid) = &self.left_owner {
                                if let Owner::Girl(dst_gid) = &self.right_owner {
                                    let src = *src_gid;
                                    let dst = *dst_gid;
                                    if let Some(girl) = state.girls.get_girl_mut(src) {
                                        if sel < girl.inventory.len() {
                                            let item_idx = girl.inventory.remove(sel);
                                            if sel < girl.equipped.len() { girl.equipped.remove(sel); }
                                            if let Some(dst_girl) = state.girls.get_girl_mut(dst) {
                                                dst_girl.inventory.push(item_idx);
                                                dst_girl.equipped.push(false);
                                            }
                                        }
                                    }
                                    self.populate_items_left(widgets, state);
                                    self.populate_items_right(widgets, state);
                                }
                            }
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Left owner selection
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.owners_left_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(sel) = lb.get_selected() {
                        self.left_owner = if sel == -1 { Owner::Player } else { Owner::Girl(sel as usize) };
                        self.populate_items_left(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
            // Right owner selection
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.owners_right_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(sel) = lb.get_selected() {
                        self.right_owner = if sel == -2 { Owner::Shop } else { Owner::Girl(sel as usize) };
                        self.populate_items_right(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
            // Item list clicks (show description)
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.items_left_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(sel) = lb.get_selected() {
                        self.update_desc(widgets, state, sel as usize);
                    }
                    return ScreenAction::None;
                }
            }
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.items_right_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(sel) = lb.get_selected() {
                        self.update_desc(widgets, state, sel as usize);
                    }
                    return ScreenAction::None;
                }
            }
        }
        ScreenAction::None
    }
}
