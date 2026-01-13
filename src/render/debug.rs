use macroquad::prelude::*;

use crate::runtime::{GameState, Variables};

/// Debug overlay configuration.
pub struct DebugConfig {
    pub width: f32,
    pub padding: f32,
    pub bg_color: Color,
    pub border_color: Color,
    pub title_color: Color,
    pub label_color: Color,
    pub value_color: Color,
    pub font_size: f32,
    pub line_height: f32,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            width: 350.0,
            padding: 10.0,
            bg_color: Color::new(0.0, 0.0, 0.0, 0.85),
            border_color: GREEN,
            title_color: GREEN,
            label_color: YELLOW,
            value_color: WHITE,
            font_size: 14.0,
            line_height: 18.0,
        }
    }
}

/// Debug overlay state.
#[derive(Debug, Default)]
pub struct DebugState {
    /// Whether debug overlay is visible.
    pub visible: bool,
    /// Scroll offset for variables list.
    scroll_offset: f32,
}

impl DebugState {
    /// Toggle debug overlay visibility.
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Scroll the debug panel.
    pub fn scroll(&mut self, delta: f32) {
        self.scroll_offset = (self.scroll_offset + delta).max(0.0);
    }
}

/// Draw debug overlay.
pub fn draw_debug(
    config: &DebugConfig,
    state: &DebugState,
    game_state: &GameState,
    font: Option<&Font>,
) {
    if !state.visible {
        return;
    }

    let screen_h = screen_height();
    let height = screen_h - 40.0;
    let x = 10.0;
    let y = 10.0;

    // Draw background
    draw_rectangle(x, y, config.width, height, config.bg_color);
    draw_rectangle_lines(x, y, config.width, height, 2.0, config.border_color);

    // Helper to draw text
    let mut draw_y = y + config.padding + config.font_size;
    let text_x = x + config.padding;

    let draw_text_row = |label: &str, value: &str, y: f32| {
        let label_params = if let Some(f) = font {
            TextParams {
                font: Some(f),
                font_size: config.font_size as u16,
                color: config.label_color,
                ..Default::default()
            }
        } else {
            TextParams {
                font_size: config.font_size as u16,
                color: config.label_color,
                ..Default::default()
            }
        };
        let value_params = if let Some(f) = font {
            TextParams {
                font: Some(f),
                font_size: config.font_size as u16,
                color: config.value_color,
                ..Default::default()
            }
        } else {
            TextParams {
                font_size: config.font_size as u16,
                color: config.value_color,
                ..Default::default()
            }
        };

        draw_text_ex(label, text_x, y, label_params);
        let label_width = measure_text(label, font, config.font_size as u16, 1.0).width;
        draw_text_ex(value, text_x + label_width + 5.0, y, value_params);
    };

    // Title
    let title_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: (config.font_size + 2.0) as u16,
            color: config.title_color,
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: (config.font_size + 2.0) as u16,
            color: config.title_color,
            ..Default::default()
        }
    };
    draw_text_ex("DEBUG CONSOLE (F12)", text_x, draw_y, title_params);
    draw_y += config.line_height + 5.0;

    // Separator
    draw_line(
        text_x,
        draw_y - 5.0,
        x + config.width - config.padding,
        draw_y - 5.0,
        1.0,
        config.border_color,
    );
    draw_y += 5.0;

    // Game state info
    draw_text_row("Index: ", &game_state.current_index().to_string(), draw_y);
    draw_y += config.line_height;

    draw_text_row("Ended: ", &game_state.is_ended().to_string(), draw_y);
    draw_y += config.line_height;

    draw_text_row("Can Rollback: ", &game_state.can_rollback().to_string(), draw_y);
    draw_y += config.line_height;

    draw_text_row(
        "History Size: ",
        &game_state.history().len().to_string(),
        draw_y,
    );
    draw_y += config.line_height + 10.0;

    // Separator
    draw_line(
        text_x,
        draw_y - 5.0,
        x + config.width - config.padding,
        draw_y - 5.0,
        1.0,
        config.border_color,
    );
    draw_y += 5.0;

    // Variables section
    let vars_title_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: config.font_size as u16,
            color: config.title_color,
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: config.font_size as u16,
            color: config.title_color,
            ..Default::default()
        }
    };
    draw_text_ex("VARIABLES:", text_x, draw_y, vars_title_params);
    draw_y += config.line_height + 3.0;

    // Draw variables (with scroll)
    let variables = game_state.variables();
    draw_variables(config, variables, text_x, draw_y, config.width - config.padding * 2.0, font);
}

/// Draw variables list.
fn draw_variables(
    config: &DebugConfig,
    variables: &Variables,
    x: f32,
    start_y: f32,
    max_width: f32,
    font: Option<&Font>,
) {
    let mut y = start_y;

    // Get all variables and sort by name
    let mut vars: Vec<_> = variables.iter().collect();
    vars.sort_by(|a, b| a.0.cmp(b.0));

    if vars.is_empty() {
        let empty_params = if let Some(f) = font {
            TextParams {
                font: Some(f),
                font_size: config.font_size as u16,
                color: GRAY,
                ..Default::default()
            }
        } else {
            TextParams {
                font_size: config.font_size as u16,
                color: GRAY,
                ..Default::default()
            }
        };
        draw_text_ex("(no variables)", x, y, empty_params);
        return;
    }

    for (name, value) in vars {
        let name_params = if let Some(f) = font {
            TextParams {
                font: Some(f),
                font_size: config.font_size as u16,
                color: config.label_color,
                ..Default::default()
            }
        } else {
            TextParams {
                font_size: config.font_size as u16,
                color: config.label_color,
                ..Default::default()
            }
        };
        let value_params = if let Some(f) = font {
            TextParams {
                font: Some(f),
                font_size: config.font_size as u16,
                color: config.value_color,
                ..Default::default()
            }
        } else {
            TextParams {
                font_size: config.font_size as u16,
                color: config.value_color,
                ..Default::default()
            }
        };

        let value_str = match value {
            crate::runtime::Value::String(s) => format!("\"{}\"", s),
            crate::runtime::Value::Int(i) => i.to_string(),
            crate::runtime::Value::Bool(b) => b.to_string(),
        };

        let name_text = format!("{}: ", name);
        draw_text_ex(&name_text, x, y, name_params);
        let name_width = measure_text(&name_text, font, config.font_size as u16, 1.0).width;

        // Truncate value if too long
        let available_width = max_width - name_width;
        let mut display_value = value_str.clone();
        let value_width = measure_text(&display_value, font, config.font_size as u16, 1.0).width;
        if value_width > available_width {
            // Truncate with ellipsis
            while measure_text(&format!("{}...", display_value), font, config.font_size as u16, 1.0).width > available_width && !display_value.is_empty() {
                display_value.pop();
            }
            display_value.push_str("...");
        }

        draw_text_ex(&display_value, x + name_width, y, value_params);
        y += config.line_height;
    }
}
