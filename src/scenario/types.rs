use serde::{Deserialize, Serialize};

use crate::i18n::LocalizedString;

/// Easing functions for smooth animations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Easing {
    /// Linear interpolation (no easing).
    #[default]
    Linear,
    /// Ease in (slow start).
    EaseIn,
    /// Ease out (slow end).
    EaseOut,
    /// Ease in and out (slow start and end).
    EaseInOut,
    /// Quadratic ease in.
    EaseInQuad,
    /// Quadratic ease out.
    EaseOutQuad,
    /// Quadratic ease in and out.
    EaseInOutQuad,
    /// Cubic ease in.
    EaseInCubic,
    /// Cubic ease out.
    EaseOutCubic,
    /// Cubic ease in and out.
    EaseInOutCubic,
    /// Back ease in (slight overshoot at start).
    EaseInBack,
    /// Back ease out (slight overshoot at end).
    EaseOutBack,
    /// Back ease in and out.
    EaseInOutBack,
    /// Bounce ease out.
    EaseOutBounce,
}

impl Easing {
    /// Apply the easing function to a value t in the range [0, 1].
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t * t,
            Easing::EaseOut => 1.0 - (1.0 - t).powi(3),
            Easing::EaseInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::EaseInQuad => t * t,
            Easing::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Easing::EaseInCubic => t * t * t,
            Easing::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Easing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::EaseInBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            Easing::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            Easing::EaseInOutBack => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
                }
            }
            Easing::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
        }
    }
}

/// Character sprite position on screen.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharPosition {
    Left,
    #[default]
    Center,
    Right,
}

/// Transition effect type.
#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionType {
    #[default]
    None,
    Fade,
    FadeWhite,
    Dissolve,
    /// Wipe transition (reveals new scene by moving edge).
    Wipe,
    /// Slide transition (slides scenes in/out).
    Slide,
    /// Pixelate transition (pixelates then clears).
    Pixelate,
    /// Iris transition (circular reveal/close).
    Iris,
    /// Blinds transition (venetian blind effect).
    Blinds,
}

/// Direction for directional transitions.
#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionDirection {
    /// Left to right (default for Wipe).
    #[default]
    LeftToRight,
    /// Right to left.
    RightToLeft,
    /// Top to bottom.
    TopToBottom,
    /// Bottom to top.
    BottomToTop,
    /// Left (for Slide).
    Left,
    /// Right (for Slide).
    Right,
    /// Up (for Slide).
    Up,
    /// Down (for Slide).
    Down,
    /// Open (for Iris - from center outward).
    Open,
    /// Close (for Iris - from edges to center).
    Close,
    /// Horizontal (for Blinds).
    Horizontal,
    /// Vertical (for Blinds).
    Vertical,
}

/// Transition configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Transition {
    #[serde(rename = "type", default)]
    pub transition_type: TransitionType,
    #[serde(default = "default_duration")]
    pub duration: f32,
    #[serde(default)]
    pub easing: Easing,
    /// Direction for directional transitions (Wipe, Slide, Iris, Blinds).
    #[serde(default)]
    pub direction: TransitionDirection,
    /// Number of blinds for Blinds transition (default: 10).
    #[serde(default = "default_blinds_count")]
    pub blinds_count: u32,
    /// Maximum pixel size for Pixelate transition (default: 32).
    #[serde(default = "default_max_pixel_size")]
    pub max_pixel_size: u32,
}

fn default_blinds_count() -> u32 {
    10
}

fn default_max_pixel_size() -> u32 {
    32
}

fn default_duration() -> f32 {
    0.5
}

/// Variable assignment command.
#[derive(Debug, Clone, Deserialize)]
pub struct SetVar {
    pub name: String,
    pub value: crate::types::Value,
}

/// Text input command for player input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    /// Variable name to store the input.
    pub var: String,
    /// Prompt text to display.
    #[serde(default)]
    pub prompt: Option<String>,
    /// Default value.
    #[serde(default)]
    pub default: Option<String>,
}

/// Conditional jump command.
#[derive(Debug, Clone, Deserialize)]
pub struct IfCondition {
    /// Variable name to check.
    pub var: String,
    /// Expected value.
    pub is: crate::types::Value,
    /// Label to jump to if condition is true.
    pub jump: String,
}

/// Character display configuration for multiple characters.
#[derive(Debug, Clone, Deserialize)]
pub struct CharacterDisplay {
    /// Character image path.
    pub image: String,
    /// Position on screen.
    #[serde(default)]
    pub pos: CharPosition,
    /// Enter animation (optional).
    pub enter: Option<CharAnimation>,
    /// Exit animation (optional).
    pub exit: Option<CharAnimation>,
    /// Idle animation (optional).
    pub idle: Option<CharIdleAnimation>,
}

/// Character animation type.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharAnimationType {
    /// No animation (instant).
    #[default]
    None,
    /// Fade in/out.
    Fade,
    /// Slide from/to left.
    SlideLeft,
    /// Slide from/to right.
    SlideRight,
}

/// Character enter/exit animation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharAnimation {
    #[serde(rename = "type", default)]
    pub animation_type: CharAnimationType,
    #[serde(default = "default_char_animation_duration")]
    pub duration: f32,
    #[serde(default)]
    pub easing: Easing,
}

fn default_char_animation_duration() -> f32 {
    0.3
}

/// Character idle animation type (looping animations).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharIdleType {
    /// No idle animation.
    #[default]
    None,
    /// Breathing animation (subtle vertical scale oscillation).
    Breath,
    /// Bobbing animation (vertical position oscillation).
    Bob,
    /// Swaying animation (horizontal position oscillation).
    Sway,
    /// Pulsing animation (uniform scale oscillation).
    Pulse,
}

/// Character idle animation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharIdleAnimation {
    #[serde(rename = "type", default)]
    pub idle_type: CharIdleType,
    /// Duration of one cycle in seconds (default: 2.0).
    #[serde(default = "default_char_idle_duration")]
    pub duration: f32,
    /// Animation intensity/amplitude (0.0 to 1.0, default: 0.3).
    #[serde(default = "default_char_idle_intensity")]
    pub intensity: f32,
    #[serde(default)]
    pub easing: Easing,
}

fn default_char_idle_duration() -> f32 {
    2.0
}

fn default_char_idle_intensity() -> f32 {
    0.3
}

/// Shake effect type.
#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShakeType {
    /// Horizontal shake (left-right).
    #[default]
    Horizontal,
    /// Vertical shake (up-down).
    Vertical,
    /// Both horizontal and vertical.
    Both,
}

/// Shake effect configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Shake {
    /// Shake type (horizontal, vertical, both).
    #[serde(rename = "type", default)]
    pub shake_type: ShakeType,
    /// Shake intensity in pixels.
    #[serde(default = "default_shake_intensity")]
    pub intensity: f32,
    /// Duration in seconds.
    #[serde(default = "default_shake_duration")]
    pub duration: f32,
    #[serde(default)]
    pub easing: Easing,
}

fn default_shake_intensity() -> f32 {
    10.0
}

fn default_shake_duration() -> f32 {
    0.5
}

/// Video playback command.
#[derive(Debug, Clone, Deserialize)]
pub struct VideoCommand {
    /// Video file path.
    pub path: String,
    /// Whether the video can be skipped by the player.
    #[serde(default = "default_video_skippable")]
    pub skippable: bool,
    /// Whether to loop the video.
    #[serde(default)]
    pub loop_video: bool,
    /// BGM fade out duration in seconds when video starts (0 = instant stop).
    #[serde(default = "default_video_bgm_fade")]
    pub bgm_fade_out: f32,
    /// BGM fade in duration in seconds when video ends.
    #[serde(default = "default_video_bgm_fade")]
    pub bgm_fade_in: f32,
}

fn default_video_skippable() -> bool {
    true
}

fn default_video_bgm_fade() -> f32 {
    0.5
}

/// A single choice option that branches the story.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    /// Display text for this choice (supports localization).
    pub label: LocalizedString,
    /// Label to jump to when this choice is selected.
    pub jump: String,
    /// Whether this is the default choice when timeout expires.
    #[serde(default)]
    pub default: bool,
}

/// A single command in the scenario script.
#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    /// Optional label for this command (used as jump target).
    pub label: Option<String>,
    /// Speaker name to display (supports localization).
    pub speaker: Option<LocalizedString>,
    /// Text to display (supports localization).
    pub text: Option<LocalizedString>,
    /// Choices to present to the player (if any).
    pub choices: Option<Vec<Choice>>,
    /// Unconditional jump to another label.
    pub jump: Option<String>,
    /// Background image path (None = keep previous, Some("") = clear).
    pub background: Option<String>,
    /// Character sprite image path (None = keep previous, Some("") = clear).
    pub character: Option<String>,
    /// Character sprite position.
    pub char_pos: Option<CharPosition>,
    /// Character entrance animation.
    pub char_enter: Option<CharAnimation>,
    /// Character exit animation.
    pub char_exit: Option<CharAnimation>,
    /// Character idle animation (looping, applied after enter animation).
    pub char_idle: Option<CharIdleAnimation>,
    /// Multiple characters to display.
    pub characters: Option<Vec<CharacterDisplay>>,
    /// BGM file path (None = keep previous, Some("") = stop).
    pub bgm: Option<String>,
    /// Sound effect file path (plays once).
    pub se: Option<String>,
    /// Voice file path (plays once).
    pub voice: Option<String>,
    /// Transition effect.
    pub transition: Option<Transition>,
    /// Shake effect.
    pub shake: Option<Shake>,
    /// Set a variable.
    pub set: Option<SetVar>,
    /// Conditional jump.
    #[serde(rename = "if")]
    pub if_cond: Option<IfCondition>,
    /// Wait duration in seconds.
    pub wait: Option<f32>,
    /// Timeout for choices in seconds (auto-selects default choice).
    pub timeout: Option<f32>,
    /// Text input for player.
    pub input: Option<Input>,
    /// Particle effect type (snow, rain, sakura, sparkle, leaves, or empty to stop).
    pub particles: Option<String>,
    /// Particle intensity (0.0 to 1.0).
    #[serde(default = "default_particle_intensity")]
    pub particle_intensity: f32,
    /// Cinematic mode (letterbox bars). true = on, false = off.
    pub cinematic: Option<bool>,
    /// Cinematic transition duration in seconds.
    #[serde(default = "default_cinematic_duration")]
    pub cinematic_duration: f32,
    /// Achievement to unlock.
    pub achievement: Option<Achievement>,
    /// Video playback command.
    pub video: Option<VideoCommand>,
}

/// Achievement unlock command.
#[derive(Debug, Clone, Deserialize)]
pub struct Achievement {
    /// Achievement ID.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Description.
    #[serde(default)]
    pub description: String,
}

fn default_cinematic_duration() -> f32 {
    0.5
}

fn default_particle_intensity() -> f32 {
    0.5
}

/// Chapter definition in scenario YAML.
#[derive(Debug, Clone, Deserialize)]
pub struct ChapterDef {
    /// Unique chapter ID.
    pub id: String,
    /// Chapter title displayed in menu.
    pub title: String,
    /// Label to jump to when starting this chapter.
    pub start_label: String,
    /// Optional description.
    #[serde(default)]
    pub description: String,
}

/// A complete scenario loaded from YAML.
#[derive(Debug, Clone, Deserialize)]
pub struct Scenario {
    /// Title of this scenario.
    pub title: String,
    /// Optional chapter definitions.
    #[serde(default)]
    pub chapters: Vec<ChapterDef>,
    /// List of commands that make up the script.
    pub script: Vec<Command>,
}
