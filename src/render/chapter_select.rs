use macroquad::prelude::*;

use crate::runtime::chapters::ChapterManager;

/// Chapter select screen configuration.
pub struct ChapterSelectConfig {
    pub title_font_size: f32,
    pub title_y: f32,
    pub item_font_size: f32,
    pub item_spacing: f32,
    pub start_y: f32,
    pub item_width: f32,
    pub item_height: f32,
    pub unlocked_color: Color,
    pub locked_color: Color,
    pub completed_color: Color,
    pub hover_color: Color,
    pub back_button_y: f32,
}

impl Default for ChapterSelectConfig {
    fn default() -> Self {
        Self {
            title_font_size: 36.0,
            title_y: 80.0,
            item_font_size: 20.0,
            item_spacing: 60.0,
            start_y: 150.0,
            item_width: 400.0,
            item_height: 50.0,
            unlocked_color: WHITE,
            locked_color: GRAY,
            completed_color: Color::new(0.5, 1.0, 0.5, 1.0),
            hover_color: YELLOW,
            back_button_y: 520.0,
        }
    }
}

/// Chapter select screen state.
#[derive(Debug, Default)]
pub struct ChapterSelectState {
    /// Scroll offset for long chapter lists.
    pub scroll_offset: f32,
}

/// Result of drawing the chapter select screen.
pub struct ChapterSelectResult {
    /// Selected chapter ID to start.
    pub selected: Option<String>,
    /// Whether back was pressed.
    pub back_pressed: bool,
}

/// Draw the chapter select screen.
pub fn draw_chapter_select(
    config: &ChapterSelectConfig,
    _state: &mut ChapterSelectState,
    manager: &ChapterManager,
    font: Option<&Font>,
) -> ChapterSelectResult {
    let screen_width = screen_width();
    let mut result = ChapterSelectResult {
        selected: None,
        back_pressed: false,
    };

    // Draw title
    let title = "Chapter Select";
    let title_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: config.title_font_size as u16,
            color: WHITE,
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: config.title_font_size as u16,
            color: WHITE,
            ..Default::default()
        }
    };

    let title_dim = measure_text(title, font, config.title_font_size as u16, 1.0);
    let title_x = (screen_width - title_dim.width) / 2.0;
    draw_text_ex(title, title_x, config.title_y, title_params);

    let mouse_pos = mouse_position();
    let chapters = manager.chapters();

    // Draw chapters
    let item_x = (screen_width - config.item_width) / 2.0;
    let mut y = config.start_y;

    for chapter in chapters {
        let is_unlocked = manager.is_unlocked(&chapter.id);
        let is_completed = manager.is_completed(&chapter.id);

        let item_rect = Rect::new(item_x, y, config.item_width, config.item_height);
        let is_hovered = is_unlocked && item_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));

        // Determine colors
        let (bg_color, text_color, border_color) = if !is_unlocked {
            (
                Color::new(0.15, 0.15, 0.15, 0.8),
                config.locked_color,
                DARKGRAY,
            )
        } else if is_hovered {
            (
                Color::new(0.3, 0.3, 0.4, 0.9),
                config.hover_color,
                config.hover_color,
            )
        } else if is_completed {
            (
                Color::new(0.2, 0.25, 0.2, 0.8),
                config.completed_color,
                Color::new(0.3, 0.6, 0.3, 1.0),
            )
        } else {
            (Color::new(0.2, 0.2, 0.25, 0.8), config.unlocked_color, GRAY)
        };

        // Draw item background
        draw_rectangle(item_x, y, config.item_width, config.item_height, bg_color);
        draw_rectangle_lines(
            item_x,
            y,
            config.item_width,
            config.item_height,
            2.0,
            border_color,
        );

        // Draw chapter title
        let text_params = if let Some(f) = font {
            TextParams {
                font: Some(f),
                font_size: config.item_font_size as u16,
                color: text_color,
                ..Default::default()
            }
        } else {
            TextParams {
                font_size: config.item_font_size as u16,
                color: text_color,
                ..Default::default()
            }
        };

        let display_title = if is_unlocked {
            chapter.title.clone()
        } else {
            "???".to_string()
        };

        let text_dim = measure_text(&display_title, font, config.item_font_size as u16, 1.0);
        let text_x = item_x + 15.0;
        let text_y = y + (config.item_height + text_dim.height) / 2.0 - 2.0;
        draw_text_ex(&display_title, text_x, text_y, text_params);

        // Draw completion marker
        if is_completed {
            let check_x = item_x + config.item_width - 30.0;
            let check_y = y + config.item_height / 2.0;
            draw_text("✓", check_x, check_y + 8.0, 24.0, config.completed_color);
        }

        // Handle click
        if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
            result.selected = Some(chapter.id.clone());
        }

        y += config.item_spacing;
    }

    // Draw back button
    let button_width = 150.0;
    let button_height = 40.0;
    let button_x = (screen_width - button_width) / 2.0;
    let button_y = config.back_button_y;

    let button_rect = Rect::new(button_x, button_y, button_width, button_height);
    let is_back_hovered = button_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));

    let bg_color = if is_back_hovered {
        Color::new(0.3, 0.3, 0.4, 0.9)
    } else {
        Color::new(0.2, 0.2, 0.25, 0.8)
    };
    draw_rectangle(button_x, button_y, button_width, button_height, bg_color);
    draw_rectangle_lines(
        button_x,
        button_y,
        button_width,
        button_height,
        2.0,
        if is_back_hovered { YELLOW } else { GRAY },
    );

    let back_text = "Back";
    let back_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: config.item_font_size as u16,
            color: if is_back_hovered { YELLOW } else { WHITE },
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: config.item_font_size as u16,
            color: if is_back_hovered { YELLOW } else { WHITE },
            ..Default::default()
        }
    };

    let back_dim = measure_text(back_text, font, config.item_font_size as u16, 1.0);
    let back_text_x = button_x + (button_width - back_dim.width) / 2.0;
    let back_text_y = button_y + (button_height + back_dim.height) / 2.0 - 2.0;
    draw_text_ex(back_text, back_text_x, back_text_y, back_params);

    if (is_back_hovered && is_mouse_button_pressed(MouseButton::Left))
        || is_key_pressed(KeyCode::Escape)
    {
        result.back_pressed = true;
    }

    result
}
