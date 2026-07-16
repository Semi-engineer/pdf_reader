/*!
PDF Viewer Component
Main PDF viewing area with scroll and annotations
*/

use crate::app::DocLensApp;
use crate::page_cache::CacheKey;
use eframe::egui;

pub struct PdfViewer {}

impl PdfViewer {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        if app.document.is_none() {
            ui.centered_and_justified(|ui| {
                ui.heading("No PDF loaded");
                ui.label("Click 'Open' to load a PDF file");
            });
            return;
        }
        
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Display current page
                self.show_page(ui, app, app.current_page);
            });
    }
    
    fn show_page(&mut self, ui: &mut egui::Ui, app: &DocLensApp, page: usize) {
        // Try to get from cache
        let cache_key = CacheKey::new(page, app.zoom_level, app.rotation);
        
        if let Some(image) = app.page_cache.get(&cache_key) {
            // Load texture
            let texture = ui.ctx().load_texture(
                format!("page_{}_{}_{}", page, app.zoom_level as u32, app.rotation),
                image.as_ref().clone(),
                Default::default(),
            );
            
            let size = egui::vec2(image.width() as f32, image.height() as f32);
            
            // Create frame for page
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let response = ui.add(
                    egui::Image::new(&texture)
                        .fit_to_exact_size(size)
                        .sense(egui::Sense::click_and_drag())
                );
                
                // Handle interactions
                if response.clicked() {
                    // Handle annotation tools
                    if let Some(_tool) = &app.current_tool {
                        // Add annotation at click position
                        // (Simplified - full implementation would track drag)
                    }
                }
                
                // Draw annotations
                self.draw_annotations(ui, app, page, &response);
                
                // Draw search results
                self.draw_search_results(ui, app, page, &response);
            });
        } else {
            // Show loading placeholder
            ui.centered_and_justified(|ui| {
                ui.spinner();
                ui.label(format!("Loading page {}...", page + 1));
            });
        }
    }
    
    fn draw_annotations(
        &self,
        ui: &mut egui::Ui,
        app: &DocLensApp,
        page: usize,
        response: &egui::Response,
    ) {
        let annotations = app.annotation_manager.get_page_annotations(page);
        
        for annotation in annotations {
            let rect = annotation.rect.to_egui();
            let color = egui::Color32::from_rgba_unmultiplied(
                annotation.color[0],
                annotation.color[1],
                annotation.color[2],
                annotation.color[3],
            );
            
            // Offset by response position
            let offset_rect = rect.translate(response.rect.min.to_vec2());
            
            match annotation.annotation_type {
                crate::annotation::AnnotationType::Highlight => {
                    ui.painter().rect_filled(offset_rect, 0.0, color);
                }
                crate::annotation::AnnotationType::Rectangle => {
                    ui.painter().rect_stroke(offset_rect, 0.0, egui::Stroke::new(2.0, color));
                }
                crate::annotation::AnnotationType::Circle => {
                    ui.painter().circle_stroke(
                        offset_rect.center(),
                        offset_rect.width().min(offset_rect.height()) / 2.0,
                        egui::Stroke::new(2.0, color),
                    );
                }
                _ => {
                    // Other types not fully implemented
                }
            }
        }
    }
    
    fn draw_search_results(
        &self,
        ui: &mut egui::Ui,
        app: &DocLensApp,
        page: usize,
        response: &egui::Response,
    ) {
        let results = app.search_manager.page_results(page);
        
        for result in results {
            let offset_rect = result.rect.translate(response.rect.min.to_vec2());
            let highlight_color = egui::Color32::from_rgba_unmultiplied(255, 255, 0, 100);
            ui.painter().rect_filled(offset_rect, 0.0, highlight_color);
        }
    }
}

impl Default for PdfViewer {
    fn default() -> Self {
        Self::new()
    }
}
