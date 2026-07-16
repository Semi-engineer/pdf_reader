/*!
PDF Document Wrapper
Handles PDF document operations using pdfium
*/

use anyhow::{Context, Result};
use pdfium_render::prelude::*;
use std::path::Path;
use std::sync::Arc;

pub struct PdfDocument {
    pdfium: Arc<Pdfium>,
    document: PdfDocument<'static>,
    path: String,
}

impl PdfDocument {
    /// Open a PDF file
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        
        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())
                .context("Failed to bind to PDFium library")?,
        );
        
        let document = pdfium
            .load_pdf_from_file(&path_str, None)
            .context("Failed to load PDF file")?;
        
        Ok(Self {
            pdfium: Arc::new(pdfium),
            document,
            path: path_str,
        })
    }
    
    /// Get number of pages
    pub fn page_count(&self) -> usize {
        self.document.pages().len() as usize
    }
    
    /// Get page at index
    pub fn get_page(&self, index: usize) -> Option<PdfPage> {
        self.document.pages().get(index as u16).ok()
    }
    
    /// Render page to image
    pub fn render_page(
        &self,
        page_index: usize,
        zoom: f32,
        rotation: PdfPageRenderRotation,
    ) -> Result<image::RgbaImage> {
        let page = self
            .get_page(page_index)
            .context("Invalid page index")?;
        
        let width = (page.width().value * zoom / 100.0) as u32;
        let height = (page.height().value * zoom / 100.0) as u32;
        
        let render_config = PdfRenderConfig::new()
            .set_target_width(width as i32)
            .set_target_height(height as i32)
            .rotate(rotation, true);
        
        let bitmap = page
            .render_with_config(&render_config)
            .context("Failed to render page")?;
        
        // Convert bitmap to image
        let rgba = bitmap.as_image_buffer();
        Ok(rgba)
    }
    
    /// Get page text
    pub fn get_page_text(&self, page_index: usize) -> Result<String> {
        let page = self
            .get_page(page_index)
            .context("Invalid page index")?;
        
        Ok(page.text()?.all())
    }
    
    /// Search text in page
    pub fn search_page(
        &self,
        page_index: usize,
        query: &str,
    ) -> Result<Vec<PdfRect>> {
        let page = self
            .get_page(page_index)
            .context("Invalid page index")?;
        
        let mut results = Vec::new();
        
        // Simple text search - get all text and find matches
        let text = page.text()?;
        let all_text = text.all();
        
        if let Some(_) = all_text.to_lowercase().find(&query.to_lowercase()) {
            // For now, return page bounds (simple implementation)
            // A more sophisticated implementation would parse text segments
            results.push(page.page_bounds());
        }
        
        Ok(results)
    }
    
    /// Get file path
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Drop for PdfDocument {
    fn drop(&mut self) {
        // Cleanup handled by pdfium-render
    }
}
