/*!
Viewport Management
Tracks visible pages and prefetch region
*/

use std::ops::Range;

/// Tracks viewport state and calculates visible/prefetch ranges.
#[derive(Debug, Clone)]
pub struct Viewport {
    /// Current page being viewed
    pub current_page: usize,
    /// Total page count
    pub page_count: usize,
    /// Number of pages visible in viewport (1 for single page, 2 for two-page mode)
    pub visible_pages: usize,
    /// Prefetch ahead count
    pub prefetch_ahead: usize,
    /// Prefetch behind count
    pub prefetch_behind: usize,
}

impl Viewport {
    pub fn new(page_count: usize, prefetch_ahead: usize, prefetch_behind: usize) -> Self {
        Self {
            current_page: 0,
            page_count,
            visible_pages: 1,
            prefetch_ahead,
            prefetch_behind,
        }
    }
    
    /// Get range of visible pages
    pub fn visible_range(&self) -> Range<usize> {
        let start = self.current_page;
        let end = (start + self.visible_pages).min(self.page_count);
        start..end
    }
    
    /// Get range of pages to prefetch (including visible)
    pub fn prefetch_range(&self) -> Range<usize> {
        let start = self.current_page.saturating_sub(self.prefetch_behind);
        let end = (self.current_page + self.visible_pages + self.prefetch_ahead)
            .min(self.page_count);
        start..end
    }
    
    /// Check if a page is currently visible
    pub fn is_visible(&self, page: usize) -> bool {
        self.visible_range().contains(&page)
    }
    
    /// Check if a page is in prefetch range
    pub fn is_prefetch(&self, page: usize) -> bool {
        self.prefetch_range().contains(&page)
    }
    
    /// Update current page
    pub fn set_current_page(&mut self, page: usize) {
        self.current_page = page.min(self.page_count.saturating_sub(1));
    }
    
    /// Set two-page mode
    pub fn set_two_page_mode(&mut self, enabled: bool) {
        self.visible_pages = if enabled { 2 } else { 1 };
    }
}
