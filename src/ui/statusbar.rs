/*!
Status Bar Component
Bottom status bar showing document info and messages.
*/

use crate::app::DocLensApp;
use eframe::egui;

pub struct StatusBar {}

impl StatusBar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        ui.horizontal(|ui| {
            // Left side: document path
            if let Some(path) = &app.doc_path {
                let filename = std::path::Path::new(path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(path.as_str());
                ui.label(format!("📄 {filename}"));
            } else {
                ui.label("No document");
            }

            ui.separator();

            // Page info
            if let Some(doc) = &app.document {
                ui.label(format!("Page {} / {}", app.current_page + 1, doc.page_count()));
                ui.separator();
            }

            // Zoom
            ui.label(format!("🔍 {:.0}%", app.zoom_level));

            // Rotation badge
            if app.rotation != 0 {
                ui.separator();
                ui.label(format!("↷ {}°", app.rotation));
            }

            // Active tool
            if let Some(tool) = &app.current_tool {
                ui.separator();
                ui.label(format!("Tool: {:?}", tool));
            }

            // Search result count
            let sc = app.search_manager.result_count();
            if sc > 0 {
                ui.separator();
                ui.label(format!(
                    "🔍 {} / {}",
                    app.search_manager.current_index() + 1,
                    sc
                ));
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Cache info (dev hint)
                ui.weak(format!("cache: {}", app.page_cache.len()));

                // Status / error message (shown briefly)
                if let Some(msg) = &app.status_message {
                    ui.separator();
                    ui.label(
                        egui::RichText::new(msg)
                            .color(egui::Color32::from_rgb(30, 140, 60)),
                    );
                }
            });
        });
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}
