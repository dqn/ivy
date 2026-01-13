use macroquad::prelude::*;

use crate::scenario::TransitionType;

/// State for managing transition effects.
pub struct TransitionState {
    active: bool,
    transition_type: TransitionType,
    start_time: f64,
    duration: f32,
    /// Phase: 0 = fade out (to color), 1 = fade in (from color)
    phase: u8,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            active: false,
            transition_type: TransitionType::None,
            start_time: 0.0,
            duration: 0.5,
            phase: 0,
        }
    }
}

impl TransitionState {
    /// Start a new transition.
    pub fn start(&mut self, transition_type: TransitionType, duration: f32) {
        if matches!(transition_type, TransitionType::None) {
            return;
        }
        self.active = true;
        self.transition_type = transition_type;
        self.start_time = get_time();
        self.duration = duration;
        self.phase = 0;
    }

    /// Check if transition is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Update transition state. Returns true if transition is still active.
    pub fn update(&mut self) -> bool {
        if !self.active {
            return false;
        }

        let elapsed = (get_time() - self.start_time) as f32;
        let half_duration = self.duration / 2.0;

        if self.phase == 0 && elapsed >= half_duration {
            // Switch to fade-in phase
            self.phase = 1;
            self.start_time = get_time();
        } else if self.phase == 1 && elapsed >= half_duration {
            // Transition complete
            self.active = false;
            self.phase = 0;
        }

        self.active
    }

    /// Draw the transition overlay.
    pub fn draw(&self) {
        if !self.active {
            return;
        }

        let elapsed = (get_time() - self.start_time) as f32;
        let half_duration = self.duration / 2.0;
        let progress = (elapsed / half_duration).clamp(0.0, 1.0);

        let alpha = if self.phase == 0 {
            // Fade out: 0 -> 1
            progress
        } else {
            // Fade in: 1 -> 0
            1.0 - progress
        };

        let color = match self.transition_type {
            TransitionType::None => return,
            TransitionType::Fade => Color::new(0.0, 0.0, 0.0, alpha),
            TransitionType::FadeWhite => Color::new(1.0, 1.0, 1.0, alpha),
            TransitionType::Dissolve => Color::new(0.0, 0.0, 0.0, alpha * 0.8),
        };

        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), color);
    }

    /// Check if we're in the "content ready" phase (after fade-out, during fade-in).
    pub fn content_ready(&self) -> bool {
        !self.active || self.phase == 1
    }
}
