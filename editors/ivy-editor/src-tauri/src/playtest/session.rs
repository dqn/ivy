//! Playtest session management.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use ivy::i18n::LocalizedString;
use ivy::runtime::display::DisplayState;
use ivy::runtime::save::SaveData;
use ivy::runtime::state::GameState;
use ivy::scenario::{CharPosition, Scenario};
use ivy::types::Value;
use serde::{Deserialize, Serialize};

/// A playtest session that manages GameState for interactive scenario execution.
pub struct PlaytestSession {
    state: Mutex<Option<SessionData>>,
}

struct SessionData {
    game_state: GameState,
    scenario: Scenario,
    language: String,
    scenario_path: Option<String>,
    base_dir: Option<PathBuf>,
}

/// Playtest save data format for the editor.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlaytestSaveData {
    pub scenario_path: String,
    pub save_data: SaveData,
}

/// Display state returned to the frontend.
#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PlaytestDisplay {
    Text {
        speaker: Option<String>,
        text: String,
        background: Option<String>,
        character: Option<String>,
        char_pos: Option<String>,
        nvl_mode: bool,
    },
    Choices {
        speaker: Option<String>,
        text: String,
        choices: Vec<PlaytestChoice>,
        background: Option<String>,
        character: Option<String>,
        char_pos: Option<String>,
        timeout: Option<f32>,
        default_choice: Option<usize>,
    },
    Input {
        prompt: String,
        var_name: String,
        default_value: Option<String>,
        background: Option<String>,
        character: Option<String>,
        char_pos: Option<String>,
    },
    Wait {
        duration: f32,
        background: Option<String>,
        character: Option<String>,
        char_pos: Option<String>,
    },
    Video {
        path: String,
        skippable: bool,
        loop_video: bool,
    },
    End,
}

#[derive(Clone, Serialize)]
pub struct PlaytestChoice {
    pub label: String,
    pub jump: String,
}

/// A history entry for the backlog display.
#[derive(Clone, Serialize)]
pub struct PlaytestHistoryEntry {
    pub index: usize,
    pub speaker: Option<String>,
    pub text: String,
}

/// Transition effect info for the frontend.
#[derive(Clone, Serialize)]
pub struct PlaytestTransition {
    #[serde(rename = "type")]
    pub transition_type: String,
    pub duration: f32,
    pub direction: String,
}

/// Full playtest state returned to the frontend.
#[derive(Clone, Serialize)]
pub struct PlaytestState {
    pub active: bool,
    pub command_index: usize,
    pub total_commands: usize,
    pub display: PlaytestDisplay,
    pub variables: HashMap<String, Value>,
    pub history_count: usize,
    pub can_rollback: bool,
    pub is_ended: bool,
    pub labels: Vec<String>,
    pub current_label: Option<String>,
    pub history: Vec<PlaytestHistoryEntry>,
    pub transition: Option<PlaytestTransition>,
}

impl Default for PlaytestSession {
    fn default() -> Self {
        Self::new()
    }
}

impl PlaytestSession {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(None),
        }
    }

    /// Start a new playtest session.
    pub fn start(
        &self,
        scenario: Scenario,
        language: Option<String>,
        scenario_path: Option<String>,
        base_dir: Option<String>,
    ) -> PlaytestState {
        let game_state = GameState::new(scenario.clone());
        let lang = language.unwrap_or_else(|| "en".to_string());

        let mut state = self.state.lock().unwrap();
        *state = Some(SessionData {
            game_state,
            scenario,
            language: lang,
            scenario_path,
            base_dir: base_dir.map(PathBuf::from),
        });

        self.build_state_inner(state.as_mut().unwrap())
    }

    /// Stop the current playtest session.
    pub fn stop(&self) {
        let mut state = self.state.lock().unwrap();
        *state = None;
    }

    /// Check if a session is active.
    #[allow(dead_code)]
    pub fn is_active(&self) -> bool {
        self.state.lock().unwrap().is_some()
    }

    /// Get the current state.
    pub fn get_state(&self) -> Option<PlaytestState> {
        let mut state = self.state.lock().unwrap();
        state.as_mut().map(|s| self.build_state_inner(s))
    }

    /// Advance to the next command.
    pub fn advance(&self) -> Option<PlaytestState> {
        let mut state = self.state.lock().unwrap();
        if let Some(ref mut data) = *state {
            data.game_state.advance();
            Some(self.build_state_inner(data))
        } else {
            None
        }
    }

    /// Select a choice.
    pub fn select_choice(&self, choice_index: usize) -> Option<PlaytestState> {
        let mut state = self.state.lock().unwrap();
        if let Some(ref mut data) = *state {
            data.game_state.select_choice(choice_index);
            Some(self.build_state_inner(data))
        } else {
            None
        }
    }

    /// Roll back to the previous state.
    pub fn rollback(&self) -> Option<PlaytestState> {
        let mut state = self.state.lock().unwrap();
        if let Some(ref mut data) = *state {
            data.game_state.rollback();
            Some(self.build_state_inner(data))
        } else {
            None
        }
    }

    /// Jump to a specific label.
    pub fn jump_to_label(&self, label: &str) -> Option<PlaytestState> {
        let mut state = self.state.lock().unwrap();
        if let Some(ref mut data) = *state {
            data.game_state.jump_to_label(label);
            Some(self.build_state_inner(data))
        } else {
            None
        }
    }

    /// Set a variable.
    pub fn set_variable(&self, name: &str, value: Value) -> Option<PlaytestState> {
        let mut state = self.state.lock().unwrap();
        if let Some(ref mut data) = *state {
            data.game_state.set_variable(name, value);
            Some(self.build_state_inner(data))
        } else {
            None
        }
    }

    /// Restart the playtest from the beginning.
    pub fn restart(&self) -> Option<PlaytestState> {
        let mut state = self.state.lock().unwrap();
        if let Some(ref mut data) = *state {
            data.game_state = GameState::new(data.scenario.clone());
            Some(self.build_state_inner(data))
        } else {
            None
        }
    }

    /// Reload the scenario (useful for hot-reload).
    pub fn reload_scenario(&self, scenario: Scenario) -> Option<PlaytestState> {
        let mut state = self.state.lock().unwrap();
        if let Some(ref mut data) = *state {
            data.game_state.reload_scenario(scenario.clone());
            data.scenario = scenario;
            Some(self.build_state_inner(data))
        } else {
            None
        }
    }

    /// Set the language for localized strings.
    pub fn set_language(&self, language: String) -> Option<PlaytestState> {
        let mut state = self.state.lock().unwrap();
        if let Some(ref mut data) = *state {
            data.language = language;
            Some(self.build_state_inner(data))
        } else {
            None
        }
    }

    /// Submit input for an input command (player name entry, etc.).
    pub fn submit_input(&self, value: String) -> Option<PlaytestState> {
        let mut state = self.state.lock().unwrap();
        if let Some(ref mut data) = *state {
            data.game_state.submit_input(value);
            Some(self.build_state_inner(data))
        } else {
            None
        }
    }

    /// Save the current playtest state to a file.
    pub fn save(&self, slot: u8) -> Result<(), String> {
        let state = self.state.lock().unwrap();
        let data = state
            .as_ref()
            .ok_or_else(|| "No active playtest session".to_string())?;

        let base_dir = data
            .base_dir
            .as_ref()
            .ok_or_else(|| "No base directory set".to_string())?;

        let scenario_path = data
            .scenario_path
            .as_ref()
            .ok_or_else(|| "No scenario path set".to_string())?;

        // Create save directory
        let save_dir = base_dir.join(".ivy-playtest-saves");
        std::fs::create_dir_all(&save_dir)
            .map_err(|e| format!("Failed to create save directory: {}", e))?;

        // Create save data
        let save_data = data.game_state.to_save_data(scenario_path);
        let playtest_save = PlaytestSaveData {
            scenario_path: scenario_path.clone(),
            save_data,
        };

        // Write to file
        let save_path = save_dir.join(format!("slot_{}.json", slot));
        let json = serde_json::to_string_pretty(&playtest_save)
            .map_err(|e| format!("Failed to serialize save data: {}", e))?;
        std::fs::write(&save_path, json)
            .map_err(|e| format!("Failed to write save file: {}", e))?;

        Ok(())
    }

    /// Load a playtest state from a file.
    pub fn load(&self, slot: u8) -> Result<PlaytestState, String> {
        let mut state = self.state.lock().unwrap();
        let data = state
            .as_mut()
            .ok_or_else(|| "No active playtest session".to_string())?;

        let base_dir = data
            .base_dir
            .as_ref()
            .ok_or_else(|| "No base directory set".to_string())?;

        // Read save file
        let save_path = base_dir
            .join(".ivy-playtest-saves")
            .join(format!("slot_{}.json", slot));
        let json = std::fs::read_to_string(&save_path)
            .map_err(|e| format!("Failed to read save file: {}", e))?;

        // Parse save data
        let playtest_save: PlaytestSaveData =
            serde_json::from_str(&json).map_err(|e| format!("Failed to parse save data: {}", e))?;

        // Restore game state
        data.game_state =
            GameState::from_save_data(&playtest_save.save_data, data.scenario.clone());

        Ok(self.build_state_inner(data))
    }

    fn build_state_inner(&self, data: &mut SessionData) -> PlaytestState {
        let display = data.game_state.display_state();
        let lang = &data.language;
        let scenario = &data.scenario;

        let labels: Vec<String> = scenario
            .script
            .iter()
            .filter_map(|cmd| cmd.label.clone())
            .collect();

        let current_label = scenario
            .script
            .iter()
            .take(data.game_state.current_index() + 1)
            .rev()
            .find_map(|cmd| cmd.label.clone());

        let variables: HashMap<String, Value> = data
            .game_state
            .variables()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Build history entries
        let history: Vec<PlaytestHistoryEntry> = data
            .game_state
            .history()
            .iter()
            .map(|entry| {
                let speaker = scenario
                    .script
                    .get(entry.index)
                    .and_then(|cmd| cmd.speaker.as_ref())
                    .map(|s| resolve_localized(s, lang));
                PlaytestHistoryEntry {
                    index: entry.index,
                    speaker,
                    text: resolve_localized(&entry.text, lang),
                }
            })
            .collect();

        // Extract transition from current command
        let transition = scenario
            .script
            .get(data.game_state.current_index())
            .and_then(|cmd| cmd.transition.as_ref())
            .map(|t| PlaytestTransition {
                transition_type: format!("{:?}", t.transition_type).to_lowercase(),
                duration: t.duration,
                direction: format!("{:?}", t.direction).to_lowercase(),
            });

        PlaytestState {
            active: true,
            command_index: data.game_state.current_index(),
            total_commands: scenario.script.len(),
            display: convert_display(&display, lang),
            variables,
            history_count: data.game_state.history().len(),
            can_rollback: data.game_state.can_rollback(),
            is_ended: data.game_state.is_ended(),
            labels,
            current_label,
            history,
            transition,
        }
    }
}

fn resolve_localized(s: &LocalizedString, lang: &str) -> String {
    match s {
        LocalizedString::Plain(text) => text.clone(),
        LocalizedString::Localized(map) => map
            .get(lang)
            .or_else(|| map.get("en"))
            .or_else(|| map.values().next())
            .cloned()
            .unwrap_or_default(),
        LocalizedString::Key(key) => format!("@{}", key),
    }
}

fn format_char_pos(pos: CharPosition) -> String {
    match pos {
        CharPosition::Left => "left".to_string(),
        CharPosition::Center => "center".to_string(),
        CharPosition::Right => "right".to_string(),
    }
}

fn convert_display(display: &DisplayState, lang: &str) -> PlaytestDisplay {
    match display {
        DisplayState::Text {
            speaker,
            text,
            visual,
        } => PlaytestDisplay::Text {
            speaker: speaker.as_ref().map(|s| resolve_localized(s, lang)),
            text: resolve_localized(text, lang),
            background: visual.background.clone(),
            character: visual.character.clone(),
            char_pos: Some(format_char_pos(visual.char_pos)),
            nvl_mode: visual.nvl_mode,
        },
        DisplayState::Choices {
            speaker,
            text,
            choices,
            visual,
            timeout,
            default_choice,
        } => PlaytestDisplay::Choices {
            speaker: speaker.as_ref().map(|s| resolve_localized(s, lang)),
            text: resolve_localized(text, lang),
            choices: choices
                .iter()
                .map(|c| PlaytestChoice {
                    label: resolve_localized(&c.label, lang),
                    jump: c.jump.clone(),
                })
                .collect(),
            background: visual.background.clone(),
            character: visual.character.clone(),
            char_pos: Some(format_char_pos(visual.char_pos)),
            timeout: *timeout,
            default_choice: *default_choice,
        },
        DisplayState::Input { input, visual } => PlaytestDisplay::Input {
            prompt: input.prompt.clone().unwrap_or_default(),
            var_name: input.var.clone(),
            default_value: input.default.clone(),
            background: visual.background.clone(),
            character: visual.character.clone(),
            char_pos: Some(format_char_pos(visual.char_pos)),
        },
        DisplayState::Wait { duration, visual } => PlaytestDisplay::Wait {
            duration: *duration,
            background: visual.background.clone(),
            character: visual.character.clone(),
            char_pos: Some(format_char_pos(visual.char_pos)),
        },
        DisplayState::Video {
            path,
            skippable,
            loop_video,
            ..
        } => PlaytestDisplay::Video {
            path: path.clone(),
            skippable: *skippable,
            loop_video: *loop_video,
        },
        DisplayState::End => PlaytestDisplay::End,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_scenario() -> Scenario {
        // Use YAML parsing to create commands since Command doesn't implement Default
        let yaml = r#"
title: Test
script:
  - text: "Hello"
  - text: "World"
"#;
        serde_yaml::from_str(yaml).unwrap()
    }

    #[test]
    fn test_start_session() {
        let session = PlaytestSession::new();
        let state = session.start(create_test_scenario(), None);

        assert!(state.active);
        assert_eq!(state.command_index, 0);
        assert_eq!(state.total_commands, 2);
        assert!(!state.is_ended);
    }

    #[test]
    fn test_advance() {
        let session = PlaytestSession::new();
        session.start(create_test_scenario(), None);

        let state = session.advance().unwrap();
        assert_eq!(state.command_index, 1);
        assert!(state.can_rollback);
    }

    #[test]
    fn test_rollback() {
        let session = PlaytestSession::new();
        session.start(create_test_scenario(), None);
        session.advance();

        let state = session.rollback().unwrap();
        assert_eq!(state.command_index, 0);
    }

    #[test]
    fn test_restart() {
        let session = PlaytestSession::new();
        session.start(create_test_scenario(), None);
        session.advance();

        let state = session.restart().unwrap();
        assert_eq!(state.command_index, 0);
        assert!(!state.can_rollback);
    }

    #[test]
    fn test_stop_session() {
        let session = PlaytestSession::new();
        session.start(create_test_scenario(), None);
        assert!(session.is_active());

        session.stop();
        assert!(!session.is_active());
        assert!(session.get_state().is_none());
    }
}
