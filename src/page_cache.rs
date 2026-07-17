/*!
Page Cache Manager
LRU cache for rendered PDF pages with memory tracking
*/

use egui::ColorImage;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CacheKey {
    pub page: usize,
    pub zoom: u32, // Store as integer to avoid float comparison issues
    pub rotation: u32,
}

impl CacheKey {
    pub fn new(page: usize, zoom: f32, rotation: u32) -> Self {
        Self {
            page,
            zoom: (zoom * 10.0) as u32, // Store with 1 decimal precision
            rotation,
        }
    }
}

pub struct PageCache {
    cache: Arc<Mutex<LruCache<CacheKey, Arc<ColorImage>>>>,
    memory_usage_bytes: Arc<Mutex<usize>>,
}

impl PageCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(
                NonZeroUsize::new(capacity).unwrap(),
            ))),
            memory_usage_bytes: Arc::new(Mutex::new(0)),
        }
    }
    
    /// Get cached page image
    pub fn get(&self, key: &CacheKey) -> Option<Arc<ColorImage>> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }
    
    /// Put page image in cache
    pub fn put(&self, key: CacheKey, image: Arc<ColorImage>) {
        // Calculate image size in bytes (RGBA = 4 bytes per pixel)
        let image_bytes = (image.width() * image.height() * 4) as usize;
        
        let mut cache = self.cache.lock().unwrap();
        
        // If evicting an old entry, subtract its size
        if let Some((_, old_image)) = cache.peek_lru() {
            if cache.len() >= cache.cap().get() {
                let old_bytes = (old_image.width() * old_image.height() * 4) as usize;
                let mut mem_usage = self.memory_usage_bytes.lock().unwrap();
                *mem_usage = mem_usage.saturating_sub(old_bytes);
            }
        }
        
        cache.put(key, image);
        
        // Add new image size
        let mut mem_usage = self.memory_usage_bytes.lock().unwrap();
        *mem_usage += image_bytes;
    }
    
    /// Clear all cached pages
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
        let mut mem_usage = self.memory_usage_bytes.lock().unwrap();
        *mem_usage = 0;
    }
    
    /// Remove specific page from cache
    pub fn remove(&self, key: &CacheKey) {
        let mut cache = self.cache.lock().unwrap();
        if let Some(image) = cache.pop(key) {
            let image_bytes = (image.width() * image.height() * 4) as usize;
            let mut mem_usage = self.memory_usage_bytes.lock().unwrap();
            *mem_usage = mem_usage.saturating_sub(image_bytes);
        }
    }
    
    /// Get cache size
    pub fn len(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }
    
    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        let cache = self.cache.lock().unwrap();
        cache.is_empty()
    }
    
    /// Get memory usage in MB
    pub fn memory_usage_mb(&self) -> f32 {
        let mem_usage = self.memory_usage_bytes.lock().unwrap();
        *mem_usage as f32 / (1024.0 * 1024.0)
    }
}

impl Clone for PageCache {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
            memory_usage_bytes: Arc::clone(&self.memory_usage_bytes),
        }
    }
}

impl Default for PageCache {
    fn default() -> Self {
        Self::new(30)
    }
}
