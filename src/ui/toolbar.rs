/*!
Toolbar — Modern, compact, icon-first top bar
Reorganized into logical groups: File | Navigation | View | Search | Annotation
*/

use super::theme::*;
use crate::app::DocLensApp;
use eframe::egui::{self, Color32, RichText, Stroke, Vec2};

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
        // TOOLBAR STRUCTURE (Industrial Minimal)
        // ══════════════════════════════════════════════════════════════════
        // File | Navigation | View | Search                      | Inspector
        // ══════════════════════════════════════════════════════════════════

        egui::Frame::new()
            .fill(BG_SURFACE)
            .inner_margin(egui::Margin::symmetric(8, 4))
            .stroke(Stroke::new(1.0, BORDER))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;
                    ui.set_height(TOOLBAR_HEIGHT - 8.0);

                    // ═══ FILE GROUP ═══════════════════════════════════════
                    if icon_btn(ui, "📂", "Open  (Ctrl+O)", false).clicked() {
                        open_file_dialog(app);
                    }
                    
                    ui.add_enabled_ui(has_doc, |ui| {
                        if icon_btn(ui, "💾", "Save", false).clicked() {
                            app.status_message = Some("Save not yet implemented".into());
                        }
                        if icon_btn(ui, "🖨", "Print", false).clicked() {
                            app.status_message = Some("Print not yet implemented".into());
                        }
                    });

                    toolbar_divider(ui);

                    // ═══ NAVIGATION GROUP ═════════════════════════════════
                    ui.add_enabled_ui(has_doc, |ui| {
                        if icon_btn(ui, "◀", "Previous  (←)", false).clicked() {
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
                                .size(FONT_SIZE_SMALL)
                        );
                        ui.add_space(2.0);

                        if icon_btn(ui, "▶", "Next  (→)", false).clicked() {
                            app.next_page();
                        }
                    });

                    toolbar_divider(ui);

                    // ═══ VIEW GROUP ═══════════════════════════════════════
                    ui.add_enabled_ui(has_doc, |ui| {
                        if icon_btn(ui, "−", "Zoom out  (Ctrl+−)", false).clicked() {
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

                        if icon_btn(ui, "+", "Zoom in  (Ctrl++)", false).clicked() {
                            app.zoom_in();
                        }

                        ui.add_space(4.0);
                        
                        if icon_btn(ui, "⊡", "Fit page", false).clicked() {
                            app.status_message = Some("Fit page not yet implemented".into());
                        }
                        if icon_btn(ui, "⊟", "Fit width", false).clicked() {
                            app.status_message = Some("Fit width not yet implemented".into());
                        }

                        toolbar_divider(ui);

                        // Rotation
                        if icon_btn(ui, "↺", "Rotate left", false).clicked() {
                            app.rotate_left();
                        }
                        if icon_btn(ui, "↻", "Rotate right", false).clicked() {
                            app.rotate_right();
                        }
                    });

                    toolbar_divider(ui);

                    // ═══ SEARCH GROUP ═════════════════════════════════════
                    ui.add_enabled_ui(has_doc, |ui| {
                        // Unified search field with icon
                        ui.add_space(4.0);
                        
                        let te = egui::TextEdit::singleline(&mut self.search_query)
                            .hint_text("🔍 Search document...")
                            .desired_width(200.0)
                            .font(egui::FontId::proportional(FONT_SIZE_UI));
                        
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

                        // Search navigation (only show when there are results)
                        let count = app.search_manager.result_count();
                        if count > 0 {
                            ui.add_space(4.0);
                            let idx = app.search_manager.current_index() + 1;
                            ui.label(
                                RichText::new(format!("{}/{}", idx, count))
                                    .color(FG_ACCENT)
                                    .size(FONT_SIZE_SMALL)
                            );
                            
                            if icon_btn(ui, "▲", "Previous result", false).clicked() {
                                let page = app.search_manager.prev_result().map(|r| r.page);
                                if let Some(p) = page {
                                    app.goto_page(p);
                                }
                            }
                            if icon_btn(ui, "▼", "Next result", false).clicked() {
                                let page = app.search_manager.next_result().map(|r| r.page);
                                if let Some(p) = page {
                                    app.goto_page(p);
                                }
                            }
                            if icon_btn(ui, "✕", "Clear search", false).clicked() {
                                self.search_query.clear();
                                app.search_manager.clear();
                            }
                        }
                    });

                    // ═══ RIGHT SIDE: INSPECTOR TOGGLE ═════════════════════
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = 2.0;

                        // Toggle inspector (right sidebar)
                        let inspector_active = app.workspace.right_sidebar_visible;
                        if icon_btn(ui, "ℹ", "Inspector", inspector_active).clicked() {
                            app.workspace.toggle_right_sidebar();
                        }
                    });
                });
            });
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ══════════════════════════════════════════════════════════════════════════════

/// Toolbar divider (thin vertical line)
fn toolbar_divider(ui: &mut egui::Ui) {
    ui.add_space(4.0);
    ui.add(
        egui::Separator::default()
            .vertical()
            .spacing(4.0)
    );
    ui.add_space(4.0);
}

/// Icon button for toolbar (square, 28x28)
fn icon_btn(ui: &mut egui::Ui, icon: &str, tooltip: &str, active: bool) -> egui::Response {
    let size = Vec2::splat(28.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    let bg = if active {
        BG_ACTIVE
    } else if response.hovered() {
        BG_HOVER
    } else {
        Color32::TRANSPARENT
    };

    let fg = if active {
        Color32::WHITE
    } else if response.hovered() {
        FG_PRIMARY
    } else {
        FG_SECONDARY
    };

    // Background
    if bg != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, 2.0, bg);
    }

    // Icon
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(14.0),
        fg,
    );

    response.on_hover_text(tooltip)
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
