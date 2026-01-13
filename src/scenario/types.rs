use serde::{Deserialize, Serialize};

/// Character sprite position on screen.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
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
}

/// Transition configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Transition {
    #[serde(rename = "type", default)]
    pub transition_type: TransitionType,
    #[serde(default = "default_duration")]
    pub duration: f32,
}

fn default_duration() -> f32 {
    0.5
}

/// Variable assignment command.
#[derive(Debug, Clone, Deserialize)]
pub struct SetVar {
    pub name: String,
    pub value: crate::runtime::Value,
}

/// Conditional jump command.
#[derive(Debug, Clone, Deserialize)]
pub struct IfCondition {
    /// Variable name to check.
    pub var: String,
    /// Expected value.
    pub is: crate::runtime::Value,
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
}

/// Character animation type.
#[derive(Debug, Clone, Copy, Default, Deserialize)]
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
#[derive(Debug, Clone, Deserialize)]
pub struct CharAnimation {
    #[serde(rename = "type", default)]
    pub animation_type: CharAnimationType,
    #[serde(default = "default_char_animation_duration")]
    pub duration: f32,
}

fn default_char_animation_duration() -> f32 {
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
}

fn default_shake_intensity() -> f32 {
    10.0
}

fn default_shake_duration() -> f32 {
    0.5
}

/// A single choice option that branches the story.
#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    /// Display text for this choice.
    pub label: String,
    /// Label to jump to when this choice is selected.
    pub jump: String,
}

/// A single command in the scenario script.
#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    /// Optional label for this command (used as jump target).
    pub label: Option<String>,
    /// Speaker name to display.
    pub speaker: Option<String>,
    /// Text to display (if any).
    pub text: Option<String>,
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
}

/// A complete scenario loaded from YAML.
#[derive(Debug, Clone, Deserialize)]
pub struct Scenario {
    /// Title of this scenario.
    pub title: String,
    /// List of commands that make up the script.
    pub script: Vec<Command>,
}
