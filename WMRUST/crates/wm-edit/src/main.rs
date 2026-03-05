mod app;
mod girls_tab;
mod items_tab;
mod traits_tab;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "WhoreMaster Editor",
        options,
        Box::new(|_cc| Ok(Box::new(app::EditorApp::default()))),
    )
}
