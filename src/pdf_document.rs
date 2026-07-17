/*!
PDF Document Wrapper
Handles PDF document operations using pdfium-render.

We store only the file path (+ cached page count) and re-open the document
for every operation.  This avoids the self-referential lifetime issue with
`pdfium_render::PdfDocument<'_>` while keeping the struct `Send + Sync`.
*/

use anyhow::{Context, Result};
use pdfium_render::prelude::*;
use std::path::Path;

pub struct PdfDocument {
    path: String,
    page_count: usize,
}

impl PdfDocument {
    /// Bind to the PDFium native library.
    pub fn bind() -> Result<Pdfium> {
        Ok(Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())
                .context("Failed to bind to PDFium library. Make sure pdfium.dll is present.")?,
        ))
    }

    /// Open a PDF file and cache its page count.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let pdfium = Self::bind()?;
        let document = pdfium
            .load_pdf_from_file(&path_str, None)
            .context("Failed to load PDF file")?;
        let page_count = document.pages().len() as usize;

        Ok(Self {
            path: path_str,
            page_count,
        })
    }

    pub fn page_count(&self) -> usize {
        self.page_count
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    /// Render one page to an RGBA image.
    ///
    /// `zoom` is a percentage — 100.0 = original size.
    pub fn render_page(
        &self,
        page_index: usize,
        zoom: f32,
        rotation: PdfPageRenderRotation,
    ) -> Result<image::RgbaImage> {
        let pdfium = Self::bind()?;
        let document = pdfium
            .load_pdf_from_file(&self.path, None)
            .context("Failed to load PDF file")?;

        let page = document
            .pages()
            .get(page_index as u16)
            .context("Invalid page index")?;

        let scale = zoom / 100.0;
        let width = (page.width().value * scale).round().max(1.0) as u32;
        let height = (page.height().value * scale).round().max(1.0) as u32;

        let render_config = PdfRenderConfig::new()
            .set_target_width(width as i32)
            .set_target_height(height as i32)
            .rotate(rotation, true);

        let bitmap = page
            .render_with_config(&render_config)
            .context("Failed to render page")?;

        Ok(bitmap.as_image().to_rgba8())
    }

    /// Extract all text from a page.
    pub fn get_page_text(&self, page_index: usize) -> Result<String> {
        let pdfium = Self::bind()?;
        let document = pdfium
            .load_pdf_from_file(&self.path, None)
            .context("Failed to load PDF file")?;
        let page = document
            .pages()
            .get(page_index as u16)
            .context("Invalid page index")?;
        let text = page.text().context("Failed to get page text")?;
        Ok(text.all())
    }

    /// Return all characters on a page with their bounding boxes already
    /// scaled to screen pixels at `zoom` %.  Y is flipped (egui top-left).
    ///
    /// Each entry is `(char, screen_rect)`.
    pub fn get_chars_with_bounds(
        &self,
        page_index: usize,
        zoom: f32,
    ) -> Result<Vec<(char, egui::Rect)>> {
        let pdfium = Self::bind()?;
        let document = pdfium
            .load_pdf_from_file(&self.path, None)
            .context("Failed to load PDF file")?;
        let page = document
            .pages()
            .get(page_index as u16)
            .context("Invalid page index")?;

        let scale = zoom / 100.0;
        let page_h = page.height().value;
        let text_obj = page.text().context("Failed to get page text")?;

        let mut result: Vec<(char, egui::Rect)> = Vec::new();

        for ch in text_obj.chars().iter() {
            let c = match ch.unicode_char() {
                Some(c) => c,
                None => continue,
            };

            if let Ok(b) = ch.loose_bounds() {
                let x0 = b.left().value * scale;
                let y0 = (page_h - b.top().value) * scale;
                let x1 = b.right().value * scale;
                let y1 = (page_h - b.bottom().value) * scale;

                // Ensure min/max ordering (PDF rects can have bottom > top)
                let rect = egui::Rect::from_min_max(
                    egui::pos2(x0.min(x1), y0.min(y1)),
                    egui::pos2(x0.max(x1), y0.max(y1)),
                );

                if rect.width() > 0.0 || rect.height() > 0.0 {
                    result.push((c, rect));
                }
            }
        }

        Ok(result)
    }

    /// Search for `query` on a page.
    ///
    /// Returns bounding boxes in **unscaled PDF points** (Y flipped for egui).
    /// Uses per-character `loose_bounds()` — the same coordinate source as
    /// `get_chars_with_bounds()` — so highlights line up with text selection.
    pub fn search_page(
        &self,
        page_index: usize,
        query: &str,
    ) -> Result<Vec<egui::Rect>> {
        if query.is_empty() {
            return Ok(vec![]);
        }

        let pdfium = Self::bind()?;
        let document = pdfium
            .load_pdf_from_file(&self.path, None)
            .context("Failed to load PDF file")?;
        let page = document
            .pages()
            .get(page_index as u16)
            .context("Invalid page index")?;

        let page_h = page.height().value;
        let text_obj = page.text().context("Failed to get page text")?;

        // Build parallel vectors: chars (for string matching) and their rects
        // in unscaled PDF points with Y flipped (egui top-left origin).
        let mut chars: Vec<char> = Vec::new();
        let mut rects: Vec<Option<egui::Rect>> = Vec::new();

        for ch in text_obj.chars().iter() {
            let c = match ch.unicode_char() {
                Some(c) => c,
                None => continue,
            };

            let rect = if let Ok(b) = ch.loose_bounds() {
                let x0 = b.left().value;
                let y0 = page_h - b.top().value;
                let x1 = b.right().value;
                let y1 = page_h - b.bottom().value;
                Some(egui::Rect::from_min_max(
                    egui::pos2(x0.min(x1), y0.min(y1)),
                    egui::pos2(x0.max(x1), y0.max(y1)),
                ))
            } else {
                None
            };

            chars.push(c);
            rects.push(rect);
        }

        // Case-insensitive search on the collected text
        let text_lower: String = chars.iter().collect::<String>().to_lowercase();
        let query_lower = query.to_lowercase();
        let query_chars: Vec<char> = query_lower.chars().collect();
        let query_len = query_chars.len();

        if query_len == 0 {
            return Ok(vec![]);
        }

        let text_chars: Vec<char> = text_lower.chars().collect();
        let mut results: Vec<egui::Rect> = Vec::new();

        let mut pos = 0usize;
        while pos + query_len <= text_chars.len() {
            if text_chars[pos..pos + query_len] == query_chars[..] {
                // Merge bounding rects of matched characters
                let mut min_x = f32::MAX;
                let mut min_y = f32::MAX;
                let mut max_x = f32::MIN;
                let mut max_y = f32::MIN;
                let mut found_any = false;

                for r in &rects[pos..pos + query_len] {
                    if let Some(rect) = r {
                        min_x = min_x.min(rect.min.x);
                        min_y = min_y.min(rect.min.y);
                        max_x = max_x.max(rect.max.x);
                        max_y = max_y.max(rect.max.y);
                        found_any = true;
                    }
                }

                if found_any {
                    results.push(egui::Rect::from_min_max(
                        egui::pos2(min_x, min_y),
                        egui::pos2(max_x, max_y),
                    ));
                }
                pos += query_len; // skip past this match
            } else {
                pos += 1;
            }
        }

        Ok(results)
    }
}

// SAFETY: PdfDocument holds only a String and usize — fully Send + Sync.
unsafe impl Send for PdfDocument {}
unsafe impl Sync for PdfDocument {}
