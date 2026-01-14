use macroquad::prelude::*;

use crate::runtime::AchievementNotifier;

/// Configuration for achievement notification rendering.
pub struct AchievementConfig {
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    pub bg_color: Color,
    pub border_color: Color,
    pub title_color: Color,
    pub text_color: Color,
    pub title_size: f32,
    pub text_size: f32,
}

impl Default for AchievementConfig {
    fn default() -> Self {
        Self {
            width: 300.0,
            height: 80.0,
            padding: 15.0,
            bg_color: Color::new(0.1, 0.1, 0.2, 0.95),
            border_color: GOLD,
            title_color: GOLD,
            text_color: WHITE,
            title_size: 18.0,
            text_size: 14.0,
        }
    }
}

/// Draw achievement notification if active.
pub fn draw_achievement(
    config: &AchievementConfig,
    notifier: &AchievementNotifier,
    font: Option<&Font>,
) {
    let notif = match notifier.current() {
        Some(n) => n,
        None => return,
    };

    let progress = notifier.progress();
    if progress <= 0.0 {
        return;
    }

    let screen_w = screen_width();

    // Position at top right, slide in from right
    let target_x = screen_w - config.width - 20.0;
    let start_x = screen_w + 10.0;
    let x = start_x + (target_x - start_x) * ease_out_cubic(progress);
    let y = 20.0;

    // Draw background
    draw_rectangle(x, y, config.width, config.height, config.bg_color);
    draw_rectangle_lines(x, y, config.width, config.height, 2.0, config.border_color);

    // Draw trophy icon (simple star)
    let icon_x = x + config.padding;
    let icon_y = y + config.height / 2.0;
    draw_star(icon_x + 15.0, icon_y, 15.0, config.border_color);

    // Draw title
    let text_x = icon_x + 40.0;
    let title = "Achievement Unlocked!";
    let title_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: config.title_size as u16,
            color: config.title_color,
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: config.title_size as u16,
            color: config.title_color,
            ..Default::default()
        }
    };
    draw_text_ex(
        title,
        text_x,
        y + config.padding + config.title_size,
        title_params,
    );

    // Draw achievement name
    let name_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: config.text_size as u16,
            color: config.text_color,
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: config.text_size as u16,
            color: config.text_color,
            ..Default::default()
        }
    };
    draw_text_ex(
        &notif.name,
        text_x,
        y + config.padding + config.title_size + config.text_size + 8.0,
        name_params,
    );
}

/// Draw a simple star shape.
fn draw_star(cx: f32, cy: f32, size: f32, color: Color) {
    let points = 5;
    let inner_radius = size * 0.4;
    let outer_radius = size;

    for i in 0..points {
        let angle1 =
            (i as f32 * 2.0 * std::f32::consts::PI / points as f32) - std::f32::consts::PI / 2.0;
        let angle2 = ((i as f32 + 0.5) * 2.0 * std::f32::consts::PI / points as f32)
            - std::f32::consts::PI / 2.0;
        let angle3 = ((i as f32 + 1.0) * 2.0 * std::f32::consts::PI / points as f32)
            - std::f32::consts::PI / 2.0;

        let x1 = cx + outer_radius * angle1.cos();
        let y1 = cy + outer_radius * angle1.sin();
        let x2 = cx + inner_radius * angle2.cos();
        let y2 = cy + inner_radius * angle2.sin();
        let x3 = cx + outer_radius * angle3.cos();
        let y3 = cy + outer_radius * angle3.sin();

        draw_triangle(
            Vec2::new(cx, cy),
            Vec2::new(x1, y1),
            Vec2::new(x2, y2),
            color,
        );
        draw_triangle(
            Vec2::new(cx, cy),
            Vec2::new(x2, y2),
            Vec2::new(x3, y3),
            color,
        );
    }
}

/// Ease out cubic function.
fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}
