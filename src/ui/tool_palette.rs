/*!
Tool Palette Component
Floating palette for annotation tools.
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
        // Color picker
        ui.horizontal(|ui| {
            ui.label("Color:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut app.current_color,
                egui::color_picker::Alpha::OnlyBlend,
            );
        });

        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);

        // Tool buttons
        let tools: &[(&str, AnnotationType, &str)] = &[
            ("✏ Highlight",   AnnotationType::Highlight,  "Drag to highlight text"),
            ("▭ Rectangle",  AnnotationType::Rectangle,  "Drag to draw rectangle"),
            ("◯ Circle",     AnnotationType::Circle,     "Drag to draw ellipse"),
            ("─ Line",       AnnotationType::Line,       "Click to add line"),
            ("→ Arrow",      AnnotationType::Arrow,      "Click to add arrow"),
            ("🖊 Pen",        AnnotationType::Pen,        "Drag to draw freehand"),
            ("T Text",       AnnotationType::Text,       "Click to add text"),
        ];

        for (label, tool_type, hint) in tools {
            let is_selected = app.current_tool.as_ref() == Some(tool_type);

            let button = if is_selected {
                egui::Button::new(
                    egui::RichText::new(*label).color(egui::Color32::WHITE),
                )
                .fill(egui::Color32::from_rgb(70, 130, 200))
            } else {
                egui::Button::new(*label)
            };

            let response = ui
                .add_sized([130.0, 28.0], button)
                .on_hover_text(*hint);

            if response.clicked() {
                if is_selected {
                    app.current_tool = None;
                } else {
                    app.current_tool = Some(tool_type.clone());
                }
            }
        }

        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);

        if ui.button("🗑 Clear page annotations").clicked() {
            app.annotation_manager.clear_page(app.current_page);
        }

        if ui.button("🗑 Clear all annotations").clicked() {
            app.annotation_manager.clear();
        }

        // Deselect tool hint
        if app.current_tool.is_some() {
            ui.add_space(4.0);
            ui.weak("Click active tool to deselect");
        }
    }
}

impl Default for ToolPalette {
    fn default() -> Self {
        Self::new()
    }
}
