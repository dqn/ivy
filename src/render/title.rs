use macroquad::prelude::*;

/// Title screen menu item.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TitleMenuItem {
    NewGame,
    Continue,
    Chapters,
    Gallery,
    Settings,
    Quit,
}

/// Title screen configuration.
pub struct TitleConfig {
    pub title_font_size: f32,
    pub title_y: f32,
    pub menu_font_size: f32,
    pub menu_start_y: f32,
    pub menu_spacing: f32,
    pub menu_item_width: f32,
    pub menu_item_height: f32,
}

impl Default for TitleConfig {
    fn default() -> Self {
        Self {
            title_font_size: 48.0,
            title_y: 150.0,
            menu_font_size: 24.0,
            menu_start_y: 300.0,
            menu_spacing: 60.0,
            menu_item_width: 200.0,
            menu_item_height: 40.0,
        }
    }
}

/// Result of drawing the title screen.
pub struct TitleResult {
    pub selected: Option<TitleMenuItem>,
}

/// Draw the title screen and return the selected menu item.
pub fn draw_title_screen(
    config: &TitleConfig,
    title: &str,
    has_save: bool,
    has_chapters: bool,
    has_gallery: bool,
    font: Option<&Font>,
) -> TitleResult {
    let screen_width = screen_width();

    // Draw title
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

    // Menu items
    let menu_items: Vec<(TitleMenuItem, &str, bool)> = vec![
        (TitleMenuItem::NewGame, "New Game", true),
        (TitleMenuItem::Continue, "Continue", has_save),
        (TitleMenuItem::Chapters, "Chapters", has_chapters),
        (TitleMenuItem::Gallery, "Gallery", has_gallery),
        (TitleMenuItem::Settings, "Settings", true),
        (TitleMenuItem::Quit, "Quit", true),
    ];

    let mouse_pos = mouse_position();
    let mut selected = None;

    for (i, (item, label, enabled)) in menu_items.iter().enumerate() {
        if !enabled {
            continue;
        }

        let y = config.menu_start_y + i as f32 * config.menu_spacing;
        let x = (screen_width - config.menu_item_width) / 2.0;

        let rect = Rect::new(x, y, config.menu_item_width, config.menu_item_height);
        let is_hovered = rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));

        // Draw button background
        let bg_color = if is_hovered {
            Color::new(0.3, 0.3, 0.4, 0.9)
        } else {
            Color::new(0.2, 0.2, 0.25, 0.8)
        };
        draw_rectangle(
            x,
            y,
            config.menu_item_width,
            config.menu_item_height,
            bg_color,
        );

        // Draw button border
        let border_color = if is_hovered { YELLOW } else { GRAY };
        draw_rectangle_lines(
            x,
            y,
            config.menu_item_width,
            config.menu_item_height,
            2.0,
            border_color,
        );

        // Draw button text
        let text_params = if let Some(f) = font {
            TextParams {
                font: Some(f),
                font_size: config.menu_font_size as u16,
                color: if is_hovered { YELLOW } else { WHITE },
                ..Default::default()
            }
        } else {
            TextParams {
                font_size: config.menu_font_size as u16,
                color: if is_hovered { YELLOW } else { WHITE },
                ..Default::default()
            }
        };

        let text_dim = measure_text(label, font, config.menu_font_size as u16, 1.0);
        let text_x = x + (config.menu_item_width - text_dim.width) / 2.0;
        let text_y =
            y + (config.menu_item_height + text_dim.height) / 2.0 - text_dim.offset_y / 2.0;
        draw_text_ex(label, text_x, text_y, text_params);

        // Check for click
        if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
            selected = Some(*item);
        }
    }

    TitleResult { selected }
}
