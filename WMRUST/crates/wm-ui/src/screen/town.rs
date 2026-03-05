use rand::Rng;
use wm_game::brothel::BROTHEL_PRICES;
use wm_game::state::GameState;
use wm_script::script_runner::ScriptRunner;

use crate::events::UiEvent;
use crate::screen::bank::BankScreen;
use crate::screen::dialog::DialogScreen;
use crate::screen::house::HouseScreen;
use crate::screen::item_management::ItemManagementScreen;
use crate::screen::mayor::MayorScreen;
use crate::screen::prison::PrisonScreen;
use crate::screen::slave_market::SlaveMarketScreen;
use crate::screen::text_input::TextInputScreen;
use crate::screen::walk_encounter::WalkEncounterScreen;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetId, WidgetStore};
use crate::xml_loader::load_screen_xml;

/// State for a pending brothel purchase.
#[derive(Debug, Clone, PartialEq, Eq)]
enum PurchaseState {
    /// No purchase in progress.
    None,
    /// Waiting for the TextInputScreen to return a name for this brothel index.
    NamingBrothel(usize),
}

#[derive(Debug)]
pub struct TownScreen {
    back_id: WidgetId,
    walk_id: WidgetId,
    slave_market_id: WidgetId,
    mayor_id: WidgetId,
    bank_id: WidgetId,
    house_id: WidgetId,
    prison_id: WidgetId,
    shop_id: WidgetId,
    brothel_ids: [WidgetId; 6],
    current_brothel_id: WidgetId,
    purchase_state: PurchaseState,
}

impl TownScreen {
    pub fn new() -> Self {
        Self {
            back_id: 0,
            walk_id: 0,
            slave_market_id: 0,
            mayor_id: 0,
            bank_id: 0,
            house_id: 0,
            prison_id: 0,
            shop_id: 0,
            brothel_ids: [0; 6],
            current_brothel_id: 0,
            purchase_state: PurchaseState::None,
        }
    }

    /// Try to create a script-based walk encounter using MeetTownDefault.
    fn try_script_walk(&self, state: &GameState) -> Option<DialogScreen> {
        let scripts = wm_core::resources_path().join("Scripts");

        // Try to load MeetTownDefault script
        let code = load_town_script(&scripts, "MeetTownDefault")?;

        // Roll for encounter and pick a girl
        let mut rng = rand::thread_rng();
        let meet_chance = state.config.initial.girl_meet;
        let roll = rng.gen_range(0..100);
        if roll >= meet_chance {
            return None; // No encounter — fall back to hardcoded
        }

        let total = state.girls.count();
        if total == 0 {
            return None;
        }

        let unassigned: Vec<usize> = (0..total)
            .filter(|&gid| state.brothels.find_girl_brothel(gid).is_none())
            .collect();
        if unassigned.is_empty() {
            return None;
        }

        let idx = rng.gen_range(0..unassigned.len());
        let girl_id = unassigned[idx];
        let girl = state.girls.get_girl(girl_id)?;

        let runner = ScriptRunner::new(&code).ok()?;
        {
            let mut ctx = runner.context().lock().unwrap();
            ctx.populate_from_girl(girl);
        }
        Some(DialogScreen::from_runner(runner).with_target_girl(girl_id))
    }

    /// Update brothel button visibility based on how many are owned.
    fn update_brothel_buttons(&self, widgets: &mut WidgetStore, state: &GameState) {
        let owned = state.brothels.num_brothels();
        for i in 0..6 {
            if let Some(w) = widgets.get_mut(self.brothel_ids[i]) {
                if i < owned {
                    // Owned — visible and clickable
                    w.base_mut().hidden = false;
                    w.base_mut().disabled = false;
                } else if i == owned && i < 6 {
                    // Next purchasable — visible but show as "buy" option
                    w.base_mut().hidden = false;
                    w.base_mut().disabled = false;
                } else {
                    // Not yet available — hidden
                    w.base_mut().hidden = true;
                }
            }
        }
    }
}

impl Screen for TownScreen {
    fn id(&self) -> ScreenId {
        "town"
    }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/town_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.back_id = widgets.get_id("BackButton").unwrap_or(0);
        self.walk_id = widgets.get_id("WalkButton").unwrap_or(0);
        self.slave_market_id = widgets.get_id("SlaveMarket").unwrap_or(0);
        self.mayor_id = widgets.get_id("MayorsOffice").unwrap_or(0);
        self.bank_id = widgets.get_id("Bank").unwrap_or(0);
        self.house_id = widgets.get_id("House").unwrap_or(0);
        self.prison_id = widgets.get_id("Prison").unwrap_or(0);
        self.shop_id = widgets.get_id("Shop").unwrap_or(0);
        for i in 0..6 {
            self.brothel_ids[i] = widgets.get_id(&format!("Brothel{}", i)).unwrap_or(0);
        }
        self.current_brothel_id = widgets.get_id("CurrentBrothel").unwrap_or(0);

        // Display current brothel name
        let name = state.brothels.current_brothel_name();
        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.current_brothel_id) {
            t.text = format!("Current: {}", name);
        }

        // Hide unowned brothels, disable walk if already used
        self.update_brothel_buttons(widgets, state);
        if state.walk_around {
            if let Some(w) = widgets.get_mut(self.walk_id) {
                w.base_mut().disabled = true;
            }
        }

        // Check if we're returning from a brothel naming TextInputScreen
        if let PurchaseState::NamingBrothel(idx) = self.purchase_state {
            if let Some(name) = state.pending_text_result.take() {
                let price = BROTHEL_PRICES.get(idx).copied().unwrap_or(0);
                state.gold.cash_on_hand -= price as f64;
                state.brothels.add_brothel(&name);
                state.brothels.set_current(idx);
                self.purchase_state = PurchaseState::None;
                // Refresh brothel button visibility
                self.update_brothel_buttons(widgets, state);
                if let Some(Widget::TextItem(t)) = widgets.get_mut(self.current_brothel_id) {
                    t.text = format!("Current: {}", name);
                }
            } else {
                // Cancelled
                self.purchase_state = PurchaseState::None;
            }
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
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Pop;
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.slave_market_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(SlaveMarketScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.bank_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(BankScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.mayor_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(MayorScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.house_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(HouseScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.prison_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(PrisonScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.shop_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Push(Box::new(ItemManagementScreen::new()));
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.walk_id) {
                if b.base.is_over(x, y) && !b.base.disabled {
                    if state.walk_around {
                        return ScreenAction::None;
                    }
                    // Try script-based walk encounter
                    if let Some(screen) = self.try_script_walk(state) {
                        state.walk_around = true;
                        return ScreenAction::Push(Box::new(screen));
                    }
                    // Fall back to hardcoded encounter
                    return ScreenAction::Push(Box::new(WalkEncounterScreen::new()));
                }
            }
            // Brothel selection / purchase
            let owned = state.brothels.num_brothels();
            for i in 0..6 {
                if let Some(Widget::Button(b)) = widgets.get(self.brothel_ids[i]) {
                    if b.base.is_over(x, y) {
                        if i < owned {
                            // Switch to owned brothel
                            state.brothels.set_current(i);
                            return ScreenAction::Pop;
                        } else if i == owned {
                            // Purchase attempt
                            let price = BROTHEL_PRICES.get(i).copied().unwrap_or(0);
                            if state.gold.cash_on_hand >= price as f64 {
                                self.purchase_state = PurchaseState::NamingBrothel(i);
                                return ScreenAction::Push(Box::new(TextInputScreen::new(
                                    &format!(
                                        "Name your new brothel (cost: {} gold):",
                                        price
                                    ),
                                    "",
                                )));
                            }
                            // Not enough gold — do nothing
                        }
                    }
                }
            }
        }
        ScreenAction::None
    }
}

/// Load a script by name — tries .lua first, then converts .script.
fn load_town_script(scripts_dir: &std::path::Path, name: &str) -> Option<String> {
    let lua_path = scripts_dir.join(format!("{}.lua", name));
    if lua_path.exists() {
        return std::fs::read_to_string(&lua_path).ok();
    }
    let script_path = scripts_dir.join(format!("{}.script", name));
    if script_path.exists() {
        return wm_script::script_converter::convert_script_to_lua(&script_path).ok();
    }
    None
}
