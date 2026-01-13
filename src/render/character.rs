use macroquad::prelude::*;

use crate::scenario::{CharAnimation, CharAnimationType, CharPosition};

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
}

impl CharAnimationState {
    /// Start an enter animation.
    pub fn start_enter(&mut self, anim: &CharAnimation) {
        self.active = true;
        self.direction = Some(AnimationDirection::Enter);
        self.animation_type = anim.animation_type;
        self.duration = anim.duration;
        self.elapsed = 0.0;
    }

    /// Start an exit animation.
    pub fn start_exit(&mut self, anim: &CharAnimation) {
        self.active = true;
        self.direction = Some(AnimationDirection::Exit);
        self.animation_type = anim.animation_type;
        self.duration = anim.duration;
        self.elapsed = 0.0;
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

    /// Get the current animation progress (0.0 to 1.0).
    fn progress(&self) -> f32 {
        if self.duration <= 0.0 {
            return 1.0;
        }
        (self.elapsed / self.duration).clamp(0.0, 1.0)
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
}

/// Draw a character with animation effects.
pub fn draw_character_animated(
    texture: &Texture2D,
    position: CharPosition,
    base_offset: (f32, f32),
    anim_state: &CharAnimationState,
) {
    let screen_height = screen_height();
    let char_height = screen_height * 0.8;
    let scale = char_height / texture.height();
    let char_width = texture.width() * scale;

    // Calculate base X position
    let base_x = match position {
        CharPosition::Left => screen_width() * 0.2 - char_width / 2.0,
        CharPosition::Center => (screen_width() - char_width) / 2.0,
        CharPosition::Right => screen_width() * 0.8 - char_width / 2.0,
    };

    let y = screen_height - char_height;

    // Apply animation offset
    let anim_offset_x = anim_state.offset_x(position);
    let x = base_x + base_offset.0 + anim_offset_x;
    let final_y = y + base_offset.1;

    // Apply animation alpha
    let alpha = anim_state.alpha();
    let color = Color::new(1.0, 1.0, 1.0, alpha);

    draw_texture_ex(
        texture,
        x,
        final_y,
        color,
        DrawTextureParams {
            dest_size: Some(Vec2::new(char_width, char_height)),
            ..Default::default()
        },
    );
}
