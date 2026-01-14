use macroquad::prelude::*;

/// Slider value display format.
pub enum SliderFormat {
    Percent,
    Value(&'static str),
    /// Display as multiplier (e.g., "1.5x")
    Multiplier,
}

/// Draw a checkbox and return the new value if toggled.
pub fn draw_checkbox(
    x: f32,
    y: f32,
    value: bool,
    label: &str,
    font: Option<&Font>,
    font_size: f32,
) -> bool {
    let box_size = 20.0;
    let box_y = y + 5.0;

    // Draw label
    let text_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: font_size as u16,
            color: WHITE,
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: font_size as u16,
            color: WHITE,
            ..Default::default()
        }
    };

    draw_text_ex(label, x, y, text_params);

    // Draw checkbox box
    let box_x = x + 200.0;
    draw_rectangle_lines(box_x, box_y, box_size, box_size, 2.0, WHITE);

    // Draw checkmark if checked
    if value {
        draw_rectangle(
            box_x + 4.0,
            box_y + 4.0,
            box_size - 8.0,
            box_size - 8.0,
            Color::new(0.3, 0.8, 0.3, 1.0),
        );
    }

    // Handle mouse click
    let mouse_pos = mouse_position();
    let checkbox_rect = Rect::new(box_x, box_y, box_size, box_size);

    if is_mouse_button_pressed(MouseButton::Left)
        && checkbox_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1))
    {
        return !value;
    }

    value
}

/// Draw a slider with extended formatting options and return the new value if changed.
#[allow(clippy::too_many_arguments)]
pub fn draw_slider_ex(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    value: f32,
    min: f32,
    max: f32,
    label: &str,
    format: SliderFormat,
    font: Option<&Font>,
    font_size: f32,
) -> f32 {
    // Draw label
    let text_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: font_size as u16,
            color: WHITE,
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: font_size as u16,
            color: WHITE,
            ..Default::default()
        }
    };

    draw_text_ex(label, x, y - 5.0, text_params);

    // Draw slider background
    draw_rectangle(x, y + 10.0, width, height, DARKGRAY);

    // Draw slider fill
    let fill_width = ((value - min) / (max - min)) * width;
    draw_rectangle(
        x,
        y + 10.0,
        fill_width,
        height,
        Color::new(0.3, 0.6, 0.9, 1.0),
    );

    // Draw slider border
    draw_rectangle_lines(x, y + 10.0, width, height, 2.0, WHITE);

    // Draw value text
    let value_text = match format {
        SliderFormat::Percent => format!("{:.0}%", value * 100.0),
        SliderFormat::Value(unit) => {
            if value == 0.0 {
                "Instant".to_string()
            } else {
                format!("{:.0}{}", value, unit)
            }
        }
        SliderFormat::Multiplier => format!("{:.1}x", value),
    };
    let value_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: (font_size * 0.8) as u16,
            color: WHITE,
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: (font_size * 0.8) as u16,
            color: WHITE,
            ..Default::default()
        }
    };
    draw_text_ex(&value_text, x + width + 10.0, y + 25.0, value_params);

    // Handle mouse interaction
    let mouse_pos = mouse_position();
    let slider_rect = Rect::new(x, y + 10.0, width, height);

    if is_mouse_button_down(MouseButton::Left)
        && slider_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1))
    {
        let new_value = min + ((mouse_pos.0 - x) / width) * (max - min);
        return new_value.clamp(min, max);
    }

    value
}

/// Draw a slider with percentage format and return the new value if changed.
#[allow(clippy::too_many_arguments)]
pub fn draw_slider(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    value: f32,
    min: f32,
    max: f32,
    label: &str,
    font: Option<&Font>,
    font_size: f32,
) -> f32 {
    draw_slider_ex(
        x,
        y,
        width,
        height,
        value,
        min,
        max,
        label,
        SliderFormat::Percent,
        font,
        font_size,
    )
}

/// Draw a button and return true if clicked.
pub fn draw_button(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: &str,
    font: Option<&Font>,
    font_size: f32,
) -> bool {
    let mouse_pos = mouse_position();
    let button_rect = Rect::new(x, y, width, height);
    let is_hovered = button_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));

    // Draw button background
    let bg_color = if is_hovered {
        Color::new(0.3, 0.3, 0.4, 0.9)
    } else {
        Color::new(0.2, 0.2, 0.25, 0.8)
    };
    draw_rectangle(x, y, width, height, bg_color);
    draw_rectangle_lines(
        x,
        y,
        width,
        height,
        2.0,
        if is_hovered { YELLOW } else { GRAY },
    );

    // Draw button text
    let text_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: font_size as u16,
            color: if is_hovered { YELLOW } else { WHITE },
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: font_size as u16,
            color: if is_hovered { YELLOW } else { WHITE },
            ..Default::default()
        }
    };

    let text_dim = measure_text(label, font, font_size as u16, 1.0);
    let text_x = x + (width - text_dim.width) / 2.0;
    let text_y = y + (height + text_dim.height) / 2.0 - text_dim.offset_y / 2.0;
    draw_text_ex(label, text_x, text_y, text_params);

    is_hovered && is_mouse_button_pressed(MouseButton::Left)
}
