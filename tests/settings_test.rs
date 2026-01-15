use serde::{Deserialize, Serialize};

/// Font type for accessibility (mirror of render::settings::FontType).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FontType {
    #[default]
    Default,
    OpenDyslexic,
}

/// Self-voicing mode (mirror of accessibility::SelfVoicingMode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SelfVoicingMode {
    #[default]
    Off,
    Tts,
    Clipboard,
}

/// Accessibility settings (mirror of render::settings::AccessibilitySettings).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccessibilitySettings {
    #[serde(default = "default_font_scale")]
    font_scale: f32,
    #[serde(default)]
    high_contrast: bool,
    #[serde(default = "default_line_spacing")]
    line_spacing: f32,
    #[serde(default)]
    font_type: FontType,
    #[serde(default)]
    letter_spacing: f32,
    #[serde(default)]
    self_voicing: SelfVoicingMode,
}

fn default_font_scale() -> f32 {
    100.0
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
    fn font_scale_multiplier(&self) -> f32 {
        self.font_scale / 100.0
    }
}

/// Game settings (mirror of render::settings::GameSettings).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameSettings {
    #[serde(default = "default_volume")]
    bgm_volume: f32,
    #[serde(default = "default_volume")]
    se_volume: f32,
    #[serde(default = "default_volume")]
    voice_volume: f32,
    #[serde(default = "default_auto_speed")]
    auto_speed: f32,
    #[serde(default = "default_text_speed")]
    text_speed: f32,
    #[serde(default = "default_skip_unread")]
    skip_unread: bool,
    #[serde(default)]
    accessibility: AccessibilitySettings,
}

fn default_volume() -> f32 {
    1.0
}

fn default_auto_speed() -> f32 {
    1.0
}

fn default_text_speed() -> f32 {
    30.0
}

fn default_skip_unread() -> bool {
    true
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
            accessibility: AccessibilitySettings::default(),
        }
    }
}

#[test]
fn test_accessibility_settings_default() {
    let settings = GameSettings::default();
    assert_eq!(settings.accessibility.font_scale, 100.0);
    assert!(!settings.accessibility.high_contrast);
    assert_eq!(settings.accessibility.line_spacing, 1.0);
}

#[test]
fn test_accessibility_font_scale_multiplier() {
    let settings = GameSettings::default();
    assert_eq!(settings.accessibility.font_scale_multiplier(), 1.0);

    let mut settings2 = GameSettings::default();
    settings2.accessibility.font_scale = 150.0;
    assert_eq!(settings2.accessibility.font_scale_multiplier(), 1.5);

    let mut settings3 = GameSettings::default();
    settings3.accessibility.font_scale = 50.0;
    assert_eq!(settings3.accessibility.font_scale_multiplier(), 0.5);
}

#[test]
fn test_accessibility_serialization_roundtrip() {
    let mut settings = GameSettings::default();
    settings.accessibility.font_scale = 120.0;
    settings.accessibility.high_contrast = true;
    settings.accessibility.line_spacing = 1.5;

    let json = serde_json::to_string(&settings).unwrap();
    let deserialized: GameSettings = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.accessibility.font_scale, 120.0);
    assert!(deserialized.accessibility.high_contrast);
    assert_eq!(deserialized.accessibility.line_spacing, 1.5);
}

#[test]
fn test_settings_backward_compatibility() {
    // Settings without accessibility field should deserialize with defaults
    let old_json = r#"{
        "bgm_volume": 0.8,
        "se_volume": 0.9,
        "voice_volume": 1.0,
        "auto_speed": 1.5,
        "text_speed": 45.0,
        "skip_unread": false
    }"#;

    let settings: GameSettings = serde_json::from_str(old_json).unwrap();
    assert_eq!(settings.bgm_volume, 0.8);
    assert_eq!(settings.accessibility.font_scale, 100.0);
    assert!(!settings.accessibility.high_contrast);
}

#[test]
fn test_accessibility_font_type_default() {
    let settings = GameSettings::default();
    assert_eq!(settings.accessibility.font_type, FontType::Default);
    assert_eq!(settings.accessibility.letter_spacing, 0.0);
}

#[test]
fn test_accessibility_font_type_serialization() {
    let mut settings = GameSettings::default();
    settings.accessibility.font_type = FontType::OpenDyslexic;
    settings.accessibility.letter_spacing = 2.5;

    let json = serde_json::to_string(&settings).unwrap();
    let deserialized: GameSettings = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.accessibility.font_type, FontType::OpenDyslexic);
    assert_eq!(deserialized.accessibility.letter_spacing, 2.5);
}

#[test]
fn test_font_type_deserialize() {
    let json = r#""default""#;
    let font_type: FontType = serde_json::from_str(json).unwrap();
    assert_eq!(font_type, FontType::Default);

    let json = r#""open_dyslexic""#;
    let font_type: FontType = serde_json::from_str(json).unwrap();
    assert_eq!(font_type, FontType::OpenDyslexic);
}

#[test]
fn test_accessibility_backward_compatibility_extended() {
    // Settings with old accessibility format should work
    let old_json = r#"{
        "bgm_volume": 1.0,
        "accessibility": {
            "font_scale": 120.0,
            "high_contrast": true,
            "line_spacing": 1.5
        }
    }"#;

    let settings: GameSettings = serde_json::from_str(old_json).unwrap();
    assert_eq!(settings.accessibility.font_scale, 120.0);
    assert!(settings.accessibility.high_contrast);
    assert_eq!(settings.accessibility.line_spacing, 1.5);
    // New fields should use defaults
    assert_eq!(settings.accessibility.font_type, FontType::Default);
    assert_eq!(settings.accessibility.letter_spacing, 0.0);
    assert_eq!(settings.accessibility.self_voicing, SelfVoicingMode::Off);
}

#[test]
fn test_self_voicing_mode_default() {
    let settings = GameSettings::default();
    assert_eq!(settings.accessibility.self_voicing, SelfVoicingMode::Off);
}

#[test]
fn test_self_voicing_mode_serialization() {
    let json = r#""off""#;
    let mode: SelfVoicingMode = serde_json::from_str(json).unwrap();
    assert_eq!(mode, SelfVoicingMode::Off);

    let json = r#""tts""#;
    let mode: SelfVoicingMode = serde_json::from_str(json).unwrap();
    assert_eq!(mode, SelfVoicingMode::Tts);

    let json = r#""clipboard""#;
    let mode: SelfVoicingMode = serde_json::from_str(json).unwrap();
    assert_eq!(mode, SelfVoicingMode::Clipboard);
}

#[test]
fn test_accessibility_self_voicing_roundtrip() {
    let mut settings = GameSettings::default();
    settings.accessibility.self_voicing = SelfVoicingMode::Tts;

    let json = serde_json::to_string(&settings).unwrap();
    let deserialized: GameSettings = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.accessibility.self_voicing, SelfVoicingMode::Tts);
}
