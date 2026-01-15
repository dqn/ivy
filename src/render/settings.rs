use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

use crate::accessibility::SelfVoicingMode;
use crate::platform;
use crate::render::widgets::{
    SliderFormat, draw_button, draw_checkbox, draw_slider, draw_slider_ex,
};
use crate::runtime::KeyBindings;

const CONFIG_PATH: &str = "config.json";

/// Text speed preset values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSpeedPreset {
    Slow,
    Normal,
    Fast,
}

impl TextSpeedPreset {
    /// Convert preset to characters per second.
    pub fn to_cps(self) -> f32 {
        match self {
            Self::Slow => 15.0,
            Self::Normal => 30.0,
            Self::Fast => 60.0,
        }
    }

    /// Get display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Slow => "Slow",
            Self::Normal => "Normal",
            Self::Fast => "Fast",
        }
    }

    /// Get preset from CPS value.
    pub fn from_cps(cps: f32) -> Option<Self> {
        if (cps - 15.0).abs() < 0.1 {
            Some(Self::Slow)
        } else if (cps - 30.0).abs() < 0.1 {
            Some(Self::Normal)
        } else if (cps - 60.0).abs() < 0.1 {
            Some(Self::Fast)
        } else {
            None
        }
    }

    /// Get all presets.
    pub fn all() -> &'static [Self] {
        &[Self::Slow, Self::Normal, Self::Fast]
    }
}

/// Font type for accessibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FontType {
    /// Default font.
    #[default]
    Default,
    /// OpenDyslexic font for dyslexia support.
    OpenDyslexic,
}

impl FontType {
    /// Get display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::OpenDyslexic => "OpenDyslexic",
        }
    }

    /// Get all font types.
    pub fn all() -> &'static [Self] {
        &[Self::Default, Self::OpenDyslexic]
    }
}

/// Accessibility settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilitySettings {
    /// Font size scale (50% - 200%).
    #[serde(default = "default_font_scale")]
    pub font_scale: f32,
    /// High contrast mode for improved readability.
    #[serde(default)]
    pub high_contrast: bool,
    /// Line spacing multiplier (1.0 - 2.0).
    #[serde(default = "default_line_spacing")]
    pub line_spacing: f32,
    /// Font type (default or dyslexia-friendly).
    #[serde(default)]
    pub font_type: FontType,
    /// Letter spacing adjustment (-2.0 to 5.0 pixels).
    #[serde(default)]
    pub letter_spacing: f32,
    /// Self-voicing mode for screen reader support.
    #[serde(default)]
    pub self_voicing: SelfVoicingMode,
}

fn default_font_scale() -> f32 {
    100.0 // 100%
}

fn default_line_spacing() -> f32 {
    1.0
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            font_scale: 100.0,
            high_contrast: false,
            line_spacing: 1.0,
            font_type: FontType::Default,
            letter_spacing: 0.0,
            self_voicing: SelfVoicingMode::Off,
        }
    }
}

impl AccessibilitySettings {
    /// Get font scale as a multiplier (0.5 - 2.0).
    pub fn font_scale_multiplier(&self) -> f32 {
        self.font_scale / 100.0
    }
}

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
    /// Accessibility settings.
    #[serde(default)]
    pub accessibility: AccessibilitySettings,
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
            accessibility: AccessibilitySettings::default(),
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
            slider_start_y: 120.0,
            slider_spacing: 55.0,
            slider_width: 300.0,
            slider_height: 20.0,
            back_button_y: 550.0,
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
    let speed_label = format!("Auto Speed ({:.1}x)", settings.auto_speed);
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

    // Text Speed Preset buttons
    let preset_button_width = 80.0;
    let preset_button_height = 30.0;
    let preset_spacing = 10.0;
    let preset_y = y + 40.0;
    let current_preset = TextSpeedPreset::from_cps(settings.text_speed);

    for (i, preset) in TextSpeedPreset::all().iter().enumerate() {
        let btn_x = slider_x + (preset_button_width + preset_spacing) * i as f32;
        let is_selected = current_preset == Some(*preset);

        // Draw button with selection highlight
        let mouse_pos = mouse_position();
        let button_rect = Rect::new(btn_x, preset_y, preset_button_width, preset_button_height);
        let is_hovered = button_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));

        let bg_color = if is_selected {
            Color::new(0.3, 0.6, 0.3, 0.9)
        } else if is_hovered {
            Color::new(0.3, 0.3, 0.4, 0.9)
        } else {
            Color::new(0.2, 0.2, 0.25, 0.8)
        };

        draw_rectangle(
            btn_x,
            preset_y,
            preset_button_width,
            preset_button_height,
            bg_color,
        );
        draw_rectangle_lines(
            btn_x,
            preset_y,
            preset_button_width,
            preset_button_height,
            2.0,
            if is_selected {
                Color::new(0.3, 0.8, 0.3, 1.0)
            } else if is_hovered {
                YELLOW
            } else {
                GRAY
            },
        );

        // Draw button text
        let text_params = if let Some(f) = font {
            TextParams {
                font: Some(f),
                font_size: (config.label_font_size * 0.8) as u16,
                color: if is_selected || is_hovered {
                    WHITE
                } else {
                    LIGHTGRAY
                },
                ..Default::default()
            }
        } else {
            TextParams {
                font_size: (config.label_font_size * 0.8) as u16,
                color: if is_selected || is_hovered {
                    WHITE
                } else {
                    LIGHTGRAY
                },
                ..Default::default()
            }
        };

        let label = preset.display_name();
        let text_dim = measure_text(label, font, (config.label_font_size * 0.8) as u16, 1.0);
        let text_x = btn_x + (preset_button_width - text_dim.width) / 2.0;
        let text_y =
            preset_y + (preset_button_height + text_dim.height) / 2.0 - text_dim.offset_y / 2.0;
        draw_text_ex(label, text_x, text_y, text_params);

        // Handle click
        if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
            settings.text_speed = preset.to_cps();
        }
    }

    // Skip Unread checkbox
    y += config.slider_spacing + 40.0; // Extra space for preset buttons
    settings.skip_unread = draw_checkbox(
        slider_x,
        y,
        settings.skip_unread,
        "Skip Unread Text",
        font,
        config.label_font_size,
    );

    // Accessibility section
    y += config.slider_spacing;

    // Font Size
    let old_font_scale = settings.accessibility.font_scale;
    settings.accessibility.font_scale = draw_slider_ex(
        slider_x,
        y,
        config.slider_width,
        config.slider_height,
        settings.accessibility.font_scale,
        50.0,
        200.0,
        "Font Size",
        SliderFormat::Value("%"),
        font,
        config.label_font_size,
    );
    // Round to 10
    if (settings.accessibility.font_scale - old_font_scale).abs() > 0.001 {
        settings.accessibility.font_scale =
            (settings.accessibility.font_scale / 10.0).round() * 10.0;
    }

    // High Contrast checkbox
    y += config.slider_spacing;
    settings.accessibility.high_contrast = draw_checkbox(
        slider_x,
        y,
        settings.accessibility.high_contrast,
        "High Contrast Mode",
        font,
        config.label_font_size,
    );

    // Line Spacing
    y += config.slider_spacing;
    let old_line_spacing = settings.accessibility.line_spacing;
    settings.accessibility.line_spacing = draw_slider_ex(
        slider_x,
        y,
        config.slider_width,
        config.slider_height,
        settings.accessibility.line_spacing,
        1.0,
        2.0,
        "Line Spacing",
        SliderFormat::Multiplier,
        font,
        config.label_font_size,
    );
    // Round to 0.1
    if (settings.accessibility.line_spacing - old_line_spacing).abs() > 0.001 {
        settings.accessibility.line_spacing =
            (settings.accessibility.line_spacing * 10.0).round() / 10.0;
    }

    // Letter Spacing
    y += config.slider_spacing;
    let old_letter_spacing = settings.accessibility.letter_spacing;
    settings.accessibility.letter_spacing = draw_slider_ex(
        slider_x,
        y,
        config.slider_width,
        config.slider_height,
        settings.accessibility.letter_spacing,
        -2.0,
        5.0,
        "Letter Spacing",
        SliderFormat::Value("px"),
        font,
        config.label_font_size,
    );
    // Round to 0.5
    if (settings.accessibility.letter_spacing - old_letter_spacing).abs() > 0.001 {
        settings.accessibility.letter_spacing =
            (settings.accessibility.letter_spacing * 2.0).round() / 2.0;
    }

    // Font Type selector
    y += config.slider_spacing;
    let font_button_width = 120.0;
    let font_button_height = 30.0;
    let font_spacing = 10.0;

    // Draw label
    let label_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: config.label_font_size as u16,
            color: WHITE,
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: config.label_font_size as u16,
            color: WHITE,
            ..Default::default()
        }
    };
    draw_text_ex("Font Type", slider_x, y + config.label_font_size, label_params);

    let font_btn_y = y + 5.0;
    for (i, font_type) in FontType::all().iter().enumerate() {
        let btn_x = slider_x + 120.0 + (font_button_width + font_spacing) * i as f32;
        let is_selected = settings.accessibility.font_type == *font_type;

        let mouse_pos = mouse_position();
        let button_rect = Rect::new(btn_x, font_btn_y, font_button_width, font_button_height);
        let is_hovered = button_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));

        let bg_color = if is_selected {
            Color::new(0.3, 0.6, 0.3, 0.9)
        } else if is_hovered {
            Color::new(0.3, 0.3, 0.4, 0.9)
        } else {
            Color::new(0.2, 0.2, 0.25, 0.8)
        };

        draw_rectangle(
            btn_x,
            font_btn_y,
            font_button_width,
            font_button_height,
            bg_color,
        );
        draw_rectangle_lines(
            btn_x,
            font_btn_y,
            font_button_width,
            font_button_height,
            2.0,
            if is_selected {
                Color::new(0.3, 0.8, 0.3, 1.0)
            } else if is_hovered {
                YELLOW
            } else {
                GRAY
            },
        );

        let text_params = if let Some(f) = font {
            TextParams {
                font: Some(f),
                font_size: (config.label_font_size * 0.8) as u16,
                color: if is_selected || is_hovered {
                    WHITE
                } else {
                    LIGHTGRAY
                },
                ..Default::default()
            }
        } else {
            TextParams {
                font_size: (config.label_font_size * 0.8) as u16,
                color: if is_selected || is_hovered {
                    WHITE
                } else {
                    LIGHTGRAY
                },
                ..Default::default()
            }
        };

        let label = font_type.display_name();
        let text_dim = measure_text(label, font, (config.label_font_size * 0.8) as u16, 1.0);
        let text_x = btn_x + (font_button_width - text_dim.width) / 2.0;
        let text_y =
            font_btn_y + (font_button_height + text_dim.height) / 2.0 - text_dim.offset_y / 2.0;
        draw_text_ex(label, text_x, text_y, text_params);

        if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
            settings.accessibility.font_type = *font_type;
        }
    }

    // Self-Voicing selector
    y += config.slider_spacing;
    let sv_button_width = 120.0;
    let sv_button_height = 30.0;
    let sv_spacing = 10.0;

    // Draw label
    let sv_label_params = if let Some(f) = font {
        TextParams {
            font: Some(f),
            font_size: config.label_font_size as u16,
            color: WHITE,
            ..Default::default()
        }
    } else {
        TextParams {
            font_size: config.label_font_size as u16,
            color: WHITE,
            ..Default::default()
        }
    };
    draw_text_ex(
        "Self-Voicing",
        slider_x,
        y + config.label_font_size,
        sv_label_params,
    );

    let sv_btn_y = y + 5.0;
    for (i, mode) in SelfVoicingMode::all().iter().enumerate() {
        let btn_x = slider_x + 120.0 + (sv_button_width + sv_spacing) * i as f32;
        let is_selected = settings.accessibility.self_voicing == *mode;

        let mouse_pos = mouse_position();
        let button_rect = Rect::new(btn_x, sv_btn_y, sv_button_width, sv_button_height);
        let is_hovered = button_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));

        let bg_color = if is_selected {
            Color::new(0.3, 0.6, 0.3, 0.9)
        } else if is_hovered {
            Color::new(0.3, 0.3, 0.4, 0.9)
        } else {
            Color::new(0.2, 0.2, 0.25, 0.8)
        };

        draw_rectangle(
            btn_x,
            sv_btn_y,
            sv_button_width,
            sv_button_height,
            bg_color,
        );
        draw_rectangle_lines(
            btn_x,
            sv_btn_y,
            sv_button_width,
            sv_button_height,
            2.0,
            if is_selected {
                Color::new(0.3, 0.8, 0.3, 1.0)
            } else if is_hovered {
                YELLOW
            } else {
                GRAY
            },
        );

        let sv_text_params = if let Some(f) = font {
            TextParams {
                font: Some(f),
                font_size: (config.label_font_size * 0.8) as u16,
                color: if is_selected || is_hovered {
                    WHITE
                } else {
                    LIGHTGRAY
                },
                ..Default::default()
            }
        } else {
            TextParams {
                font_size: (config.label_font_size * 0.8) as u16,
                color: if is_selected || is_hovered {
                    WHITE
                } else {
                    LIGHTGRAY
                },
                ..Default::default()
            }
        };

        let label = mode.display_name();
        let text_dim = measure_text(label, font, (config.label_font_size * 0.8) as u16, 1.0);
        let text_x = btn_x + (sv_button_width - text_dim.width) / 2.0;
        let text_y =
            sv_btn_y + (sv_button_height + text_dim.height) / 2.0 - text_dim.offset_y / 2.0;
        draw_text_ex(label, text_x, text_y, sv_text_params);

        if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
            settings.accessibility.self_voicing = *mode;
        }
    }

    // Back button
    let button_width = 150.0;
    let button_height = 40.0;
    let button_x = (screen_width - button_width) / 2.0;
    let button_y = config.back_button_y;

    let back_pressed = draw_button(
        button_x,
        button_y,
        button_width,
        button_height,
        "Back",
        font,
        config.label_font_size,
    ) || is_key_pressed(KeyCode::Escape);

    SettingsResult { back_pressed }
}
