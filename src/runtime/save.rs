use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::platform;
use crate::runtime::{Variables, VisualState};

/// Save data format.
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveData {
    pub scenario_path: String,
    pub current_index: usize,
    pub visual: VisualState,
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub variables: Variables,
}

impl SaveData {
    /// Save to a JSON file (or localStorage on WASM).
    pub fn save(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        platform::write_file(path, &json)?;
        Ok(())
    }

    /// Load from a JSON file (or localStorage on WASM).
    pub fn load(path: &str) -> Result<Self> {
        let content = platform::read_file(path)?;
        let save: SaveData = serde_json::from_str(&content)?;
        Ok(save)
    }

    /// Get the path for a specific save slot.
    pub fn slot_path(slot: u8) -> String {
        format!("saves/slot_{}.json", slot)
    }

    /// Check if a save slot exists.
    pub fn slot_exists(slot: u8) -> bool {
        platform::file_exists(&Self::slot_path(slot))
    }

    /// List all existing save slots with their timestamps.
    pub fn list_slots() -> Vec<(u8, i64)> {
        (1..=10)
            .filter_map(|slot| {
                Self::load(&Self::slot_path(slot))
                    .ok()
                    .map(|save| (slot, save.timestamp))
            })
            .collect()
    }
}
