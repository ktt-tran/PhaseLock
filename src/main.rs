mod app;
mod gui;
mod audio;
mod crypto;
mod file;
mod worker;
mod utils;


fn main() -> eframe::Result<()> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };


    eframe::run_native(
        "PhaseLock",
        options,
        Box::new(|cc| {
            Ok(Box::new(app::PhaseLockApp::new(cc)))
        }),
    )
}