/*!
Tool Palette Component
Floating tool palette for annotations
*/

use crate::annotation::AnnotationType;
use crate::app::DocLensApp;
use eframe::egui;

pub struct ToolPalette {}

impl ToolPalette {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        ui.heading("Annotation Tools");
        ui.separator();
        
        // Color picker
        ui.horizontal(|ui| {
            ui.label("Color:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut app.current_color,
                egui::color_picker::Alpha::OnlyBlend,
            );
        });
        
        ui.separator();
        
        // Tool buttons
        ui.vertical(|ui| {
            let tools = vec![
                (AnnotationType::Highlight, "✏ Highlight"),
                (AnnotationType::Rectangle, "▭ Rectangle"),
                (AnnotationType::Circle, "◯ Circle"),
                (AnnotationType::Line, "─ Line"),
                (AnnotationType::Arrow, "→ Arrow"),
                (AnnotationType::Pen, "✎ Pen"),
                (AnnotationType::Text, "T Text"),
            ];
            
            for (tool_type, label) in tools {
                let is_selected = app.current_tool.as_ref() == Some(&tool_type);
                
                let button = if is_selected {
                    egui::Button::new(label).fill(egui::Color32::from_rgb(200, 220, 255))
                } else {
                    egui::Button::new(label)
                };
                
                if ui.add_sized([120.0, 30.0], button).clicked() {
                    if is_selected {
                        app.current_tool = None;
                    } else {
                        app.current_tool = Some(tool_type);
                    }
                }
            }
        });
        
        ui.separator();
        
        // Clear button
        if ui.button("🗑 Clear All").clicked() {
            app.annotation_manager.clear();
        }
    }
}

impl Default for ToolPalette {
    fn default() -> Self {
        Self::new()
    }
}
