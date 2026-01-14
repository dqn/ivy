use macroquad::prelude::*;

/// Configuration for text input rendering.
pub struct InputConfig {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    pub bg_color: Color,
    pub input_bg_color: Color,
    pub text_color: Color,
    pub prompt_color: Color,
    pub cursor_color: Color,
    pub font_size: f32,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            x: 150.0,
            y: 200.0,
            width: 500.0,
            height: 200.0,
            padding: 20.0,
            bg_color: Color::new(0.1, 0.1, 0.2, 0.95),
            input_bg_color: Color::new(0.15, 0.15, 0.25, 1.0),
            text_color: WHITE,
            prompt_color: Color::new(0.8, 0.8, 0.8, 1.0),
            cursor_color: WHITE,
            font_size: 24.0,
        }
    }
}

/// State for text input.
pub struct InputState {
    pub text: String,
    pub cursor_pos: usize,
    pub cursor_blink: f32,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            text: String::new(),
            cursor_pos: 0,
            cursor_blink: 0.0,
        }
    }
}

impl InputState {
    /// Create a new input state with default text.
    pub fn new(default: Option<&str>) -> Self {
        let text = default.unwrap_or("").to_string();
        let cursor_pos = text.chars().count();
        Self {
            text,
            cursor_pos,
            cursor_blink: 0.0,
        }
    }

    /// Reset the input state.
    pub fn reset(&mut self, default: Option<&str>) {
        self.text = default.unwrap_or("").to_string();
        self.cursor_pos = self.text.chars().count();
        self.cursor_blink = 0.0;
    }
}

/// Result of drawing input.
pub struct InputResult {
    /// True if user submitted the input (pressed Enter).
    pub submitted: bool,
    /// True if user cancelled (pressed Escape).
    pub cancelled: bool,
}

/// Draw text input dialog and handle input.
pub fn draw_input(
    config: &InputConfig,
    state: &mut InputState,
    prompt: Option<&str>,
    font: Option<&Font>,
) -> InputResult {
    let mut submitted = false;
    let mut cancelled = false;

    // Update cursor blink
    state.cursor_blink += get_frame_time();
    if state.cursor_blink > 1.0 {
        state.cursor_blink = 0.0;
    }

    // Handle keyboard input
    handle_text_input(state);

    // Check for submit/cancel
    if is_key_pressed(KeyCode::Enter) {
        submitted = true;
    }
    if is_key_pressed(KeyCode::Escape) {
        cancelled = true;
    }

    // Draw dialog background
    draw_rectangle(
        config.x,
        config.y,
        config.width,
        config.height,
        config.bg_color,
    );
    draw_rectangle_lines(config.x, config.y, config.width, config.height, 2.0, WHITE);

    // Draw prompt text
    let prompt_text = prompt.unwrap_or("Enter your name:");
    let prompt_y = config.y + config.padding + config.font_size;

    if let Some(f) = font {
        draw_text_ex(
            prompt_text,
            config.x + config.padding,
            prompt_y,
            TextParams {
                font: Some(f),
                font_size: config.font_size as u16,
                color: config.prompt_color,
                ..Default::default()
            },
        );
    } else {
        draw_text(
            prompt_text,
            config.x + config.padding,
            prompt_y,
            config.font_size,
            config.prompt_color,
        );
    }

    // Draw input field background
    let input_y = prompt_y + config.padding;
    let input_width = config.width - config.padding * 2.0;
    let input_height = 40.0;

    draw_rectangle(
        config.x + config.padding,
        input_y,
        input_width,
        input_height,
        config.input_bg_color,
    );
    draw_rectangle_lines(
        config.x + config.padding,
        input_y,
        input_width,
        input_height,
        2.0,
        Color::new(0.4, 0.4, 0.5, 1.0),
    );

    // Draw input text
    let text_y = input_y + input_height - 10.0;
    let text_x = config.x + config.padding + 10.0;

    if let Some(f) = font {
        draw_text_ex(
            &state.text,
            text_x,
            text_y,
            TextParams {
                font: Some(f),
                font_size: config.font_size as u16,
                color: config.text_color,
                ..Default::default()
            },
        );
    } else {
        draw_text(
            &state.text,
            text_x,
            text_y,
            config.font_size,
            config.text_color,
        );
    }

    // Draw cursor
    if state.cursor_blink < 0.5 {
        let text_before_cursor: String = state.text.chars().take(state.cursor_pos).collect();
        let cursor_x = if let Some(f) = font {
            text_x + measure_text(&text_before_cursor, Some(f), config.font_size as u16, 1.0).width
        } else {
            text_x + measure_text(&text_before_cursor, None, config.font_size as u16, 1.0).width
        };

        draw_line(
            cursor_x,
            input_y + 8.0,
            cursor_x,
            input_y + input_height - 8.0,
            2.0,
            config.cursor_color,
        );
    }

    // Draw hint text
    let hint_y = input_y + input_height + config.padding + 16.0;
    let hint_text = "Press Enter to confirm, Escape to cancel";
    let hint_color = Color::new(0.5, 0.5, 0.5, 1.0);

    if let Some(f) = font {
        draw_text_ex(
            hint_text,
            config.x + config.padding,
            hint_y,
            TextParams {
                font: Some(f),
                font_size: 16,
                color: hint_color,
                ..Default::default()
            },
        );
    } else {
        draw_text(
            hint_text,
            config.x + config.padding,
            hint_y,
            16.0,
            hint_color,
        );
    }

    InputResult {
        submitted,
        cancelled,
    }
}

/// Handle text input from keyboard.
fn handle_text_input(state: &mut InputState) {
    // Get character input
    while let Some(ch) = get_char_pressed() {
        // Filter out control characters
        if ch.is_control() {
            continue;
        }

        // Insert character at cursor position
        let char_count = state.text.chars().count();
        if state.cursor_pos <= char_count {
            let mut new_text = String::new();
            for (i, c) in state.text.chars().enumerate() {
                if i == state.cursor_pos {
                    new_text.push(ch);
                }
                new_text.push(c);
            }
            if state.cursor_pos == char_count {
                new_text.push(ch);
            }
            state.text = new_text;
            state.cursor_pos += 1;
        }
    }

    // Handle backspace
    if is_key_pressed(KeyCode::Backspace) && state.cursor_pos > 0 {
        let mut new_text = String::new();
        for (i, c) in state.text.chars().enumerate() {
            if i != state.cursor_pos - 1 {
                new_text.push(c);
            }
        }
        state.text = new_text;
        state.cursor_pos -= 1;
    }

    // Handle delete
    if is_key_pressed(KeyCode::Delete) {
        let char_count = state.text.chars().count();
        if state.cursor_pos < char_count {
            let mut new_text = String::new();
            for (i, c) in state.text.chars().enumerate() {
                if i != state.cursor_pos {
                    new_text.push(c);
                }
            }
            state.text = new_text;
        }
    }

    // Handle arrow keys
    if is_key_pressed(KeyCode::Left) && state.cursor_pos > 0 {
        state.cursor_pos -= 1;
    }
    if is_key_pressed(KeyCode::Right) {
        let char_count = state.text.chars().count();
        if state.cursor_pos < char_count {
            state.cursor_pos += 1;
        }
    }

    // Handle Home/End
    if is_key_pressed(KeyCode::Home) {
        state.cursor_pos = 0;
    }
    if is_key_pressed(KeyCode::End) {
        state.cursor_pos = state.text.chars().count();
    }
}
