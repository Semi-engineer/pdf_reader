/*!
Status Bar Component
Bottom status bar showing document info
*/

use crate::app::DocLensApp;
use eframe::egui;

pub struct StatusBar {}

impl StatusBar {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, app: &DocLensApp) {
        ui.horizontal(|ui| {
            // Document info
            if let Some(doc_path) = &app.doc_path {
                let filename = std::path::Path::new(doc_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown");
                ui.label(format!("📄 {}", filename));
            } else {
                ui.label("No document");
            }
            
            ui.separator();
            
            // Page info
            if let Some(doc) = &app.document {
                ui.label(format!(
                    "Page {} / {}",
                    app.current_page + 1,
                    doc.page_count()
                ));
            }
            
            ui.separator();
            
            // Zoom level
            ui.label(format!("🔍 {:.0}%", app.zoom_level));
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Cache info
                ui.label(format!("Cache: {} pages", app.page_cache.len()));
            });
        });
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}
