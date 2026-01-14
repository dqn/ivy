use macroquad::prelude::*;

use crate::i18n::LanguageConfig;
use crate::scenario::Choice;

/// Input source for choice navigation.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum InputSource {
    #[default]
    Mouse,
    Gamepad,
}

/// State for choice navigation (gamepad/keyboard).
#[derive(Default)]
pub struct ChoiceNavState {
    /// Currently focused choice index.
    pub focus_index: Option<usize>,
    /// Input source for visual feedback.
    pub input_source: InputSource,
    /// Debounce timer for analog stick.
    pub stick_debounce: f32,
}

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
pub fn draw_choices(
    config: &ChoiceButtonConfig,
    choices: &[Choice],
    lang: &LanguageConfig,
    nav_state: &ChoiceNavState,
) -> ChoiceResult {
    draw_choices_with_timer(config, choices, None, None, lang, nav_state)
}

/// Draw choice buttons with optional timer display.
pub fn draw_choices_with_timer(
    config: &ChoiceButtonConfig,
    choices: &[Choice],
    remaining_time: Option<f32>,
    default_choice: Option<usize>,
    lang: &LanguageConfig,
    nav_state: &ChoiceNavState,
) -> ChoiceResult {
    let mouse_pos = mouse_position();
    let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);

    // Draw timer bar if timeout is set
    if let Some(remaining) = remaining_time {
        let timer_y = config.start_y - 40.0;
        let timer_height = 20.0;
        let timer_width = config.width;

        // Background
        draw_rectangle(config.x, timer_y, timer_width, timer_height, DARKGRAY);

        // Progress bar (decreasing from right)
        let progress = remaining.max(0.0);
        let max_time = remaining_time.unwrap_or(1.0).max(0.001);
        // Estimate original timeout from remaining time (assumes this is called early enough)
        // For better accuracy, we'd pass the total timeout as well
        let fill_width = timer_width * (progress / max_time.max(progress));
        let bar_color = if progress <= 3.0 {
            Color::new(0.9, 0.2, 0.2, 1.0) // Red when low
        } else if progress <= 5.0 {
            Color::new(0.9, 0.6, 0.1, 1.0) // Orange when medium
        } else {
            Color::new(0.2, 0.7, 0.3, 1.0) // Green when plenty
        };
        draw_rectangle(config.x, timer_y, fill_width, timer_height, bar_color);

        // Border
        draw_rectangle_lines(config.x, timer_y, timer_width, timer_height, 2.0, WHITE);

        // Timer text
        let timer_text = format!("{:.1}s", progress);
        let text_dim = measure_text(&timer_text, None, 16, 1.0);
        draw_text(
            &timer_text,
            config.x + (timer_width - text_dim.width) / 2.0,
            timer_y + timer_height - 4.0,
            16.0,
            WHITE,
        );
    }

    let mut selected = None;

    for (i, choice) in choices.iter().enumerate() {
        let y = config.start_y + (config.height + config.spacing) * i as f32;

        // Check if mouse is hovering
        let is_hover = mouse_pos.0 >= config.x
            && mouse_pos.0 <= config.x + config.width
            && mouse_pos.1 >= y
            && mouse_pos.1 <= y + config.height;

        // Check if gamepad focus is on this choice
        let is_focused =
            nav_state.input_source == InputSource::Gamepad && nav_state.focus_index == Some(i);

        // Check if this is the default choice
        let is_default = default_choice == Some(i);

        // Determine background color (hover or focus)
        let bg_color = if is_hover || is_focused {
            config.hover_color
        } else if is_default && remaining_time.is_some() {
            // Highlight default choice when timer is active
            Color::new(0.25, 0.25, 0.35, 0.9)
        } else {
            config.bg_color
        };

        // Draw button background
        draw_rectangle(config.x, y, config.width, config.height, bg_color);

        // Draw border (highlight focus, then default choice)
        let border_color = if is_focused {
            // Cyan border for gamepad focus
            Color::new(0.4, 0.8, 1.0, 1.0)
        } else if is_default && remaining_time.is_some() {
            YELLOW
        } else {
            WHITE
        };
        draw_rectangle_lines(config.x, y, config.width, config.height, 2.0, border_color);

        // Draw button text (centered)
        let resolved_label = lang.resolve(&choice.label);
        let label = if is_default && remaining_time.is_some() {
            format!("{} [Default]", resolved_label)
        } else {
            resolved_label
        };
        let text_width = measure_text(&label, None, config.font_size as u16, 1.0).width;
        let text_x = config.x + (config.width - text_width) / 2.0;
        let text_y = y + (config.height + config.font_size) / 2.0 - 4.0;

        draw_text(&label, text_x, text_y, config.font_size, config.text_color);

        // Check for click
        if is_hover && mouse_clicked {
            selected = Some(i);
        }
    }

    ChoiceResult { selected }
}
