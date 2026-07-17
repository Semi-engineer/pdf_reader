/*!
Performance Configuration
Memory budgets and performance tuning parameters
*/

/// Performance configuration for large document handling.
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Maximum memory for rendered page cache (in MB)
    pub page_cache_mb: usize,
    /// Maximum memory for thumbnail cache (in MB)
    pub thumbnail_cache_mb: usize,
    /// Maximum memory for texture pool (in MB)
    pub texture_cache_mb: usize,
    
    /// Number of pages to render ahead of viewport
    pub prefetch_ahead: usize,
    /// Number of pages to render behind viewport
    pub prefetch_behind: usize,
    
    /// Maximum concurrent render jobs
    pub max_concurrent_renders: usize,
    
    /// Thumbnail viewport margin (pages)
    pub thumbnail_viewport_margin: usize,
    
    /// Enable progressive rendering
    pub progressive_rendering: bool,
    /// Progressive rendering quality levels
    pub progressive_quality_levels: Vec<f32>,
    
    /// Enable render cancellation
    pub enable_render_cancellation: bool,
    
    /// Enable performance metrics
    pub enable_metrics: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            page_cache_mb: 256,
            thumbnail_cache_mb: 64,
            texture_cache_mb: 512,
            prefetch_ahead: 2,
            prefetch_behind: 1,
            max_concurrent_renders: 4,
            thumbnail_viewport_margin: 10,
            progressive_rendering: true,
            progressive_quality_levels: vec![0.25, 0.5, 1.0],
            enable_render_cancellation: true,
            enable_metrics: true,
        }
    }
}

impl PerformanceConfig {
    /// Calculate page cache capacity based on memory budget.
    /// Assumes average page size of ~2MB at 100% zoom (A4 at 150 DPI).
    pub fn page_cache_capacity(&self) -> usize {
        (self.page_cache_mb / 2).max(10)
    }
    
    /// Calculate thumbnail cache capacity.
    /// Assumes average thumbnail size of ~100KB.
    pub fn thumbnail_cache_capacity(&self) -> usize {
        (self.thumbnail_cache_mb * 10).max(50)
    }
    
    /// Calculate texture pool size.
    pub fn texture_pool_capacity(&self) -> usize {
        (self.texture_cache_mb / 2).max(10)
    }
}
