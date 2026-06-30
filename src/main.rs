mod platform_layer;
mod ui;

use eframe::egui;
use ui::AppState;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "applauncher",
        options,
        Box::new(|_cc| Ok(Box::new(AppState::default()))),
    )
}
