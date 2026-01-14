use std::collections::HashSet;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::platform;

const READ_STATE_PATH: &str = "saves/read_state.json";

/// Read state for tracking which commands have been read.
/// Used for the "skip read only" feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadState {
    /// Set of read command keys (format: "scenario_path:index").
    #[serde(default)]
    pub read_indices: HashSet<String>,
}

impl ReadState {
    /// Create a new empty read state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load read state from file.
    pub fn load() -> Self {
        match Self::load_internal() {
            Ok(state) => state,
            Err(_) => Self::new(),
        }
    }

    fn load_internal() -> Result<Self> {
        let content = platform::read_file(READ_STATE_PATH)?;
        let state: ReadState = serde_json::from_str(&content)?;
        Ok(state)
    }

    /// Save read state to file.
    pub fn save(&self) {
        if let Err(e) = self.save_internal() {
            eprintln!("Failed to save read state: {}", e);
        }
    }

    fn save_internal(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        platform::write_file(READ_STATE_PATH, &json)?;
        Ok(())
    }

    /// Mark a command as read.
    /// Returns true if the command was not previously read.
    pub fn mark_read(&mut self, scenario_path: &str, index: usize) -> bool {
        let key = format!("{}:{}", scenario_path, index);
        let was_new = self.read_indices.insert(key);
        if was_new {
            self.save();
        }
        was_new
    }

    /// Check if a command has been read.
    pub fn is_read(&self, scenario_path: &str, index: usize) -> bool {
        let key = format!("{}:{}", scenario_path, index);
        self.read_indices.contains(&key)
    }

    /// Get the number of read commands.
    pub fn read_count(&self) -> usize {
        self.read_indices.len()
    }
}
