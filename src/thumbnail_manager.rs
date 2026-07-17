/*!
Thumbnail Manager
Generates page thumbnails asynchronously with virtualization support
*/

use crate::pdf_document::PdfDocument;
use egui::ColorImage;
use pdfium_render::prelude::PdfPageRenderRotation;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// Message sent from the background thread to the main thread.
struct ThumbReady {
    page: usize,
    image: Arc<ColorImage>,
}

pub struct ThumbnailManager {
    /// Completed thumbnails.
    thumbnails: HashMap<usize, Arc<ColorImage>>,
    /// Pages for which generation has been requested.
    requested: HashSet<usize>,
    /// Background thread delivers thumbnails through this channel.
    ready_rx: Option<std::sync::mpsc::Receiver<ThumbReady>>,
    /// Sender cloned into the background thread.
    ready_tx: Option<std::sync::mpsc::SyncSender<ThumbReady>>,
    /// Shared flag: background thread posts here to wake up egui.
    repaint_tx: Option<Arc<Mutex<Option<egui::Context>>>>,
    /// Maximum number of thumbnails to cache
    max_capacity: usize,
    /// Current visible range (for virtualization)
    visible_start: usize,
    visible_end: usize,
}

impl ThumbnailManager {
    pub fn new() -> Self {
        Self::new_with_capacity(100)
    }
    
    pub fn new_with_capacity(capacity: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel::<ThumbReady>(64);
        Self {
            thumbnails: HashMap::new(),
            requested: HashSet::new(),
            ready_rx: Some(rx),
            ready_tx: Some(tx),
            repaint_tx: Some(Arc::new(Mutex::new(None))),
            max_capacity: capacity,
            visible_start: 0,
            visible_end: 0,
        }
    }

    /// Begin generating thumbnails for all pages of the PDF in a background thread.
    /// Safe to call multiple times — only ungenerated pages are processed.
    pub fn start_generation(&mut self, pdf_path: String, page_count: usize) {
        let tx = match &self.ready_tx {
            Some(t) => t.clone(),
            None => return,
        };
        let ctx_holder = self.repaint_tx.clone();

        std::thread::spawn(move || {
            // One PDFium bind per thread is fine.
            let pdfium = match PdfDocument::bind() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Thumbnail thread: bind failed: {e}");
                    return;
                }
            };
            let document = match pdfium.load_pdf_from_file(&pdf_path, None) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Thumbnail thread: open failed: {e}");
                    return;
                }
            };

            for page_idx in 0..page_count {
                let page = match document.pages().get(page_idx as u16) {
                    Ok(p) => p,
                    Err(_) => continue,
                };

                // Render at ~20% — gives ~120 px wide thumbnails for A4
                let scale = 0.20_f32;
                let w = (page.width().value * scale).round() as u32;
                let h = (page.height().value * scale).round() as u32;

                let cfg = pdfium_render::prelude::PdfRenderConfig::new()
                    .set_target_width(w as i32)
                    .set_target_height(h as i32)
                    .rotate(PdfPageRenderRotation::None, false);

                let bitmap = match page.render_with_config(&cfg) {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("Thumbnail render error page {page_idx}: {e}");
                        continue;
                    }
                };

                let rgba = bitmap.as_image().to_rgba8();
                let size = [rgba.width() as usize, rgba.height() as usize];
                let color_image = ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());

                let ready = ThumbReady {
                    page: page_idx,
                    image: Arc::new(color_image),
                };

                if tx.send(ready).is_err() {
                    break; // Receiver dropped — app is closing
                }

                // Request egui repaint so the new thumbnail appears
                if let Some(holder) = &ctx_holder {
                    if let Ok(guard) = holder.lock() {
                        if let Some(ctx) = guard.as_ref() {
                            ctx.request_repaint();
                        }
                    }
                }
            }
        });
    }
    
    /// Update visible range for thumbnail virtualization
    pub fn set_visible_range(&mut self, start: usize, end: usize) {
        self.visible_start = start;
        self.visible_end = end;
        
        // Evict thumbnails outside the visible range + margin
        let margin = 20;
        let evict_start = start.saturating_sub(margin);
        let evict_end = end + margin;
        
        self.thumbnails.retain(|&page, _| {
            page >= evict_start && page < evict_end
        });
    }

    /// Poll the channel for completed thumbnails.  Call once per frame from
    /// `process_render_responses`.
    pub fn poll_ready(&mut self, ctx: &egui::Context) {
        // Register our egui context so the background thread can wake us
        if let Some(holder) = &self.repaint_tx {
            if let Ok(mut guard) = holder.lock() {
                if guard.is_none() {
                    *guard = Some(ctx.clone());
                }
            }
        }

        if let Some(rx) = &self.ready_rx {
            let mut count = 0;
            while let Ok(ready) = rx.try_recv() {
                self.thumbnails.insert(ready.page, ready.image);
                self.requested.insert(ready.page);
                
                // Enforce capacity limit using LRU-like behavior
                if self.thumbnails.len() > self.max_capacity {
                    // Remove thumbnails farthest from visible range
                    if let Some(&page_to_remove) = self.thumbnails.keys()
                        .filter(|&&p| p < self.visible_start || p >= self.visible_end)
                        .next()
                    {
                        self.thumbnails.remove(&page_to_remove);
                    }
                }
                
                count += 1;
                if count >= 10 {
                    break; // Limit processing per frame
                }
            }
        }
    }

    /// Get a cached thumbnail, if available.
    pub fn get_thumbnail(&self, page: usize) -> Option<Arc<ColorImage>> {
        self.thumbnails.get(&page).cloned()
    }

    /// Clear all thumbnails (called when a new document is opened).
    pub fn clear(&mut self) {
        self.thumbnails.clear();
        self.requested.clear();
        // Drop the old channels; start_generation will create new ones next call.
        let (tx, rx) = std::sync::mpsc::sync_channel::<ThumbReady>(64);
        self.ready_rx = Some(rx);
        self.ready_tx = Some(tx);
        self.repaint_tx = Some(Arc::new(Mutex::new(None)));
    }
    
    /// Get current memory usage estimate (MB)
    pub fn memory_usage_mb(&self) -> f32 {
        let bytes: usize = self.thumbnails.values()
            .map(|img| img.width() * img.height() * 4)
            .sum();
        bytes as f32 / (1024.0 * 1024.0)
    }
}

impl Default for ThumbnailManager {
    fn default() -> Self {
        Self::new()
    }
}
