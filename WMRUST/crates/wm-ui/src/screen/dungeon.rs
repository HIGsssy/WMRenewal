use wm_core::enums::Stat;
use wm_game::girls::GirlManager;
use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetId, WidgetStore};
use crate::xml_loader::load_screen_xml;

#[derive(Debug)]
pub struct DungeonScreen {
    girl_list_id: WidgetId,
    details_id: WidgetId,
    torture_id: WidgetId,
    release_id: WidgetId,
    release_all_id: WidgetId,
    release_cust_id: WidgetId,
    brand_slave_id: WidgetId,
    sell_id: WidgetId,
    interact_id: WidgetId,
    feed_id: WidgetId,
    no_feed_id: WidgetId,
    back_id: WidgetId,
}

impl DungeonScreen {
    pub fn new() -> Self {
        Self {
            girl_list_id: 0,
            details_id: 0,
            torture_id: 0,
            release_id: 0,
            release_all_id: 0,
            release_cust_id: 0,
            brand_slave_id: 0,
            sell_id: 0,
            interact_id: 0,
            feed_id: 0,
            no_feed_id: 0,
            back_id: 0,
        }
    }

    fn populate_list(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.girl_list_id) {
            lb.clear();
            for (i, inmate) in state.dungeon.inmates.iter().enumerate() {
                let health = GirlManager::get_stat(&inmate.girl, Stat::Health);
                let obedience = GirlManager::get_stat(&inmate.girl, Stat::Obedience);
                let feeding = if inmate.fed { "Yes" } else { "No" };
                let reason = format!("{:?}", inmate.reason);
                let data = format!(
                    "{}|{}|{}|{}|{}|{}|{}",
                    inmate.girl.name,
                    health,
                    obedience,
                    inmate.weeks,
                    feeding,
                    if inmate.is_customer { "N/A" } else { "No" },
                    reason,
                );
                lb.add_element(i as i32, &data);
            }
        }
    }
}

impl Screen for DungeonScreen {
    fn id(&self) -> ScreenId {
        "dungeon"
    }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/dungeon_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.girl_list_id = widgets.get_id("GirlList").unwrap_or(0);
        self.details_id = widgets.get_id("DetailsButton").unwrap_or(0);
        self.torture_id = widgets.get_id("TortureButton").unwrap_or(0);
        self.release_id = widgets.get_id("ReleaseButton").unwrap_or(0);
        self.release_all_id = widgets.get_id("ReleaseAllButton").unwrap_or(0);
        self.release_cust_id = widgets.get_id("ReleaseCustButton").unwrap_or(0);
        self.brand_slave_id = widgets.get_id("BrandSlaveButton").unwrap_or(0);
        self.sell_id = widgets.get_id("SellButton").unwrap_or(0);
        self.interact_id = widgets.get_id("InteractButton").unwrap_or(0);
        self.feed_id = widgets.get_id("AllowFoodButton").unwrap_or(0);
        self.no_feed_id = widgets.get_id("StopFeedingButton").unwrap_or(0);
        self.back_id = widgets.get_id("BackButton").unwrap_or(0);

        self.populate_list(widgets, state);
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
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Pop;
                }
            }
            // Release selected inmate
            if let Some(Widget::Button(b)) = widgets.get(self.release_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.girl_list_id) {
                        if let Some(sel) = lb.get_selected() {
                            let _ = state.dungeon.release(sel as usize);
                            self.populate_list(widgets, state);
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Release all girls
            if let Some(Widget::Button(b)) = widgets.get(self.release_all_id) {
                if b.base.is_over(x, y) {
                    // Release in reverse to avoid index shifting issues
                    let girl_indices: Vec<usize> = state
                        .dungeon
                        .inmates
                        .iter()
                        .enumerate()
                        .filter(|(_, i)| !i.is_customer)
                        .map(|(idx, _)| idx)
                        .rev()
                        .collect();
                    for idx in girl_indices {
                        let _ = state.dungeon.release(idx);
                    }
                    self.populate_list(widgets, state);
                    return ScreenAction::None;
                }
            }
            // Release all customers
            if let Some(Widget::Button(b)) = widgets.get(self.release_cust_id) {
                if b.base.is_over(x, y) {
                    state.dungeon.inmates.retain(|i| !i.is_customer);
                    self.populate_list(widgets, state);
                    return ScreenAction::None;
                }
            }
            // Torture
            if let Some(Widget::Button(b)) = widgets.get(self.torture_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.girl_list_id) {
                        if let Some(sel) = lb.get_selected() {
                            let mut rng = rand::thread_rng();
                            let _ = state.dungeon.torture(sel as usize, 0, true, &mut rng);
                            self.populate_list(widgets, state);
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Toggle feeding
            if let Some(Widget::Button(b)) = widgets.get(self.feed_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.girl_list_id) {
                        if let Some(sel) = lb.get_selected() {
                            if let Some(inmate) = state.dungeon.inmates.get_mut(sel as usize) {
                                inmate.fed = true;
                            }
                            self.populate_list(widgets, state);
                        }
                    }
                    return ScreenAction::None;
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.no_feed_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.girl_list_id) {
                        if let Some(sel) = lb.get_selected() {
                            if let Some(inmate) = state.dungeon.inmates.get_mut(sel as usize) {
                                inmate.fed = false;
                            }
                            self.populate_list(widgets, state);
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Girl list click
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.girl_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                }
            }
        }
        ScreenAction::None
    }
}
