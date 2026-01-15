//! Lip sync animation for character sprites.
//!
//! Provides timing-based mouth animation that syncs with voice playback.
//! Uses a simple oscillation pattern to simulate speech.

use serde::{Deserialize, Serialize};

/// Lip sync configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipSyncConfig {
    /// Speed of mouth movement (oscillations per second).
    #[serde(default = "default_speed")]
    pub speed: f32,
    /// Minimum mouth openness (0.0 - 1.0).
    #[serde(default)]
    pub min_openness: f32,
    /// Maximum mouth openness (0.0 - 1.0).
    #[serde(default = "default_max_openness")]
    pub max_openness: f32,
}

fn default_speed() -> f32 {
    8.0 // 8 oscillations per second
}

fn default_max_openness() -> f32 {
    1.0
}

impl Default for LipSyncConfig {
    fn default() -> Self {
        Self {
            speed: 8.0,
            min_openness: 0.0,
            max_openness: 1.0,
        }
    }
}

/// Lip sync state for a character.
#[derive(Debug, Clone, Default)]
pub struct LipSyncState {
    /// Whether voice is currently playing.
    is_speaking: bool,
    /// Elapsed time since voice started.
    elapsed: f32,
    /// Current mouth openness (0.0 - 1.0).
    openness: f32,
    /// Configuration.
    config: LipSyncConfig,
}

impl LipSyncState {
    /// Create a new lip sync state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with custom configuration.
    pub fn with_config(config: LipSyncConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// Start speaking (voice playback started).
    pub fn start_speaking(&mut self) {
        self.is_speaking = true;
        self.elapsed = 0.0;
    }

    /// Stop speaking (voice playback ended).
    pub fn stop_speaking(&mut self) {
        self.is_speaking = false;
        self.openness = 0.0;
    }

    /// Check if currently speaking.
    pub fn is_speaking(&self) -> bool {
        self.is_speaking
    }

    /// Update lip sync state.
    pub fn update(&mut self, delta: f32) {
        if !self.is_speaking {
            // Smoothly close mouth when not speaking
            self.openness = (self.openness - delta * 10.0).max(0.0);
            return;
        }

        self.elapsed += delta;

        // Use sine wave with some randomness for natural movement
        let base_wave = (self.elapsed * self.config.speed * std::f32::consts::TAU).sin();

        // Add a secondary faster wave for variation
        let secondary_wave = (self.elapsed * self.config.speed * 1.7 * std::f32::consts::TAU).sin() * 0.3;

        // Combine and normalize to 0-1 range
        let combined = (base_wave + secondary_wave + 1.3) / 2.6;

        // Map to min-max range
        self.openness = self.config.min_openness
            + combined * (self.config.max_openness - self.config.min_openness);
    }

    /// Get current mouth openness (0.0 = closed, 1.0 = fully open).
    pub fn openness(&self) -> f32 {
        self.openness
    }

    /// Get mouth sprite index for discrete mouth frames.
    /// Returns 0-2 for closed, half-open, and open.
    pub fn mouth_frame(&self) -> usize {
        if self.openness < 0.33 {
            0 // Closed
        } else if self.openness < 0.66 {
            1 // Half-open
        } else {
            2 // Open
        }
    }

    /// Get mouth sprite index for more frames (0 to frame_count-1).
    pub fn mouth_frame_n(&self, frame_count: usize) -> usize {
        if frame_count == 0 {
            return 0;
        }
        let frame = (self.openness * frame_count as f32) as usize;
        frame.min(frame_count - 1)
    }
}

/// Lip sync manager for multiple characters.
#[derive(Debug, Default)]
pub struct LipSyncManager {
    /// Lip sync states by character name.
    states: std::collections::HashMap<String, LipSyncState>,
    /// Default configuration for new characters.
    default_config: LipSyncConfig,
}

impl LipSyncManager {
    /// Create a new lip sync manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set default configuration for new characters.
    pub fn set_default_config(&mut self, config: LipSyncConfig) {
        self.default_config = config;
    }

    /// Start speaking for a character.
    pub fn start_speaking(&mut self, character: &str) {
        let state = self
            .states
            .entry(character.to_string())
            .or_insert_with(|| LipSyncState::with_config(self.default_config.clone()));
        state.start_speaking();
    }

    /// Stop speaking for a character.
    pub fn stop_speaking(&mut self, character: &str) {
        if let Some(state) = self.states.get_mut(character) {
            state.stop_speaking();
        }
    }

    /// Stop all characters from speaking.
    pub fn stop_all(&mut self) {
        for state in self.states.values_mut() {
            state.stop_speaking();
        }
    }

    /// Update all lip sync states.
    pub fn update(&mut self, delta: f32) {
        for state in self.states.values_mut() {
            state.update(delta);
        }
    }

    /// Get mouth openness for a character.
    pub fn openness(&self, character: &str) -> f32 {
        self.states
            .get(character)
            .map(|s| s.openness())
            .unwrap_or(0.0)
    }

    /// Get mouth frame for a character (0-2).
    pub fn mouth_frame(&self, character: &str) -> usize {
        self.states
            .get(character)
            .map(|s| s.mouth_frame())
            .unwrap_or(0)
    }

    /// Check if a character is speaking.
    pub fn is_speaking(&self, character: &str) -> bool {
        self.states
            .get(character)
            .map(|s| s.is_speaking())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lipsync_default_state() {
        let state = LipSyncState::new();
        assert!(!state.is_speaking());
        assert_eq!(state.openness(), 0.0);
    }

    #[test]
    fn test_lipsync_speaking() {
        let mut state = LipSyncState::new();
        state.start_speaking();
        assert!(state.is_speaking());

        // Update a few times
        for _ in 0..10 {
            state.update(0.016); // ~60fps
        }

        // Mouth should be moving
        assert!(state.openness() > 0.0);
    }

    #[test]
    fn test_lipsync_stop_speaking() {
        let mut state = LipSyncState::new();
        state.start_speaking();
        state.update(0.1);
        state.stop_speaking();

        assert!(!state.is_speaking());

        // After some updates, mouth should close
        for _ in 0..10 {
            state.update(0.1);
        }
        assert_eq!(state.openness(), 0.0);
    }

    #[test]
    fn test_mouth_frame() {
        let mut state = LipSyncState::new();
        assert_eq!(state.mouth_frame(), 0);

        state.openness = 0.5;
        assert_eq!(state.mouth_frame(), 1);

        state.openness = 0.9;
        assert_eq!(state.mouth_frame(), 2);
    }

    #[test]
    fn test_manager() {
        let mut manager = LipSyncManager::new();

        manager.start_speaking("sakura");
        assert!(manager.is_speaking("sakura"));
        assert!(!manager.is_speaking("yuki"));

        manager.update(0.1);
        assert!(manager.openness("sakura") > 0.0);

        manager.stop_all();
        assert!(!manager.is_speaking("sakura"));
    }
}
