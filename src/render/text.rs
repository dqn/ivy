use macroquad::prelude::*;

use crate::runtime::Variables;

/// A segment of text with color and ruby information.
#[derive(Debug, Clone)]
struct TextSegment {
    text: String,
    color: Option<Color>,
    /// Ruby text (furigana) to display above the base text.
    ruby: Option<String>,
}

/// A text element that can be a single character or a ruby group.
#[derive(Debug, Clone)]
enum TextElement {
    /// Single character with color.
    Char(char, Color),
    /// Ruby group: base text with reading above.
    Ruby {
        base: String,
        reading: String,
        color: Color,
    },
}

/// Interpolate variables in text.
/// Replaces {var:name} with the variable value.
pub fn interpolate_variables(text: &str, variables: &Variables) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            // Check for var tag
            let mut tag = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '}' {
                    chars.next();
                    break;
                }
                tag.push(chars.next().unwrap());
            }

            // Check if it's a variable reference
            if tag.starts_with("var:") {
                let var_name = &tag[4..];
                if let Some(value) = variables.get(var_name) {
                    match value {
                        crate::runtime::Value::String(s) => result.push_str(s),
                        crate::runtime::Value::Int(i) => result.push_str(&i.to_string()),
                        crate::runtime::Value::Bool(b) => result.push_str(&b.to_string()),
                    }
                } else {
                    // Variable not found, keep original tag
                    result.push('{');
                    result.push_str(&tag);
                    result.push('}');
                }
            } else {
                // Not a var tag, keep as-is
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

/// Parse rich text with color and ruby tags into segments.
/// Supports: {color:red}, {color:#ff0000}, {/color}, {ruby:base:reading}
fn parse_rich_text(text: &str, default_color: Color) -> Vec<TextSegment> {
    let mut segments = Vec::new();
    let mut current_text = String::new();
    let mut current_color: Option<Color> = None;
    let mut color_stack: Vec<Color> = Vec::new();

    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            // Check for tag
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
                        ruby: None,
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
                        ruby: None,
                    });
                    current_text.clear();
                }

                // Pop color from stack
                current_color = color_stack.pop();
            } else if tag.starts_with("ruby:") {
                // Ruby tag: {ruby:base:reading}
                // Save current segment if not empty
                if !current_text.is_empty() {
                    segments.push(TextSegment {
                        text: current_text.clone(),
                        color: current_color,
                        ruby: None,
                    });
                    current_text.clear();
                }

                // Parse ruby tag
                let ruby_content = &tag[5..];
                if let Some(colon_pos) = ruby_content.find(':') {
                    let base = &ruby_content[..colon_pos];
                    let reading = &ruby_content[colon_pos + 1..];
                    segments.push(TextSegment {
                        text: base.to_string(),
                        color: current_color,
                        ruby: Some(reading.to_string()),
                    });
                } else {
                    // Malformed ruby tag, treat as plain text
                    current_text.push('{');
                    current_text.push_str(&tag);
                    current_text.push('}');
                }
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
            ruby: None,
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

/// Strip color and ruby tags from text for measurement purposes.
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

            // Handle ruby tags specially - extract base text
            if tag.starts_with("ruby:") {
                let ruby_content = &tag[5..];
                if let Some(colon_pos) = ruby_content.find(':') {
                    let base = &ruby_content[..colon_pos];
                    result.push_str(base);
                }
            } else if !tag.starts_with("color:") && tag != "/color" {
                // Unknown tags are kept as-is
                result.push('{');
                result.push_str(&tag);
                result.push('}');
            }
            // color tags and /color are simply skipped
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
    // Add extra space for ruby text at the top
    let ruby_space = 14.0;
    let text_y = config.y + config.padding + config.font_size + ruby_space;
    let max_width = config.width - config.padding * 2.0;

    // Build element list with colors and ruby for proper wrapping
    let mut elements: Vec<TextElement> = Vec::new();
    for segment in &segments {
        let color = segment.color.unwrap_or(config.text_color);
        if let Some(ref ruby) = segment.ruby {
            // Ruby group counts as base text length for display count
            elements.push(TextElement::Ruby {
                base: segment.text.clone(),
                reading: ruby.clone(),
                color,
            });
        } else {
            for ch in segment.text.chars() {
                elements.push(TextElement::Char(ch, color));
            }
        }
    }

    // Count total characters (base text for ruby groups)
    let total_chars: usize = elements
        .iter()
        .map(|e| match e {
            TextElement::Char(_, _) => 1,
            TextElement::Ruby { base, .. } => base.chars().count(),
        })
        .sum();

    // Apply character limit if specified
    let display_count = match char_limit {
        Some(limit) => limit.min(total_chars),
        None => total_chars,
    };

    // Simple element-based wrapping
    let mut current_line: Vec<TextElement> = Vec::new();
    let mut line_num = 0;
    let max_lines =
        ((config.height - config.padding * 2.0 - ruby_space) / config.line_height) as usize;
    let mut chars_displayed = 0;

    for element in elements {
        // Get element char count
        let elem_chars = match &element {
            TextElement::Char(_, _) => 1,
            TextElement::Ruby { base, .. } => base.chars().count(),
        };

        // Stop if we've reached the character limit
        if chars_displayed >= display_count {
            break;
        }

        // For ruby elements, we display the whole thing or nothing
        if chars_displayed + elem_chars > display_count {
            // Partial display not supported for ruby, skip
            if matches!(element, TextElement::Ruby { .. }) {
                break;
            }
        }

        current_line.push(element.clone());
        chars_displayed += elem_chars;

        // Measure current line width
        let line_width = measure_line_width(&current_line, config.font_size, font);

        // Check for newline character
        let is_newline = matches!(&element, TextElement::Char('\n', _));

        // Check if we need to wrap
        if line_width > max_width || is_newline {
            // Remove last element if it caused overflow (not newline)
            let overflow_elem = if !is_newline && current_line.len() > 1 {
                current_line.pop()
            } else {
                None
            };

            // Draw the line with colors and ruby
            let y_pos = text_y + line_num as f32 * config.line_height;
            draw_line_with_ruby(&current_line, text_x, y_pos, config.font_size, font);

            line_num += 1;
            if line_num >= max_lines {
                break;
            }

            // Start new line
            current_line = if let Some(elem) = overflow_elem {
                vec![elem]
            } else {
                Vec::new()
            };
        }
    }

    // Draw remaining text
    if !current_line.is_empty() && line_num < max_lines {
        let y_pos = text_y + line_num as f32 * config.line_height;
        draw_line_with_ruby(&current_line, text_x, y_pos, config.font_size, font);
    }

    total_chars
}

/// Measure the width of a line of text elements.
fn measure_line_width(elements: &[TextElement], font_size: f32, font: Option<&Font>) -> f32 {
    let mut width = 0.0;
    for element in elements {
        let text = match element {
            TextElement::Char(ch, _) => ch.to_string(),
            TextElement::Ruby { base, .. } => base.clone(),
        };
        width += measure_text(&text, font, font_size as u16, 1.0).width;
    }
    width
}

/// Draw a line of text elements with ruby support.
fn draw_line_with_ruby(
    elements: &[TextElement],
    start_x: f32,
    y: f32,
    font_size: f32,
    font: Option<&Font>,
) {
    let ruby_font_size = (font_size * 0.5).max(10.0);
    let ruby_offset = font_size * 0.6;

    let mut x = start_x;
    for element in elements {
        match element {
            TextElement::Char(ch, color) => {
                let text = ch.to_string();
                if let Some(f) = font {
                    draw_text_ex(
                        &text,
                        x,
                        y,
                        TextParams {
                            font: Some(f),
                            font_size: font_size as u16,
                            color: *color,
                            ..Default::default()
                        },
                    );
                    x += measure_text(&text, Some(f), font_size as u16, 1.0).width;
                } else {
                    draw_text(&text, x, y, font_size, *color);
                    x += measure_text(&text, None, font_size as u16, 1.0).width;
                }
            }
            TextElement::Ruby {
                base,
                reading,
                color,
            } => {
                // Measure base text width
                let base_width = measure_text(base, font, font_size as u16, 1.0).width;

                // Draw base text
                if let Some(f) = font {
                    draw_text_ex(
                        base,
                        x,
                        y,
                        TextParams {
                            font: Some(f),
                            font_size: font_size as u16,
                            color: *color,
                            ..Default::default()
                        },
                    );
                } else {
                    draw_text(base, x, y, font_size, *color);
                }

                // Measure ruby text width
                let ruby_width = measure_text(reading, font, ruby_font_size as u16, 1.0).width;

                // Center ruby above base text
                let ruby_x = x + (base_width - ruby_width) / 2.0;
                let ruby_y = y - ruby_offset;

                // Draw ruby text (slightly transparent)
                let ruby_color = Color::new(color.r, color.g, color.b, color.a * 0.9);
                if let Some(f) = font {
                    draw_text_ex(
                        reading,
                        ruby_x,
                        ruby_y,
                        TextParams {
                            font: Some(f),
                            font_size: ruby_font_size as u16,
                            color: ruby_color,
                            ..Default::default()
                        },
                    );
                } else {
                    draw_text(reading, ruby_x, ruby_y, ruby_font_size, ruby_color);
                }

                x += base_width;
            }
        }
    }
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
