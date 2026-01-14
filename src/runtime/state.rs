use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::runtime::display::{DisplayState, HistoryEntry};
use crate::runtime::save::SaveData;
use crate::runtime::variables::Variables;
use crate::runtime::visual::{CharacterState, VisualState};
use crate::scenario::Scenario;

/// Maximum number of history entries for rollback.
const MAX_HISTORY_SIZE: usize = 50;

/// Runtime state for the visual novel engine.
#[derive(Debug)]
pub struct GameState {
    scenario: Scenario,
    current_index: usize,
    visual: VisualState,
    history: VecDeque<HistoryEntry>,
    variables: Variables,
    /// Label to index mapping for O(1) lookup.
    label_index: HashMap<String, usize>,
}

/// Build label index from scenario.
fn build_label_index(scenario: &Scenario) -> HashMap<String, usize> {
    scenario
        .script
        .iter()
        .enumerate()
        .filter_map(|(i, cmd)| cmd.label.as_ref().map(|label| (label.clone(), i)))
        .collect()
}

impl GameState {
    /// Create a new game state from a scenario.
    pub fn new(scenario: Scenario) -> Self {
        let label_index = build_label_index(&scenario);
        let mut state = Self {
            scenario,
            current_index: 0,
            visual: VisualState::default(),
            history: VecDeque::new(),
            variables: Variables::new(),
            label_index,
        };
        state.skip_labels();
        state
    }

    /// Create a save data snapshot.
    pub fn to_save_data(&self, scenario_path: &str) -> SaveData {
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
        let label_index = build_label_index(&scenario);
        let mut state = Self {
            scenario,
            current_index,
            visual: save.visual.clone(),
            history: VecDeque::new(),
            variables: save.variables.clone(),
            label_index,
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
            let text = command
                .text
                .clone()
                .unwrap_or_default();
            // Find default choice index
            let default_choice = choices.iter().position(|c| c.default).or(Some(0)); // Default to first choice if none marked
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
                    enter: c.enter.clone(),
                    exit: c.exit.clone(),
                    idle: c.idle.clone(),
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
        let text = self
            .scenario
            .script
            .get(self.current_index)
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

    /// Jump to a labeled command (public wrapper).
    pub fn jump_to_label(&mut self, label: &str) {
        self.jump_to(label);
    }

    /// Jump to a labeled command (internal). O(1) lookup using label index.
    fn jump_to(&mut self, label: &str) {
        if let Some(&index) = self.label_index.get(label) {
            self.current_index = index;
            self.skip_labels();
        } else {
            // Label not found, go to end
            self.current_index = self.scenario.script.len();
        }
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
    pub fn set_variable(&mut self, name: impl Into<String>, value: crate::types::Value) {
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

    /// Get current character idle animation.
    pub fn current_char_idle(&self) -> Option<&crate::scenario::types::CharIdleAnimation> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.char_idle.as_ref())
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

    /// Get the label at current position (if any).
    pub fn current_label(&self) -> Option<String> {
        self.scenario
            .script
            .get(self.current_index)
            .and_then(|cmd| cmd.label.clone())
    }

    /// Reload scenario while preserving state.
    ///
    /// Attempts to maintain the current position by finding the same label
    /// in the new scenario. If the label is not found, tries to use the
    /// same index. Variables are preserved.
    pub fn reload_scenario(&mut self, scenario: Scenario) {
        let old_label = self.current_label();
        let old_index = self.current_index;

        self.label_index = build_label_index(&scenario);
        self.scenario = scenario;

        // Try to jump to the same label first
        if let Some(label) = old_label
            && let Some(&index) = self.label_index.get(&label) {
            self.current_index = index;
            self.skip_labels();
            return;
            }

        // Fall back to the same index (clamped to script length)
        self.current_index = old_index.min(self.scenario.script.len().saturating_sub(1));
        self.skip_labels();
    }
}
