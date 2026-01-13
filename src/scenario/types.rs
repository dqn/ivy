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
    /// BGM file path (None = keep previous, Some("") = stop).
    pub bgm: Option<String>,
    /// Sound effect file path (plays once).
    pub se: Option<String>,
    /// Voice file path (plays once).
    pub voice: Option<String>,
    /// Transition effect.
    pub transition: Option<Transition>,
}

/// A complete scenario loaded from YAML.
#[derive(Debug, Clone, Deserialize)]
pub struct Scenario {
    /// Title of this scenario.
    pub title: String,
    /// List of commands that make up the script.
    pub script: Vec<Command>,
}
