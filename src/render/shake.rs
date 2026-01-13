use macroquad::prelude::*;

use crate::scenario::{Shake, ShakeType};

/// Manages screen shake effect state.
#[derive(Default)]
pub struct ShakeState {
    active: bool,
    shake_type: ShakeType,
    intensity: f32,
    duration: f32,
    elapsed: f32,
}

impl ShakeState {
    /// Start a shake effect.
    pub fn start(&mut self, shake: &Shake) {
        self.active = true;
        self.shake_type = shake.shake_type;
        self.intensity = shake.intensity;
        self.duration = shake.duration;
        self.elapsed = 0.0;
    }

    /// Update the shake state.
    pub fn update(&mut self) {
        if !self.active {
            return;
        }

        self.elapsed += get_frame_time();
        if self.elapsed >= self.duration {
            self.active = false;
        }
    }

    /// Get the current offset to apply.
    pub fn offset(&self) -> (f32, f32) {
        if !self.active {
            return (0.0, 0.0);
        }

        // Calculate shake progress (0.0 to 1.0)
        let progress = self.elapsed / self.duration;

        // Decay intensity over time
        let decay = 1.0 - progress;
        let current_intensity = self.intensity * decay;

        // Generate pseudo-random offset using sine waves at different frequencies
        let time = get_time() as f32;
        let x_offset = match self.shake_type {
            ShakeType::Horizontal | ShakeType::Both => {
                (time * 50.0).sin() * current_intensity
            }
            ShakeType::Vertical => 0.0,
        };
        let y_offset = match self.shake_type {
            ShakeType::Vertical | ShakeType::Both => {
                (time * 60.0).cos() * current_intensity
            }
            ShakeType::Horizontal => 0.0,
        };

        (x_offset, y_offset)
    }

    /// Check if shake is currently active.
    pub fn is_active(&self) -> bool {
        self.active
    }
}
