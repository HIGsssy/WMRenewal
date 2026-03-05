use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

/// Caches loaded textures by file path to avoid redundant disk/GPU loads.
pub struct TextureCache {
    cache: HashMap<PathBuf, Texture>,
    failed: HashSet<PathBuf>,
}

impl Default for TextureCache {
    fn default() -> Self {
        Self::new()
    }
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            failed: HashSet::new(),
        }
    }

    /// Load a texture from file, using the cache if already loaded.
    pub fn load(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        path: &Path,
    ) -> Result<&Texture, String> {
        if self.failed.contains(path) {
            return Err("previously failed".into());
        }
        if !self.cache.contains_key(path) {
            match texture_creator.load_texture(path) {
                Ok(texture) => {
                    self.cache.insert(path.to_path_buf(), texture);
                }
                Err(e) => {
                    self.failed.insert(path.to_path_buf());
                    return Err(e);
                }
            }
        }
        Ok(&self.cache[path])
    }

    /// Clear all cached textures (call before dropping TextureCreator).
    pub fn clear(&mut self) {
        self.cache.clear();
        self.failed.clear();
    }

    /// Check if a texture is already cached.
    pub fn contains(&self, path: &Path) -> bool {
        self.cache.contains_key(path)
    }
}

impl std::fmt::Debug for TextureCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureCache")
            .field("cached_count", &self.cache.len())
            .finish()
    }
}
