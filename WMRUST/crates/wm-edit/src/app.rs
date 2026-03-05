/// Placeholder editor tab selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Girls,
    Items,
    Rooms,
    Traits,
    Config,
}

pub struct EditorApp {
    tab: Tab,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self { tab: Tab::Girls }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, Tab::Girls, "Girls");
                ui.selectable_value(&mut self.tab, Tab::Items, "Items");
                ui.selectable_value(&mut self.tab, Tab::Rooms, "Rooms");
                ui.selectable_value(&mut self.tab, Tab::Traits, "Traits");
                ui.selectable_value(&mut self.tab, Tab::Config, "Config");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab {
                Tab::Girls => ui.label("Girls editor — not yet implemented"),
                Tab::Items => ui.label("Items editor — not yet implemented"),
                Tab::Rooms => ui.label("Rooms editor — not yet implemented"),
                Tab::Traits => ui.label("Traits editor — not yet implemented"),
                Tab::Config => ui.label("Config editor — not yet implemented"),
            };
        });
    }
}
