#![cfg(windows)]

mod app;
mod ui;
mod file_system;
mod markdown;

use app::TreeMapMD;
use eframe::IconData;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions {  // Add 'mut' here
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        min_window_size: Some(egui::vec2(400.0, 300.0)),
        resizable: true,
        ..Default::default()
    };

    // Load the icon
    let icon_path = "C:\\Users\\Owner\\Documents\\Github_Repositories\\treemap_md\\treeMD-icon.png";
    if let Ok(mut file) = File::open(icon_path) {
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_ok() {
            options.icon_data = Some(IconData {
                rgba: buffer,
                width: 1024,  // Set this to the actual width of your icon
                height: 1024, // Set this to the actual height of your icon
            });
        }
    }

    eframe::run_native(
        "TreeMapMD",
        options,
        Box::new(|_cc| Box::new(TreeMapMD::default())),
    )
}