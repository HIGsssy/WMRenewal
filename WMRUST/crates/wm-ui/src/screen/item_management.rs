use wm_core::girl::MAX_INVENTORY;
use wm_game::inventory;
use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetId, WidgetStore};
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
    gold_id: WidgetId,
    left_owner: Owner,
    right_owner: Owner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Owner {
    Shop,
    Player,
    Girl(usize), // girl_id
}

impl ItemManagementScreen {
    pub fn new() -> Self {
        Self {
            owners_left_id: 0,
            owners_right_id: 0,
            items_left_id: 0,
            items_right_id: 0,
            filter_list_id: 0,
            shift_right_id: 0,
            shift_left_id: 0,
            equip_id: 0,
            unequip_id: 0,
            back_id: 0,
            desc_id: 0,
            gold_id: 0,
            left_owner: Owner::Player,
            right_owner: Owner::Shop,
        }
    }

    fn populate_owners(&self, widgets: &mut WidgetStore, state: &GameState) {
        // Left owners: Player + Shop + all girls in current brothel
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.owners_left_id) {
            lb.clear();
            lb.add_element(-1, "Player Inventory");
            lb.add_element(-2, "Shop");
            let brothel = state.brothels.current_brothel();
            for &gid in &brothel.girls {
                if let Some(girl) = state.girls.get_girl(gid) {
                    lb.add_element(gid as i32, &girl.name);
                }
            }
        }
        // Right owners: Shop + Player + all girls
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.owners_right_id) {
            lb.clear();
            lb.add_element(-2, "Shop");
            lb.add_element(-1, "Player Inventory");
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
                    for (i, item) in state.items.iter().enumerate() {
                        lb.add_element(
                            i as i32,
                            &format!("{}|{:?}|{}", item.name, item.item_type, item.cost),
                        );
                    }
                }
                Owner::Girl(gid) => {
                    if let Some(girl) = state.girls.get_girl(*gid) {
                        for (slot, &item_idx) in girl.inventory.iter().enumerate() {
                            if item_idx < state.items.len() {
                                let item = &state.items[item_idx];
                                let eq = if slot < girl.equipped.len() && girl.equipped[slot] {
                                    " [E]"
                                } else {
                                    ""
                                };
                                lb.add_element(slot as i32, &format!("{}{}", item.name, eq));
                            }
                        }
                    }
                }
                Owner::Shop => {
                    for &item_idx in &state.shop_stock {
                        if item_idx < state.items.len() {
                            let item = &state.items[item_idx];
                            lb.add_element(
                                item_idx as i32,
                                &format!("{}|{:?}|{}", item.name, item.item_type, item.cost),
                            );
                        }
                    }
                }
            }
        }
    }

    fn populate_items_right(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.items_right_id) {
            lb.clear();
            match &self.right_owner {
                Owner::Shop => {
                    for &item_idx in &state.shop_stock {
                        if item_idx < state.items.len() {
                            let item = &state.items[item_idx];
                            lb.add_element(
                                item_idx as i32,
                                &format!("{}|{:?}|{}", item.name, item.item_type, item.cost),
                            );
                        }
                    }
                }
                Owner::Girl(gid) => {
                    if let Some(girl) = state.girls.get_girl(*gid) {
                        for (slot, &item_idx) in girl.inventory.iter().enumerate() {
                            if item_idx < state.items.len() {
                                let item = &state.items[item_idx];
                                let eq = if slot < girl.equipped.len() && girl.equipped[slot] {
                                    " [E]"
                                } else {
                                    ""
                                };
                                lb.add_element(slot as i32, &format!("{}{}", item.name, eq));
                            }
                        }
                    }
                }
                Owner::Player => {
                    for (i, item) in state.items.iter().enumerate() {
                        lb.add_element(
                            i as i32,
                            &format!("{}|{:?}|{}", item.name, item.item_type, item.cost),
                        );
                    }
                }
            }
        }
    }

    fn update_desc(&self, widgets: &mut WidgetStore, state: &GameState, item_idx: usize) {
        if item_idx < state.items.len() {
            let item = &state.items[item_idx];
            let effects: Vec<String> = item
                .effects
                .iter()
                .map(|e| format!("{:?} {} {:+}", e.target, e.name, e.amount))
                .collect();
            let desc = format!(
                "{}\n\nType: {:?}\nCost: {} (sell: {})\nRarity: {:?}\n\nEffects:\n{}",
                item.desc,
                item.item_type,
                item.cost,
                item.cost / 2,
                item.rarity,
                if effects.is_empty() {
                    "None".to_string()
                } else {
                    effects.join("\n")
                },
            );
            if let Some(Widget::TextItem(t)) = widgets.get_mut(self.desc_id) {
                t.text = desc;
            }
        }
    }

    fn update_gold(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.gold_id) {
            t.text = format!("Gold: {:.0}", state.gold.cash_on_hand);
        }
    }

    /// Transfer an item from the left pane to the right pane.
    fn transfer_right(&mut self, sel: i32, widgets: &mut WidgetStore, state: &mut GameState) {
        let src = self.left_owner.clone();
        let dst = self.right_owner.clone();

        match (&src, &dst) {
            // Girl → Shop: sell at 50%
            (Owner::Girl(gid), Owner::Shop) => {
                let gid = *gid;
                let slot = sel as usize;
                if let Some(girl) = state.girls.get_girl(gid) {
                    if slot < girl.inventory.len() {
                        let item_idx = girl.inventory[slot];
                        if item_idx < state.items.len() {
                            let sell_price = (state.items[item_idx].cost / 2) as f64;
                            state.gold.add_item_sales(sell_price);
                            inventory::remove_item(
                                state.girls.get_girl_mut(gid).unwrap(),
                                &state.items,
                                slot,
                            );
                        }
                    }
                }
            }
            // Shop → Girl: buy at full price, apply gift effects
            (Owner::Shop, Owner::Girl(gid)) => {
                let gid = *gid;
                let item_idx = sel as usize;
                if item_idx < state.items.len() {
                    let cost = state.items[item_idx].cost as f64;
                    if !state.gold.pay_item_cost(cost) {
                        return; // Can't afford
                    }
                    if let Some(girl) = state.girls.get_girl_mut(gid) {
                        if girl.inventory.len() < MAX_INVENTORY {
                            inventory::add_item(girl, item_idx);
                            // Gift effects: happiness, obedience, love
                            let happiness = inventory::happiness_from_item(&state.items[item_idx]);
                            use wm_game::girls::GirlManager;
                            use wm_core::enums::Stat;
                            GirlManager::update_stat(girl, Stat::Happiness, happiness);
                            GirlManager::update_stat(girl, Stat::Obedience, 1);
                            GirlManager::update_stat(girl, Stat::PCLove, happiness.max(1) - 1);
                            GirlManager::update_stat(girl, Stat::PCHate, -2);
                            GirlManager::update_stat(girl, Stat::PCFear, -1);
                        }
                    }
                }
            }
            // Girl → Girl: direct transfer (no gold)
            (Owner::Girl(src_gid), Owner::Girl(dst_gid)) => {
                let src_g = *src_gid;
                let dst_g = *dst_gid;
                let slot = sel as usize;
                if let Some(girl) = state.girls.get_girl_mut(src_g) {
                    if slot < girl.inventory.len() {
                        let item_idx = girl.inventory.remove(slot);
                        if slot < girl.equipped.len() {
                            girl.equipped.remove(slot);
                        }
                        if let Some(dst_girl) = state.girls.get_girl_mut(dst_g) {
                            if dst_girl.inventory.len() < MAX_INVENTORY {
                                dst_girl.inventory.push(item_idx);
                                dst_girl.equipped.push(false);
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        self.populate_items_left(widgets, state);
        self.populate_items_right(widgets, state);
        self.update_gold(widgets, state);
    }

    /// Transfer an item from the right pane to the left pane.
    fn transfer_left(&mut self, sel: i32, widgets: &mut WidgetStore, state: &mut GameState) {
        // Swap owners and delegate to the same logic
        std::mem::swap(&mut self.left_owner, &mut self.right_owner);
        self.transfer_right(sel, widgets, state);
        std::mem::swap(&mut self.left_owner, &mut self.right_owner);
        // Re-populate both sides with correct owners
        self.populate_items_left(widgets, state);
        self.populate_items_right(widgets, state);
    }
}

impl Screen for ItemManagementScreen {
    fn id(&self) -> ScreenId {
        "item_management"
    }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/itemmanagement_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.owners_left_id = widgets.get_id("OwnersLeftList").unwrap_or(0);
        self.owners_right_id = widgets.get_id("OwnersRightList").unwrap_or(0);
        self.items_left_id = widgets.get_id("ItemsLeftList").unwrap_or(0);
        self.items_right_id = widgets.get_id("ItemsRightList").unwrap_or(0);
        self.filter_list_id = widgets.get_id("FilterList").unwrap_or(0);
        self.shift_right_id = widgets.get_id("ShiftRightButton").unwrap_or(0);
        self.shift_left_id = widgets.get_id("ShiftLeftButton").unwrap_or(0);
        self.equip_id = widgets.get_id("EquipLeftButton").unwrap_or(0);
        self.unequip_id = widgets.get_id("UnequipLeftButton").unwrap_or(0);
        self.back_id = widgets.get_id("BackButton").unwrap_or(0);
        self.desc_id = widgets.get_id("ItemDesc").unwrap_or(0);
        self.gold_id = widgets.get_id("PlayerGold").unwrap_or(0);

        self.populate_owners(widgets, state);
        self.populate_items_left(widgets, state);
        self.populate_items_right(widgets, state);
        self.update_gold(widgets, state);
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
            // Equip button (left pane girl)
            if let Some(Widget::Button(b)) = widgets.get(self.equip_id) {
                if b.base.is_over(x, y) {
                    if let Owner::Girl(gid) = &self.left_owner {
                        let gid = *gid;
                        if let Some(Widget::ListBox(lb)) = widgets.get(self.items_left_id) {
                            if let Some(slot) = lb.get_selected() {
                                let slot = slot as usize;
                                if let Some(girl) = state.girls.get_girl_mut(gid) {
                                    inventory::equip(girl, &state.items, slot, false);
                                    self.populate_items_left(widgets, state);
                                }
                            }
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Unequip button (left pane girl)
            if let Some(Widget::Button(b)) = widgets.get(self.unequip_id) {
                if b.base.is_over(x, y) {
                    if let Owner::Girl(gid) = &self.left_owner {
                        let gid = *gid;
                        if let Some(Widget::ListBox(lb)) = widgets.get(self.items_left_id) {
                            if let Some(slot) = lb.get_selected() {
                                let slot = slot as usize;
                                if let Some(girl) = state.girls.get_girl_mut(gid) {
                                    inventory::unequip(girl, &state.items, slot);
                                    self.populate_items_left(widgets, state);
                                }
                            }
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Transfer right (left → right)
            if let Some(Widget::Button(b)) = widgets.get(self.shift_right_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.items_left_id) {
                        if let Some(sel) = lb.get_selected() {
                            self.transfer_right(sel, widgets, state);
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Transfer left (right → left)
            if let Some(Widget::Button(b)) = widgets.get(self.shift_left_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.items_right_id) {
                        if let Some(sel) = lb.get_selected() {
                            self.transfer_left(sel, widgets, state);
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
                        self.left_owner = if sel == -1 {
                            Owner::Player
                        } else if sel == -2 {
                            Owner::Shop
                        } else {
                            Owner::Girl(sel as usize)
                        };
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
                        self.right_owner = if sel == -1 {
                            Owner::Player
                        } else if sel == -2 {
                            Owner::Shop
                        } else {
                            Owner::Girl(sel as usize)
                        };
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
                        let item_idx = match &self.left_owner {
                            Owner::Girl(gid) => {
                                state.girls.get_girl(*gid)
                                    .and_then(|g| g.inventory.get(sel as usize).copied())
                                    .unwrap_or(sel as usize)
                            }
                            _ => sel as usize,
                        };
                        self.update_desc(widgets, state, item_idx);
                    }
                    return ScreenAction::None;
                }
            }
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.items_right_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(sel) = lb.get_selected() {
                        let item_idx = match &self.right_owner {
                            Owner::Girl(gid) => {
                                state.girls.get_girl(*gid)
                                    .and_then(|g| g.inventory.get(sel as usize).copied())
                                    .unwrap_or(sel as usize)
                            }
                            _ => sel as usize,
                        };
                        self.update_desc(widgets, state, item_idx);
                    }
                    return ScreenAction::None;
                }
            }
        }
        ScreenAction::None
    }
}
