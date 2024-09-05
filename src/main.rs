#![cfg(windows)]

mod app;
mod ui;
mod file_system;
mod markdown;

use app::TreeMapMD;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        min_window_size: Some(egui::vec2(400.0, 300.0)),
        resizable: true,
        ..Default::default()
    };

    eframe::run_native(
        "TreeMapMD",
        options,
        Box::new(|_cc| Box::new(TreeMapMD::default())),
    )
}