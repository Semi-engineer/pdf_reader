/*!
Tool Palette — vertical ribbon embedded in a side panel.
*/

use super::theme::{BG_ACTIVE, BG_ELEVATED, BG_SURFACE, BORDER, FG_PRIMARY, FG_SECONDARY};
use crate::annotation::AnnotationType;
use crate::app::DocLensApp;
use eframe::egui::{self, Color32, RichText, Stroke, Vec2};

const TOOL_BTN_SIZE: Vec2 = Vec2::new(108.0, 30.0);

pub struct ToolPalette {}

impl ToolPalette {
    pub fn new() -> Self { Self {} }

    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        egui::Frame::new()
            .fill(BG_SURFACE)
            .inner_margin(egui::Margin { left: 6, right: 6, top: 8, bottom: 8 })
            .show(ui, |ui| {
                ui.set_min_width(120.0);

                // ── Section: Mode ─────────────────────────────────────────
                section_header(ui, "MODE");

                // Select / text-selection (default when no tool)
                let sel_active = app.current_tool.is_none();
                if tool_btn(ui, "↖ Select", "Text selection mode", sel_active).clicked() {
                    app.current_tool = None;
                }

                ui.add_space(6.0);
                ui.add(egui::Separator::default().spacing(4.0));
                ui.add_space(6.0);

                // ── Section: Annotate ─────────────────────────────────────
                section_header(ui, "ANNOTATE");

                let tools: &[(&str, AnnotationType, &str)] = &[
                    ("🖍  Highlight",  AnnotationType::Highlight,  "Drag to highlight"),
                    ("▭  Rectangle",  AnnotationType::Rectangle,  "Drag to draw rectangle"),
                    ("○  Circle",     AnnotationType::Circle,     "Drag to draw ellipse"),
                    ("╱  Line",       AnnotationType::Line,       "Drag to draw line"),
                    ("➜  Arrow",      AnnotationType::Arrow,      "Drag to draw arrow"),
                    ("✎  Pen",        AnnotationType::Pen,        "Freehand draw"),
                    ("📝  Text",       AnnotationType::Text,       "Click to add text note"),
                ];

                for (label, tool_type, hint) in tools {
                    let active = app.current_tool.as_ref() == Some(tool_type);
                    if tool_btn(ui, label, hint, active).clicked() {
                        app.current_tool = if active { None } else { Some(tool_type.clone()) };
                    }
                }

                ui.add_space(6.0);
                ui.add(egui::Separator::default().spacing(4.0));
                ui.add_space(6.0);

                // ── Section: Color ────────────────────────────────────────
                section_header(ui, "COLOR");

                ui.horizontal(|ui| {
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut app.current_color,
                        egui::color_picker::Alpha::OnlyBlend,
                    );
                    ui.label(
                        RichText::new("Annotation color").size(11.5).color(FG_SECONDARY)
                    );
                });

                // Quick colour presets
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::splat(4.0);
                    let presets: &[Color32] = &[
                        Color32::from_rgba_unmultiplied(255, 220,  50, 130), // yellow
                        Color32::from_rgba_unmultiplied( 80, 200, 120, 130), // green
                        Color32::from_rgba_unmultiplied(100, 170, 255, 130), // blue
                        Color32::from_rgba_unmultiplied(255,  90,  80, 130), // red
                        Color32::from_rgba_unmultiplied(200, 100, 255, 130), // purple
                        Color32::from_rgba_unmultiplied(255, 160,  40, 130), // orange
                    ];
                    for &color in presets {
                        let selected = app.current_color == color;
                        let stroke = if selected {
                            Stroke::new(2.0, Color32::WHITE)
                        } else {
                            Stroke::new(1.0, BORDER)
                        };
                        let (rect, resp) = ui.allocate_exact_size(
                            Vec2::splat(18.0), egui::Sense::click()
                        );
                        ui.painter().rect_filled(rect, 3.0, color);
                        ui.painter().rect_stroke(rect, 3.0, stroke, egui::StrokeKind::Outside);
                        if resp.clicked() { app.current_color = color; }
                    }
                });

                ui.add_space(6.0);
                ui.add(egui::Separator::default().spacing(4.0));
                ui.add_space(6.0);

                // ── Section: Clear ────────────────────────────────────────
                section_header(ui, "CLEAR");

                let del_page = egui::Button::new(
                    RichText::new("x  This page").size(12.5).color(FG_SECONDARY)
                )
                .min_size(TOOL_BTN_SIZE)
                .fill(BG_ELEVATED);
                if ui.add(del_page).clicked() {
                    app.annotation_manager.clear_page(app.current_page);
                }

                let del_all = egui::Button::new(
                    RichText::new("x  All pages").size(12.5).color(FG_SECONDARY)
                )
                .min_size(TOOL_BTN_SIZE)
                .fill(BG_ELEVATED);
                if ui.add(del_all).clicked() {
                    app.annotation_manager.clear();
                }

                // Active tool hint at bottom
                if let Some(tool) = &app.current_tool {
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(format!("Active: {:?}", tool))
                            .size(11.0).color(FG_SECONDARY).italics()
                    );
                    ui.label(
                        RichText::new("Click tool again to deselect")
                            .size(10.5).color(FG_SECONDARY)
                    );
                }
            });
    }
}

fn section_header(ui: &mut egui::Ui, label: &str) {
    ui.label(
        RichText::new(label)
            .size(10.0)
            .color(FG_SECONDARY)
            .strong()
    );
    ui.add_space(3.0);
}

fn tool_btn(ui: &mut egui::Ui, label: &str, hint: &str, active: bool) -> egui::Response {
    let text = if active {
        RichText::new(label).size(13.0).color(Color32::WHITE)
    } else {
        RichText::new(label).size(13.0).color(FG_PRIMARY)
    };
    let btn = egui::Button::new(text)
        .min_size(TOOL_BTN_SIZE)
        .corner_radius(egui::CornerRadius::same(4))
        .fill(if active { BG_ACTIVE } else { Color32::TRANSPARENT })
        .stroke(Stroke::NONE);
    ui.add(btn).on_hover_text(hint)
}

impl Default for ToolPalette {
    fn default() -> Self { Self::new() }
}
