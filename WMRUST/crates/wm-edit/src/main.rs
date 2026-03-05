mod app;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "WhoreMaster Editor",
        options,
        Box::new(|_cc| Ok(Box::new(app::EditorApp::default()))),
    )
}
