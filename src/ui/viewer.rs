/*!
PDF Viewer Component
Main PDF viewing area with scroll, annotations, and search highlights.
*/

use crate::annotation::AnnotationType;
use crate::app::DocLensApp;
use crate::page_cache::CacheKey;
use eframe::egui;

/// Per-viewer drag state for annotation creation.
#[derive(Default)]
pub struct PdfViewer {
    drag_start: Option<egui::Pos2>,
    /// Pen strokes being drawn
    current_stroke: Vec<egui::Pos2>,
}

impl PdfViewer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        if app.document.is_none() {
            // Welcome screen
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(60.0);
                    ui.heading("📄 DocLens");
                    ui.add_space(10.0);
                    ui.label("Open a PDF file to get started.");
                    ui.add_space(20.0);
                    if ui.button("  📁  Open PDF…  ").clicked() {
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

        let current_page = app.current_page;

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                self.show_page(ui, app, current_page);
            });
    }

    fn show_page(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp, page: usize) {
        let cache_key = CacheKey::new(page, app.zoom_level, app.rotation);

        if let Some(image) = app.page_cache.get(&cache_key) {
            let img_size = egui::vec2(image.width() as f32, image.height() as f32);

            // Center the page horizontally
            let available_w = ui.available_width();
            let left_pad = ((available_w - img_size.x) / 2.0).max(0.0);
            ui.add_space(8.0);

            let (page_rect, response) = ui.allocate_exact_size(
                img_size + egui::vec2(left_pad * 2.0, 16.0),
                egui::Sense::click_and_drag(),
            );
            let page_origin = egui::pos2(page_rect.min.x + left_pad, page_rect.min.y + 8.0);
            let image_rect = egui::Rect::from_min_size(page_origin, img_size);

            // Draw the page texture
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

            // Drop shadow
            ui.painter().rect_stroke(
                image_rect,
                2.0,
                egui::Stroke::new(1.0, egui::Color32::from_black_alpha(60)),
                egui::StrokeKind::Outside,
            );

            // ── Annotation interaction ────────────────────────────────────
            self.handle_annotation_input(ui, app, page, page_origin, &response);

            // ── Draw existing annotations ─────────────────────────────────
            self.draw_annotations(ui, app, page, page_origin);

            // ── Draw search highlights ────────────────────────────────────
            self.draw_search_results(ui, app, page, page_origin);
        } else {
            // Placeholder while rendering
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), 400.0),
                egui::Sense::hover(),
            );
            ui.painter().rect_filled(rect, 4.0, egui::Color32::from_gray(240));
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("⏳ Rendering page {}…", page + 1),
                egui::FontId::proportional(16.0),
                egui::Color32::GRAY,
            );
        }
    }

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

        match tool {
            AnnotationType::Pen => {
                if response.dragged() {
                    if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                        self.current_stroke.push(pos);
                    }
                }
                if response.drag_stopped() && self.current_stroke.len() > 1 {
                    let stroke = std::mem::take(&mut self.current_stroke);
                    let id = app.annotation_manager.next_id();
                    let color = app.current_color;
                    app.annotation_manager.add_stroke_annotation(
                        page,
                        stroke.iter().map(|p| {
                            let scale = app.zoom_level / 100.0;
                            egui::pos2(
                                (p.x - page_origin.x) / scale,
                                (p.y - page_origin.y) / scale,
                            )
                        }).collect(),
                        color,
                        AnnotationType::Pen,
                    );
                    let _ = id;
                }
                // Draw in-progress stroke
                if self.current_stroke.len() >= 2 {
                    let pts: Vec<egui::Pos2> = self.current_stroke.clone();
                    for w in pts.windows(2) {
                        ui.painter().line_segment(
                            [w[0], w[1]],
                            egui::Stroke::new(2.0, app.current_color),
                        );
                    }
                }
            }
            AnnotationType::Highlight
            | AnnotationType::Rectangle
            | AnnotationType::Circle => {
                // Drag to define rect
                if response.drag_started() {
                    self.drag_start = ui.input(|i| i.pointer.hover_pos());
                }
                if response.dragged() {
                    if let (Some(start), Some(cur)) = (
                        self.drag_start,
                        ui.input(|i| i.pointer.hover_pos()),
                    ) {
                        let rect = egui::Rect::from_two_pos(start, cur);
                        let color = egui::Color32::from_rgba_unmultiplied(
                            app.current_color.r(),
                            app.current_color.g(),
                            app.current_color.b(),
                            60,
                        );
                        // Live preview
                        match app.current_tool.as_ref().unwrap() {
                            AnnotationType::Highlight => {
                                ui.painter().rect_filled(rect, 0.0, color);
                            }
                            AnnotationType::Rectangle => {
                                ui.painter().rect_stroke(
                                    rect,
                                    0.0,
                                    egui::Stroke::new(2.0, app.current_color),
                                    egui::StrokeKind::Outside,
                                );
                            }
                            AnnotationType::Circle => {
                                ui.painter().circle_stroke(
                                    rect.center(),
                                    rect.size().min_elem() / 2.0,
                                    egui::Stroke::new(2.0, app.current_color),
                                );
                            }
                            _ => {}
                        }
                    }
                }
                if response.drag_stopped() {
                    if let (Some(start), Some(end)) = (
                        self.drag_start.take(),
                        ui.input(|i| i.pointer.hover_pos()),
                    ) {
                        let screen_rect = egui::Rect::from_two_pos(start, end);
                        if screen_rect.area() > 4.0 {
                            app.add_annotation(page, screen_rect, page_origin);
                        }
                    }
                }
            }
            _ => {
                // Text, Arrow, Line — click-to-place placeholder
                if response.clicked() {
                    if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                        let small = egui::Rect::from_center_size(pos, egui::vec2(80.0, 24.0));
                        app.add_annotation(page, small, page_origin);
                    }
                }
            }
        }
    }

    fn draw_annotations(
        &self,
        ui: &mut egui::Ui,
        app: &DocLensApp,
        page: usize,
        page_origin: egui::Pos2,
    ) {
        let annotations = app.annotation_manager.get_page_annotations(page);
        for ann in annotations {
            let screen_rect = ann.rect.to_screen(app.zoom_level, page_origin);
            let color = egui::Color32::from_rgba_unmultiplied(
                ann.color[0],
                ann.color[1],
                ann.color[2],
                ann.color[3],
            );

            match ann.annotation_type {
                AnnotationType::Highlight => {
                    ui.painter().rect_filled(screen_rect, 0.0, color);
                }
                AnnotationType::Rectangle => {
                    ui.painter().rect_stroke(
                        screen_rect,
                        0.0,
                        egui::Stroke::new(2.0, color),
                        egui::StrokeKind::Outside,
                    );
                }
                AnnotationType::Circle => {
                    ui.painter().circle_stroke(
                        screen_rect.center(),
                        screen_rect.size().min_elem() / 2.0,
                        egui::Stroke::new(2.0, color),
                    );
                }
                AnnotationType::Pen | AnnotationType::Line => {
                    let scale = app.zoom_level / 100.0;
                    let pts: Vec<egui::Pos2> = ann
                        .points
                        .iter()
                        .map(|p| {
                            egui::pos2(
                                page_origin.x + p.x * scale,
                                page_origin.y + p.y * scale,
                            )
                        })
                        .collect();
                    for w in pts.windows(2) {
                        ui.painter().line_segment(
                            [w[0], w[1]],
                            egui::Stroke::new(2.0, color),
                        );
                    }
                }
                AnnotationType::Arrow => {
                    let scale = app.zoom_level / 100.0;
                    if ann.points.len() >= 2 {
                        let p0 = ann.points.first().unwrap().to_pos2();
                        let p1 = ann.points.last().unwrap().to_pos2();
                        let s = egui::pos2(page_origin.x + p0.x * scale, page_origin.y + p0.y * scale);
                        let e = egui::pos2(page_origin.x + p1.x * scale, page_origin.y + p1.y * scale);
                        ui.painter().arrow(s, e - s, egui::Stroke::new(2.0, color));
                    }
                }
                AnnotationType::Text => {
                    if let Some(txt) = &ann.text {
                        ui.painter().text(
                            screen_rect.min,
                            egui::Align2::LEFT_TOP,
                            txt,
                            egui::FontId::proportional(14.0),
                            color,
                        );
                    }
                }
            }
        }
    }

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
            // result.rect is already in screen-pixel coordinates (scaled during search)
            let screen_rect = result.rect.translate(page_origin.to_vec2());

            let highlight_color = if i == current_idx {
                // Current result: brighter orange
                egui::Color32::from_rgba_unmultiplied(255, 140, 0, 160)
            } else {
                egui::Color32::from_rgba_unmultiplied(255, 230, 0, 100)
            };

            ui.painter().rect_filled(screen_rect, 1.0, highlight_color);
            ui.painter().rect_stroke(
                screen_rect,
                1.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 100, 0)),
                egui::StrokeKind::Outside,
            );
        }
    }
}
