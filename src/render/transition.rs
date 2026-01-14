use macroquad::prelude::*;

use crate::scenario::{Easing, TransitionDirection, TransitionType};

/// State for managing transition effects.
pub struct TransitionState {
    active: bool,
    transition_type: TransitionType,
    start_time: f64,
    duration: f32,
    /// Phase: 0 = fade out (to color), 1 = fade in (from color)
    phase: u8,
    easing: Easing,
    direction: TransitionDirection,
    blinds_count: u32,
    max_pixel_size: u32,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            active: false,
            transition_type: TransitionType::None,
            start_time: 0.0,
            duration: 0.5,
            phase: 0,
            easing: Easing::Linear,
            direction: TransitionDirection::LeftToRight,
            blinds_count: 10,
            max_pixel_size: 32,
        }
    }
}

impl TransitionState {
    /// Start a new transition with full configuration.
    pub fn start_with_config(
        &mut self,
        transition_type: TransitionType,
        duration: f32,
        easing: Easing,
        direction: TransitionDirection,
        blinds_count: u32,
        max_pixel_size: u32,
    ) {
        if matches!(transition_type, TransitionType::None) {
            return;
        }
        self.active = true;
        self.transition_type = transition_type;
        self.start_time = get_time();
        self.duration = duration;
        self.phase = 0;
        self.easing = easing;
        self.direction = direction;
        self.blinds_count = blinds_count;
        self.max_pixel_size = max_pixel_size;
    }

    /// Start a new transition (backwards compatible).
    pub fn start(&mut self, transition_type: TransitionType, duration: f32, easing: Easing) {
        self.start_with_config(
            transition_type,
            duration,
            easing,
            TransitionDirection::LeftToRight,
            10,
            32,
        );
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
        let raw_progress = (elapsed / half_duration).clamp(0.0, 1.0);
        let progress = self.easing.apply(raw_progress);

        match self.transition_type {
            TransitionType::None => {}
            TransitionType::Fade | TransitionType::FadeWhite | TransitionType::Dissolve => {
                self.draw_fade(progress);
            }
            TransitionType::Wipe => {
                self.draw_wipe(progress);
            }
            TransitionType::Slide => {
                self.draw_slide(progress);
            }
            TransitionType::Pixelate => {
                self.draw_pixelate(progress);
            }
            TransitionType::Iris => {
                self.draw_iris(progress);
            }
            TransitionType::Blinds => {
                self.draw_blinds(progress);
            }
        }
    }

    /// Draw fade-based transitions.
    fn draw_fade(&self, progress: f32) {
        let alpha = if self.phase == 0 {
            progress
        } else {
            1.0 - progress
        };

        let color = match self.transition_type {
            TransitionType::Fade => Color::new(0.0, 0.0, 0.0, alpha),
            TransitionType::FadeWhite => Color::new(1.0, 1.0, 1.0, alpha),
            TransitionType::Dissolve => Color::new(0.0, 0.0, 0.0, alpha * 0.8),
            _ => return,
        };

        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), color);
    }

    /// Draw wipe transition.
    fn draw_wipe(&self, progress: f32) {
        let w = screen_width();
        let h = screen_height();

        // Phase 0: wipe to black, Phase 1: wipe from black
        let wipe_progress = if self.phase == 0 {
            progress
        } else {
            1.0 - progress
        };

        // Draw black rectangle based on direction
        match self.direction {
            TransitionDirection::LeftToRight => {
                let edge = w * wipe_progress;
                draw_rectangle(0.0, 0.0, edge, h, BLACK);
            }
            TransitionDirection::RightToLeft => {
                let edge = w * (1.0 - wipe_progress);
                draw_rectangle(edge, 0.0, w - edge, h, BLACK);
            }
            TransitionDirection::TopToBottom => {
                let edge = h * wipe_progress;
                draw_rectangle(0.0, 0.0, w, edge, BLACK);
            }
            TransitionDirection::BottomToTop => {
                let edge = h * (1.0 - wipe_progress);
                draw_rectangle(0.0, edge, w, h - edge, BLACK);
            }
            _ => {
                // Default: left to right
                let edge = w * wipe_progress;
                draw_rectangle(0.0, 0.0, edge, h, BLACK);
            }
        }
    }

    /// Draw slide transition.
    fn draw_slide(&self, progress: f32) {
        let w = screen_width();
        let h = screen_height();

        // Phase 0: slide out, Phase 1: slide in (opposite direction)
        let slide_progress = if self.phase == 0 {
            progress
        } else {
            1.0 - progress
        };

        // Cover the screen with black, then use direction to "reveal"
        match self.direction {
            TransitionDirection::Left | TransitionDirection::LeftToRight => {
                let offset = w * slide_progress;
                draw_rectangle(w - offset, 0.0, offset, h, BLACK);
            }
            TransitionDirection::Right | TransitionDirection::RightToLeft => {
                let offset = w * slide_progress;
                draw_rectangle(0.0, 0.0, offset, h, BLACK);
            }
            TransitionDirection::Up | TransitionDirection::TopToBottom => {
                let offset = h * slide_progress;
                draw_rectangle(0.0, h - offset, w, offset, BLACK);
            }
            TransitionDirection::Down | TransitionDirection::BottomToTop => {
                let offset = h * slide_progress;
                draw_rectangle(0.0, 0.0, w, offset, BLACK);
            }
            _ => {
                // Default: slide from left
                let offset = w * slide_progress;
                draw_rectangle(w - offset, 0.0, offset, h, BLACK);
            }
        }
    }

    /// Draw pixelate transition (simplified version using rectangles).
    fn draw_pixelate(&self, progress: f32) {
        let w = screen_width();
        let h = screen_height();

        // Phase 0: increase pixelation, Phase 1: decrease pixelation
        let pixel_progress = if self.phase == 0 {
            progress
        } else {
            1.0 - progress
        };

        // Calculate pixel size based on progress
        let pixel_size = (self.max_pixel_size as f32 * pixel_progress).max(1.0) as u32;

        if pixel_size <= 1 {
            return;
        }

        // Draw a semi-transparent overlay with grid pattern
        let alpha = pixel_progress * 0.8;
        let color = Color::new(0.0, 0.0, 0.0, alpha);

        // Draw grid of squares to simulate pixelation
        let cols = (w / pixel_size as f32).ceil() as u32;
        let rows = (h / pixel_size as f32).ceil() as u32;

        for row in 0..rows {
            for col in 0..cols {
                // Create checkerboard pattern for visual effect
                if (row + col) % 2 == 0 {
                    let x = col as f32 * pixel_size as f32;
                    let y = row as f32 * pixel_size as f32;
                    draw_rectangle(x, y, pixel_size as f32, pixel_size as f32, color);
                }
            }
        }
    }

    /// Draw iris transition (circular reveal).
    fn draw_iris(&self, progress: f32) {
        let w = screen_width();
        let h = screen_height();
        let center_x = w / 2.0;
        let center_y = h / 2.0;

        // Maximum radius to cover entire screen
        let max_radius = (center_x * center_x + center_y * center_y).sqrt();

        // Iris progress based on direction
        let iris_progress = match self.direction {
            TransitionDirection::Open => {
                if self.phase == 0 {
                    1.0 - progress // Close during phase 0
                } else {
                    progress // Open during phase 1
                }
            }
            TransitionDirection::Close | _ => {
                if self.phase == 0 {
                    1.0 - progress
                } else {
                    progress
                }
            }
        };

        let radius = max_radius * iris_progress;

        // Draw black overlay with circular hole
        // Since macroquad doesn't have a native "ring" or "mask", we approximate
        // by drawing many rectangles around the circle

        // For a clean implementation, draw black rectangles in corners
        // This is a simplified version - for full effect, use stencil buffer
        if radius < max_radius {
            let alpha = 1.0;
            let color = Color::new(0.0, 0.0, 0.0, alpha);

            // Draw rectangles to cover area outside the circle
            // Top rectangle
            if center_y - radius > 0.0 {
                draw_rectangle(0.0, 0.0, w, center_y - radius, color);
            }
            // Bottom rectangle
            if center_y + radius < h {
                draw_rectangle(0.0, center_y + radius, w, h - (center_y + radius), color);
            }
            // Left rectangle (within circle bounds)
            if center_x - radius > 0.0 {
                draw_rectangle(0.0, 0.0, center_x - radius, h, color);
            }
            // Right rectangle (within circle bounds)
            if center_x + radius < w {
                draw_rectangle(center_x + radius, 0.0, w - (center_x + radius), h, color);
            }

            // Fill the area outside the circle by drawing segments
            let segments = 32;
            for i in 0..segments {
                let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;

                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();

                // Draw small rectangles along the edge to approximate masking
                if x >= 0.0 && x <= w && y >= 0.0 && y <= h {
                    let edge_x = if x < center_x { 0.0 } else { x };
                    let edge_y = if y < center_y { 0.0 } else { y };
                    let rect_w = if x < center_x { x } else { w - x };
                    let rect_h = if y < center_y { y } else { h - y };

                    if rect_w > 0.0 && rect_h > 0.0 {
                        draw_rectangle(edge_x, edge_y, rect_w, rect_h, color);
                    }
                }
            }
        }
    }

    /// Draw blinds transition.
    fn draw_blinds(&self, progress: f32) {
        let w = screen_width();
        let h = screen_height();
        let count = self.blinds_count.max(1);

        // Phase 0: blinds close, Phase 1: blinds open
        let blind_progress = if self.phase == 0 {
            progress
        } else {
            1.0 - progress
        };

        match self.direction {
            TransitionDirection::Horizontal => {
                let blind_height = h / count as f32;
                for i in 0..count {
                    let y = i as f32 * blind_height;
                    let covered_height = blind_height * blind_progress;
                    draw_rectangle(0.0, y, w, covered_height, BLACK);
                }
            }
            TransitionDirection::Vertical | _ => {
                let blind_width = w / count as f32;
                for i in 0..count {
                    let x = i as f32 * blind_width;
                    let covered_width = blind_width * blind_progress;
                    draw_rectangle(x, 0.0, covered_width, h, BLACK);
                }
            }
        }
    }

    /// Check if we're in the "content ready" phase (after fade-out, during fade-in).
    pub fn content_ready(&self) -> bool {
        !self.active || self.phase == 1
    }
}
