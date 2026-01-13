use macroquad::prelude::*;

use crate::scenario::Choice;

/// Configuration for choice button rendering.
pub struct ChoiceButtonConfig {
    pub x: f32,
    pub start_y: f32,
    pub width: f32,
    pub height: f32,
    pub spacing: f32,
    pub bg_color: Color,
    pub hover_color: Color,
    pub text_color: Color,
    pub font_size: f32,
}

impl Default for ChoiceButtonConfig {
    fn default() -> Self {
        Self {
            x: 200.0,
            start_y: 200.0,
            width: 400.0,
            height: 50.0,
            spacing: 10.0,
            bg_color: Color::new(0.2, 0.2, 0.4, 0.9),
            hover_color: Color::new(0.3, 0.3, 0.6, 0.9),
            text_color: WHITE,
            font_size: 20.0,
        }
    }
}

/// Result of drawing choices (which one was clicked, if any).
pub struct ChoiceResult {
    pub selected: Option<usize>,
}

/// Draw choice buttons and return which one was clicked.
pub fn draw_choices(config: &ChoiceButtonConfig, choices: &[Choice]) -> ChoiceResult {
    let mouse_pos = mouse_position();
    let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);

    let mut selected = None;

    for (i, choice) in choices.iter().enumerate() {
        let y = config.start_y + (config.height + config.spacing) * i as f32;

        // Check if mouse is hovering
        let is_hover = mouse_pos.0 >= config.x
            && mouse_pos.0 <= config.x + config.width
            && mouse_pos.1 >= y
            && mouse_pos.1 <= y + config.height;

        // Determine background color
        let bg_color = if is_hover {
            config.hover_color
        } else {
            config.bg_color
        };

        // Draw button background
        draw_rectangle(config.x, y, config.width, config.height, bg_color);
        draw_rectangle_lines(config.x, y, config.width, config.height, 2.0, WHITE);

        // Draw button text (centered)
        let text_width = measure_text(&choice.label, None, config.font_size as u16, 1.0).width;
        let text_x = config.x + (config.width - text_width) / 2.0;
        let text_y = y + (config.height + config.font_size) / 2.0 - 4.0;

        draw_text(&choice.label, text_x, text_y, config.font_size, config.text_color);

        // Check for click
        if is_hover && mouse_clicked {
            selected = Some(i);
        }
    }

    ChoiceResult { selected }
}
