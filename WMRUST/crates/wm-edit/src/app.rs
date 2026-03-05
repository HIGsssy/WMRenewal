use crate::girls_tab::GirlsTab;
use crate::items_tab::ItemsTab;
use crate::traits_tab::TraitsTab;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Girls,
    Items,
    Traits,
}

pub struct EditorApp {
    tab: Tab,
    girls_tab: GirlsTab,
    items_tab: ItemsTab,
    traits_tab: TraitsTab,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            tab: Tab::Girls,
            girls_tab: GirlsTab::default(),
            items_tab: ItemsTab::default(),
            traits_tab: TraitsTab::default(),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("WhoreMaster Editor");
                ui.separator();
                ui.selectable_value(&mut self.tab, Tab::Girls, "Girls");
                ui.selectable_value(&mut self.tab, Tab::Items, "Items");
                ui.selectable_value(&mut self.tab, Tab::Traits, "Traits");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab {
                Tab::Girls => self.girls_tab.show(ui),
                Tab::Items => self.items_tab.show(ui),
                Tab::Traits => self.traits_tab.show(ui),
            }
        });
    }
}
