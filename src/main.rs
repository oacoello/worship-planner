#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod db;
mod export;
mod i18n;
mod models;

use app::GrancoPlannerApp;
use export::APP_ICON_BYTES;
use std::sync::Arc;

fn load_window_icon() -> Option<Arc<egui::IconData>> {
    let img = image::load_from_memory(APP_ICON_BYTES).ok()?.to_rgba8();
    let (width, height) = img.dimensions();
    Some(Arc::new(egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    }))
}

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Granco Planner")
        .with_inner_size([1200.0, 800.0])
        .with_min_inner_size([800.0, 600.0]);

    if let Some(icon) = load_window_icon() {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Granco Planner",
        native_options,
        Box::new(|cc| Ok(Box::new(GrancoPlannerApp::new(cc)))),
    )
}
