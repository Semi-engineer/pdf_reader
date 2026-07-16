/*!
Render Worker
Background thread pool for rendering PDF pages
*/

use crate::pdf_document::PdfDocument;
use anyhow::Result;
use egui::ColorImage;
use pdfium_render::prelude::PdfPageRenderRotation;
use std::sync::Arc;
use tokio::sync::mpsc;

pub enum RenderRequest {
    RenderPage {
        page: usize,
        zoom: f32,
        rotation: u32,
    },
}

pub enum RenderResponse {
    PageRendered {
        page: usize,
        zoom: f32,
        rotation: u32,
        image: Arc<ColorImage>,
    },
    Error {
        page: usize,
        error: String,
    },
}

pub struct RenderWorker {
    tx: mpsc::UnboundedSender<RenderRequest>,
    rx: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<RenderResponse>>>,
}

impl RenderWorker {
    pub fn new(pdf_path: String) -> Self {
        let (request_tx, mut request_rx) = mpsc::unbounded_channel::<RenderRequest>();
        let (response_tx, response_rx) = mpsc::unbounded_channel::<RenderResponse>();
        
        // Spawn worker thread
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async move {
                // Open PDF document
                let doc = match PdfDocument::open(&pdf_path) {
                    Ok(doc) => Arc::new(doc),
                    Err(e) => {
                        eprintln!("Failed to open PDF: {}", e);
                        return;
                    }
                };
                
                while let Some(request) = request_rx.recv().await {
                    match request {
                        RenderRequest::RenderPage { page, zoom, rotation } => {
                            let doc = Arc::clone(&doc);
                            let response_tx = response_tx.clone();
                            
                            // Render in parallel using rayon
                            tokio::task::spawn_blocking(move || {
                                let result = render_page(&doc, page, zoom, rotation);
                                
                                let response = match result {
                                    Ok(image) => RenderResponse::PageRendered {
                                        page,
                                        zoom,
                                        rotation,
                                        image: Arc::new(image),
                                    },
                                    Err(e) => RenderResponse::Error {
                                        page,
                                        error: e.to_string(),
                                    },
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
        }
    }
    
    /// Request page render
    pub fn render_page(&self, page: usize, zoom: f32, rotation: u32) {
        let _ = self.tx.send(RenderRequest::RenderPage {
            page,
            zoom,
            rotation,
        });
    }
    
    /// Try to receive rendered page (non-blocking)
    pub fn try_recv(&self) -> Option<RenderResponse> {
        if let Ok(mut rx) = self.rx.try_lock() {
            rx.try_recv().ok()
        } else {
            None
        }
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
    
    // Convert to egui ColorImage
    let size = [rgba_image.width() as usize, rgba_image.height() as usize];
    let pixels = rgba_image.into_raw();
    
    Ok(ColorImage::from_rgba_unmultiplied(size, &pixels))
}
