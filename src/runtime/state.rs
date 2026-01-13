use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::scenario::{CharPosition, Choice, Scenario};

/// Visual state (background and character sprite).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualState {
    pub background: Option<String>,
    pub character: Option<String>,
    pub char_pos: CharPosition,
}

/// Save data format.
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveData {
    pub scenario_path: String,
    pub current_index: usize,
    pub visual: VisualState,
}

impl SaveData {
    /// Save to a JSON file.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load from a JSON file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let save: SaveData = serde_json::from_str(&content)?;
        Ok(save)
    }
}

/// Current display state of the game.
#[derive(Debug, Clone)]
pub enum DisplayState {
    /// Showing text, waiting for player to advance.
    Text { text: String, visual: VisualState },
    /// Showing choices, waiting for player to select.
    Choices {
        text: String,
        choices: Vec<Choice>,
        visual: VisualState,
    },
    /// Scenario has ended.
    End,
}

/// Runtime state for the visual novel engine.
#[derive(Debug)]
pub struct GameState {
    scenario: Scenario,
    current_index: usize,
    visual: VisualState,
}

impl GameState {
    /// Create a new game state from a scenario.
    pub fn new(scenario: Scenario) -> Self {
        Self {
            scenario,
            current_index: 0,
            visual: VisualState::default(),
        }
    }

    /// Create a save data snapshot.
    pub fn to_save_data(&self, scenario_path: &str) -> SaveData {
        SaveData {
            scenario_path: scenario_path.to_string(),
            current_index: self.current_index,
            visual: self.visual.clone(),
        }
    }

    /// Restore from save data.
    pub fn from_save_data(save: &SaveData, scenario: Scenario) -> Self {
        let current_index = save.current_index.min(scenario.script.len());
        Self {
            scenario,
            current_index,
            visual: save.visual.clone(),
        }
    }

    /// Get the current display state.
    pub fn display_state(&self) -> DisplayState {
        if self.current_index >= self.scenario.script.len() {
            return DisplayState::End;
        }

        let command = &self.scenario.script[self.current_index];
        let visual = self.current_visual();

        if let Some(choices) = &command.choices {
            let text = command.text.clone().unwrap_or_default();
            return DisplayState::Choices {
                text,
                choices: choices.clone(),
                visual,
            };
        }

        if let Some(text) = &command.text {
            return DisplayState::Text {
                text: text.clone(),
                visual,
            };
        }

        // Command has no displayable content, skip it
        DisplayState::End
    }

    /// Get the current visual state, applying command overrides.
    fn current_visual(&self) -> VisualState {
        if self.current_index >= self.scenario.script.len() {
            return self.visual.clone();
        }

        let command = &self.scenario.script[self.current_index];
        let mut visual = self.visual.clone();

        // Apply background override (empty string = clear)
        if let Some(bg) = &command.background {
            visual.background = if bg.is_empty() {
                None
            } else {
                Some(bg.clone())
            };
        }

        // Apply character override (empty string = clear)
        if let Some(ch) = &command.character {
            visual.character = if ch.is_empty() {
                None
            } else {
                Some(ch.clone())
            };
        }

        // Apply position override
        if let Some(pos) = command.char_pos {
            visual.char_pos = pos;
        }

        visual
    }

    /// Advance to the next command (for text display).
    pub fn advance(&mut self) {
        if self.current_index >= self.scenario.script.len() {
            return;
        }

        // Update visual state before advancing
        self.visual = self.current_visual();

        // Clone jump target before mutating self
        let jump_target = self.scenario.script[self.current_index].jump.clone();

        // If there's an unconditional jump, follow it
        if let Some(jump_label) = jump_target {
            self.jump_to(&jump_label);
            return;
        }

        self.current_index += 1;
        self.skip_labels();
    }

    /// Select a choice and jump to the target label.
    pub fn select_choice(&mut self, choice_index: usize) {
        if self.current_index >= self.scenario.script.len() {
            return;
        }

        // Update visual state before jumping
        self.visual = self.current_visual();

        // Clone jump target before mutating self
        let jump_target = self.scenario.script[self.current_index]
            .choices
            .as_ref()
            .and_then(|choices| choices.get(choice_index))
            .map(|choice| choice.jump.clone());

        if let Some(label) = jump_target {
            self.jump_to(&label);
        }
    }

    /// Jump to a labeled command.
    fn jump_to(&mut self, label: &str) {
        for (i, cmd) in self.scenario.script.iter().enumerate() {
            if cmd.label.as_deref() == Some(label) {
                self.current_index = i;
                self.skip_labels();
                return;
            }
        }
        // Label not found, go to end
        self.current_index = self.scenario.script.len();
    }

    /// Skip commands that only have labels (no content).
    fn skip_labels(&mut self) {
        while self.current_index < self.scenario.script.len() {
            let command = &self.scenario.script[self.current_index];

            // If command has displayable content, stop
            if command.text.is_some() || command.choices.is_some() {
                break;
            }

            // Update visual state for skipped commands
            self.visual = self.current_visual();

            // Clone jump target before mutating self
            let jump_target = command.jump.clone();

            // If command has jump, follow it
            if let Some(jump_label) = jump_target {
                self.jump_to(&jump_label);
                return;
            }

            self.current_index += 1;
        }
    }

    /// Check if the game has ended.
    pub fn is_ended(&self) -> bool {
        matches!(self.display_state(), DisplayState::End)
    }
}
