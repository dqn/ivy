use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

use crate::platform;
use crate::render::widgets::{draw_button, draw_checkbox, draw_slider, draw_slider_ex, SliderFormat};
use crate::runtime::KeyBindings;

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
    /// Text speed in characters per second (0 = instant).
    #[serde(default = "default_text_speed")]
    pub text_speed: f32,
    /// Skip unread text (true = skip all, false = skip read only).
    #[serde(default = "default_skip_unread")]
    pub skip_unread: bool,
    /// Key bindings.
    #[serde(default)]
    pub keybinds: KeyBindings,
}

fn default_skip_unread() -> bool {
    true // Default to skip all text
}

fn default_volume() -> f32 {
    1.0
}

fn default_auto_speed() -> f32 {
    1.0
}

fn default_text_speed() -> f32 {
    30.0 // 30 characters per second
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            bgm_volume: 1.0,
            se_volume: 1.0,
            voice_volume: 1.0,
            auto_speed: 1.0,
            text_speed: 30.0,
            skip_unread: true,
            keybinds: KeyBindings::default(),
        }
    }
}

impl GameSettings {
    /// Load settings from config file (or localStorage on WASM).
    pub fn load() -> Self {
        if !platform::file_exists(CONFIG_PATH) {
            return Self::default();
        }

        match platform::read_file(CONFIG_PATH) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save settings to config file (or localStorage on WASM).
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = platform::write_file(CONFIG_PATH, &json);
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

    // Text Speed
    y += config.slider_spacing;
    let old_text_speed = settings.text_speed;
    settings.text_speed = draw_slider_ex(
        slider_x,
        y,
        config.slider_width,
        config.slider_height,
        settings.text_speed,
        0.0,
        100.0,
        "Text Speed",
        SliderFormat::Value(" CPS"),
        font,
        config.label_font_size,
    );
    // Round to 5
    if (settings.text_speed - old_text_speed).abs() > 0.001 {
        settings.text_speed = (settings.text_speed / 5.0).round() * 5.0;
    }

    // Skip Unread checkbox
    y += config.slider_spacing;
    settings.skip_unread = draw_checkbox(
        slider_x,
        y,
        settings.skip_unread,
        "Skip Unread Text",
        font,
        config.label_font_size,
    );

    // Back button
    let button_width = 150.0;
    let button_height = 40.0;
    let button_x = (screen_width - button_width) / 2.0;
    let button_y = config.back_button_y;

    let back_pressed =
        draw_button(button_x, button_y, button_width, button_height, "Back", font, config.label_font_size)
            || is_key_pressed(KeyCode::Escape);

    SettingsResult { back_pressed }
}
