/*!
Thumbnail Manager
Generates and caches page thumbnails
*/

use crate::pdf_document::PdfDocument;
use egui::ColorImage;
use pdfium_render::prelude::PdfPageRenderRotation;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ThumbnailManager {
    thumbnails: HashMap<usize, Arc<ColorImage>>,
    thumbnail_size: f32,
}

impl ThumbnailManager {
    pub fn new() -> Self {
        Self {
            thumbnails: HashMap::new(),
            thumbnail_size: 25.0, // 25% of original size
        }
    }
    
    /// Generate thumbnail for a page
    pub fn generate_thumbnail(
        &mut self,
        doc: &PdfDocument,
        page: usize,
    ) -> Option<Arc<ColorImage>> {
        // Check if already cached
        if let Some(thumbnail) = self.thumbnails.get(&page) {
            return Some(Arc::clone(thumbnail));
        }
        
        // Render at thumbnail size
        match doc.render_page(page, self.thumbnail_size, PdfPageRenderRotation::None) {
            Ok(rgba_image) => {
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();
                let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
                let thumbnail = Arc::new(color_image);
                
                self.thumbnails.insert(page, Arc::clone(&thumbnail));
                Some(thumbnail)
            }
            Err(e) => {
                eprintln!("Failed to generate thumbnail for page {}: {}", page, e);
                None
            }
        }
    }
    
    /// Get cached thumbnail
    pub fn get_thumbnail(&self, page: usize) -> Option<Arc<ColorImage>> {
        self.thumbnails.get(&page).cloned()
    }
    
    /// Clear all thumbnails
    pub fn clear(&mut self) {
        self.thumbnails.clear();
    }
    
    /// Set thumbnail size (as percentage)
    pub fn set_thumbnail_size(&mut self, size: f32) {
        self.thumbnail_size = size;
        self.clear(); // Clear cache when size changes
    }
}

impl Default for ThumbnailManager {
    fn default() -> Self {
        Self::new()
    }
}
