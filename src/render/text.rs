use macroquad::prelude::*;

/// Configuration for text box rendering.
pub struct TextBoxConfig {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    pub bg_color: Color,
    pub text_color: Color,
    pub font_size: f32,
}

impl Default for TextBoxConfig {
    fn default() -> Self {
        Self {
            x: 50.0,
            y: 400.0,
            width: 700.0,
            height: 150.0,
            padding: 20.0,
            bg_color: Color::new(0.0, 0.0, 0.0, 0.8),
            text_color: WHITE,
            font_size: 24.0,
        }
    }
}

/// Draw a text box with the given text.
pub fn draw_text_box(config: &TextBoxConfig, text: &str) {
    // Draw background
    draw_rectangle(config.x, config.y, config.width, config.height, config.bg_color);

    // Draw border
    draw_rectangle_lines(config.x, config.y, config.width, config.height, 2.0, WHITE);

    // Draw text
    let text_x = config.x + config.padding;
    let text_y = config.y + config.padding + config.font_size;

    draw_text(text, text_x, text_y, config.font_size, config.text_color);
}

/// Draw a "click to continue" indicator.
pub fn draw_continue_indicator(config: &TextBoxConfig) {
    let indicator = "▼";
    let x = config.x + config.width - config.padding - 20.0;
    let y = config.y + config.height - config.padding;

    // Simple blinking effect
    let alpha = ((get_time() * 3.0).sin() * 0.5 + 0.5) as f32;
    let color = Color::new(1.0, 1.0, 1.0, alpha);

    draw_text(indicator, x, y, 20.0, color);
}
