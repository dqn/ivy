use std::collections::VecDeque;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::platform;
use crate::runtime::Variables;
use crate::scenario::{CharPosition, Choice, Input, Scenario};

/// Maximum number of history entries for rollback.
const MAX_HISTORY_SIZE: usize = 50;

/// Single character state for multi-character support.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterState {
    pub path: String,
    pub position: CharPosition,
}

/// Visual state (background and character sprite).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualState {
    pub background: Option<String>,
    pub character: Option<String>,
    pub char_pos: CharPosition,
    /// Multiple characters (used when `characters` field is set in command).
    #[serde(default)]
    pub characters: Vec<CharacterState>,
}

/// Save data format.
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveData {
    pub scenario_path: String,
    pub current_index: usize,
    pub visual: VisualState,
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub variables: Variables,
}

impl SaveData {
    /// Save to a JSON file (or localStorage on WASM).
    pub fn save(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        platform::write_file(path, &json)?;
        Ok(())
    }

    /// Load from a JSON file (or localStorage on WASM).
    pub fn load(path: &str) -> Result<Self> {
        let content = platform::read_file(path)?;
        let save: SaveData = serde_json::from_str(&content)?;
        Ok(save)
    }

    /// Get the path for a specific save slot.
    pub fn slot_path(slot: u8) -> String {
        format!("saves/slot_{}.json", slot)
    }

    /// Check if a save slot exists.
    pub fn slot_exists(slot: u8) -> bool {
        platform::file_exists(&Self::slot_path(slot))
    }

    /// List all existing save slots with their timestamps.
    pub fn list_slots() -> Vec<(u8, i64)> {
        (1..=10)
            .filter_map(|slot| {
                Self::load(&Self::slot_path(slot))
                    .ok()
                    .map(|save| (slot, save.timestamp))
            })
            .collect()
    }
}

/// History entry for rollback functionality.
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub index: usize,
    pub visual: VisualState,
    pub text: String,
}

/// Current display state of the game.
#[derive(Debug, Clone)]
pub enum DisplayState {
    /// Showing text, waiting for player to advance.
    Text {
        speaker: Option<String>,
        text: String,
        visual: VisualState,
    },
    /// Showing choices, waiting for player to select.
    Choices {
        speaker: Option<String>,
        text: String,
        choices: Vec<Choice>,
        visual: VisualState,
        /// Optional timeout in seconds for timed choices.
        timeout: Option<f32>,
        /// Index of the default choice (selected on timeout).
        default_choice: Option<usize>,
    },
    /// Waiting for a specified duration.
    Wait {
        duration: f32,
        visual: VisualState,
    },
    /// Waiting for player text input.
    Input {
        input: Input,
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
    history: VecDeque<HistoryEntry>,
    variables: Variables,
}

impl GameState {
    /// Create a new game state from a scenario.
    pub fn new(scenario: Scenario) -> Self {
        let mut state = Self {
            scenario,
            current_index: 0,
            visual: VisualState::default(),
            history: VecDeque::new(),
            variables: Variables::new(),
        };
        state.skip_labels();
        state
    }

    /// Create a save data snapshot.
    pub fn to_save_data(&self, scenario_path: &str) -> SaveData {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        SaveData {
            scenario_path: scenario_path.to_string(),
            current_index: self.current_index,
            visual: self.current_visual(),
            timestamp,
            variables: self.variables.clone(),
        }
    }

    /// Restore from save data.
    pub fn from_save_data(save: &SaveData, scenario: Scenario) -> Self {
        let current_index = save.current_index.min(scenario.script.len());
        let mut state = Self {
            scenario,
            current_index,
            visual: save.visual.clone(),
            history: VecDeque::new(),
            variables: save.variables.clone(),
        };
        state.skip_labels();
        state
    }

    /// Get the current display state.
    pub fn display_state(&mut self) -> DisplayState {
        self.skip_labels();

        if self.current_index >= self.scenario.script.len() {
            return DisplayState::End;
        }

        let command = &self.scenario.script[self.current_index];
        let visual = self.current_visual();
        let speaker = command.speaker.clone();

        if let Some(choices) = &command.choices {
            let text = command.text.clone().unwrap_or_default();
            // Find default choice index
            let default_choice = choices
                .iter()
                .position(|c| c.default)
                .or(Some(0)); // Default to first choice if none marked
            return DisplayState::Choices {
                speaker,
                text,
                choices: choices.clone(),
                visual,
                timeout: command.timeout,
                default_choice,
            };
        }

        if let Some(text) = &command.text {
            return DisplayState::Text {
                speaker,
                text: text.clone(),
                visual,
            };
        }

        // Wait command without text
        if let Some(duration) = command.wait {
            return DisplayState::Wait { duration, visual };
        }

        // Input command
        if let Some(input) = &command.input {
            return DisplayState::Input {
                input: input.clone(),
                visual,
            };
        }

        // Command has no displayable content (should be unreachable after skip_labels).
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

        // Apply multiple characters override
        if let Some(chars) = &command.characters {
            visual.characters = chars
                .iter()
                .map(|c| CharacterState {
                    path: c.image.clone(),
                    position: c.pos,
                })
                .collect();
            // Clear single character when using multiple
            visual.character = None;
        } else {
            // Apply single character override (empty string = clear)
            if let Some(ch) = &command.character {
                visual.character = if ch.is_empty() {
                    None
                } else {
                    Some(ch.clone())
                };
                // Clear multiple characters when using single
                visual.characters.clear();
            }

            // Apply position override
            if let Some(pos) = command.char_pos {
                visual.char_pos = pos;
            }
        }

        visual
    }

    /// Process set command for current index.
    fn process_set(&mut self) {
        if let Some(set) = self
            .scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.set.as_ref())
        {
            self.variables.set(set.name.clone(), set.value.clone());
        }
    }

    /// Check if condition for current index and return jump target if true.
    fn check_condition(&self) -> Option<String> {
        let if_cond = self
            .scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.if_cond.as_ref())?;

        if self.variables.equals(&if_cond.var, &if_cond.is) {
            Some(if_cond.jump.clone())
        } else {
            None
        }
    }

    /// Push current state to history for rollback.
    fn push_history(&mut self) {
        let text = self.scenario.script.get(self.current_index)
            .and_then(|cmd| cmd.text.clone())
            .unwrap_or_default();

        let entry = HistoryEntry {
            index: self.current_index,
            visual: self.visual.clone(),
            text,
        };

        self.history.push_back(entry);

        // Limit history size
        if self.history.len() > MAX_HISTORY_SIZE {
            self.history.pop_front();
        }
    }

    /// Advance to the next command (for text display).
    pub fn advance(&mut self) {
        if self.current_index >= self.scenario.script.len() {
            return;
        }

        // Save current state for rollback
        self.push_history();

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

        // Save current state for rollback
        self.push_history();

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
            // Check for displayable content (scope the borrow)
            let has_displayable = {
                let command = &self.scenario.script[self.current_index];
                command.text.is_some()
                    || command.choices.is_some()
                    || command.wait.is_some()
                    || command.input.is_some()
            };

            if has_displayable {
                self.process_set();
                // Check conditional jump before displaying
                if let Some(jump_label) = self.check_condition() {
                    self.jump_to(&jump_label);
                    return;
                }
                break;
            }

            // Update visual state for skipped commands
            self.visual = self.current_visual();

            // Process set command
            self.process_set();

            // Check conditional jump first
            if let Some(jump_label) = self.check_condition() {
                self.jump_to(&jump_label);
                return;
            }

            // Clone unconditional jump target (scope the borrow)
            let jump_target = self.scenario.script[self.current_index].jump.clone();

            // If command has unconditional jump, follow it
            if let Some(jump_label) = jump_target {
                self.jump_to(&jump_label);
                return;
            }

            self.current_index += 1;
        }
    }

    /// Check if the game has ended.
    pub fn is_ended(&self) -> bool {
        self.current_index >= self.scenario.script.len()
    }

    /// Check if rollback is available.
    pub fn can_rollback(&self) -> bool {
        !self.history.is_empty()
    }

    /// Roll back to the previous state.
    pub fn rollback(&mut self) -> bool {
        if let Some(entry) = self.history.pop_back() {
            self.current_index = entry.index;
            self.visual = entry.visual;
            true
        } else {
            false
        }
    }

    /// Get history entries for backlog display.
    pub fn history(&self) -> &VecDeque<HistoryEntry> {
        &self.history
    }

    /// Get current BGM command (None = keep, Some("") = stop, Some(path) = play).
    pub fn current_bgm(&self) -> Option<&String> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.bgm.as_ref())
    }

    /// Get current SE command.
    pub fn current_se(&self) -> Option<&String> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.se.as_ref())
    }

    /// Get current voice command.
    pub fn current_voice(&self) -> Option<&String> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.voice.as_ref())
    }

    /// Get current script index.
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Get variables for reading.
    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    /// Set a variable value.
    pub fn set_variable(&mut self, name: impl Into<String>, value: crate::runtime::Value) {
        self.variables.set(name, value);
    }

    /// Get current transition command.
    pub fn current_transition(&self) -> Option<&crate::scenario::types::Transition> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.transition.as_ref())
    }

    /// Get current shake command.
    pub fn current_shake(&self) -> Option<&crate::scenario::types::Shake> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.shake.as_ref())
    }

    /// Get current character enter animation.
    pub fn current_char_enter(&self) -> Option<&crate::scenario::types::CharAnimation> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.char_enter.as_ref())
    }

    /// Get current character exit animation.
    pub fn current_char_exit(&self) -> Option<&crate::scenario::types::CharAnimation> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.char_exit.as_ref())
    }

    /// Get current particles command (None = keep, Some("") = stop, Some(type) = start).
    pub fn current_particles(&self) -> Option<(&String, f32)> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.particles.as_ref().map(|p| (p, cmd.particle_intensity)))
    }

    /// Get current cinematic command (None = keep, Some(bool) = set on/off).
    pub fn current_cinematic(&self) -> Option<(bool, f32)> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.cinematic.map(|c| (c, cmd.cinematic_duration)))
    }

    /// Get current achievement unlock command.
    pub fn current_achievement(&self) -> Option<&crate::scenario::types::Achievement> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.achievement.as_ref())
    }
}
