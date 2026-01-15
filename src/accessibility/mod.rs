//! Accessibility features for screen reader support.

use serde::{Deserialize, Serialize};

use crate::platform;

/// Self-voicing mode for screen reader support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelfVoicingMode {
    /// Disabled (no text-to-speech).
    #[default]
    Off,
    /// Use built-in TTS (Web Speech API on WASM).
    Tts,
    /// Copy text to clipboard for external screen readers.
    Clipboard,
}

impl SelfVoicingMode {
    /// Get display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Tts => "Text-to-Speech",
            Self::Clipboard => "Clipboard",
        }
    }

    /// Get all modes.
    pub fn all() -> &'static [Self] {
        &[Self::Off, Self::Tts, Self::Clipboard]
    }

    /// Check if self-voicing is enabled.
    pub fn is_enabled(&self) -> bool {
        !matches!(self, Self::Off)
    }
}

/// Self-voicing controller for screen reader support.
#[derive(Default)]
pub struct SelfVoicing {
    mode: SelfVoicingMode,
    last_text: String,
}

impl SelfVoicing {
    /// Create a new self-voicing controller.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the self-voicing mode.
    pub fn set_mode(&mut self, mode: SelfVoicingMode) {
        self.mode = mode;
    }

    /// Get the current mode.
    pub fn mode(&self) -> SelfVoicingMode {
        self.mode
    }

    /// Speak dialogue text with optional speaker name.
    pub fn speak_dialogue(&mut self, speaker: Option<&str>, text: &str) {
        if !self.mode.is_enabled() {
            return;
        }

        let full_text = match speaker {
            Some(name) => format!("{}: {}", name, text),
            None => text.to_string(),
        };

        // Avoid repeating the same text
        if full_text == self.last_text {
            return;
        }
        self.last_text = full_text.clone();

        self.output_text(&full_text);
    }

    /// Announce UI elements or scene changes.
    pub fn announce(&mut self, text: &str) {
        if !self.mode.is_enabled() {
            return;
        }

        // UI announcements always get spoken even if same text
        self.output_text(text);
    }

    /// Speak choice options.
    pub fn speak_choices(&mut self, choices: &[&str]) {
        if !self.mode.is_enabled() || choices.is_empty() {
            return;
        }

        let text = format!(
            "Choices: {}",
            choices
                .iter()
                .enumerate()
                .map(|(i, c)| format!("{}. {}", i + 1, c))
                .collect::<Vec<_>>()
                .join(", ")
        );

        self.output_text(&text);
    }

    /// Output text using the current mode.
    fn output_text(&self, text: &str) {
        match self.mode {
            SelfVoicingMode::Off => {}
            SelfVoicingMode::Tts => {
                platform::speak_text(text);
            }
            SelfVoicingMode::Clipboard => {
                platform::copy_to_clipboard(text);
            }
        }
    }

    /// Check if TTS is available on this platform.
    pub fn tts_available() -> bool {
        platform::tts_available()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_voicing_off_by_default() {
        let sv = SelfVoicing::new();
        assert_eq!(sv.mode(), SelfVoicingMode::Off);
        assert!(!sv.mode().is_enabled());
    }

    #[test]
    fn test_self_voicing_mode_change() {
        let mut sv = SelfVoicing::new();
        sv.set_mode(SelfVoicingMode::Tts);
        assert_eq!(sv.mode(), SelfVoicingMode::Tts);
        assert!(sv.mode().is_enabled());

        sv.set_mode(SelfVoicingMode::Clipboard);
        assert_eq!(sv.mode(), SelfVoicingMode::Clipboard);
        assert!(sv.mode().is_enabled());
    }
}
