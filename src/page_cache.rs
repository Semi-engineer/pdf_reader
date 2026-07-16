/*!
Page Cache Manager
LRU cache for rendered PDF pages
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
}

impl PageCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(
                NonZeroUsize::new(capacity).unwrap(),
            ))),
        }
    }
    
    /// Get cached page image
    pub fn get(&self, key: &CacheKey) -> Option<Arc<ColorImage>> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }
    
    /// Put page image in cache
    pub fn put(&self, key: CacheKey, image: Arc<ColorImage>) {
        let mut cache = self.cache.lock().unwrap();
        cache.put(key, image);
    }
    
    /// Clear all cached pages
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
    
    /// Remove specific page from cache
    pub fn remove(&self, key: &CacheKey) {
        let mut cache = self.cache.lock().unwrap();
        cache.pop(key);
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
}

impl Clone for PageCache {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
        }
    }
}

impl Default for PageCache {
    fn default() -> Self {
        Self::new(30)
    }
}
