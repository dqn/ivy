use macroquad::prelude::*;

use crate::runtime::HistoryEntry;

/// Configuration for backlog display.
pub struct BacklogConfig {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub line_height: f32,
    pub font_size: f32,
    pub padding: f32,
    pub background_color: Color,
    pub text_color: Color,
}

impl Default for BacklogConfig {
    fn default() -> Self {
        Self {
            x: 50.0,
            y: 50.0,
            width: 700.0,
            height: 500.0,
            line_height: 30.0,
            font_size: 20.0,
            padding: 20.0,
            background_color: Color::new(0.0, 0.0, 0.0, 0.9),
            text_color: Color::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

/// Backlog state for scroll position.
pub struct BacklogState {
    pub scroll_offset: f32,
}

impl Default for BacklogState {
    fn default() -> Self {
        Self { scroll_offset: 0.0 }
    }
}

impl BacklogState {
    /// Handle scroll input (mouse wheel).
    pub fn handle_scroll(&mut self, history_len: usize, config: &BacklogConfig) {
        let wheel = mouse_wheel();
        let total_height = history_len as f32 * config.line_height;
        let visible_height = config.height - config.padding * 2.0;
        let max_scroll = (total_height - visible_height).max(0.0);

        self.scroll_offset -= wheel.1 * 30.0;
        self.scroll_offset = self.scroll_offset.clamp(0.0, max_scroll);
    }
}

/// Draw the backlog overlay.
pub fn draw_backlog(config: &BacklogConfig, state: &mut BacklogState, history: &[HistoryEntry]) {
    // Handle scroll
    state.handle_scroll(history.len(), config);

    // Draw background overlay
    draw_rectangle(
        config.x,
        config.y,
        config.width,
        config.height,
        config.background_color,
    );

    // Draw border
    draw_rectangle_lines(config.x, config.y, config.width, config.height, 2.0, WHITE);

    // Draw title
    draw_text(
        "[ Backlog - Press L to close ]",
        config.x + config.padding,
        config.y + config.padding + config.font_size,
        config.font_size,
        GRAY,
    );

    // Calculate visible area
    let content_y = config.y + config.padding + config.font_size + 20.0;
    let content_height = config.height - config.padding * 2.0 - config.font_size - 20.0;

    // Draw history entries (newest at bottom)
    let entries: Vec<&HistoryEntry> = history.iter().collect();
    let total_height = entries.len() as f32 * config.line_height;

    for (i, entry) in entries.iter().enumerate() {
        if entry.text.is_empty() {
            continue;
        }

        let y_pos = content_y + (i as f32 * config.line_height) - state.scroll_offset;

        // Only draw visible entries
        if y_pos < content_y - config.line_height
            || y_pos > config.y + config.height - config.padding
        {
            continue;
        }

        // Truncate long text
        let display_text = if entry.text.len() > 60 {
            format!("{}...", &entry.text[..60])
        } else {
            entry.text.clone()
        };

        draw_text(
            &display_text,
            config.x + config.padding,
            y_pos,
            config.font_size,
            config.text_color,
        );
    }

    // Draw scroll indicator if needed
    if total_height > content_height {
        let scroll_ratio = state.scroll_offset / (total_height - content_height);
        let indicator_height = 50.0;
        let indicator_y = config.y
            + config.padding
            + scroll_ratio * (config.height - config.padding * 2.0 - indicator_height);

        draw_rectangle(
            config.x + config.width - 10.0,
            indicator_y,
            5.0,
            indicator_height,
            Color::new(1.0, 1.0, 1.0, 0.5),
        );
    }
}
