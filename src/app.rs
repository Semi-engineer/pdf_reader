/*!
Main Application
Core application state and logic
*/

use crate::annotation::{AnnotationManager, AnnotationType};
use crate::config::Settings;
use crate::page_cache::PageCache;
use crate::pdf_document::PdfDocument;
use crate::render_worker::{RenderResponse, RenderWorker};
use crate::search::SearchManager;
use crate::thumbnail_manager::ThumbnailManager;
use crate::ui::{PdfViewer, Sidebar, StatusBar, ToolPalette, Toolbar};
use eframe::egui;
use std::sync::Arc;

pub struct DocLensApp {
    // Configuration
    settings: Settings,
    
    // Document
    document: Option<Arc<PdfDocument>>,
    doc_path: Option<String>,
    
    // State
    current_page: usize,
    zoom_level: f32,
    rotation: u32,
    two_page_mode: bool,
    dark_mode: bool,
    
    // Managers
    page_cache: PageCache,
    thumbnail_manager: ThumbnailManager,
    annotation_manager: AnnotationManager,
    search_manager: SearchManager,
    
    // Render worker
    render_worker: Option<RenderWorker>,
    
    // UI State
    sidebar_visible: bool,
    tool_palette_visible: bool,
    current_tool: Option<AnnotationType>,
    current_color: egui::Color32,
    
    // UI Components
    toolbar: Toolbar,
    sidebar: Sidebar,
    viewer: PdfViewer,
    statusbar: StatusBar,
    tool_palette: ToolPalette,
}

impl DocLensApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load settings
        let settings = Settings::load().unwrap_or_default();
        
        // Configure egui style
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
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
            current_color: egui::Color32::from_rgba_unmultiplied(255, 255, 0, 100),
            toolbar: Toolbar::new(),
            sidebar: Sidebar::new(),
            viewer: PdfViewer::new(),
            statusbar: StatusBar::new(),
            tool_palette: ToolPalette::new(),
        };
        
        // Restore last file if exists
        if let Some(last_file) = app.settings.last_file.clone() {
            if std::path::Path::new(&last_file).exists() {
                let _ = app.open_file(&last_file);
                
                if let Some(last_page) = app.settings.last_page {
                    app.goto_page(last_page);
                }
            }
        }
        
        app
    }
    
    /// Open a PDF file
    pub fn open_file(&mut self, path: &str) -> anyhow::Result<()> {
        // Open document
        let document = PdfDocument::open(path)?;
        let page_count = document.page_count();
        
        // Create render worker
        let render_worker = RenderWorker::new(path.to_string());
        
        // Update state
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
        
        // Generate thumbnails
        if let Some(doc) = &self.document {
            for page in 0..page_count {
                self.thumbnail_manager.generate_thumbnail(doc, page);
            }
        }
        
        // Request initial page renders
        self.request_visible_page_renders();
        
        Ok(())
    }
    
    /// Go to specific page
    pub fn goto_page(&mut self, page: usize) {
        if let Some(doc) = &self.document {
            if page < doc.page_count() {
                self.current_page = page;
                self.request_visible_page_renders();
            }
        }
    }
    
    /// Next page
    pub fn next_page(&mut self) {
        if let Some(doc) = &self.document {
            if self.current_page + 1 < doc.page_count() {
                self.current_page += 1;
                self.request_visible_page_renders();
            }
        }
    }
    
    /// Previous page
    pub fn prev_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.request_visible_page_renders();
        }
    }
    
    /// Set zoom level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(25.0, 400.0);
        self.request_visible_page_renders();
    }
    
    /// Zoom in
    pub fn zoom_in(&mut self) {
        self.set_zoom(self.zoom_level + 25.0);
    }
    
    /// Zoom out
    pub fn zoom_out(&mut self) {
        self.set_zoom(self.zoom_level - 25.0);
    }
    
    /// Rotate left
    pub fn rotate_left(&mut self) {
        self.rotation = (self.rotation + 270) % 360;
        self.request_visible_page_renders();
    }
    
    /// Rotate right
    pub fn rotate_right(&mut self) {
        self.rotation = (self.rotation + 90) % 360;
        self.request_visible_page_renders();
    }
    
    /// Request renders for visible pages
    fn request_visible_page_renders(&mut self) {
        if let (Some(worker), Some(doc)) = (&self.render_worker, &self.document) {
            // Render current page and nearby pages
            let start = self.current_page.saturating_sub(2);
            let end = (self.current_page + 3).min(doc.page_count());
            
            for page in start..end {
                worker.render_page(page, self.zoom_level, self.rotation);
            }
        }
    }
    
    /// Process render responses
    fn process_render_responses(&mut self, ctx: &egui::Context) {
        if let Some(worker) = &self.render_worker {
            while let Some(response) = worker.try_recv() {
                match response {
                    RenderResponse::PageRendered {
                        page,
                        zoom,
                        rotation,
                        image,
                    } => {
                        // Store in cache
                        let key = crate::page_cache::CacheKey::new(page, zoom, rotation);
                        self.page_cache.put(key, image);
                        
                        // Request repaint
                        ctx.request_repaint();
                    }
                    RenderResponse::Error { page, error } => {
                        eprintln!("Error rendering page {}: {}", page, error);
                    }
                }
            }
        }
    }
    
    /// Save settings on exit
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
        // Process render responses
        self.process_render_responses(ctx);
        
        // Top toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.toolbar.show(ui, self);
        });
        
        // Bottom status bar
        egui::TopBottomPanel::bottom("statusbar").show(ctx, |ui| {
            self.statusbar.show(ui, self);
        });
        
        // Left sidebar (thumbnails)
        if self.sidebar_visible {
            egui::SidePanel::left("sidebar")
                .default_width(150.0)
                .show(ctx, |ui| {
                    self.sidebar.show(ui, self);
                });
        }
        
        // Floating tool palette
        if self.tool_palette_visible {
            egui::Window::new("Tools")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    self.tool_palette.show(ui, self);
                });
        }
        
        // Central PDF viewer
        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewer.show(ui, self);
        });
    }
    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_settings();
    }
}
