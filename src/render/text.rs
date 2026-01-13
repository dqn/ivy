use macroquad::prelude::*;

/// A segment of text with color information.
#[derive(Debug, Clone)]
struct TextSegment {
    text: String,
    color: Option<Color>,
}

/// Parse color name or hex code to Color.
fn parse_color(name: &str) -> Option<Color> {
    // Check for hex color
    if name.starts_with('#') && name.len() == 7 {
        let r = u8::from_str_radix(&name[1..3], 16).ok()?;
        let g = u8::from_str_radix(&name[3..5], 16).ok()?;
        let b = u8::from_str_radix(&name[5..7], 16).ok()?;
        return Some(Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0));
    }

    // Named colors
    match name.to_lowercase().as_str() {
        "red" => Some(RED),
        "green" => Some(GREEN),
        "blue" => Some(BLUE),
        "yellow" => Some(YELLOW),
        "orange" => Some(ORANGE),
        "pink" => Some(PINK),
        "purple" => Some(PURPLE),
        "white" => Some(WHITE),
        "gray" | "grey" => Some(GRAY),
        "black" => Some(BLACK),
        "cyan" => Some(Color::new(0.0, 1.0, 1.0, 1.0)),
        "magenta" => Some(Color::new(1.0, 0.0, 1.0, 1.0)),
        "gold" => Some(GOLD),
        "lime" => Some(LIME),
        _ => None,
    }
}

/// Parse rich text with color tags into segments.
/// Supports: {color:red}, {color:#ff0000}, {/color}
fn parse_rich_text(text: &str, default_color: Color) -> Vec<TextSegment> {
    let mut segments = Vec::new();
    let mut current_text = String::new();
    let mut current_color: Option<Color> = None;
    let mut color_stack: Vec<Color> = Vec::new();

    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            // Check for color tag
            let mut tag = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '}' {
                    chars.next();
                    break;
                }
                tag.push(chars.next().unwrap());
            }

            // Process tag
            if tag.starts_with("color:") {
                // Save current segment if not empty
                if !current_text.is_empty() {
                    segments.push(TextSegment {
                        text: current_text.clone(),
                        color: current_color,
                    });
                    current_text.clear();
                }

                // Parse and push new color
                let color_name = &tag[6..];
                if let Some(color) = parse_color(color_name) {
                    if let Some(old_color) = current_color {
                        color_stack.push(old_color);
                    } else {
                        color_stack.push(default_color);
                    }
                    current_color = Some(color);
                }
            } else if tag == "/color" {
                // Save current segment if not empty
                if !current_text.is_empty() {
                    segments.push(TextSegment {
                        text: current_text.clone(),
                        color: current_color,
                    });
                    current_text.clear();
                }

                // Pop color from stack
                current_color = color_stack.pop();
            } else {
                // Not a recognized tag, keep as literal text
                current_text.push('{');
                current_text.push_str(&tag);
                current_text.push('}');
            }
        } else {
            current_text.push(ch);
        }
    }

    // Add remaining text
    if !current_text.is_empty() {
        segments.push(TextSegment {
            text: current_text,
            color: current_color,
        });
    }

    segments
}

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

/// Draw a text box with a character limit (for typewriter effect).
/// Returns the total number of visible characters (excluding tags).
pub fn draw_text_box_typewriter(
    config: &TextBoxConfig,
    text: &str,
    font: Option<&Font>,
    char_limit: usize,
) -> usize {
    draw_text_box_internal(config, text, font, Some(char_limit))
}

/// Strip color tags from text for measurement purposes.
fn strip_tags(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            let mut tag = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '}' {
                    chars.next();
                    break;
                }
                tag.push(chars.next().unwrap());
            }

            // Only skip recognized tags
            if !tag.starts_with("color:") && tag != "/color" {
                result.push('{');
                result.push_str(&tag);
                result.push('}');
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Draw a text box with the given text and optional custom font.
/// Supports rich text color tags: {color:red}, {color:#ff0000}, {/color}
pub fn draw_text_box_with_font(config: &TextBoxConfig, text: &str, font: Option<&Font>) {
    draw_text_box_internal(config, text, font, None);
}

/// Internal function for text box rendering with optional character limit.
/// Returns the total number of visible characters (excluding tags).
fn draw_text_box_internal(
    config: &TextBoxConfig,
    text: &str,
    font: Option<&Font>,
    char_limit: Option<usize>,
) -> usize {
    // Draw background
    draw_rectangle(config.x, config.y, config.width, config.height, config.bg_color);

    // Draw border
    draw_rectangle_lines(config.x, config.y, config.width, config.height, 2.0, WHITE);

    // Parse rich text into segments
    let segments = parse_rich_text(text, config.text_color);

    // Draw text with word wrapping
    let text_x = config.x + config.padding;
    let text_y = config.y + config.padding + config.font_size;
    let max_width = config.width - config.padding * 2.0;

    // Build character list with colors for proper wrapping
    let mut chars_with_colors: Vec<(char, Color)> = Vec::new();
    for segment in &segments {
        let color = segment.color.unwrap_or(config.text_color);
        for ch in segment.text.chars() {
            chars_with_colors.push((ch, color));
        }
    }

    let total_chars = chars_with_colors.len();

    // Apply character limit if specified
    let display_count = match char_limit {
        Some(limit) => limit.min(total_chars),
        None => total_chars,
    };

    // Simple character-based wrapping
    let mut current_line: Vec<(char, Color)> = Vec::new();
    let mut line_num = 0;
    let max_lines = ((config.height - config.padding * 2.0) / config.line_height) as usize;
    let mut chars_displayed = 0;

    for (ch, color) in chars_with_colors {
        // Stop if we've reached the character limit
        if chars_displayed >= display_count {
            break;
        }

        current_line.push((ch, color));
        chars_displayed += 1;

        // Measure current line width (plain text only)
        let plain_line: String = current_line.iter().map(|(c, _)| c).collect();
        let line_width = if let Some(f) = font {
            measure_text(&plain_line, Some(f), config.font_size as u16, 1.0).width
        } else {
            measure_text(&plain_line, None, config.font_size as u16, 1.0).width
        };

        // Check if we need to wrap
        if line_width > max_width || ch == '\n' {
            // Remove last character if it caused overflow (not newline)
            let overflow_char = if ch != '\n' && current_line.len() > 1 {
                current_line.pop()
            } else {
                None
            };

            // Draw the line with colors
            let y_pos = text_y + line_num as f32 * config.line_height;
            draw_colored_line(&current_line, text_x, y_pos, config.font_size, font);

            line_num += 1;
            if line_num >= max_lines {
                break;
            }

            // Start new line
            current_line = if let Some(oc) = overflow_char {
                vec![oc]
            } else {
                Vec::new()
            };
        }
    }

    // Draw remaining text
    if !current_line.is_empty() && line_num < max_lines {
        let y_pos = text_y + line_num as f32 * config.line_height;
        draw_colored_line(&current_line, text_x, y_pos, config.font_size, font);
    }

    total_chars
}

/// Draw a line of text with different colors for each character.
fn draw_colored_line(
    chars: &[(char, Color)],
    start_x: f32,
    y: f32,
    font_size: f32,
    font: Option<&Font>,
) {
    // Group consecutive characters with the same color
    let mut segments: Vec<(String, Color)> = Vec::new();

    for (ch, color) in chars {
        if let Some((text, last_color)) = segments.last_mut() {
            if last_color == color {
                text.push(*ch);
                continue;
            }
        }
        segments.push((ch.to_string(), *color));
    }

    // Draw each segment
    let mut x = start_x;
    for (text, color) in segments {
        if let Some(f) = font {
            draw_text_ex(
                &text,
                x,
                y,
                TextParams {
                    font: Some(f),
                    font_size: font_size as u16,
                    color,
                    ..Default::default()
                },
            );
            x += measure_text(&text, Some(f), font_size as u16, 1.0).width;
        } else {
            draw_text(&text, x, y, font_size, color);
            x += measure_text(&text, None, font_size as u16, 1.0).width;
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
