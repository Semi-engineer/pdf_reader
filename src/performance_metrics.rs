/*!
Performance Metrics
Tracks rendering performance and resource usage
*/

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Performance metrics for monitoring rendering pipeline.
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Frame times (last N frames)
    frame_times: VecDeque<Duration>,
    /// Render queue length samples
    queue_lengths: VecDeque<usize>,
    /// Cache hit ratios
    cache_hits: usize,
    cache_misses: usize,
    /// Memory usage estimates (MB)
    pub page_cache_memory_mb: f32,
    pub thumbnail_cache_memory_mb: f32,
    pub texture_cache_memory_mb: f32,
    /// Render statistics
    pub total_renders: usize,
    pub cancelled_renders: usize,
    pub failed_renders: usize,
    /// Timing
    last_frame: Instant,
    render_durations: VecDeque<Duration>,
    thumbnail_durations: VecDeque<Duration>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(60),
            queue_lengths: VecDeque::with_capacity(60),
            cache_hits: 0,
            cache_misses: 0,
            page_cache_memory_mb: 0.0,
            thumbnail_cache_memory_mb: 0.0,
            texture_cache_memory_mb: 0.0,
            total_renders: 0,
            cancelled_renders: 0,
            failed_renders: 0,
            last_frame: Instant::now(),
            render_durations: VecDeque::with_capacity(100),
            thumbnail_durations: VecDeque::with_capacity(100),
        }
    }
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record frame time
    pub fn record_frame(&mut self) {
        let now = Instant::now();
        let duration = now.duration_since(self.last_frame);
        self.frame_times.push_back(duration);
        if self.frame_times.len() > 60 {
            self.frame_times.pop_front();
        }
        self.last_frame = now;
    }
    
    /// Record render queue length
    pub fn record_queue_length(&mut self, length: usize) {
        self.queue_lengths.push_back(length);
        if self.queue_lengths.len() > 60 {
            self.queue_lengths.pop_front();
        }
    }
    
    /// Record cache hit
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    
    /// Record cache miss
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }
    
    /// Record render duration
    pub fn record_render_duration(&mut self, duration: Duration) {
        self.render_durations.push_back(duration);
        if self.render_durations.len() > 100 {
            self.render_durations.pop_front();
        }
    }
    
    /// Record thumbnail generation duration
    pub fn record_thumbnail_duration(&mut self, duration: Duration) {
        self.thumbnail_durations.push_back(duration);
        if self.thumbnail_durations.len() > 100 {
            self.thumbnail_durations.pop_front();
        }
    }
    
    /// Get average frame time (ms)
    pub fn avg_frame_time_ms(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let sum: Duration = self.frame_times.iter().sum();
        sum.as_secs_f32() * 1000.0 / self.frame_times.len() as f32
    }
    
    /// Get current FPS
    pub fn fps(&self) -> f32 {
        let avg_ms = self.avg_frame_time_ms();
        if avg_ms > 0.0 {
            1000.0 / avg_ms
        } else {
            0.0
        }
    }
    
    /// Get average render queue length
    pub fn avg_queue_length(&self) -> f32 {
        if self.queue_lengths.is_empty() {
            return 0.0;
        }
        self.queue_lengths.iter().sum::<usize>() as f32 / self.queue_lengths.len() as f32
    }
    
    /// Get cache hit ratio (0.0 to 1.0)
    pub fn cache_hit_ratio(&self) -> f32 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            return 0.0;
        }
        self.cache_hits as f32 / total as f32
    }
    
    /// Get average render duration (ms)
    pub fn avg_render_duration_ms(&self) -> f32 {
        if self.render_durations.is_empty() {
            return 0.0;
        }
        let sum: Duration = self.render_durations.iter().sum();
        sum.as_secs_f32() * 1000.0 / self.render_durations.len() as f32
    }
    
    /// Get average thumbnail generation time (ms)
    pub fn avg_thumbnail_duration_ms(&self) -> f32 {
        if self.thumbnail_durations.is_empty() {
            return 0.0;
        }
        let sum: Duration = self.thumbnail_durations.iter().sum();
        sum.as_secs_f32() * 1000.0 / self.thumbnail_durations.len() as f32
    }
    
    /// Get total memory usage (MB)
    pub fn total_memory_mb(&self) -> f32 {
        self.page_cache_memory_mb + self.thumbnail_cache_memory_mb + self.texture_cache_memory_mb
    }
    
    /// Reset statistics
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
