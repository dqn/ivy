use macroquad::prelude::*;

/// Entry in the NVL text buffer.
#[derive(Debug, Clone)]
pub struct NvlEntry {
    /// Speaker name (optional).
    pub speaker: Option<String>,
    /// Text content.
    pub text: String,
}

/// State for NVL mode text display.
#[derive(Debug, Clone, Default)]
pub struct NvlState {
    /// Accumulated text entries on the current page.
    entries: Vec<NvlEntry>,
    /// Whether NVL mode is currently active.
    pub active: bool,
}

impl NvlState {
    /// Create a new NVL state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear the text buffer (start a new page).
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Add a new entry to the buffer.
    pub fn push(&mut self, speaker: Option<String>, text: String) {
        self.entries.push(NvlEntry { speaker, text });
    }

    /// Get all entries.
    pub fn entries(&self) -> &[NvlEntry] {
        &self.entries
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Set active state.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
        if !active {
            self.clear();
        }
    }
}

/// Configuration for NVL text box rendering.
pub struct NvlConfig {
    /// Left padding from screen edge.
    pub padding_x: f32,
    /// Top padding from screen edge.
    pub padding_y: f32,
    /// Width of the text area.
    pub width: f32,
    /// Height of the text area.
    pub height: f32,
    /// Background color (semi-transparent).
    pub bg_color: Color,
    /// Default text color.
    pub text_color: Color,
    /// Speaker name color.
    pub speaker_color: Color,
    /// Font size for main text.
    pub font_size: f32,
    /// Line height.
    pub line_height: f32,
    /// Spacing between entries.
    pub entry_spacing: f32,
}

impl Default for NvlConfig {
    fn default() -> Self {
        Self {
            padding_x: 50.0,
            padding_y: 50.0,
            width: 700.0,
            height: 500.0,
            bg_color: Color::new(0.0, 0.0, 0.0, 0.85),
            text_color: WHITE,
            speaker_color: YELLOW,
            font_size: 22.0,
            line_height: 30.0,
            entry_spacing: 20.0,
        }
    }
}

/// Draw the NVL mode text box.
/// Returns the total number of visible characters for typewriter effect.
pub fn draw_nvl_text_box(
    config: &NvlConfig,
    state: &NvlState,
    current_speaker: Option<&str>,
    current_text: &str,
    font: Option<&Font>,
    char_limit: usize,
) -> usize {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Center the text box
    let box_x = (screen_width - config.width) / 2.0;
    let box_y = (screen_height - config.height) / 2.0;

    // Draw semi-transparent background
    draw_rectangle(
        box_x - config.padding_x,
        box_y - config.padding_y,
        config.width + config.padding_x * 2.0,
        config.height + config.padding_y * 2.0,
        config.bg_color,
    );

    // Draw border
    draw_rectangle_lines(
        box_x - config.padding_x,
        box_y - config.padding_y,
        config.width + config.padding_x * 2.0,
        config.height + config.padding_y * 2.0,
        2.0,
        Color::new(0.5, 0.5, 0.5, 0.8),
    );

    let mut y = box_y;
    let mut total_chars = 0;
    let mut chars_displayed = 0;

    // Draw previous entries (fully displayed)
    for entry in state.entries() {
        // Draw speaker name if present
        if let Some(ref speaker) = entry.speaker {
            draw_text_line(
                speaker,
                box_x,
                y,
                config.font_size,
                config.speaker_color,
                font,
            );
            y += config.line_height;
        }

        // Draw text with word wrapping
        let lines = wrap_text(&entry.text, config.width, config.font_size, font);
        for line in &lines {
            let line_chars = line.chars().count();
            draw_text_line(line, box_x, y, config.font_size, config.text_color, font);
            total_chars += line_chars;
            chars_displayed += line_chars;
            y += config.line_height;

            // Check if we've exceeded the visible area
            if y > box_y + config.height {
                break;
            }
        }

        y += config.entry_spacing;

        if y > box_y + config.height {
            break;
        }
    }

    // Draw current entry with typewriter effect
    if y <= box_y + config.height {
        // Draw speaker name if present
        if let Some(speaker) = current_speaker {
            draw_text_line(
                speaker,
                box_x,
                y,
                config.font_size,
                config.speaker_color,
                font,
            );
            y += config.line_height;
        }

        // Draw current text with character limit
        let lines = wrap_text(current_text, config.width, config.font_size, font);
        let mut remaining_chars = char_limit.saturating_sub(chars_displayed);

        for line in &lines {
            let line_chars = line.chars().count();
            total_chars += line_chars;

            if remaining_chars > 0 {
                let display_chars = remaining_chars.min(line_chars);
                let display_text: String = line.chars().take(display_chars).collect();
                draw_text_line(
                    &display_text,
                    box_x,
                    y,
                    config.font_size,
                    config.text_color,
                    font,
                );
                remaining_chars = remaining_chars.saturating_sub(line_chars);
            }

            y += config.line_height;

            if y > box_y + config.height {
                break;
            }
        }
    }

    total_chars
}

/// Draw a single line of text.
fn draw_text_line(text: &str, x: f32, y: f32, font_size: f32, color: Color, font: Option<&Font>) {
    if let Some(f) = font {
        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: Some(f),
                font_size: font_size as u16,
                color,
                ..Default::default()
            },
        );
    } else {
        draw_text(text, x, y, font_size, color);
    }
}

/// Wrap text to fit within a given width.
fn wrap_text(text: &str, max_width: f32, font_size: f32, font: Option<&Font>) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for ch in text.chars() {
        if ch == '\n' {
            lines.push(current_line.clone());
            current_line.clear();
            continue;
        }

        current_line.push(ch);
        let width = measure_text(&current_line, font, font_size as u16, 1.0).width;

        if width > max_width {
            // Remove last character and push line
            current_line.pop();
            if !current_line.is_empty() {
                lines.push(current_line.clone());
            }
            current_line = ch.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

/// Count visible characters in NVL entries plus current text.
pub fn count_nvl_chars(state: &NvlState, current_text: &str) -> usize {
    let mut total = 0;

    for entry in state.entries() {
        total += entry.text.chars().count();
    }

    total += current_text.chars().count();
    total
}
