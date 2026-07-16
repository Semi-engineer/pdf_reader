/*!
Toolbar Component
Main toolbar: file ops, navigation, zoom, rotation, search, view toggles.
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
        // Keyboard shortcuts (checked outside any widget so they always fire)
        ui.input_mut(|i| {
            // Ctrl+O → open file
            if i.consume_key(egui::Modifiers::CTRL, egui::Key::O) {
                open_file_dialog(app);
            }
            // Arrow Left / Right → page navigation
            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft)
                || i.consume_key(egui::Modifiers::NONE, egui::Key::PageUp)
            {
                app.prev_page();
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight)
                || i.consume_key(egui::Modifiers::NONE, egui::Key::PageDown)
            {
                app.next_page();
            }
            // Ctrl+= / Ctrl+- → zoom
            if i.consume_key(egui::Modifiers::CTRL, egui::Key::Equals) {
                app.zoom_in();
            }
            if i.consume_key(egui::Modifiers::CTRL, egui::Key::Minus) {
                app.zoom_out();
            }
            // Ctrl+0 → reset zoom
            if i.consume_key(egui::Modifiers::CTRL, egui::Key::Num0) {
                app.set_zoom(100.0);
            }
        });

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            // ── File ──────────────────────────────────────────────────────
            if ui.button("📁 Open").on_hover_text("Open PDF (Ctrl+O)").clicked() {
                open_file_dialog(app);
            }

            ui.separator();

            // ── Navigation ───────────────────────────────────────────────
            let has_doc = app.document.is_some();

            ui.add_enabled_ui(has_doc, |ui| {
                if ui.button("◀").on_hover_text("Previous page (←)").clicked() {
                    app.prev_page();
                }

                // Editable page number
                let page_count = app.document.as_ref().map_or(1, |d| d.page_count());
                let mut page_input = app.current_page + 1;
                let drag = egui::DragValue::new(&mut page_input)
                    .range(1..=page_count)
                    .speed(1.0);
                if ui.add_sized([42.0, 20.0], drag).changed() {
                    app.goto_page(page_input.saturating_sub(1));
                }
                ui.label(format!("/ {page_count}"));

                if ui.button("▶").on_hover_text("Next page (→)").clicked() {
                    app.next_page();
                }
            });

            ui.separator();

            // ── Zoom ─────────────────────────────────────────────────────
            ui.add_enabled_ui(has_doc, |ui| {
                if ui.button("➖").on_hover_text("Zoom out (Ctrl+-)").clicked() {
                    app.zoom_out();
                }

                // Editable zoom %
                let mut zoom = app.zoom_level;
                let drag = egui::DragValue::new(&mut zoom)
                    .range(10.0..=500.0)
                    .speed(1.0)
                    .suffix("%");
                if ui.add_sized([52.0, 20.0], drag).changed() {
                    app.set_zoom(zoom);
                }

                if ui.button("➕").on_hover_text("Zoom in (Ctrl+=)").clicked() {
                    app.zoom_in();
                }
                if ui.small_button("1:1").on_hover_text("Reset zoom (Ctrl+0)").clicked() {
                    app.set_zoom(100.0);
                }
                if ui.small_button("Fit").on_hover_text("Fit page width").clicked() {
                    // Rough fit — will be refined next frame when we know panel width
                    app.set_zoom(100.0);
                }
            });

            ui.separator();

            // ── Rotation ─────────────────────────────────────────────────
            ui.add_enabled_ui(has_doc, |ui| {
                if ui.button("↶").on_hover_text("Rotate left 90°").clicked() {
                    app.rotate_left();
                }
                if ui.button("↷").on_hover_text("Rotate right 90°").clicked() {
                    app.rotate_right();
                }
            });

            ui.separator();

            // ── Search ────────────────────────────────────────────────────
            ui.add_enabled_ui(has_doc, |ui| {
                let search_response = ui.add_sized(
                    [140.0, 20.0],
                    egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text("🔍 Search…"),
                );

                let enter_pressed = search_response.lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if ui.button("Find").clicked() || enter_pressed {
                    let q = self.search_query.trim().to_string();
                    app.perform_search(q);
                }

                // Show result count and navigation
                let count = app.search_manager.result_count();
                if count > 0 {
                    let idx = app.search_manager.current_index() + 1;
                    ui.label(format!("{idx}/{count}"));

                    if ui.small_button("◀").on_hover_text("Previous result").clicked() {
                        if let Some(result) = app.search_manager.prev_result() {
                            let page = result.page;
                            app.goto_page(page);
                        }
                    }
                    if ui.small_button("▶").on_hover_text("Next result").clicked() {
                        if let Some(result) = app.search_manager.next_result() {
                            let page = result.page;
                            app.goto_page(page);
                        }
                    }
                    if ui.small_button("✕").on_hover_text("Clear search").clicked() {
                        self.search_query.clear();
                        app.search_manager.clear();
                    }
                }
            });

            ui.separator();

            // ── View toggles ─────────────────────────────────────────────
            let sidebar_label = if app.sidebar_visible { "📋 Hide Pages" } else { "📋 Pages" };
            if ui.button(sidebar_label).clicked() {
                app.sidebar_visible = !app.sidebar_visible;
            }

            let tools_label = if app.tool_palette_visible { "🎨 Hide Tools" } else { "🎨 Tools" };
            if ui.button(tools_label).clicked() {
                app.tool_palette_visible = !app.tool_palette_visible;
            }
        });
    }
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
    fn default() -> Self {
        Self::new()
    }
}
