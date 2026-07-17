/*!
Toolbar — Modern, compact, icon-first top bar
Reorganized into logical groups: File | Navigation | View | Annotation | Search
Annotation tools integrated from the former ToolPalette.
Uses Design System tokens and shared icon_button component.
*/

use crate::ui::theme::*;
use crate::ui::icons;
use crate::ui::components::{icon_button, toolbar_divider};
use crate::annotation::AnnotationType;
use crate::app::DocLensApp;
use eframe::egui::{self, RichText, Stroke};

pub struct Toolbar {
    search_query: String,
    search_focused: bool,
}

impl Toolbar {
    pub fn new() -> Self {
        Self { search_query: String::new(), search_focused: false }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        // ── Keyboard shortcuts ────────────────────────────────────────────
        ui.input_mut(|i| {
            use egui::{Key, Modifiers};
            if i.consume_key(Modifiers::CTRL, Key::O) { open_file_dialog(app); }
            if i.consume_key(Modifiers::NONE, Key::ArrowLeft)
                || i.consume_key(Modifiers::NONE, Key::PageUp) { app.prev_page(); }
            if i.consume_key(Modifiers::NONE, Key::ArrowRight)
                || i.consume_key(Modifiers::NONE, Key::PageDown) { app.next_page(); }
            if i.consume_key(Modifiers::CTRL, Key::Equals) { app.zoom_in(); }
            if i.consume_key(Modifiers::CTRL, Key::Minus)  { app.zoom_out(); }
            if i.consume_key(Modifiers::CTRL, Key::Num0)   { app.set_zoom(100.0); }
            if i.consume_key(Modifiers::CTRL, Key::F) { self.search_focused = true; }
        });

        let has_doc = app.document.is_some();

        // ══════════════════════════════════════════════════════════════════
        // TOOLBAR LAYOUT
        // File | Navigation | View | Annotation | Search        | Inspector
        // ══════════════════════════════════════════════════════════════════

        egui::Frame::new()
            .fill(BG_SURFACE)
            .inner_margin(egui::Margin::symmetric(SP_SM as i8, SP_XS as i8))
            .stroke(Stroke::new(1.0, BORDER))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;
                    ui.set_height(TOOLBAR_HEIGHT - SP_SM);

                    // ═══ FILE GROUP ═══════════════════════════════════════
                    if icon_button(ui, icons::ICON_OPEN, "Open  (Ctrl+O)", false).clicked() {
                        open_file_dialog(app);
                    }

                    ui.add_enabled_ui(has_doc, |ui| {
                        if icon_button(ui, icons::ICON_SAVE, "Save", false).clicked() {
                            app.status_message = Some("Save not yet implemented".into());
                        }
                        if icon_button(ui, icons::ICON_PRINT, "Print", false).clicked() {
                            app.status_message = Some("Print not yet implemented".into());
                        }
                    });

                    toolbar_divider(ui);

                    // ═══ NAVIGATION GROUP ═════════════════════════════════
                    ui.add_enabled_ui(has_doc, |ui| {
                        if icon_button(ui, icons::ICON_PREV, "Previous  (←)", false).clicked() {
                            app.prev_page();
                        }

                        // Page number input
                        let page_count = app.document.as_ref().map_or(1, |d| d.page_count());
                        let mut page = app.current_page + 1;

                        ui.add_space(2.0);
                        let dv = egui::DragValue::new(&mut page)
                            .range(1..=page_count)
                            .speed(1.0)
                            .max_decimals(0);
                        if ui.add_sized([40.0, 22.0], dv).changed() {
                            app.goto_page(page.saturating_sub(1));
                        }

                        ui.label(
                            RichText::new(format!("/ {}", page_count))
                                .color(FG_TERTIARY)
                                .size(FONT_SIZE_CAPTION)
                        );
                        ui.add_space(2.0);

                        if icon_button(ui, icons::ICON_NEXT, "Next  (→)", false).clicked() {
                            app.next_page();
                        }
                    });

                    toolbar_divider(ui);

                    // ═══ VIEW GROUP ═══════════════════════════════════════
                    ui.add_enabled_ui(has_doc, |ui| {
                        if icon_button(ui, icons::ICON_ZOOM_OUT, "Zoom out  (Ctrl+−)", false).clicked() {
                            app.zoom_out();
                        }

                        // Zoom percentage
                        ui.add_space(2.0);
                        let mut zoom = app.zoom_level;
                        let dv = egui::DragValue::new(&mut zoom)
                            .range(10.0..=500.0)
                            .speed(1.0)
                            .suffix("%")
                            .max_decimals(0);
                        if ui.add_sized([56.0, 22.0], dv).changed() {
                            app.set_zoom(zoom);
                        }
                        ui.add_space(2.0);

                        if icon_button(ui, icons::ICON_ZOOM_IN, "Zoom in  (Ctrl++)", false).clicked() {
                            app.zoom_in();
                        }

                        ui.add_space(SP_XS);

                        if icon_button(ui, icons::ICON_FIT_PAGE, "Fit page", false).clicked() {
                            app.status_message = Some("Fit page not yet implemented".into());
                        }
                        if icon_button(ui, icons::ICON_FIT_WIDTH, "Fit width", false).clicked() {
                            app.status_message = Some("Fit width not yet implemented".into());
                        }

                        toolbar_divider(ui);

                        // Rotation
                        if icon_button(ui, icons::ICON_ROTATE_LEFT, "Rotate left", false).clicked() {
                            app.rotate_left();
                        }
                        if icon_button(ui, icons::ICON_ROTATE_RIGHT, "Rotate right", false).clicked() {
                            app.rotate_right();
                        }
                    });

                    toolbar_divider(ui);

                    // ═══ ANNOTATION GROUP ═════════════════════════════════
                    ui.add_enabled_ui(has_doc, |ui| {
                        let sel_active = app.current_tool.is_none();
                        if icon_button(ui, icons::ICON_SELECT, "Select / Text", sel_active).clicked() {
                            app.current_tool = None;
                        }

                        let tools: &[(&str, AnnotationType, &str)] = &[
                            (icons::ICON_HIGHLIGHT, AnnotationType::Highlight, "Highlight"),
                            (icons::ICON_PEN,       AnnotationType::Pen,       "Pen"),
                            (icons::ICON_TEXT,      AnnotationType::Text,      "Text note"),
                        ];

                        for (icon, tool_type, tooltip) in tools {
                            let active = app.current_tool.as_ref() == Some(tool_type);
                            if icon_button(ui, icon, tooltip, active).clicked() {
                                app.current_tool = if active { None } else { Some(tool_type.clone()) };
                            }
                        }

                        // Color picker (compact)
                        ui.add_space(2.0);
                        egui::color_picker::color_edit_button_srgba(
                            ui,
                            &mut app.current_color,
                            egui::color_picker::Alpha::OnlyBlend,
                        );
                    });

                    toolbar_divider(ui);

                    // ═══ SEARCH GROUP ═════════════════════════════════════
                    ui.add_enabled_ui(has_doc, |ui| {
                        ui.add_space(SP_XS);

                        let te = egui::TextEdit::singleline(&mut self.search_query)
                            .hint_text(format!("{} Search document...", icons::ICON_SEARCH_DOC))
                            .desired_width(200.0)
                            .font(egui::FontId::proportional(FONT_SIZE_BODY));

                        let te_r = ui.add_sized([200.0, 22.0], te);

                        if self.search_focused {
                            te_r.request_focus();
                            self.search_focused = false;
                        }

                        let enter = te_r.lost_focus()
                            && ui.input(|i| i.key_pressed(egui::Key::Enter));

                        if enter {
                            app.perform_search(self.search_query.trim().to_string());
                        }

                        // Search navigation
                        let count = app.search_manager.result_count();
                        if count > 0 {
                            ui.add_space(SP_XS);
                            let idx = app.search_manager.current_index() + 1;
                            ui.label(
                                RichText::new(format!("{}/{}", idx, count))
                                    .color(FG_ACCENT)
                                    .size(FONT_SIZE_CAPTION)
                            );

                            if icon_button(ui, icons::ICON_PREV_RESULT, "Previous result", false).clicked() {
                                let page = app.search_manager.prev_result().map(|r| r.page);
                                if let Some(p) = page {
                                    app.goto_page(p);
                                }
                            }
                            if icon_button(ui, icons::ICON_NEXT_RESULT, "Next result", false).clicked() {
                                let page = app.search_manager.next_result().map(|r| r.page);
                                if let Some(p) = page {
                                    app.goto_page(p);
                                }
                            }
                            if icon_button(ui, icons::ICON_CLEAR, "Clear search", false).clicked() {
                                self.search_query.clear();
                                app.search_manager.clear();
                            }
                        }
                    });

                    // ═══ RIGHT SIDE: INSPECTOR TOGGLE ═════════════════════
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = 2.0;

                        let inspector_active = app.workspace.right_sidebar_visible;
                        if icon_button(ui, icons::ICON_INSPECTOR, "Inspector", inspector_active).clicked() {
                            app.workspace.toggle_right_sidebar();
                        }
                    });
                });
            });
    }
}

/// Open file dialog
fn open_file_dialog(app: &mut DocLensApp) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("PDF Files", &["pdf"])
        .pick_file()
    {
        if let Err(e) = app.open_file(&path.to_string_lossy()) {
            app.status_message = Some(format!("Failed to open: {}", e));
        }
    }
}

impl Default for Toolbar {
    fn default() -> Self { Self::new() }
}
