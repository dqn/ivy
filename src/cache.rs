use std::collections::HashMap;

use macroquad::prelude::*;

/// Texture cache for efficient image loading.
pub struct TextureCache {
    textures: HashMap<String, Texture2D>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Load a texture, using cache if available.
    pub async fn get(&mut self, path: &str) -> Option<Texture2D> {
        if let Some(texture) = self.textures.get(path) {
            return Some(texture.clone());
        }

        match load_texture(path).await {
            Ok(texture) => {
                texture.set_filter(FilterMode::Linear);
                self.textures.insert(path.to_string(), texture.clone());
                Some(texture)
            }
            Err(e) => {
                eprintln!("Failed to load texture '{}': {}", path, e);
                None
            }
        }
    }

    /// Check if a texture is cached.
    pub fn contains(&self, path: &str) -> bool {
        self.textures.contains_key(path)
    }

    /// Insert a texture into the cache.
    pub fn insert(&mut self, path: String, texture: Texture2D) {
        self.textures.insert(path, texture);
    }

    /// Get read-only access to the internal texture map.
    pub fn as_map(&self) -> &HashMap<String, Texture2D> {
        &self.textures
    }
}

impl Default for TextureCache {
    fn default() -> Self {
        Self::new()
    }
}
