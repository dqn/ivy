use std::fs;
use std::path::Path;

use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "config.json";

/// Game settings that can be adjusted by the player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    /// BGM volume (0.0 - 1.0).
    #[serde(default = "default_volume")]
    pub bgm_volume: f32,
    /// SE volume (0.0 - 1.0).
    #[serde(default = "default_volume")]
    pub se_volume: f32,
    /// Voice volume (0.0 - 1.0).
    #[serde(default = "default_volume")]
    pub voice_volume: f32,
    /// Auto mode wait time multiplier (0.5 - 2.0).
    #[serde(default = "default_auto_speed")]
    pub auto_speed: f32,
}

fn default_volume() -> f32 {
    1.0
}

fn default_auto_speed() -> f32 {
    1.0
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            bgm_volume: 1.0,
            se_volume: 1.0,
            voice_volume: 1.0,
            auto_speed: 1.0,
        }
    }
}

impl GameSettings {
    /// Load settings from config file.
    pub fn load() -> Self {
        if !Path::new(CONFIG_PATH).exists() {
            return Self::default();
        }

        match fs::read_to_string(CONFIG_PATH) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save settings to config file.
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(CONFIG_PATH, json);
        }
    }
}

/// Settings screen configuration.
pub struct SettingsConfig {
    pub title_font_size: f32,
    pub title_y: f32,
    pub label_font_size: f32,
    pub slider_start_y: f32,
    pub slider_spacing: f32,
    pub slider_width: f32,
    pub slider_height: f32,
    pub back_button_y: f32,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            title_font_size: 36.0,
            title_y: 80.0,
            label_font_size: 20.0,
            slider_start_y: 150.0,
            slider_spacing: 70.0,
            slider_width: 300.0,
            slider_height: 20.0,
            back_button_y: 500.0,
        }
    }
}

/// Result of drawing the settings screen.
pub struct SettingsResult {
    pub back_pressed: bool,
}

/// Draw a slider and return the new value if changed.
fn draw_slider(
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
    draw_rectangle(x, y + 10.0, fill_width, height, Color::new(0.3, 0.6, 0.9, 1.0));

    // Draw slider border
    draw_rectangle_lines(x, y + 10.0, width, height, 2.0, WHITE);

    // Draw value text
    let value_text = format!("{:.0}%", value * 100.0);
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

    if is_mouse_button_down(MouseButton::Left) && slider_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
        let new_value = min + ((mouse_pos.0 - x) / width) * (max - min);
        return new_value.clamp(min, max);
    }

    value
}

/// Draw the settings screen.
pub fn draw_settings_screen(
    config: &SettingsConfig,
    settings: &mut GameSettings,
    font: Option<&Font>,
) -> SettingsResult {
    let screen_width = screen_width();

    // Draw title
    let title = "Settings";
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

    // Calculate slider position
    let slider_x = (screen_width - config.slider_width - 80.0) / 2.0;

    // BGM Volume
    let mut y = config.slider_start_y;
    settings.bgm_volume = draw_slider(
        slider_x,
        y,
        config.slider_width,
        config.slider_height,
        settings.bgm_volume,
        0.0,
        1.0,
        "BGM Volume",
        font,
        config.label_font_size,
    );

    // SE Volume
    y += config.slider_spacing;
    settings.se_volume = draw_slider(
        slider_x,
        y,
        config.slider_width,
        config.slider_height,
        settings.se_volume,
        0.0,
        1.0,
        "SE Volume",
        font,
        config.label_font_size,
    );

    // Voice Volume
    y += config.slider_spacing;
    settings.voice_volume = draw_slider(
        slider_x,
        y,
        config.slider_width,
        config.slider_height,
        settings.voice_volume,
        0.0,
        1.0,
        "Voice Volume",
        font,
        config.label_font_size,
    );

    // Auto Speed
    y += config.slider_spacing;
    let speed_label = format!("Auto Speed ({}x)", format!("{:.1}", settings.auto_speed));
    let old_auto_speed = settings.auto_speed;
    settings.auto_speed = draw_slider(
        slider_x,
        y,
        config.slider_width,
        config.slider_height,
        settings.auto_speed,
        0.5,
        2.0,
        &speed_label,
        font,
        config.label_font_size,
    );
    // Round to 0.1
    if (settings.auto_speed - old_auto_speed).abs() > 0.001 {
        settings.auto_speed = (settings.auto_speed * 10.0).round() / 10.0;
    }

    // Back button
    let button_width = 150.0;
    let button_height = 40.0;
    let button_x = (screen_width - button_width) / 2.0;
    let button_y = config.back_button_y;

    let mouse_pos = mouse_position();
    let button_rect = Rect::new(button_x, button_y, button_width, button_height);
    let is_hovered = button_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));

    // Draw button
    let bg_color = if is_hovered {
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
        if is_hovered { YELLOW } else { GRAY },
    );

    // Draw button text
    let back_text = "Back";
    let text_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: config.label_font_size as u16,
            color: if is_hovered { YELLOW } else { WHITE },
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: config.label_font_size as u16,
            color: if is_hovered { YELLOW } else { WHITE },
            ..Default::default()
        }
    };

    let text_dim = measure_text(back_text, font, config.label_font_size as u16, 1.0);
    let text_x = button_x + (button_width - text_dim.width) / 2.0;
    let text_y = button_y + (button_height + text_dim.height) / 2.0 - text_dim.offset_y / 2.0;
    draw_text_ex(back_text, text_x, text_y, text_params);

    let back_pressed = (is_hovered && is_mouse_button_pressed(MouseButton::Left))
        || is_key_pressed(KeyCode::Escape);

    SettingsResult { back_pressed }
}
