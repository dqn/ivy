use macroquad::prelude::*;

use crate::scenario::{
    CharAnimation, CharAnimationType, CharIdleAnimation, CharIdleType, CharPosition, Easing,
};

/// Character animation direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationDirection {
    Enter,
    Exit,
}

/// Character animation state.
#[derive(Default)]
pub struct CharAnimationState {
    active: bool,
    direction: Option<AnimationDirection>,
    animation_type: CharAnimationType,
    duration: f32,
    elapsed: f32,
    easing: Easing,
}

impl CharAnimationState {
    /// Start an enter animation.
    pub fn start_enter(&mut self, anim: &CharAnimation) {
        self.active = true;
        self.direction = Some(AnimationDirection::Enter);
        self.animation_type = anim.animation_type;
        self.duration = anim.duration;
        self.elapsed = 0.0;
        self.easing = anim.easing;
    }

    /// Start an exit animation.
    pub fn start_exit(&mut self, anim: &CharAnimation) {
        self.active = true;
        self.direction = Some(AnimationDirection::Exit);
        self.animation_type = anim.animation_type;
        self.duration = anim.duration;
        self.elapsed = 0.0;
        self.easing = anim.easing;
    }

    /// Update the animation state.
    pub fn update(&mut self) {
        if !self.active {
            return;
        }

        self.elapsed += get_frame_time();

        if self.elapsed >= self.duration {
            self.active = false;
        }
    }

    /// Check if animation is complete.
    pub fn is_complete(&self) -> bool {
        !self.active
    }

    /// Check if animation is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the current animation progress (0.0 to 1.0) with easing applied.
    fn progress(&self) -> f32 {
        if self.duration <= 0.0 {
            return 1.0;
        }
        let raw_progress = (self.elapsed / self.duration).clamp(0.0, 1.0);
        self.easing.apply(raw_progress)
    }

    /// Get the current alpha value for fade animations.
    pub fn alpha(&self) -> f32 {
        if !self.active {
            return 1.0;
        }

        let progress = self.progress();

        match (self.animation_type, self.direction) {
            (CharAnimationType::Fade, Some(AnimationDirection::Enter)) => progress,
            (CharAnimationType::Fade, Some(AnimationDirection::Exit)) => 1.0 - progress,
            _ => 1.0,
        }
    }

    /// Get the current X offset for slide animations.
    pub fn offset_x(&self, position: CharPosition) -> f32 {
        if !self.active {
            return 0.0;
        }

        let progress = self.progress();
        let screen_width = screen_width();
        let slide_distance = screen_width * 0.5;

        match (self.animation_type, self.direction, position) {
            // Enter from left: start off-screen left, slide to position
            (CharAnimationType::SlideLeft, Some(AnimationDirection::Enter), _) => {
                -slide_distance * (1.0 - progress)
            }
            // Enter from right: start off-screen right, slide to position
            (CharAnimationType::SlideRight, Some(AnimationDirection::Enter), _) => {
                slide_distance * (1.0 - progress)
            }
            // Exit to left: slide off-screen left
            (CharAnimationType::SlideLeft, Some(AnimationDirection::Exit), _) => {
                -slide_distance * progress
            }
            // Exit to right: slide off-screen right
            (CharAnimationType::SlideRight, Some(AnimationDirection::Exit), _) => {
                slide_distance * progress
            }
            _ => 0.0,
        }
    }

    /// Reset the animation state.
    pub fn reset(&mut self) {
        self.active = false;
        self.direction = None;
        self.elapsed = 0.0;
    }

    /// Get the animation direction.
    pub fn direction(&self) -> Option<AnimationDirection> {
        self.direction
    }
}

/// Character idle animation state (looping).
#[derive(Default)]
pub struct CharIdleState {
    active: bool,
    idle_type: CharIdleType,
    duration: f32,
    intensity: f32,
    elapsed: f32,
    #[allow(dead_code)]
    easing: Easing,
}

impl CharIdleState {
    /// Start an idle animation.
    pub fn start(&mut self, idle: &CharIdleAnimation) {
        self.active = true;
        self.idle_type = idle.idle_type;
        self.duration = idle.duration;
        self.intensity = idle.intensity.clamp(0.0, 1.0);
        self.elapsed = 0.0;
        self.easing = idle.easing;
    }

    /// Stop the idle animation.
    pub fn stop(&mut self) {
        self.active = false;
        self.idle_type = CharIdleType::None;
    }

    /// Update the idle animation state (loops continuously).
    pub fn update(&mut self) {
        if !self.active || self.duration <= 0.0 {
            return;
        }

        self.elapsed += get_frame_time();
        // Loop: wrap elapsed time
        while self.elapsed >= self.duration {
            self.elapsed -= self.duration;
        }
    }

    /// Get the cycle value (-1.0 to 1.0) using sine wave for smooth looping.
    fn cycle_value(&self) -> f32 {
        if !self.active || self.duration <= 0.0 {
            return 0.0;
        }
        let raw_progress = self.elapsed / self.duration;
        // Use sine wave for smooth oscillation: sin(2*PI*t) gives -1 to 1
        (raw_progress * std::f32::consts::TAU).sin()
    }

    /// Get Y offset for Bob animation.
    pub fn offset_y(&self) -> f32 {
        if !self.active {
            return 0.0;
        }
        match self.idle_type {
            CharIdleType::Bob => {
                let amplitude = 8.0 * self.intensity; // max 8 pixels
                self.cycle_value() * amplitude
            }
            _ => 0.0,
        }
    }

    /// Get X offset for Sway animation.
    pub fn offset_x(&self) -> f32 {
        if !self.active {
            return 0.0;
        }
        match self.idle_type {
            CharIdleType::Sway => {
                let amplitude = 6.0 * self.intensity; // max 6 pixels
                self.cycle_value() * amplitude
            }
            _ => 0.0,
        }
    }

    /// Get scale factor for Breath/Pulse animations.
    pub fn scale(&self) -> f32 {
        if !self.active {
            return 1.0;
        }
        match self.idle_type {
            CharIdleType::Breath => {
                // Subtle vertical scale: 1.0 +/- 0.02*intensity
                let amplitude = 0.02 * self.intensity;
                1.0 + self.cycle_value() * amplitude
            }
            CharIdleType::Pulse => {
                // Uniform scale: 1.0 +/- 0.03*intensity
                let amplitude = 0.03 * self.intensity;
                1.0 + self.cycle_value() * amplitude
            }
            _ => 1.0,
        }
    }

    /// Check if idle animation is active.
    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// Draw a character with animation effects.
pub fn draw_character_animated(
    texture: &Texture2D,
    position: CharPosition,
    base_offset: (f32, f32),
    anim_state: &CharAnimationState,
    idle_state: &CharIdleState,
) {
    let screen_height = screen_height();
    let char_height = screen_height * 0.8;
    let scale = char_height / texture.height();
    let char_width = texture.width() * scale;

    // Apply idle scale
    let idle_scale = idle_state.scale();
    let final_char_height = char_height * idle_scale;
    let final_char_width = char_width * idle_scale;

    // Calculate base X position
    let base_x = match position {
        CharPosition::Left => screen_width() * 0.2 - final_char_width / 2.0,
        CharPosition::Center => (screen_width() - final_char_width) / 2.0,
        CharPosition::Right => screen_width() * 0.8 - final_char_width / 2.0,
    };

    let y = screen_height - final_char_height;

    // Apply animation offsets
    let anim_offset_x = anim_state.offset_x(position);
    let idle_offset_x = idle_state.offset_x();
    let idle_offset_y = idle_state.offset_y();

    let x = base_x + base_offset.0 + anim_offset_x + idle_offset_x;
    let final_y = y + base_offset.1 + idle_offset_y;

    // Apply animation alpha
    let alpha = anim_state.alpha();
    let color = Color::new(1.0, 1.0, 1.0, alpha);

    draw_texture_ex(
        texture,
        x,
        final_y,
        color,
        DrawTextureParams {
            dest_size: Some(Vec2::new(final_char_width, final_char_height)),
            ..Default::default()
        },
    );
}
