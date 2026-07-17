/*!
Main Application
Core application state and logic
*/

use crate::annotation::{Annotation, AnnotationManager, AnnotationRect, AnnotationType};
use crate::commands::{Command, CommandDispatcher};
use crate::config::Settings;
use crate::page_cache::PageCache;
use crate::pdf_document::PdfDocument;
use crate::performance_config::PerformanceConfig;
use crate::performance_metrics::PerformanceMetrics;
use crate::render_scheduler::{RenderPriority, RenderScheduler};
use crate::render_worker::{RenderResponse, RenderWorker};
use crate::search::{SearchManager, SearchResult};
use crate::texture_pool::TexturePool;
use crate::thumbnail_manager::ThumbnailManager;
use crate::ui::{ActivityBar, PdfViewer, Sidebar, StatusBar, ToolPalette, Toolbar};
use crate::ui::panels::{CommandPalette, LeftSidebar, RightSidebar, show_menubar};
use crate::ui::theme::BG_BASE;
use crate::viewport::Viewport;
use crate::workspace::WorkspaceState;
use eframe::egui;
use std::sync::Arc;

pub struct DocLensApp {
    // Configuration
    pub settings: Settings,
    pub perf_config: PerformanceConfig,

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

    // Performance Systems
    pub render_scheduler: RenderScheduler,
    pub render_worker: Option<RenderWorker>,
    pub viewport: Viewport,
    pub texture_pool: TexturePool,
    pub metrics: PerformanceMetrics,

    // New Workspace System
    pub workspace: WorkspaceState,
    pub command_dispatcher: CommandDispatcher,
    pub command_palette: CommandPalette,

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
    activity_bar: ActivityBar,
}

impl DocLensApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = Settings::load().unwrap_or_default();
        let perf_config = PerformanceConfig::default();

        // Apply custom theme
        crate::ui::theme::apply(&cc.egui_ctx);

        let sidebar_visible = settings.sidebar_visible;
        let tool_palette_visible = settings.tool_palette_visible;
        let zoom_level = settings.last_zoom.unwrap_or(100.0);
        
        // Initialize performance systems
        let render_scheduler = RenderScheduler::new(perf_config.max_concurrent_renders);
        let viewport = Viewport::new(0, perf_config.prefetch_ahead, perf_config.prefetch_behind);
        let page_cache = PageCache::new(perf_config.page_cache_capacity());
        let texture_pool = TexturePool::new(perf_config.texture_pool_capacity());
        let thumbnail_manager = ThumbnailManager::new_with_capacity(perf_config.thumbnail_cache_capacity());

        let mut app = Self {
            settings,
            perf_config,
            document: None,
            doc_path: None,
            current_page: 0,
            zoom_level,
            rotation: 0,
            two_page_mode: false,
            dark_mode: false,
            page_cache,
            thumbnail_manager,
            annotation_manager: AnnotationManager::default(),
            search_manager: SearchManager::default(),
            render_scheduler,
            render_worker: None,
            viewport,
            texture_pool,
            metrics: PerformanceMetrics::default(),
            workspace: WorkspaceState::default(),
            command_dispatcher: CommandDispatcher::default(),
            command_palette: CommandPalette::default(),
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
            activity_bar: ActivityBar::new(),
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

        // Create render worker with scheduler
        let render_worker = RenderWorker::new(path.to_string(), self.render_scheduler.clone());

        self.document = Some(Arc::new(document));
        self.doc_path = Some(path.to_string());
        self.current_page = 0;
        self.rotation = 0;
        self.render_worker = Some(render_worker);

        // Clear caches
        self.page_cache.clear();
        self.thumbnail_manager.clear();
        self.annotation_manager.clear();
        self.search_manager.clear();
        self.texture_pool.clear();
        
        // Update viewport
        self.viewport = Viewport::new(
            page_count,
            self.perf_config.prefetch_ahead,
            self.perf_config.prefetch_behind,
        );
        self.viewport.set_two_page_mode(self.two_page_mode);

        // Start lazy thumbnail generation (only for visible range)
        self.thumbnail_manager.start_generation(path.to_string(), page_count);

        // Request only visible + prefetch page renders (not all pages!)
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
                self.viewport.set_current_page(page);
                
                // Cancel low-priority renders when navigating
                if self.perf_config.enable_render_cancellation {
                    if let Some(worker) = &self.render_worker {
                        worker.cancel_below_priority(RenderPriority::Prefetch);
                    }
                }
                
                self.request_visible_page_renders();
            }
        }
    }

    pub fn next_page(&mut self) {
        let count = self.document.as_ref().map_or(0, |d| d.page_count());
        if self.current_page + 1 < count {
            self.current_page += 1;
            self.viewport.set_current_page(self.current_page);
            
            // Cancel obsolete renders when navigating
            if self.perf_config.enable_render_cancellation {
                if let Some(worker) = &self.render_worker {
                    worker.cancel_below_priority(RenderPriority::Prefetch);
                }
            }
            
            self.request_visible_page_renders();
        }
    }

    pub fn prev_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.viewport.set_current_page(self.current_page);
            
            // Cancel obsolete renders when navigating
            if self.perf_config.enable_render_cancellation {
                if let Some(worker) = &self.render_worker {
                    worker.cancel_below_priority(RenderPriority::Prefetch);
                }
            }
            
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
        if let (Some(worker), Some(_doc)) = (&self.render_worker, &self.document) {
            // Render visible pages with highest priority
            for page in self.viewport.visible_range() {
                worker.render_page(page, self.zoom_level, self.rotation, RenderPriority::Visible);
            }
            
            // Prefetch nearby pages with lower priority
            for page in self.viewport.prefetch_range() {
                if !self.viewport.is_visible(page) {
                    worker.render_page(page, self.zoom_level, self.rotation, RenderPriority::Prefetch);
                }
            }
        }
    }

    fn process_render_responses(&mut self, ctx: &egui::Context) {
        if let Some(worker) = &self.render_worker {
            let mut received_count = 0;
            while let Some(response) = worker.try_recv() {
                received_count += 1;
                match response {
                    RenderResponse::PageRendered { page, zoom, rotation, image, request_id: _, render_time_ms: _ } => {
                        let key = crate::page_cache::CacheKey::new(page, zoom, rotation);
                        self.page_cache.put(key, image);
                        
                        // Track cache hit
                        self.metrics.record_cache_miss();
                        
                        // Update memory usage metric
                        self.metrics.page_cache_memory_mb = self.page_cache.memory_usage_mb();
                        
                        ctx.request_repaint();
                    }
                    RenderResponse::Error { page, error, request_id: _ } => {
                        eprintln!("Render error page {page}: {error}");
                    }
                }
                
                // Limit responses per frame to avoid blocking
                if received_count >= 10 {
                    break;
                }
            }
        }

        // Poll async thumbnails
        self.thumbnail_manager.poll_ready(ctx);
        
        // Update scheduler metrics
        self.render_scheduler.update_metrics();
        self.render_scheduler.cleanup_cancelled();
        
        // Record frame
        if self.perf_config.enable_metrics {
            self.metrics.record_frame();
        }
    }

    fn save_settings(&mut self) {
        self.settings.last_file = self.doc_path.clone();
        self.settings.last_page = Some(self.current_page);
        self.settings.last_zoom = Some(self.zoom_level);
        self.settings.sidebar_visible = self.sidebar_visible;
        self.settings.tool_palette_visible = self.tool_palette_visible;
        let _ = self.settings.save();
    }
    
    // ─── Command Handler ─────────────────────────────────────────────────────
    
    fn handle_command(&mut self, ctx: &egui::Context, cmd: Command) {
        match cmd {
            Command::OpenDocument => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("PDF", &["pdf"])
                    .pick_file()
                {
                    let _ = self.open_file(path.to_string_lossy().as_ref());
                }
            }
            Command::SaveDocument => {
                self.status_message = Some("Save not yet implemented".to_string());
            }
            Command::ExportPDF => {
                self.status_message = Some("Export not yet implemented".to_string());
            }
            Command::PrintDocument => {
                self.status_message = Some("Print not yet implemented".to_string());
            }
            Command::CloseDocument => {
                self.document = None;
                self.doc_path = None;
                self.current_page = 0;
                self.page_cache.clear();
                self.thumbnail_manager.clear();
            }
            Command::NextPage => self.next_page(),
            Command::PreviousPage => self.prev_page(),
            Command::FirstPage => self.goto_page(0),
            Command::LastPage => {
                if let Some(doc) = &self.document {
                    let last = doc.page_count().saturating_sub(1);
                    self.goto_page(last);
                }
            }
            Command::GoToPage => {
                self.status_message = Some("Go to page dialog not yet implemented".to_string());
            }
            Command::ZoomIn => self.zoom_in(),
            Command::ZoomOut => self.zoom_out(),
            Command::ZoomFit => {
                self.status_message = Some("Fit page not yet implemented".to_string());
            }
            Command::ZoomWidth => {
                self.status_message = Some("Fit width not yet implemented".to_string());
            }
            Command::Zoom100 => self.set_zoom(100.0),
            Command::RotateLeft => self.rotate_left(),
            Command::RotateRight => self.rotate_right(),
            Command::ToggleSidebarLeft => {
                self.workspace.toggle_left_sidebar();
            }
            Command::ToggleSidebarRight => {
                self.workspace.toggle_right_sidebar();
            }
            Command::ToggleTwoPageMode => {
                self.two_page_mode = !self.two_page_mode;
                self.workspace.two_page_mode = self.two_page_mode;
            }
            Command::ToggleFullscreen => {
                self.status_message = Some("Fullscreen not yet implemented".to_string());
            }
            Command::Search => {
                self.workspace.set_active_left_panel(crate::workspace::PanelId::Search);
            }
            Command::SearchNext => {
                self.search_manager.next_result();
                if let Some(result) = self.search_manager.current_result() {
                    self.goto_page(result.page);
                }
            }
            Command::SearchPrevious => {
                self.search_manager.prev_result();
                if let Some(result) = self.search_manager.current_result() {
                    self.goto_page(result.page);
                }
            }
            Command::ClearSearch => {
                self.search_manager.clear();
            }
            Command::AddHighlight => {
                self.current_tool = Some(AnnotationType::Highlight);
            }
            Command::AddNote => {
                self.current_tool = Some(AnnotationType::Text);
            }
            Command::AddDrawing => {
                self.current_tool = Some(AnnotationType::Pen);
            }
            Command::DeleteAnnotation => {
                self.status_message = Some("Delete annotation not yet implemented".to_string());
            }
            Command::SelectAll => {
                self.status_message = Some("Select all not yet implemented".to_string());
            }
            Command::CopyText => {
                self.copy_selected_text(ctx);
            }
            Command::ToggleCommandPalette => {
                self.command_palette.toggle();
            }
            Command::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                self.status_message = Some("Theme toggle not yet fully implemented".to_string());
            }
            Command::ShowProperties => {
                self.workspace.set_active_right_panel(crate::workspace::PanelId::Properties);
            }
            Command::Quit => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }
}

impl eframe::App for DocLensApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_render_responses(ctx);
        
        // Check keyboard shortcuts
        self.command_dispatcher.check_shortcuts(ctx);
        
        // Command palette
        if let Some(cmd) = self.command_palette.show(ctx) {
            self.command_dispatcher.execute(cmd);
        }

        // Process commands
        let commands = self.command_dispatcher.take_pending();
        for cmd in commands {
            self.handle_command(ctx, cmd);
        }

        // ══════════════════════════════════════════════════════════════════
        // LAYOUT STRUCTURE (Modern Engineering Workspace)
        // ══════════════════════════════════════════════════════════════════
        //
        //  +---------------------------------------------------------------+
        //  | Title Bar                                                      |
        //  +---------------------------------------------------------------+
        //  | Menu Bar                                                       |
        //  +---------------------------------------------------------------+
        //  | Toolbar                                                        |
        //  +----+------------------+---------------------------+------------+
        //  | A  | Left Sidebar     | PDF Workspace             | Inspector  |
        //  | c  | (Thumbnails,     | (Viewer)                  | (Right     |
        //  | t  |  Outline,        |                           |  Sidebar)  |
        //  | i  |  Search, etc.)   |                           |            |
        //  | v  |                  |                           |            |
        //  | i  |                  |                           |            |
        //  | t  |                  |                           |            |
        //  | y  |                  |                           |            |
        //  +----+------------------+---------------------------+------------+
        //  | Status Bar                                                     |
        //  +---------------------------------------------------------------+
        //
        // ══════════════════════════════════════════════════════════════════

        // ── Custom title bar (must be first panel) ────────────────────────
        let doc_name = self.doc_path.as_deref().and_then(|p| {
            std::path::Path::new(p)
                .file_name()
                .and_then(|n| n.to_str())
        });
        if crate::ui::show_title_bar(ctx, doc_name) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }
        
        // ── Menu bar ──────────────────────────────────────────────────────
        egui::TopBottomPanel::top("menubar")
            .frame(egui::Frame::new().fill(crate::ui::theme::BG_SURFACE).inner_margin(6.0))
            .show(ctx, |ui| {
                show_menubar(ui, &mut self.command_dispatcher);
            });

        // ── Toolbar ───────────────────────────────────────────────────────
        egui::TopBottomPanel::top("toolbar")
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let mut toolbar = std::mem::take(&mut self.toolbar);
                toolbar.show(ui, self);
                self.toolbar = toolbar;
            });

        // ── Status bar ────────────────────────────────────────────────────
        egui::TopBottomPanel::bottom("statusbar")
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let mut statusbar = std::mem::take(&mut self.statusbar);
                statusbar.show(ui, self);
                self.statusbar = statusbar;
            });

        // ── Activity Bar (vertical left edge) ─────────────────────────────
        let activity_action = egui::SidePanel::left("activity_bar")
            .exact_width(crate::ui::theme::ACTIVITY_BAR_WIDTH)
            .resizable(false)
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let mut activity_bar = std::mem::take(&mut self.activity_bar);
                let action = activity_bar.show(
                    ui,
                    self.workspace.active_left_panel,
                    self.workspace.left_sidebar_visible,
                );
                self.activity_bar = activity_bar;
                action
            })
            .inner;

        // Handle activity bar clicks
        if let Some(action) = activity_action {
            use crate::ui::activity_bar::ActivityBarAction;
            match action {
                ActivityBarAction::ToggleThumbnails => {
                    if self.workspace.active_left_panel == crate::workspace::PanelId::Thumbnails
                        && self.workspace.left_sidebar_visible
                    {
                        self.workspace.left_sidebar_visible = false;
                    } else {
                        self.workspace.set_active_left_panel(crate::workspace::PanelId::Thumbnails);
                    }
                }
                ActivityBarAction::ToggleSearch => {
                    if self.workspace.active_left_panel == crate::workspace::PanelId::Search
                        && self.workspace.left_sidebar_visible
                    {
                        self.workspace.left_sidebar_visible = false;
                    } else {
                        self.workspace.set_active_left_panel(crate::workspace::PanelId::Search);
                    }
                }
                ActivityBarAction::ToggleOutline => {
                    if self.workspace.active_left_panel == crate::workspace::PanelId::Outline
                        && self.workspace.left_sidebar_visible
                    {
                        self.workspace.left_sidebar_visible = false;
                    } else {
                        self.workspace.set_active_left_panel(crate::workspace::PanelId::Outline);
                    }
                }
                ActivityBarAction::ToggleBookmarks => {
                    if self.workspace.active_left_panel == crate::workspace::PanelId::Bookmarks
                        && self.workspace.left_sidebar_visible
                    {
                        self.workspace.left_sidebar_visible = false;
                    } else {
                        self.workspace.set_active_left_panel(crate::workspace::PanelId::Bookmarks);
                    }
                }
                ActivityBarAction::ToggleAnnotations => {
                    if self.workspace.active_left_panel == crate::workspace::PanelId::Annotations
                        && self.workspace.left_sidebar_visible
                    {
                        self.workspace.left_sidebar_visible = false;
                    } else {
                        self.workspace.set_active_left_panel(crate::workspace::PanelId::Annotations);
                    }
                }
                ActivityBarAction::ToggleAttachments => {
                    if self.workspace.active_left_panel == crate::workspace::PanelId::Attachments
                        && self.workspace.left_sidebar_visible
                    {
                        self.workspace.left_sidebar_visible = false;
                    } else {
                        self.workspace.set_active_left_panel(crate::workspace::PanelId::Attachments);
                    }
                }
            }
        }

        // ── Left Sidebar (Modern workspace panels) ────────────────────────
        if self.workspace.left_sidebar_visible {
            egui::SidePanel::left("left_sidebar")
                .min_width(crate::ui::theme::SIDEBAR_MIN_WIDTH)
                .default_width(crate::ui::theme::SIDEBAR_DEFAULT_WIDTH)
                .resizable(true)
                .frame(egui::Frame::new()
                    .fill(crate::ui::theme::BG_SURFACE)
                    .stroke(egui::Stroke::new(1.0, crate::ui::theme::BORDER))
                    .inner_margin(0.0))
                .show(ctx, |ui| {
                    LeftSidebar::show(ui, self);
                });
        }
        
        // ── Right Sidebar (Inspector panel) ───────────────────────────────
        if self.workspace.right_sidebar_visible {
            egui::SidePanel::right("right_sidebar")
                .min_width(crate::ui::theme::SIDEBAR_MIN_WIDTH)
                .default_width(crate::ui::theme::INSPECTOR_DEFAULT_WIDTH)
                .resizable(true)
                .frame(egui::Frame::new()
                    .fill(crate::ui::theme::BG_SURFACE)
                    .stroke(egui::Stroke::new(1.0, crate::ui::theme::BORDER))
                    .inner_margin(0.0))
                .show(ctx, |ui| {
                    RightSidebar::show(ui, self);
                });
        }

        // ── Central Workspace (PDF Viewer) ────────────────────────────────
        egui::CentralPanel::default()
            .frame(egui::Frame::new()
                .fill(BG_BASE)
                .inner_margin(0.0))
            .show(ctx, |ui| {
                ui.style_mut().visuals.extreme_bg_color = BG_BASE;
                
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
