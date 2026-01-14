use macroquad::prelude::*;

/// Configuration for gallery rendering.
pub struct GalleryConfig {
    pub padding: f32,
    pub thumbnail_width: f32,
    pub thumbnail_height: f32,
    pub columns: usize,
    pub spacing: f32,
    pub bg_color: Color,
    pub border_color: Color,
    pub selected_color: Color,
    pub locked_color: Color,
    pub title_font_size: f32,
    pub info_font_size: f32,
}

impl Default for GalleryConfig {
    fn default() -> Self {
        Self {
            padding: 40.0,
            thumbnail_width: 160.0,
            thumbnail_height: 90.0,
            columns: 4,
            spacing: 20.0,
            bg_color: Color::new(0.1, 0.1, 0.15, 1.0),
            border_color: WHITE,
            selected_color: YELLOW,
            locked_color: Color::new(0.3, 0.3, 0.3, 1.0),
            title_font_size: 32.0,
            info_font_size: 18.0,
        }
    }
}

/// State for gallery navigation.
#[derive(Default)]
pub struct GalleryState {
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub viewing_image: bool,
}

/// Result of drawing gallery.
pub struct GalleryResult {
    /// True if user wants to go back.
    pub back_pressed: bool,
    /// Path of image to view fullscreen (if any).
    pub view_image: Option<String>,
}

/// Draw the CG gallery.
pub fn draw_gallery(
    config: &GalleryConfig,
    state: &mut GalleryState,
    images: &[String],
    textures: &std::collections::HashMap<String, Texture2D>,
    font: Option<&Font>,
) -> GalleryResult {
    let mut back_pressed = false;
    let view_image = None;

    let screen_w = screen_width();
    let screen_h = screen_height();

    // Draw background
    draw_rectangle(0.0, 0.0, screen_w, screen_h, config.bg_color);

    // Draw title
    let title = "CG Gallery";
    if let Some(f) = font {
        draw_text_ex(
            title,
            config.padding,
            config.padding + config.title_font_size,
            TextParams {
                font: Some(f),
                font_size: config.title_font_size as u16,
                color: WHITE,
                ..Default::default()
            },
        );
    } else {
        draw_text(
            title,
            config.padding,
            config.padding + config.title_font_size,
            config.title_font_size,
            WHITE,
        );
    }

    // Draw image count
    let count_text = format!("{} images unlocked", images.len());
    if let Some(f) = font {
        draw_text_ex(
            &count_text,
            config.padding,
            config.padding + config.title_font_size + 30.0,
            TextParams {
                font: Some(f),
                font_size: config.info_font_size as u16,
                color: GRAY,
                ..Default::default()
            },
        );
    } else {
        draw_text(
            &count_text,
            config.padding,
            config.padding + config.title_font_size + 30.0,
            config.info_font_size,
            GRAY,
        );
    }

    // Calculate grid area
    let grid_y = config.padding + config.title_font_size + 60.0;
    let grid_x = config.padding;

    // Calculate visible rows
    let available_height = screen_h - grid_y - config.padding;
    let row_height = config.thumbnail_height + config.spacing;
    let visible_rows = (available_height / row_height) as usize;

    // Handle input
    if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Backspace) {
        back_pressed = true;
    }

    // Navigate with arrow keys
    if is_key_pressed(KeyCode::Left) && state.selected_index > 0 {
        state.selected_index -= 1;
    }
    if is_key_pressed(KeyCode::Right) && state.selected_index < images.len().saturating_sub(1) {
        state.selected_index += 1;
    }
    if is_key_pressed(KeyCode::Up) && state.selected_index >= config.columns {
        state.selected_index -= config.columns;
    }
    if is_key_pressed(KeyCode::Down) && state.selected_index + config.columns < images.len() {
        state.selected_index += config.columns;
    }

    // Adjust scroll offset to keep selection visible
    let selected_row = state.selected_index / config.columns;
    if selected_row < state.scroll_offset {
        state.scroll_offset = selected_row;
    }
    if selected_row >= state.scroll_offset + visible_rows {
        state.scroll_offset = selected_row - visible_rows + 1;
    }

    // Draw thumbnails
    let start_idx = state.scroll_offset * config.columns;
    let end_idx = ((state.scroll_offset + visible_rows + 1) * config.columns).min(images.len());

    for (i, path) in images
        .iter()
        .enumerate()
        .skip(start_idx)
        .take(end_idx - start_idx)
    {
        let row = i / config.columns - state.scroll_offset;
        let col = i % config.columns;

        let x = grid_x + col as f32 * (config.thumbnail_width + config.spacing);
        let y = grid_y + row as f32 * row_height;

        // Check if visible
        if y + config.thumbnail_height < 0.0 || y > screen_h {
            continue;
        }

        // Check if selected
        let is_selected = i == state.selected_index;

        // Draw thumbnail background
        let bg_color = if is_selected {
            Color::new(0.2, 0.2, 0.3, 1.0)
        } else {
            Color::new(0.15, 0.15, 0.2, 1.0)
        };
        draw_rectangle(
            x,
            y,
            config.thumbnail_width,
            config.thumbnail_height,
            bg_color,
        );

        // Draw thumbnail image if loaded
        if let Some(texture) = textures.get(path) {
            // Scale to fit thumbnail
            let tex_w = texture.width();
            let tex_h = texture.height();
            let scale_x = config.thumbnail_width / tex_w;
            let scale_y = config.thumbnail_height / tex_h;
            let scale = scale_x.min(scale_y);

            let draw_w = tex_w * scale;
            let draw_h = tex_h * scale;
            let draw_x = x + (config.thumbnail_width - draw_w) / 2.0;
            let draw_y = y + (config.thumbnail_height - draw_h) / 2.0;

            draw_texture_ex(
                texture,
                draw_x,
                draw_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(draw_w, draw_h)),
                    ..Default::default()
                },
            );
        } else {
            // Draw placeholder
            let placeholder = "Loading...";
            let text_dim = measure_text(placeholder, None, 12, 1.0);
            draw_text(
                placeholder,
                x + (config.thumbnail_width - text_dim.width) / 2.0,
                y + config.thumbnail_height / 2.0,
                12.0,
                GRAY,
            );
        }

        // Draw border
        let border_color = if is_selected {
            config.selected_color
        } else {
            config.border_color
        };
        let border_width = if is_selected { 3.0 } else { 1.0 };
        draw_rectangle_lines(
            x,
            y,
            config.thumbnail_width,
            config.thumbnail_height,
            border_width,
            border_color,
        );

        // Handle click
        let mouse = mouse_position();
        if mouse.0 >= x
            && mouse.0 <= x + config.thumbnail_width
            && mouse.1 >= y
            && mouse.1 <= y + config.thumbnail_height
            && is_mouse_button_pressed(MouseButton::Left)
        {
            state.selected_index = i;
        }
    }

    // Draw scrollbar if needed
    let total_rows = images.len().div_ceil(config.columns);
    if total_rows > visible_rows {
        let scrollbar_x = screen_w - config.padding;
        let scrollbar_height = screen_h - grid_y - config.padding;
        let thumb_height = scrollbar_height * (visible_rows as f32 / total_rows as f32);
        let thumb_y = grid_y + scrollbar_height * (state.scroll_offset as f32 / total_rows as f32);

        // Draw scrollbar track
        draw_rectangle(scrollbar_x - 5.0, grid_y, 10.0, scrollbar_height, DARKGRAY);

        // Draw scrollbar thumb
        draw_rectangle(scrollbar_x - 5.0, thumb_y, 10.0, thumb_height, LIGHTGRAY);
    }

    // Draw hint at bottom
    let hint = "Arrow keys to navigate, Escape to go back";
    if let Some(f) = font {
        draw_text_ex(
            hint,
            config.padding,
            screen_h - config.padding,
            TextParams {
                font: Some(f),
                font_size: 14,
                color: GRAY,
                ..Default::default()
            },
        );
    } else {
        draw_text(hint, config.padding, screen_h - config.padding, 14.0, GRAY);
    }

    GalleryResult {
        back_pressed,
        view_image,
    }
}

/// Draw a fullscreen image view.
pub fn draw_fullscreen_image(texture: &Texture2D) -> bool {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Draw black background
    draw_rectangle(0.0, 0.0, screen_w, screen_h, BLACK);

    // Calculate scale to fit screen while maintaining aspect ratio
    let tex_w = texture.width();
    let tex_h = texture.height();
    let scale_x = screen_w / tex_w;
    let scale_y = screen_h / tex_h;
    let scale = scale_x.min(scale_y);

    let draw_w = tex_w * scale;
    let draw_h = tex_h * scale;
    let draw_x = (screen_w - draw_w) / 2.0;
    let draw_y = (screen_h - draw_h) / 2.0;

    draw_texture_ex(
        texture,
        draw_x,
        draw_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(draw_w, draw_h)),
            ..Default::default()
        },
    );

    // Close on click or escape
    is_mouse_button_pressed(MouseButton::Left)
        || is_key_pressed(KeyCode::Escape)
        || is_key_pressed(KeyCode::Enter)
}
