/*!
Main Application
Core application state and logic
*/

use crate::annotation::{Annotation, AnnotationManager, AnnotationRect, AnnotationType};
use crate::config::Settings;
use crate::page_cache::PageCache;
use crate::pdf_document::PdfDocument;
use crate::render_worker::{RenderResponse, RenderWorker};
use crate::search::{SearchManager, SearchResult};
use crate::thumbnail_manager::ThumbnailManager;
use crate::ui::{PdfViewer, Sidebar, StatusBar, ToolPalette, Toolbar};
use eframe::egui;
use std::sync::Arc;

pub struct DocLensApp {
    // Configuration
    pub settings: Settings,

    // Document
    pub document: Option<Arc<PdfDocument>>,
    pub doc_path: Option<String>,

    // State
    pub current_page: usize,
    pub zoom_level: f32,
    pub rotation: u32,
    pub two_page_mode: bool,
    pub dark_mode: bool,

    // Managers
    pub page_cache: PageCache,
    pub thumbnail_manager: ThumbnailManager,
    pub annotation_manager: AnnotationManager,
    pub search_manager: SearchManager,

    // Render worker
    pub render_worker: Option<RenderWorker>,

    // UI State
    pub sidebar_visible: bool,
    pub tool_palette_visible: bool,
    pub current_tool: Option<AnnotationType>,
    pub current_color: egui::Color32,

    // Status / error message shown in status bar
    pub status_message: Option<String>,

    // Currently selected text (from text-selection drag)
    pub selected_text: Option<String>,

    // UI Components (private — only accessed through update())
    toolbar: Toolbar,
    sidebar: Sidebar,
    viewer: PdfViewer,
    statusbar: StatusBar,
    tool_palette: ToolPalette,
}

impl DocLensApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = Settings::load().unwrap_or_default();

        // Slightly tighter spacing
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(6.0, 4.0);
        cc.egui_ctx.set_style(style);

        let sidebar_visible = settings.sidebar_visible;
        let tool_palette_visible = settings.tool_palette_visible;
        let zoom_level = settings.last_zoom.unwrap_or(100.0);

        let mut app = Self {
            settings,
            document: None,
            doc_path: None,
            current_page: 0,
            zoom_level,
            rotation: 0,
            two_page_mode: false,
            dark_mode: false,
            page_cache: PageCache::default(),
            thumbnail_manager: ThumbnailManager::default(),
            annotation_manager: AnnotationManager::default(),
            search_manager: SearchManager::default(),
            render_worker: None,
            sidebar_visible,
            tool_palette_visible,
            current_tool: None,
            current_color: egui::Color32::from_rgba_unmultiplied(255, 220, 0, 130),
            status_message: None,
            selected_text: None,
            toolbar: Toolbar::new(),
            sidebar: Sidebar::new(),
            viewer: PdfViewer::new(),
            statusbar: StatusBar::new(),
            tool_palette: ToolPalette::new(),
        };

        // Restore last file
        if let Some(last_file) = app.settings.last_file.clone() {
            if std::path::Path::new(&last_file).exists() {
                match app.open_file(&last_file) {
                    Ok(_) => {
                        if let Some(last_page) = app.settings.last_page {
                            app.goto_page(last_page);
                        }
                    }
                    Err(e) => {
                        app.status_message = Some(format!("Could not reopen last file: {e}"));
                    }
                }
            }
        }

        app
    }

    // ─── Document ────────────────────────────────────────────────────────────

    /// Open a PDF file.
    pub fn open_file(&mut self, path: &str) -> anyhow::Result<()> {
        let document = PdfDocument::open(path)?;
        let page_count = document.page_count();

        let render_worker = RenderWorker::new(path.to_string());

        self.document = Some(Arc::new(document));
        self.doc_path = Some(path.to_string());
        self.current_page = 0;
        self.rotation = 0;
        self.render_worker = Some(render_worker);

        self.page_cache.clear();
        self.thumbnail_manager.clear();
        self.annotation_manager.clear();
        self.search_manager.clear();

        // Kick off async thumbnail generation (done lazily inside ThumbnailManager)
        self.thumbnail_manager.start_generation(path.to_string(), page_count);

        // Request initial page renders
        self.request_visible_page_renders();

        self.status_message = Some(format!(
            "Opened: {}  ({} pages)",
            std::path::Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(path),
            page_count
        ));

        Ok(())
    }

    // ─── Navigation ──────────────────────────────────────────────────────────

    pub fn goto_page(&mut self, page: usize) {
        if let Some(doc) = &self.document {
            if page < doc.page_count() {
                self.current_page = page;
                self.request_visible_page_renders();
            }
        }
    }

    pub fn next_page(&mut self) {
        let count = self.document.as_ref().map_or(0, |d| d.page_count());
        if self.current_page + 1 < count {
            self.current_page += 1;
            self.request_visible_page_renders();
        }
    }

    pub fn prev_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.request_visible_page_renders();
        }
    }

    // ─── Zoom / Rotation ─────────────────────────────────────────────────────

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(10.0, 500.0);
        self.page_cache.clear();
        self.request_visible_page_renders();
    }

    pub fn zoom_in(&mut self) {
        let next = next_zoom_step(self.zoom_level, true);
        self.set_zoom(next);
    }

    pub fn zoom_out(&mut self) {
        let next = next_zoom_step(self.zoom_level, false);
        self.set_zoom(next);
    }

    pub fn rotate_left(&mut self) {
        self.rotation = (self.rotation + 270) % 360;
        self.page_cache.clear();
        self.request_visible_page_renders();
    }

    pub fn rotate_right(&mut self) {
        self.rotation = (self.rotation + 90) % 360;
        self.page_cache.clear();
        self.request_visible_page_renders();
    }

    // ─── Search ──────────────────────────────────────────────────────────────

    /// Run search across all pages and store results.
    pub fn perform_search(&mut self, query: String) {
        if query.is_empty() {
            self.search_manager.clear();
            return;
        }

        let doc = match &self.document {
            Some(d) => Arc::clone(d),
            None => return,
        };

        self.search_manager.set_query(query.clone());

        let zoom = self.zoom_level;
        let mut results: Vec<SearchResult> = Vec::new();

        for page in 0..doc.page_count() {
            match doc.search_page(page, &query, zoom) {
                Ok(rects) => {
                    for rect in rects {
                        results.push(SearchResult {
                            page,
                            rect,
                            text: query.clone(),
                        });
                    }
                }
                Err(e) => {
                    eprintln!("Search error on page {page}: {e}");
                }
            }
        }

        let count = results.len();
        self.search_manager.set_results(results);
        self.status_message = Some(format!("Found {count} result(s) for \"{query}\""));

        // Navigate to first result
        if let Some(first) = self.search_manager.results().first() {
            let page = first.page;
            self.goto_page(page);
        }
    }

    // ─── Text Selection ───────────────────────────────────────────────────────

    /// Extract text whose character bounding boxes overlap `screen_rect`.
    /// `page_origin` is the top-left of the rendered page in screen space.
    /// Returns the selected string and also stores it in `self.selected_text`.
    pub fn select_text_in_rect(
        &mut self,
        page: usize,
        screen_rect: egui::Rect,
        page_origin: egui::Pos2,
    ) -> String {
        let doc = match &self.document {
            Some(d) => Arc::clone(d),
            None => return String::new(),
        };

        // The chars returned by get_chars_with_bounds are in page-local
        // screen coords (origin at page top-left).  Translate rect to match.
        let local_rect = screen_rect.translate(-page_origin.to_vec2());

        match doc.get_chars_with_bounds(page, self.zoom_level) {
            Ok(chars) => {
                // Collect chars whose rect intersects the selection rect,
                // preserving document order.
                let selected: String = chars
                    .iter()
                    .filter(|(_, r)| r.intersects(local_rect))
                    .map(|(c, _)| *c)
                    .collect();

                self.selected_text = if selected.is_empty() {
                    None
                } else {
                    Some(selected.clone())
                };
                selected
            }
            Err(e) => {
                eprintln!("Text selection error: {e}");
                String::new()
            }
        }
    }

    /// Copy currently selected text to the system clipboard.
    pub fn copy_selected_text(&self, ctx: &egui::Context) {
        if let Some(text) = &self.selected_text {
            ctx.copy_text(text.clone());
        }
    }

    // ─── Annotations ─────────────────────────────────────────────────────────

    /// Add an annotation at the given **screen** rect (will be converted to PDF coords).
    pub fn add_annotation(&mut self, page: usize, screen_rect: egui::Rect, page_origin: egui::Pos2) {
        if let Some(tool) = self.current_tool.clone() {
            // Convert screen rect → PDF coordinate rect
            let scale = self.zoom_level / 100.0;
            let pdf_rect = egui::Rect::from_min_max(
                egui::pos2(
                    (screen_rect.min.x - page_origin.x) / scale,
                    (screen_rect.min.y - page_origin.y) / scale,
                ),
                egui::pos2(
                    (screen_rect.max.x - page_origin.x) / scale,
                    (screen_rect.max.y - page_origin.y) / scale,
                ),
            );

            let id = self.annotation_manager.next_id();
            self.annotation_manager.push(Annotation {
                id,
                page,
                rect: AnnotationRect::from_egui(pdf_rect),
                color: self.current_color.to_array(),
                annotation_type: tool,
                points: vec![],
                text: None,
            });
        }
    }

    // ─── Internal ────────────────────────────────────────────────────────────

    fn request_visible_page_renders(&mut self) {
        if let (Some(worker), Some(doc)) = (&self.render_worker, &self.document) {
            let start = self.current_page.saturating_sub(1);
            let end = (self.current_page + 2).min(doc.page_count());
            for page in start..end {
                worker.render_page(page, self.zoom_level, self.rotation);
            }
        }
    }

    fn process_render_responses(&mut self, ctx: &egui::Context) {
        if let Some(worker) = &self.render_worker {
            while let Some(response) = worker.try_recv() {
                match response {
                    RenderResponse::PageRendered { page, zoom, rotation, image } => {
                        let key = crate::page_cache::CacheKey::new(page, zoom, rotation);
                        self.page_cache.put(key, image);
                        ctx.request_repaint();
                    }
                    RenderResponse::Error { page, error } => {
                        eprintln!("Render error page {page}: {error}");
                    }
                }
            }
        }

        // Poll async thumbnails
        self.thumbnail_manager.poll_ready(ctx);
    }

    fn save_settings(&mut self) {
        self.settings.last_file = self.doc_path.clone();
        self.settings.last_page = Some(self.current_page);
        self.settings.last_zoom = Some(self.zoom_level);
        self.settings.sidebar_visible = self.sidebar_visible;
        self.settings.tool_palette_visible = self.tool_palette_visible;
        let _ = self.settings.save();
    }
}

impl eframe::App for DocLensApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_render_responses(ctx);

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let mut toolbar = std::mem::take(&mut self.toolbar);
            toolbar.show(ui, self);
            self.toolbar = toolbar;
        });

        egui::TopBottomPanel::bottom("statusbar").show(ctx, |ui| {
            let mut statusbar = std::mem::take(&mut self.statusbar);
            statusbar.show(ui, self);
            self.statusbar = statusbar;
        });

        if self.sidebar_visible {
            egui::SidePanel::left("sidebar")
                .min_width(140.0)
                .default_width(160.0)
                .show(ctx, |ui| {
                    let mut sidebar = std::mem::take(&mut self.sidebar);
                    sidebar.show(ui, self);
                    self.sidebar = sidebar;
                });
        }

        if self.tool_palette_visible {
            egui::Window::new("🎨 Tools")
                .resizable(false)
                .collapsible(true)
                .show(ctx, |ui| {
                    let mut tool_palette = std::mem::take(&mut self.tool_palette);
                    tool_palette.show(ui, self);
                    self.tool_palette = tool_palette;
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut viewer = std::mem::take(&mut self.viewer);
            viewer.show(ui, self);
            self.viewer = viewer;
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_settings();
    }
}

/// Snap to common zoom levels.
fn next_zoom_step(current: f32, up: bool) -> f32 {
    let steps = [10.0, 25.0, 50.0, 75.0, 100.0, 125.0, 150.0, 175.0, 200.0, 300.0, 400.0, 500.0f32];
    if up {
        steps.iter().copied().find(|&s| s > current + 0.5).unwrap_or(500.0)
    } else {
        steps.iter().copied().rev().find(|&s| s < current - 0.5).unwrap_or(10.0)
    }
}
