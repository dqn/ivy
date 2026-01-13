use macroquad::prelude::*;

/// Cinematic bar (letterbox) state.
#[derive(Debug, Clone)]
pub struct CinematicState {
    /// Target bar height (0 = off, > 0 = on).
    target_height: f32,
    /// Current bar height.
    current_height: f32,
    /// Animation duration.
    duration: f32,
    /// Animation timer.
    timer: f32,
    /// Starting height for animation.
    start_height: f32,
}

impl Default for CinematicState {
    fn default() -> Self {
        Self {
            target_height: 0.0,
            current_height: 0.0,
            duration: 0.5,
            timer: 0.0,
            start_height: 0.0,
        }
    }
}

impl CinematicState {
    /// Set cinematic bars on or off with animation.
    pub fn set(&mut self, enabled: bool, duration: f32) {
        let screen_h = screen_height();
        // Default bar height is 10% of screen height
        let bar_height = if enabled { screen_h * 0.1 } else { 0.0 };

        self.start_height = self.current_height;
        self.target_height = bar_height;
        self.duration = duration.max(0.01);
        self.timer = 0.0;
    }

    /// Set cinematic bars with custom height.
    pub fn set_height(&mut self, height: f32, duration: f32) {
        self.start_height = self.current_height;
        self.target_height = height.max(0.0);
        self.duration = duration.max(0.01);
        self.timer = 0.0;
    }

    /// Update animation state.
    pub fn update(&mut self) {
        if (self.current_height - self.target_height).abs() < 0.01 {
            self.current_height = self.target_height;
            return;
        }

        self.timer += get_frame_time();
        let t = (self.timer / self.duration).min(1.0);

        // Ease out cubic
        let ease = 1.0 - (1.0 - t).powi(3);

        self.current_height = self.start_height + (self.target_height - self.start_height) * ease;
    }

    /// Draw the cinematic bars.
    pub fn draw(&self) {
        if self.current_height <= 0.0 {
            return;
        }

        let screen_w = screen_width();
        let screen_h = screen_height();

        // Top bar
        draw_rectangle(0.0, 0.0, screen_w, self.current_height, BLACK);

        // Bottom bar
        draw_rectangle(
            0.0,
            screen_h - self.current_height,
            screen_w,
            self.current_height,
            BLACK,
        );
    }

    /// Check if cinematic mode is active.
    pub fn is_active(&self) -> bool {
        self.current_height > 0.0 || self.target_height > 0.0
    }

    /// Check if bars are fully deployed.
    pub fn is_deployed(&self) -> bool {
        self.target_height > 0.0 && (self.current_height - self.target_height).abs() < 0.01
    }
}
