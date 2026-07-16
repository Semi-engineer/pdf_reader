/*!
PDF Viewer Component
Main PDF viewing area with scroll, annotations, search highlights,
and text selection.

Modes:
  • No tool selected  → text-selection mode (drag to select, Ctrl+C to copy)
  • Tool selected     → annotation drawing mode
*/

use crate::annotation::{Annotation, AnnotationRect, AnnotationType};
use crate::app::DocLensApp;
use crate::page_cache::CacheKey;
use eframe::egui;

// ─── Internal state structs ───────────────────────────────────────────────────

struct TextInput {
    screen_pos: egui::Pos2,
    page_origin: egui::Pos2,
    page: usize,
    text: String,
}

/// Live text-selection drag state.
struct TextSelection {
    /// Page the selection is on.
    page: usize,
    /// Drag start in *screen* coords.
    start: egui::Pos2,
    /// Drag current/end in *screen* coords.
    end: egui::Pos2,
    /// Resolved selected string (empty while dragging, filled on release).
    text: String,
    /// Per-character rects in *screen* coords for highlighting.
    /// Populated after drag ends so we can draw precise highlights.
    char_rects: Vec<egui::Rect>,
}

// ─── PdfViewer ────────────────────────────────────────────────────────────────

pub struct PdfViewer {
    // Annotation drawing
    drag_start: Option<egui::Pos2>,
    current_stroke: Vec<egui::Pos2>,
    pending_text: Option<TextInput>,

    // Text selection
    sel_drag_start: Option<egui::Pos2>,
    selection: Option<TextSelection>,
}

impl Default for PdfViewer {
    fn default() -> Self {
        Self {
            drag_start: None,
            current_stroke: Vec::new(),
            pending_text: None,
            sel_drag_start: None,
            selection: None,
        }
    }
}

impl PdfViewer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        if app.document.is_none() {
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(60.0);
                    ui.heading("DocLens");
                    ui.add_space(10.0);
                    ui.label("Open a PDF file to get started.");
                    ui.add_space(20.0);
                    if ui.button("  Open PDF...  ").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("PDF Files", &["pdf"])
                            .pick_file()
                        {
                            let _ = app.open_file(&path.to_string_lossy());
                        }
                    }
                });
            });
            return;
        }

        // Ctrl+C → copy selected text
        if ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::C)) {
            app.copy_selected_text(ui.ctx());
        }

        // Text annotation popup (rendered above the scroll area)
        self.show_text_popup(ui, app);

        let current_page = app.current_page;
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                self.show_page(ui, app, current_page);
            });
    }

    // ── Text annotation popup ─────────────────────────────────────────────────

    fn show_text_popup(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        let pending = match &mut self.pending_text {
            Some(p) => p,
            None => return,
        };

        let pos = pending.screen_pos;
        let page_origin = pending.page_origin;
        let page = pending.page;
        let mut commit = false;
        let mut cancel = false;

        egui::Window::new("Add Text Annotation")
            .fixed_pos(pos)
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.set_min_width(220.0);
                ui.label("Enter text:");
                ui.add_space(4.0);

                let te = egui::TextEdit::multiline(&mut pending.text)
                    .desired_rows(3)
                    .desired_width(200.0)
                    .hint_text("Type here…");
                ui.add(te).request_focus();

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui.button("OK").clicked()
                        || ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.ctrl)
                    {
                        commit = true;
                    }
                    if ui.button("Cancel").clicked()
                        || ui.input(|i| i.key_pressed(egui::Key::Escape))
                    {
                        cancel = true;
                    }
                });
                ui.weak("Ctrl+Enter to confirm, Esc to cancel");
            });

        if commit {
            let text = pending.text.trim().to_string();
            if !text.is_empty() {
                let color = app.current_color;
                let scale = app.zoom_level / 100.0;
                let max_line = text.lines().map(|l| l.chars().count()).max().unwrap_or(1) as f32;
                let lines = text.lines().count().max(1) as f32;
                let id = app.annotation_manager.next_id();
                app.annotation_manager.push(Annotation {
                    id,
                    page,
                    rect: AnnotationRect {
                        x: (pos.x - page_origin.x) / scale,
                        y: (pos.y - page_origin.y) / scale,
                        width: (max_line * 7.5).clamp(60.0, 300.0) / scale,
                        height: (lines * 18.0 + 8.0) / scale,
                    },
                    color: color.to_array(),
                    annotation_type: AnnotationType::Text,
                    points: vec![],
                    text: Some(text),
                });
            }
            self.pending_text = None;
        } else if cancel {
            self.pending_text = None;
        }
    }

    // ── Page rendering ────────────────────────────────────────────────────────

    fn show_page(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp, page: usize) {
        let cache_key = CacheKey::new(page, app.zoom_level, app.rotation);

        if let Some(image) = app.page_cache.get(&cache_key) {
            let img_size = egui::vec2(image.width() as f32, image.height() as f32);
            let available_w = ui.available_width();
            let left_pad = ((available_w - img_size.x) / 2.0).max(0.0);
            ui.add_space(8.0);

            let (page_rect, response) = ui.allocate_exact_size(
                img_size + egui::vec2(left_pad * 2.0, 16.0),
                egui::Sense::click_and_drag(),
            );
            let page_origin = egui::pos2(page_rect.min.x + left_pad, page_rect.min.y + 8.0);
            let image_rect = egui::Rect::from_min_size(page_origin, img_size);

            // Draw page
            let texture = ui.ctx().load_texture(
                format!("page_{}_{:.0}_{}", page, app.zoom_level, app.rotation),
                image.as_ref().clone(),
                egui::TextureOptions::LINEAR,
            );
            ui.painter().image(
                texture.id(),
                image_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
            ui.painter().rect_stroke(
                image_rect,
                2.0,
                egui::Stroke::new(1.0, egui::Color32::from_black_alpha(60)),
                egui::StrokeKind::Outside,
            );

            if app.current_tool.is_some() {
                // ── Annotation mode ───────────────────────────────────────
                self.handle_annotation_input(ui, app, page, page_origin, &response);
            } else {
                // ── Text-selection mode ───────────────────────────────────
                self.handle_text_selection(ui, app, page, page_origin, &response);
            }

            self.draw_text_selection(ui, page, page_origin);
            self.draw_annotations(ui, app, page, page_origin);
            self.draw_search_results(ui, app, page, page_origin);
        } else {
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), 400.0),
                egui::Sense::hover(),
            );
            ui.painter().rect_filled(rect, 4.0, egui::Color32::from_gray(240));
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("Loading page {}...", page + 1),
                egui::FontId::proportional(16.0),
                egui::Color32::GRAY,
            );
        }
    }

    // ── Text selection ────────────────────────────────────────────────────────

    fn handle_text_selection(
        &mut self,
        ui: &mut egui::Ui,
        app: &mut DocLensApp,
        page: usize,
        page_origin: egui::Pos2,
        response: &egui::Response,
    ) {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Text);

        // Click anywhere (without drag) → clear selection
        if response.clicked() && !response.dragged() {
            self.selection = None;
            app.selected_text = None;
        }

        if response.drag_started() {
            self.sel_drag_start = ui.input(|i| i.pointer.hover_pos());
            self.selection = None;
            app.selected_text = None;
        }

        if response.dragged() {
            if let (Some(start), Some(cur)) =
                (self.sel_drag_start, ui.input(|i| i.pointer.hover_pos()))
            {
                // Live drag rect preview
                self.selection = Some(TextSelection {
                    page,
                    start,
                    end: cur,
                    text: String::new(),
                    char_rects: vec![],
                });
            }
        }

        if response.drag_stopped() {
            if let (Some(start), Some(end)) = (
                self.sel_drag_start.take(),
                ui.input(|i| i.pointer.hover_pos()),
            ) {
                let drag_rect = egui::Rect::from_two_pos(start, end);

                if drag_rect.area() > 4.0 {
                    // Extract text & per-char rects from pdfium
                    let selected = app.select_text_in_rect(page, drag_rect, page_origin);

                    // Build per-char highlight rects
                    let char_rects = if let Some(doc) = &app.document {
                        let local_rect = drag_rect.translate(-page_origin.to_vec2());
                        doc.get_chars_with_bounds(page, app.zoom_level)
                            .unwrap_or_default()
                            .into_iter()
                            .filter(|(_, r)| r.intersects(local_rect))
                            .map(|(_, r)| r.translate(page_origin.to_vec2()))
                            .collect()
                    } else {
                        vec![]
                    };

                    self.selection = Some(TextSelection {
                        page,
                        start,
                        end,
                        text: selected.clone(),
                        char_rects,
                    });

                    if !selected.is_empty() {
                        // Auto-copy to clipboard on release (like most PDF viewers)
                        app.copy_selected_text(ui.ctx());
                        app.status_message = Some(format!(
                            "Selected {} character(s) — copied to clipboard",
                            selected.chars().count()
                        ));
                    }
                } else {
                    self.selection = None;
                }
            }
        }
    }

    fn draw_text_selection(
        &self,
        ui: &mut egui::Ui,
        page: usize,
        page_origin: egui::Pos2,
    ) {
        let sel = match &self.selection {
            Some(s) if s.page == page => s,
            _ => return,
        };

        let sel_color = egui::Color32::from_rgba_unmultiplied(51, 153, 255, 80);
        let border_color = egui::Color32::from_rgba_unmultiplied(51, 153, 255, 180);

        if sel.char_rects.is_empty() {
            // Still dragging — draw a simple rubber-band rect
            let drag_rect = egui::Rect::from_two_pos(sel.start, sel.end);
            ui.painter().rect_filled(drag_rect, 1.0, sel_color);
            ui.painter().rect_stroke(
                drag_rect,
                1.0,
                egui::Stroke::new(1.0, border_color),
                egui::StrokeKind::Outside,
            );
        } else {
            // Draw per-character highlight boxes
            for rect in &sel.char_rects {
                ui.painter().rect_filled(*rect, 0.0, sel_color);
            }
        }

        // Show copy hint near the end of the selection
        if !sel.text.is_empty() {
            let hint_pos = egui::pos2(
                sel.end.x.max(sel.start.x) + 6.0,
                sel.end.y.min(sel.start.y) - 18.0,
            );
            let hint_bg = egui::Color32::from_rgba_unmultiplied(30, 30, 30, 200);
            let hint_text = "Ctrl+C to copy";
            let galley = ui.painter().layout_no_wrap(
                hint_text.to_string(),
                egui::FontId::proportional(11.0),
                egui::Color32::WHITE,
            );
            let hint_rect = egui::Rect::from_min_size(
                hint_pos,
                galley.size() + egui::vec2(6.0, 4.0),
            );
            // Keep hint inside the viewport
            let _ = page_origin; // used for future clipping
            ui.painter().rect_filled(hint_rect, 3.0, hint_bg);
            ui.painter().galley(hint_pos + egui::vec2(3.0, 2.0), galley, egui::Color32::WHITE);
        }
    }

    // ── Annotation input ──────────────────────────────────────────────────────

    fn handle_annotation_input(
        &mut self,
        ui: &mut egui::Ui,
        app: &mut DocLensApp,
        page: usize,
        page_origin: egui::Pos2,
        response: &egui::Response,
    ) {
        let tool = match &app.current_tool {
            Some(t) => t.clone(),
            None => return,
        };

        if self.pending_text.is_some() {
            return;
        }

        match tool {
            AnnotationType::Text => {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Text);
                if response.clicked() {
                    if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                        self.pending_text = Some(TextInput {
                            screen_pos: pos,
                            page_origin,
                            page,
                            text: String::new(),
                        });
                    }
                }
            }

            AnnotationType::Pen => {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
                if response.dragged() {
                    if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                        self.current_stroke.push(pos);
                    }
                }
                if response.drag_stopped() && self.current_stroke.len() > 1 {
                    let stroke = std::mem::take(&mut self.current_stroke);
                    let scale = app.zoom_level / 100.0;
                    let color = app.current_color;
                    app.annotation_manager.add_stroke_annotation(
                        page,
                        stroke
                            .iter()
                            .map(|p| egui::pos2(
                                (p.x - page_origin.x) / scale,
                                (p.y - page_origin.y) / scale,
                            ))
                            .collect(),
                        color,
                        AnnotationType::Pen,
                    );
                }
                for w in self.current_stroke.windows(2) {
                    ui.painter().line_segment(
                        [w[0], w[1]],
                        egui::Stroke::new(2.0, app.current_color),
                    );
                }
            }

            AnnotationType::Highlight
            | AnnotationType::Rectangle
            | AnnotationType::Circle => {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
                if response.drag_started() {
                    self.drag_start = ui.input(|i| i.pointer.hover_pos());
                }
                if response.dragged() {
                    if let (Some(s), Some(c)) =
                        (self.drag_start, ui.input(|i| i.pointer.hover_pos()))
                    {
                        let rect = egui::Rect::from_two_pos(s, c);
                        let preview = egui::Color32::from_rgba_unmultiplied(
                            app.current_color.r(), app.current_color.g(),
                            app.current_color.b(), 60,
                        );
                        match tool {
                            AnnotationType::Highlight => { ui.painter().rect_filled(rect, 0.0, preview); }
                            AnnotationType::Rectangle => { ui.painter().rect_stroke(
                                rect, 0.0,
                                egui::Stroke::new(2.0, app.current_color),
                                egui::StrokeKind::Outside,
                            ); }
                            AnnotationType::Circle => { ui.painter().circle_stroke(
                                rect.center(), rect.size().min_elem() / 2.0,
                                egui::Stroke::new(2.0, app.current_color),
                            ); }
                            _ => {}
                        }
                    }
                }
                if response.drag_stopped() {
                    if let (Some(s), Some(e)) =
                        (self.drag_start.take(), ui.input(|i| i.pointer.hover_pos()))
                    {
                        let sr = egui::Rect::from_two_pos(s, e);
                        if sr.area() > 4.0 {
                            app.add_annotation(page, sr, page_origin);
                        }
                    }
                }
            }

            AnnotationType::Line | AnnotationType::Arrow => {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
                if response.drag_started() {
                    self.drag_start = ui.input(|i| i.pointer.hover_pos());
                }
                if response.dragged() {
                    if let (Some(s), Some(c)) =
                        (self.drag_start, ui.input(|i| i.pointer.hover_pos()))
                    {
                        if tool == AnnotationType::Arrow {
                            ui.painter().arrow(s, c - s, egui::Stroke::new(2.0, app.current_color));
                        } else {
                            ui.painter().line_segment([s, c], egui::Stroke::new(2.0, app.current_color));
                        }
                    }
                }
                if response.drag_stopped() {
                    if let (Some(s), Some(e)) =
                        (self.drag_start.take(), ui.input(|i| i.pointer.hover_pos()))
                    {
                        if (e - s).length() > 4.0 {
                            let scale = app.zoom_level / 100.0;
                            let color = app.current_color;
                            app.annotation_manager.add_stroke_annotation(
                                page,
                                vec![
                                    egui::pos2((s.x - page_origin.x) / scale, (s.y - page_origin.y) / scale),
                                    egui::pos2((e.x - page_origin.x) / scale, (e.y - page_origin.y) / scale),
                                ],
                                color,
                                tool,
                            );
                        }
                    }
                }
            }
        }
    }

    // ── Draw annotations ──────────────────────────────────────────────────────

    fn draw_annotations(
        &self,
        ui: &mut egui::Ui,
        app: &DocLensApp,
        page: usize,
        page_origin: egui::Pos2,
    ) {
        for ann in app.annotation_manager.get_page_annotations(page) {
            let sr = ann.rect.to_screen(app.zoom_level, page_origin);
            let color = egui::Color32::from_rgba_unmultiplied(
                ann.color[0], ann.color[1], ann.color[2], ann.color[3],
            );
            match ann.annotation_type {
                AnnotationType::Highlight => { ui.painter().rect_filled(sr, 0.0, color); }
                AnnotationType::Rectangle => { ui.painter().rect_stroke(
                    sr, 0.0, egui::Stroke::new(2.0, color), egui::StrokeKind::Outside,
                ); }
                AnnotationType::Circle => { ui.painter().circle_stroke(
                    sr.center(), sr.size().min_elem() / 2.0, egui::Stroke::new(2.0, color),
                ); }
                AnnotationType::Pen | AnnotationType::Line => {
                    let scale = app.zoom_level / 100.0;
                    let pts: Vec<egui::Pos2> = ann.points.iter().map(|p| egui::pos2(
                        page_origin.x + p.x * scale, page_origin.y + p.y * scale,
                    )).collect();
                    for w in pts.windows(2) {
                        ui.painter().line_segment([w[0], w[1]], egui::Stroke::new(2.0, color));
                    }
                }
                AnnotationType::Arrow => {
                    let scale = app.zoom_level / 100.0;
                    if ann.points.len() >= 2 {
                        let p0 = ann.points[0].to_pos2();
                        let p1 = ann.points[ann.points.len() - 1].to_pos2();
                        let s = egui::pos2(page_origin.x + p0.x * scale, page_origin.y + p0.y * scale);
                        let e = egui::pos2(page_origin.x + p1.x * scale, page_origin.y + p1.y * scale);
                        ui.painter().arrow(s, e - s, egui::Stroke::new(2.0, color));
                    }
                }
                AnnotationType::Text => {
                    if let Some(txt) = &ann.text {
                        let scale = app.zoom_level / 100.0;
                        let font_size = (14.0 * scale).max(8.0);
                        let bg = egui::Color32::from_rgba_unmultiplied(255, 255, 220, 200);
                        ui.painter().rect_filled(sr, 2.0, bg);
                        ui.painter().rect_stroke(sr, 2.0, egui::Stroke::new(1.0, color), egui::StrokeKind::Outside);
                        ui.painter().text(
                            sr.min + egui::vec2(3.0, 2.0),
                            egui::Align2::LEFT_TOP,
                            txt,
                            egui::FontId::proportional(font_size),
                            color,
                        );
                    }
                }
            }
        }
    }

    // ── Draw search highlights ────────────────────────────────────────────────

    fn draw_search_results(
        &self,
        ui: &mut egui::Ui,
        app: &DocLensApp,
        page: usize,
        page_origin: egui::Pos2,
    ) {
        let results = app.search_manager.page_results(page);
        let current_idx = app.search_manager.current_index();
        for (i, result) in results.iter().enumerate() {
            let sr = result.rect.translate(page_origin.to_vec2());
            let color = if i == current_idx {
                egui::Color32::from_rgba_unmultiplied(255, 140, 0, 160)
            } else {
                egui::Color32::from_rgba_unmultiplied(255, 230, 0, 100)
            };
            ui.painter().rect_filled(sr, 1.0, color);
            ui.painter().rect_stroke(
                sr, 1.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 100, 0)),
                egui::StrokeKind::Outside,
            );
        }
    }
}
