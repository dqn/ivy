use serde::Serialize;

use crate::i18n::LocalizedString;
use crate::runtime::VisualState;
use crate::scenario::{Choice, Input};

/// History entry for rollback functionality.
#[derive(Debug, Clone, Serialize)]
pub struct HistoryEntry {
    pub index: usize,
    pub visual: VisualState,
    pub text: LocalizedString,
}

/// Current display state of the game.
#[derive(Debug, Clone, Serialize)]
pub enum DisplayState {
    /// Showing text, waiting for player to advance.
    Text {
        speaker: Option<LocalizedString>,
        text: LocalizedString,
        visual: VisualState,
    },
    /// Showing choices, waiting for player to select.
    Choices {
        speaker: Option<LocalizedString>,
        text: LocalizedString,
        choices: Vec<Choice>,
        visual: VisualState,
        /// Optional timeout in seconds for timed choices.
        timeout: Option<f32>,
        /// Index of the default choice (selected on timeout).
        default_choice: Option<usize>,
    },
    /// Waiting for a specified duration.
    Wait { duration: f32, visual: VisualState },
    /// Waiting for player text input.
    Input { input: Input, visual: VisualState },
    /// Playing a video.
    Video {
        path: String,
        skippable: bool,
        loop_video: bool,
        visual: VisualState,
    },
    /// Scenario has ended.
    End,
}
