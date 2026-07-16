/*!
DocLens - A Feature-Rich PDF Viewer in Rust
Main entry point
*/

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod app;
mod config;
mod pdf_document;
mod page_cache;
mod render_worker;
mod thumbnail_manager;
mod annotation;
mod search;
mod ui;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(load_icon()),
        ..Default::default()
    };
    
    eframe::run_native(
        "DocLens",
        options,
        Box::new(|cc| Ok(Box::new(app::DocLensApp::new(cc)))),
    )
}

fn load_icon() -> egui::viewport::IconData {
    // Load icon from file
    let icon_path = std::path::Path::new("icon/icon.ico");
    
    if icon_path.exists() {
        if let Ok(icon_bytes) = std::fs::read(icon_path) {
            if let Ok(img) = image::load_from_memory(&icon_bytes) {
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                return egui::viewport::IconData {
                    rgba: rgba.into_raw(),
                    width,
                    height,
                };
            }
        }
    }
    
    // Default icon (empty)
    egui::viewport::IconData {
        rgba: vec![0; 32 * 32 * 4],
        width: 32,
        height: 32,
    }
}
