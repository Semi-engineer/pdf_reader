/*!
Sidebar Component
Thumbnail sidebar for page navigation
*/

use crate::app::DocLensApp;
use eframe::egui;

pub struct Sidebar {}

impl Sidebar {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        ui.heading("Pages");
        ui.separator();
        
        if let Some(doc) = &app.document {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for page in 0..doc.page_count() {
                    let is_current = page == app.current_page;
                    
                    ui.group(|ui| {
                        if is_current {
                            ui.visuals_mut().widgets.noninteractive.bg_fill = 
                                egui::Color32::from_rgb(200, 220, 255);
                        }
                        
                        // Try to get thumbnail
                        if let Some(thumbnail) = app.thumbnail_manager.get_thumbnail(page) {
                            let texture = ui.ctx().load_texture(
                                format!("thumb_{}", page),
                                thumbnail.as_ref().clone(),
                                Default::default(),
                            );
                            
                            let size = egui::vec2(120.0, 160.0);
                            let response = ui.add(
                                egui::Image::new(&texture)
                                    .fit_to_exact_size(size)
                                    .sense(egui::Sense::click())
                            );
                            
                            if response.clicked() {
                                app.goto_page(page);
                            }
                        } else {
                            // Placeholder
                            let response = ui.button(format!("Page {}", page + 1));
                            if response.clicked() {
                                app.goto_page(page);
                            }
                        }
                        
                        ui.label(format!("Page {}", page + 1));
                    });
                    
                    ui.add_space(5.0);
                }
            });
        } else {
            ui.label("No document open");
        }
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}
