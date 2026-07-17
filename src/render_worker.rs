/*!
Render Worker
Background thread pool for rendering PDF pages with scheduler integration
*/

use crate::pdf_document::PdfDocument;
use crate::render_scheduler::{RenderPriority, RenderScheduler};
use anyhow::Result;
use egui::ColorImage;
use pdfium_render::prelude::PdfPageRenderRotation;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;

pub enum RenderRequest {
    RenderPage {
        page: usize,
        zoom: f32,
        rotation: u32,
        priority: RenderPriority,
        request_id: u64,
    },
}

pub enum RenderResponse {
    PageRendered {
        page: usize,
        zoom: f32,
        rotation: u32,
        image: Arc<ColorImage>,
        request_id: u64,
        render_time_ms: f32,
    },
    Error {
        page: usize,
        error: String,
        request_id: u64,
    },
}

pub struct RenderWorker {
    tx: mpsc::UnboundedSender<RenderRequest>,
    rx: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<RenderResponse>>>,
    scheduler: RenderScheduler,
}

impl RenderWorker {
    pub fn new(pdf_path: String, scheduler: RenderScheduler) -> Self {
        let (request_tx, mut request_rx) = mpsc::unbounded_channel::<RenderRequest>();
        let (response_tx, response_rx) = mpsc::unbounded_channel::<RenderResponse>();
        let scheduler_clone = scheduler.clone();

        // Spawn a dedicated OS thread that owns the Tokio runtime.
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async move {
                // Open PDF document once on the worker thread.
                let doc = match PdfDocument::open(&pdf_path) {
                    Ok(d) => Arc::new(d),
                    Err(e) => {
                        eprintln!("Failed to open PDF in worker: {}", e);
                        return;
                    }
                };

                while let Some(request) = request_rx.recv().await {
                    match request {
                        RenderRequest::RenderPage { page, zoom, rotation, priority: _, request_id } => {
                            // Check if cancelled before starting
                            if scheduler_clone.is_cancelled(request_id) {
                                scheduler_clone.complete_request(request_id);
                                continue;
                            }
                            
                            let doc = Arc::clone(&doc);
                            let response_tx = response_tx.clone();
                            let scheduler = scheduler_clone.clone();

                            tokio::task::spawn_blocking(move || {
                                let start = Instant::now();
                                
                                // Check cancellation again before expensive work
                                if scheduler.is_cancelled(request_id) {
                                    scheduler.complete_request(request_id);
                                    return;
                                }
                                
                                let result = render_page(&doc, page, zoom, rotation);
                                let render_time_ms = start.elapsed().as_secs_f32() * 1000.0;

                                let response = match result {
                                    Ok(image) => {
                                        scheduler.complete_request(request_id);
                                        RenderResponse::PageRendered {
                                            page,
                                            zoom,
                                            rotation,
                                            image: Arc::new(image),
                                            request_id,
                                            render_time_ms,
                                        }
                                    }
                                    Err(e) => {
                                        scheduler.fail_request(request_id);
                                        RenderResponse::Error {
                                            page,
                                            error: e.to_string(),
                                            request_id,
                                        }
                                    }
                                };

                                let _ = response_tx.send(response);
                            });
                        }
                    }
                }
            });
        });

        Self {
            tx: request_tx,
            rx: Arc::new(tokio::sync::Mutex::new(response_rx)),
            scheduler,
        }
    }

    /// Request page render with priority.
    pub fn render_page(&self, page: usize, zoom: f32, rotation: u32, priority: RenderPriority) -> u64 {
        use crate::page_cache::CacheKey;
        let key = CacheKey::new(page, zoom, rotation);
        let request_id = self.scheduler.request(key, priority);
        
        let _ = self.tx.send(RenderRequest::RenderPage {
            page,
            zoom,
            rotation,
            priority,
            request_id,
        });
        
        request_id
    }

    /// Cancel a specific render request
    pub fn cancel_request(&self, request_id: u64) {
        self.scheduler.cancel_request(request_id);
    }
    
    /// Cancel all pending low-priority renders
    pub fn cancel_below_priority(&self, min_priority: RenderPriority) {
        self.scheduler.cancel_below_priority(min_priority);
    }

    /// Try to receive a rendered page (non-blocking).
    pub fn try_recv(&self) -> Option<RenderResponse> {
        if let Ok(mut rx) = self.rx.try_lock() {
            rx.try_recv().ok()
        } else {
            None
        }
    }
    
    /// Get scheduler reference
    pub fn scheduler(&self) -> &RenderScheduler {
        &self.scheduler
    }
}

fn render_page(doc: &PdfDocument, page: usize, zoom: f32, rotation: u32) -> Result<ColorImage> {
    let rotation_enum = match rotation {
        90 => PdfPageRenderRotation::Degrees90,
        180 => PdfPageRenderRotation::Degrees180,
        270 => PdfPageRenderRotation::Degrees270,
        _ => PdfPageRenderRotation::None,
    };

    let rgba_image = doc.render_page(page, zoom, rotation_enum)?;

    let size = [rgba_image.width() as usize, rgba_image.height() as usize];
    let pixels = rgba_image.into_raw();

    Ok(ColorImage::from_rgba_unmultiplied(size, &pixels))
}
