use wm_core::enums::Stat;
use wm_game::girls::GirlManager;
use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetId, WidgetStore};
use crate::xml_loader::load_screen_xml;

/// Number of girls available for purchase in the slave market per visit.
const MARKET_SIZE: usize = 10;

#[derive(Debug)]
pub struct SlaveMarketScreen {
    back_id: WidgetId,
    buy_id: WidgetId,
    show_more_id: WidgetId,
    slave_list_id: WidgetId,
    trait_list_id: WidgetId,
    details_id: WidgetId,
    trait_desc_id: WidgetId,
    current_brothel_id: WidgetId,
    available: Vec<usize>, // girl IDs available for purchase
}

impl SlaveMarketScreen {
    pub fn new() -> Self {
        Self {
            back_id: 0,
            buy_id: 0,
            show_more_id: 0,
            slave_list_id: 0,
            trait_list_id: 0,
            details_id: 0,
            trait_desc_id: 0,
            current_brothel_id: 0,
            available: Vec::new(),
        }
    }

    fn populate_list(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.slave_list_id) {
            lb.clear();
            for &gid in &self.available {
                if let Some(girl) = state.girls.get_girl(gid) {
                    lb.add_element(gid as i32, &girl.name);
                }
            }
        }
    }

    fn show_girl_details(&self, widgets: &mut WidgetStore, state: &GameState, girl_id: usize) {
        if let Some(girl) = state.girls.get_girl(girl_id) {
            let details = format!(
                "Name: {}\nAge: {}\nHealth: {}\nHappiness: {}\nLooks: {}\nCharisma: {}\n\nPrice: {} gold",
                girl.name,
                GirlManager::get_stat(girl, Stat::Age),
                GirlManager::get_stat(girl, Stat::Health),
                GirlManager::get_stat(girl, Stat::Happiness),
                GirlManager::get_stat(girl, Stat::Beauty),
                GirlManager::get_stat(girl, Stat::Charisma),
                GirlManager::get_stat(girl, Stat::AskPrice),
            );
            if let Some(Widget::TextItem(t)) = widgets.get_mut(self.details_id) {
                t.text = details;
            }
            // Show traits
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.trait_list_id) {
                lb.clear();
                for (i, tr) in girl.traits.iter().enumerate() {
                    lb.add_element(i as i32, tr);
                }
            }
        }
    }
}

impl Screen for SlaveMarketScreen {
    fn id(&self) -> ScreenId {
        "slave_market"
    }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/slavemarket_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.back_id = widgets.get_id("BackButton").unwrap_or(0);
        self.buy_id = widgets.get_id("BuySlaveButton").unwrap_or(0);
        self.show_more_id = widgets.get_id("ShowMoreButton").unwrap_or(0);
        self.slave_list_id = widgets.get_id("SlaveList").unwrap_or(0);
        self.trait_list_id = widgets.get_id("TraitList").unwrap_or(0);
        self.details_id = widgets.get_id("SlaveDetails").unwrap_or(0);
        self.trait_desc_id = widgets.get_id("TraitDesc").unwrap_or(0);
        self.current_brothel_id = widgets.get_id("CurrentBrothel").unwrap_or(0);

        // Populate available girls (take unassigned ones)
        self.available.clear();
        let total = state.girls.count();
        for gid in 0..total {
            if state.brothels.find_girl_brothel(gid).is_none() && self.available.len() < MARKET_SIZE
            {
                self.available.push(gid);
            }
        }

        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.current_brothel_id) {
            t.text = state.brothels.current_brothel_name().to_string();
        }
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
            // Buy slave
            if let Some(Widget::Button(b)) = widgets.get(self.buy_id) {
                if b.base.is_over(x, y) {
                    if let Some(Widget::ListBox(lb)) = widgets.get(self.slave_list_id) {
                        if let Some(sel) = lb.get_selected() {
                            let gid = sel as usize;
                            if let Some(girl) = state.girls.get_girl(gid) {
                                let price = GirlManager::get_stat(girl, Stat::AskPrice) as f64;
                                if state.gold.cash_on_hand >= price {
                                    state.gold.cash_on_hand -= price;
                                    let cur = state.brothels.current_index();
                                    state.brothels.assign_girl(cur, gid);
                                    self.available.retain(|&id| id != gid);
                                    self.populate_list(widgets, state);
                                }
                            }
                        }
                    }
                    return ScreenAction::None;
                }
            }
            // Slave list click
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.slave_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(sel) = lb.get_selected() {
                        self.show_girl_details(widgets, state, sel as usize);
                    }
                    return ScreenAction::None;
                }
            }
        }
        ScreenAction::None
    }
}
