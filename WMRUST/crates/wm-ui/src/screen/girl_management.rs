use wm_core::enums::{JobType, Stat};
use wm_game::girls::GirlManager;
use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::girl_details::GirlDetailsScreen;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetId, WidgetStore};
use crate::xml_loader::load_screen_xml;

fn job_from_id(id: i32) -> Option<JobType> {
    match id {
        0 => Some(JobType::Resting),
        1 => Some(JobType::Training),
        2 => Some(JobType::Cleaning),
        3 => Some(JobType::Security),
        4 => Some(JobType::Advertising),
        5 => Some(JobType::Matron),
        6 => Some(JobType::Torturer),
        7 => Some(JobType::ExploreCatacombs),
        8 => Some(JobType::BeastCapture),
        9 => Some(JobType::BeastCarer),
        10 => Some(JobType::WhoreBrothel),
        11 => Some(JobType::WhoreStreets),
        12 => Some(JobType::BrothelStripper),
        13 => Some(JobType::Masseuse),
        14 => Some(JobType::CustomerService),
        15 => Some(JobType::WhoreGambHall),
        16 => Some(JobType::Dealer),
        17 => Some(JobType::Entertainment),
        18 => Some(JobType::XXXEntertainment),
        19 => Some(JobType::Barmaid),
        20 => Some(JobType::Waitress),
        21 => Some(JobType::Stripper),
        22 => Some(JobType::WhoreBar),
        23 => Some(JobType::Singer),
        _ => None,
    }
}

#[derive(Debug)]
pub struct GirlManagementScreen {
    girl_list_id: WidgetId,
    job_type_list_id: WidgetId,
    job_list_id: WidgetId,
    view_details_id: WidgetId,
    transfer_id: WidgetId,
    fire_id: WidgetId,
    back_id: WidgetId,
    day_btn_id: WidgetId,
    night_btn_id: WidgetId,
    girl_desc_id: WidgetId,
    current_brothel_id: WidgetId,
    is_day_shift: bool,
    selected_girl: Option<usize>,
}

impl GirlManagementScreen {
    pub fn new() -> Self {
        Self {
            girl_list_id: 0,
            job_type_list_id: 0,
            job_list_id: 0,
            view_details_id: 0,
            transfer_id: 0,
            fire_id: 0,
            back_id: 0,
            day_btn_id: 0,
            night_btn_id: 0,
            girl_desc_id: 0,
            current_brothel_id: 0,
            is_day_shift: true,
            selected_girl: None,
        }
    }

    fn populate_girl_list(&self, widgets: &mut WidgetStore, state: &GameState) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.girl_list_id) {
            lb.clear();
            let brothel = state.brothels.current_brothel();
            for (_i, &girl_id) in brothel.girls.iter().enumerate() {
                if let Some(girl) = state.girls.get_girl(girl_id) {
                    let job_day = girl
                        .job_day
                        .map(|j| format!("{:?}", j))
                        .unwrap_or_else(|| "None".into());
                    let job_night = girl
                        .job_night
                        .map(|j| format!("{:?}", j))
                        .unwrap_or_else(|| "None".into());
                    let data = format!(
                        "{}|{}|{}|{}|{}|{}|{}",
                        girl.name,
                        GirlManager::get_stat(girl, Stat::Age),
                        GirlManager::get_stat(girl, Stat::Health),
                        GirlManager::get_stat(girl, Stat::Happiness),
                        GirlManager::get_stat(girl, Stat::Tiredness),
                        job_day,
                        job_night,
                    );
                    lb.add_element(girl_id as i32, &data);
                }
            }
        }
    }

    fn populate_job_types(&self, widgets: &mut WidgetStore) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.job_type_list_id) {
            lb.clear();
            let categories = [
                (0, "General"),
                (1, "Brothel"),
                (2, "Gambling Hall"),
                (3, "Bar"),
                (4, "Movie Studio"),
                (5, "Community Centre"),
                (6, "Drug Lab"),
                (7, "Alchemist Lab"),
                (8, "Arena"),
            ];
            for (id, name) in categories {
                lb.add_element(id, name);
            }
        }
    }

    fn populate_jobs_for_category(&self, widgets: &mut WidgetStore, category: i32) {
        if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.job_list_id) {
            lb.clear();
            let jobs: Vec<(i32, &str)> = match category {
                0 => vec![
                    (0, "Resting"),
                    (1, "Training"),
                    (2, "Cleaning"),
                    (3, "Security"),
                    (4, "Advertising"),
                    (5, "Matron"),
                    (6, "Torturer"),
                    (7, "ExploreCatacombs"),
                    (8, "BeastCapture"),
                    (9, "BeastCarer"),
                ],
                1 => vec![
                    (10, "WhoreBrothel"),
                    (11, "WhoreStreets"),
                    (12, "Stripper"),
                    (13, "Masseuse"),
                ],
                2 => vec![
                    (14, "CustomerService"),
                    (15, "WhoreGambHall"),
                    (16, "Dealer"),
                    (17, "Entertainment"),
                    (18, "XXXEntertainment"),
                ],
                3 => vec![
                    (19, "Barmaid"),
                    (20, "Waitress"),
                    (21, "Stripper"),
                    (22, "WhoreBar"),
                    (23, "Singer"),
                ],
                _ => vec![],
            };
            for (id, name) in jobs {
                lb.add_element(id, name);
            }
        }
    }
}

impl Screen for GirlManagementScreen {
    fn id(&self) -> ScreenId {
        "girl_management"
    }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        let path = wm_core::resources_path().join("Interface/girl_management_screen.xml");
        let _ = load_screen_xml(&path, widgets);

        self.girl_list_id = widgets.get_id("GirlList").unwrap_or(0);
        self.job_type_list_id = widgets.get_id("JobTypeList").unwrap_or(0);
        self.job_list_id = widgets.get_id("JobList").unwrap_or(0);
        self.view_details_id = widgets.get_id("ViewDetailsButton").unwrap_or(0);
        self.transfer_id = widgets.get_id("TransferButton").unwrap_or(0);
        self.fire_id = widgets.get_id("FireButton").unwrap_or(0);
        self.back_id = widgets.get_id("BackButton").unwrap_or(0);
        self.day_btn_id = widgets.get_id("DayButton").unwrap_or(0);
        self.night_btn_id = widgets.get_id("NightButton").unwrap_or(0);
        self.girl_desc_id = widgets.get_id("GirlDescription").unwrap_or(0);
        self.current_brothel_id = widgets.get_id("CurrentBrothel").unwrap_or(0);

        if let Some(Widget::TextItem(t)) = widgets.get_mut(self.current_brothel_id) {
            t.text = state.brothels.current_brothel_name().to_string();
        }

        self.populate_girl_list(widgets, state);
        self.populate_job_types(widgets);
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
            // Back button
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) {
                    return ScreenAction::Pop;
                }
            }
            // View details
            if let Some(Widget::Button(b)) = widgets.get(self.view_details_id) {
                if b.base.is_over(x, y) {
                    if let Some(gid) = self.selected_girl {
                        return ScreenAction::Push(Box::new(GirlDetailsScreen::with_girl(gid)));
                    }
                }
            }
            // Fire girl
            if let Some(Widget::Button(b)) = widgets.get(self.fire_id) {
                if b.base.is_over(x, y) {
                    if let Some(gid) = self.selected_girl {
                        let cur = state.brothels.current_index();
                        state.brothels.unassign_girl(cur, gid);
                        self.selected_girl = None;
                        self.populate_girl_list(widgets, state);
                    }
                    return ScreenAction::None;
                }
            }
            // Day/Night toggle
            if let Some(Widget::Button(b)) = widgets.get(self.day_btn_id) {
                if b.base.is_over(x, y) {
                    self.is_day_shift = true;
                }
            }
            if let Some(Widget::Button(b)) = widgets.get(self.night_btn_id) {
                if b.base.is_over(x, y) {
                    self.is_day_shift = false;
                }
            }
            // Girl list click
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.girl_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(sel) = lb.get_selected() {
                        self.selected_girl = Some(sel as usize);
                        // Update description
                        if let Some(girl) = state.girls.get_girl(sel as usize) {
                            let desc = format!(
                                "{}\nAge: {}\nHealth: {}\nHappiness: {}",
                                girl.name,
                                GirlManager::get_stat(girl, Stat::Age),
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
            // Job type list click
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.job_type_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(cat) = lb.get_selected() {
                        self.populate_jobs_for_category(widgets, cat);
                    }
                    return ScreenAction::None;
                }
            }
            // Job list click — assign job to selected girl
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.job_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                    if let Some(job_id) = lb.get_selected() {
                        if let Some(gid) = self.selected_girl {
                            if let Some(girl) = state.girls.get_girl_mut(gid) {
                                if let Some(job) = job_from_id(job_id) {
                                    if self.is_day_shift {
                                        girl.job_day = Some(job);
                                    } else {
                                        girl.job_night = Some(job);
                                    }
                                    self.populate_girl_list(widgets, state);
                                }
                            }
                        }
                    }
                    return ScreenAction::None;
                }
            }
        }
        ScreenAction::None
    }
}
