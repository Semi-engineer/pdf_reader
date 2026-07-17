/*!
Texture Pool
Reusable texture object pool to reduce GPU allocations
*/

use egui::{ColorImage, TextureHandle};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Texture pool for reusing GPU texture allocations.
pub struct TexturePool {
    /// Textures indexed by their identifier
    textures: Arc<Mutex<HashMap<String, TextureHandle>>>,
    /// Maximum number of textures to pool
    max_capacity: usize,
}

impl TexturePool {
    pub fn new(capacity: usize) -> Self {
        Self {
            textures: Arc::new(Mutex::new(HashMap::new())),
            max_capacity: capacity,
        }
    }
    
    /// Get or create a texture for the given image.
    /// Reuses existing texture handle if available.
    pub fn get_or_create(
        &self,
        ctx: &egui::Context,
        id: String,
        image: &ColorImage,
    ) -> TextureHandle {
        let mut textures = self.textures.lock().unwrap();
        
        // Check if we already have this texture
        if let Some(handle) = textures.get(&id) {
            // Update texture data if size matches
            if handle.size() == [image.width(), image.height()] {
                // Texture exists and size matches - update it
                return handle.clone();
            } else {
                // Size mismatch - remove old texture
                textures.remove(&id);
            }
        }
        
        // Create new texture
        let handle = ctx.load_texture(id.clone(), image.clone(), egui::TextureOptions::LINEAR);
        
        // Enforce capacity limit
        if textures.len() >= self.max_capacity {
            // Remove oldest texture (in practice, use LRU)
            if let Some(key) = textures.keys().next().cloned() {
                textures.remove(&key);
            }
        }
        
        textures.insert(id, handle.clone());
        handle
    }
    
    /// Remove a texture from the pool
    pub fn remove(&self, id: &str) {
        let mut textures = self.textures.lock().unwrap();
        textures.remove(id);
    }
    
    /// Clear all textures
    pub fn clear(&self) {
        let mut textures = self.textures.lock().unwrap();
        textures.clear();
    }
    
    /// Get current pool size
    pub fn len(&self) -> usize {
        self.textures.lock().unwrap().len()
    }
    
    /// Check if pool is empty
    pub fn is_empty(&self) -> bool {
        self.textures.lock().unwrap().is_empty()
    }
}

impl Clone for TexturePool {
    fn clone(&self) -> Self {
        Self {
            textures: Arc::clone(&self.textures),
            max_capacity: self.max_capacity,
        }
    }
}

impl Default for TexturePool {
    fn default() -> Self {
        Self::new(50)
    }
}
