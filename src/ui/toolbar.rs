/*!
Toolbar — compact icon-first design.
*/

use super::theme::{self, BG_ELEVATED, BORDER, FG_ACCENT, FG_SECONDARY};
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
            // Ctrl+F → focus search
            if i.consume_key(Modifiers::CTRL, Key::F) { self.search_focused = true; }
        });

        let has_doc = app.document.is_some();

        // Toolbar frame
        egui::Frame::new()
            .fill(super::theme::BG_SURFACE)
            .inner_margin(egui::Margin { left: 8, right: 8, top: 5, bottom: 5 })
            .stroke(Stroke::new(1.0, BORDER))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 3.0;

                    // ── Open ──────────────────────────────────────────────
                    let open_r = ui.add(
                        egui::Button::new(
                            RichText::new("▤").size(15.0)
                        )
                        .min_size(Vec2::new(32.0, 28.0))
                        .fill(BG_ELEVATED)
                    ).on_hover_text("Open PDF  (Ctrl+O)");
                    if open_r.clicked() { open_file_dialog(app); }

                    divider(ui);

                    // ── Navigation ────────────────────────────────────────
                    ui.add_enabled_ui(has_doc, |ui| {
                        if theme::icon_btn(ui, "◀", "Previous page  (←)").clicked() {
                            app.prev_page();
                        }

                        let page_count = app.document.as_ref().map_or(1, |d| d.page_count());
                        let mut page = app.current_page + 1;
                        let dv = egui::DragValue::new(&mut page)
                            .range(1..=page_count)
                            .speed(1.0)
                            .max_decimals(0);
                        if ui.add_sized([38.0, 24.0], dv).changed() {
                            app.goto_page(page.saturating_sub(1));
                        }
                        ui.label(
                            RichText::new(format!("/ {page_count}"))
                                .color(FG_SECONDARY).size(12.5)
                        );

                        if theme::icon_btn(ui, "▶", "Next page  (→)").clicked() {
                            app.next_page();
                        }
                    });

                    divider(ui);

                    // ── Zoom ──────────────────────────────────────────────
                    ui.add_enabled_ui(has_doc, |ui| {
                        if theme::icon_btn(ui, "−", "Zoom out  (Ctrl+−)").clicked() {
                            app.zoom_out();
                        }

                        let mut zoom = app.zoom_level;
                        let dv = egui::DragValue::new(&mut zoom)
                            .range(10.0..=500.0)
                            .speed(1.0)
                            .suffix("%")
                            .max_decimals(0);
                        if ui.add_sized([52.0, 24.0], dv).changed() {
                            app.set_zoom(zoom);
                        }

                        if theme::icon_btn(ui, "+", "Zoom in  (Ctrl+=)").clicked() {
                            app.zoom_in();
                        }

                        let r = ui.add(
                            egui::Button::new(RichText::new("1:1").size(11.5).color(FG_SECONDARY))
                                .min_size(Vec2::new(28.0, 24.0))
                                .fill(Color32::TRANSPARENT)
                        ).on_hover_text("Reset zoom  (Ctrl+0)");
                        if r.clicked() { app.set_zoom(100.0); }
                    });

                    divider(ui);

                    // ── Rotation ──────────────────────────────────────────
                    ui.add_enabled_ui(has_doc, |ui| {
                        if theme::icon_btn(ui, "↶", "Rotate left").clicked()  { app.rotate_left(); }
                        if theme::icon_btn(ui, "↷", "Rotate right").clicked() { app.rotate_right(); }
                    });

                    divider(ui);

                    // ── Search ────────────────────────────────────────────
                    ui.add_enabled_ui(has_doc, |ui| {
                        // Text input with inner icon
                        let te = egui::TextEdit::singleline(&mut self.search_query)
                            .desired_width(150.0)
                            .hint_text("/ Search...")
                            .font(egui::FontId::proportional(13.0));
                        let te_r = ui.add_sized([150.0, 24.0], te);

                        if self.search_focused {
                            te_r.request_focus();
                            self.search_focused = false;
                        }

                        let enter = te_r.lost_focus()
                            && ui.input(|i| i.key_pressed(egui::Key::Enter));

                        let find_r = ui.add(
                            egui::Button::new(RichText::new("Find").size(12.5))
                                .min_size(Vec2::new(38.0, 24.0))
                                .fill(BG_ELEVATED)
                        );
                        if find_r.clicked() || enter {
                            app.perform_search(self.search_query.trim().to_string());
                        }

                        let count = app.search_manager.result_count();
                        if count > 0 {
                            let idx = app.search_manager.current_index() + 1;
                            ui.label(
                                RichText::new(format!("{idx}/{count}"))
                                    .color(FG_ACCENT).size(12.0)
                            );
                            if theme::icon_btn(ui, "◀", "Previous result").clicked() {
                                if let Some(r) = app.search_manager.prev_result() {
                                    let p = r.page; app.goto_page(p);
                                }
                            }
                            if theme::icon_btn(ui, "▶", "Next result").clicked() {
                                if let Some(r) = app.search_manager.next_result() {
                                    let p = r.page; app.goto_page(p);
                                }
                            }
                            if theme::icon_btn(ui, "✕", "Clear search").clicked() {
                                self.search_query.clear();
                                app.search_manager.clear();
                            }
                        }
                    });

                    // ── Right-side toggles ────────────────────────────────
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = 3.0;

                        if theme::icon_toggle_btn(ui, "✦", "Annotation tools", app.tool_palette_visible).clicked() {
                            app.tool_palette_visible = !app.tool_palette_visible;
                        }
                        if theme::icon_toggle_btn(ui, "▣", "Page thumbnails", app.sidebar_visible).clicked() {
                            app.sidebar_visible = !app.sidebar_visible;
                        }
                    });
                });
            });
    }
}

fn divider(ui: &mut egui::Ui) {
    ui.add_space(2.0);
    ui.add(egui::Separator::default().vertical().spacing(6.0));
    ui.add_space(2.0);
}

fn open_file_dialog(app: &mut DocLensApp) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("PDF Files", &["pdf"])
        .pick_file()
    {
        if let Err(e) = app.open_file(&path.to_string_lossy()) {
            app.status_message = Some(format!("Failed to open: {e}"));
        }
    }
}

impl Default for Toolbar {
    fn default() -> Self { Self::new() }
}
