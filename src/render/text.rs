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
    pub line_height: f32,
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
            line_height: 32.0,
        }
    }
}

/// Draw a text box with the given text.
pub fn draw_text_box(config: &TextBoxConfig, text: &str) {
    draw_text_box_with_font(config, text, None);
}

/// Draw a text box with the given text and optional custom font.
pub fn draw_text_box_with_font(config: &TextBoxConfig, text: &str, font: Option<&Font>) {
    // Draw background
    draw_rectangle(config.x, config.y, config.width, config.height, config.bg_color);

    // Draw border
    draw_rectangle_lines(config.x, config.y, config.width, config.height, 2.0, WHITE);

    // Draw text with word wrapping
    let text_x = config.x + config.padding;
    let text_y = config.y + config.padding + config.font_size;
    let max_width = config.width - config.padding * 2.0;

    // Simple character-based wrapping for Japanese text
    let mut current_line = String::new();
    let mut line_num = 0;
    let max_lines = ((config.height - config.padding * 2.0) / config.line_height) as usize;

    for ch in text.chars() {
        current_line.push(ch);

        // Measure current line width
        let line_width = if let Some(f) = font {
            measure_text(&current_line, Some(f), config.font_size as u16, 1.0).width
        } else {
            measure_text(&current_line, None, config.font_size as u16, 1.0).width
        };

        // Check if we need to wrap
        if line_width > max_width || ch == '\n' {
            // Remove last character if it caused overflow (not newline)
            if ch != '\n' && current_line.len() > 1 {
                current_line.pop();
            }

            // Draw the line
            let y_pos = text_y + line_num as f32 * config.line_height;
            if let Some(f) = font {
                draw_text_ex(
                    &current_line,
                    text_x,
                    y_pos,
                    TextParams {
                        font: Some(f),
                        font_size: config.font_size as u16,
                        color: config.text_color,
                        ..Default::default()
                    },
                );
            } else {
                draw_text(&current_line, text_x, y_pos, config.font_size, config.text_color);
            }

            line_num += 1;
            if line_num >= max_lines {
                break;
            }

            // Start new line with the character that caused overflow
            current_line = if ch != '\n' { ch.to_string() } else { String::new() };
        }
    }

    // Draw remaining text
    if !current_line.is_empty() && line_num < max_lines {
        let y_pos = text_y + line_num as f32 * config.line_height;
        if let Some(f) = font {
            draw_text_ex(
                &current_line,
                text_x,
                y_pos,
                TextParams {
                    font: Some(f),
                    font_size: config.font_size as u16,
                    color: config.text_color,
                    ..Default::default()
                },
            );
        } else {
            draw_text(&current_line, text_x, y_pos, config.font_size, config.text_color);
        }
    }
}

/// Draw the speaker name box above the text box.
pub fn draw_speaker_name(config: &TextBoxConfig, speaker: &str, font: Option<&Font>) {
    let speaker_height = 30.0;
    let speaker_y = config.y - speaker_height - 5.0;
    let speaker_padding = 10.0;

    // Measure speaker name width
    let name_width = if let Some(f) = font {
        measure_text(speaker, Some(f), config.font_size as u16, 1.0).width
    } else {
        measure_text(speaker, None, config.font_size as u16, 1.0).width
    };

    let box_width = name_width + speaker_padding * 2.0;

    // Draw speaker name box
    draw_rectangle(
        config.x,
        speaker_y,
        box_width,
        speaker_height,
        config.bg_color,
    );
    draw_rectangle_lines(config.x, speaker_y, box_width, speaker_height, 2.0, YELLOW);

    // Draw speaker name
    if let Some(f) = font {
        draw_text_ex(
            speaker,
            config.x + speaker_padding,
            speaker_y + speaker_height - 8.0,
            TextParams {
                font: Some(f),
                font_size: config.font_size as u16,
                color: YELLOW,
                ..Default::default()
            },
        );
    } else {
        draw_text(
            speaker,
            config.x + speaker_padding,
            speaker_y + speaker_height - 8.0,
            config.font_size,
            YELLOW,
        );
    }
}

/// Draw a "click to continue" indicator.
pub fn draw_continue_indicator(config: &TextBoxConfig) {
    draw_continue_indicator_with_font(config, None);
}

/// Draw a "click to continue" indicator with optional custom font.
pub fn draw_continue_indicator_with_font(config: &TextBoxConfig, font: Option<&Font>) {
    let indicator = "▼";
    let x = config.x + config.width - config.padding - 20.0;
    let y = config.y + config.height - config.padding;

    // Simple blinking effect
    let alpha = ((get_time() * 3.0).sin() * 0.5 + 0.5) as f32;
    let color = Color::new(1.0, 1.0, 1.0, alpha);

    if let Some(f) = font {
        draw_text_ex(
            indicator,
            x,
            y,
            TextParams {
                font: Some(f),
                font_size: 20,
                color,
                ..Default::default()
            },
        );
    } else {
        draw_text(indicator, x, y, 20.0, color);
    }
}
