/*!
Toolbar Component
Main toolbar with file operations, navigation, and view controls
*/

use crate::app::DocLensApp;
use eframe::egui;

pub struct Toolbar {
    search_query: String,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            
            // File operations
            if ui.button("📁 Open").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("PDF Files", &["pdf"])
                    .pick_file()
                {
                    let _ = app.open_file(&path.to_string_lossy());
                }
            }
            
            ui.separator();
            
            // Navigation
            if ui.button("◀").clicked() {
                app.prev_page();
            }
            
            if ui.button("▶").clicked() {
                app.next_page();
            }
            
            ui.label("Page:");
            let mut page_input = app.current_page + 1;
            if ui.add(egui::DragValue::new(&mut page_input).range(1..=app.document.as_ref().map_or(1, |d| d.page_count()))).changed() {
                app.goto_page(page_input.saturating_sub(1));
            }
            
            if let Some(doc) = &app.document {
                ui.label(format!("/ {}", doc.page_count()));
            }
            
            ui.separator();
            
            // Zoom controls
            if ui.button("🔍-").clicked() {
                app.zoom_out();
            }
            
            if ui.button("🔍+").clicked() {
                app.zoom_in();
            }
            
            ui.label(format!("{:.0}%", app.zoom_level));
            
            ui.separator();
            
            // Rotation
            if ui.button("↶").clicked() {
                app.rotate_left();
            }
            
            if ui.button("↷").clicked() {
                app.rotate_right();
            }
            
            ui.separator();
            
            // Search
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
            
            if ui.button("🔍").clicked() && !self.search_query.is_empty() {
                // Trigger search
                app.search_manager.set_query(self.search_query.clone());
            }
            
            ui.separator();
            
            // View options
            if ui.button("📋 Sidebar").clicked() {
                app.sidebar_visible = !app.sidebar_visible;
            }
            
            if ui.button("🎨 Tools").clicked() {
                app.tool_palette_visible = !app.tool_palette_visible;
            }
        });
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}
