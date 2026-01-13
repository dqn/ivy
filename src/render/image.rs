use macroquad::prelude::*;

use crate::scenario::CharPosition;

/// Screen dimensions.
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

/// Draw a background image scaled to fill the screen.
pub fn draw_background(texture: &Texture2D) {
    draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
            ..Default::default()
        },
    );
}

/// Draw a character sprite at the specified position.
pub fn draw_character(texture: &Texture2D, position: CharPosition) {
    let texture_width = texture.width();
    let texture_height = texture.height();

    // Scale to fit reasonable size (max height ~500px, preserve aspect ratio)
    let max_height = 500.0;
    let scale = if texture_height > max_height {
        max_height / texture_height
    } else {
        1.0
    };

    let draw_width = texture_width * scale;
    let draw_height = texture_height * scale;

    // Calculate x position based on CharPosition
    let x = match position {
        CharPosition::Left => SCREEN_WIDTH * 0.15 - draw_width / 2.0,
        CharPosition::Center => SCREEN_WIDTH * 0.5 - draw_width / 2.0,
        CharPosition::Right => SCREEN_WIDTH * 0.85 - draw_width / 2.0,
    };

    // Position at bottom of screen (above text box area)
    let y = SCREEN_HEIGHT - draw_height - 160.0;

    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(draw_width, draw_height)),
            ..Default::default()
        },
    );
}
