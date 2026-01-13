use std::collections::HashSet;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::platform;

const UNLOCKS_PATH: &str = "saves/unlocks.json";

/// Unlock data for CG gallery and other collectibles.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Unlocks {
    /// Set of unlocked image paths.
    #[serde(default)]
    pub images: HashSet<String>,
    /// Set of unlocked ending IDs.
    #[serde(default)]
    pub endings: HashSet<String>,
}

impl Unlocks {
    /// Create a new empty unlocks state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load unlocks from file.
    pub fn load() -> Self {
        match Self::load_internal() {
            Ok(unlocks) => unlocks,
            Err(_) => Self::new(),
        }
    }

    fn load_internal() -> Result<Self> {
        let content = platform::read_file(UNLOCKS_PATH)?;
        let unlocks: Unlocks = serde_json::from_str(&content)?;
        Ok(unlocks)
    }

    /// Save unlocks to file.
    pub fn save(&self) {
        if let Err(e) = self.save_internal() {
            eprintln!("Failed to save unlocks: {}", e);
        }
    }

    fn save_internal(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        platform::write_file(UNLOCKS_PATH, &json)?;
        Ok(())
    }

    /// Unlock an image by path.
    pub fn unlock_image(&mut self, path: &str) -> bool {
        // Only track actual image files, not empty strings
        if path.is_empty() {
            return false;
        }
        let was_new = self.images.insert(path.to_string());
        if was_new {
            self.save();
        }
        was_new
    }

    /// Check if an image is unlocked.
    pub fn is_image_unlocked(&self, path: &str) -> bool {
        self.images.contains(path)
    }

    /// Unlock an ending by ID.
    pub fn unlock_ending(&mut self, id: &str) -> bool {
        let was_new = self.endings.insert(id.to_string());
        if was_new {
            self.save();
        }
        was_new
    }

    /// Check if an ending is unlocked.
    pub fn is_ending_unlocked(&self, id: &str) -> bool {
        self.endings.contains(id)
    }

    /// Get all unlocked images as a sorted vector.
    pub fn unlocked_images(&self) -> Vec<String> {
        let mut images: Vec<String> = self.images.iter().cloned().collect();
        images.sort();
        images
    }

    /// Get the number of unlocked images.
    pub fn image_count(&self) -> usize {
        self.images.len()
    }
}
