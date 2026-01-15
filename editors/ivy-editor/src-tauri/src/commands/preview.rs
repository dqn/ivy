use ivy::i18n::LocalizedString;
use ivy::scenario::{CharPosition, Scenario};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub struct ChoiceInfo {
    pub label: String,
    pub jump: String,
}

#[derive(Clone, Serialize)]
pub struct PreviewState {
    pub title: String,
    pub command_index: usize,
    pub total_commands: usize,
    pub text: Option<String>,
    pub speaker: Option<String>,
    pub background: Option<String>,
    pub character: Option<String>,
    pub char_pos: Option<String>,
    pub choices: Vec<ChoiceInfo>,
    pub variables: HashMap<String, String>,
    pub labels: Vec<String>,
    pub current_label: Option<String>,
    pub nvl_mode: bool,
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

fn format_char_pos(pos: &CharPosition) -> String {
    match pos {
        CharPosition::Left => "left".to_string(),
        CharPosition::Center => "center".to_string(),
        CharPosition::Right => "right".to_string(),
    }
}

#[tauri::command]
pub fn get_preview_state(
    scenario: Scenario,
    index: usize,
    variables: HashMap<String, String>,
    lang: Option<String>,
) -> PreviewState {
    let lang = lang.as_deref().unwrap_or("en");
    let total = scenario.script.len();
    let idx = index.min(total.saturating_sub(1));

    // Collect all labels
    let labels: Vec<String> = scenario
        .script
        .iter()
        .filter_map(|cmd| cmd.label.clone())
        .collect();

    // Find current label (scan backwards from index)
    let current_label = scenario
        .script
        .iter()
        .take(idx + 1)
        .rev()
        .find_map(|cmd| cmd.label.clone());

    // Reconstruct visual state by scanning from start
    let mut background: Option<String> = None;
    let mut character: Option<String> = None;
    let mut char_pos: Option<String> = None;
    let mut nvl_mode = false;

    for cmd in scenario.script.iter().take(idx + 1) {
        if let Some(ref bg) = cmd.background {
            if bg.is_empty() {
                background = None;
            } else {
                background = Some(bg.clone());
            }
        }
        if let Some(ref ch) = cmd.character {
            if ch.is_empty() {
                character = None;
            } else {
                character = Some(ch.clone());
            }
        }
        if let Some(ref pos) = cmd.char_pos {
            char_pos = Some(format_char_pos(pos));
        }
        if let Some(nvl) = cmd.nvl {
            nvl_mode = nvl;
        }
    }

    // Current command info
    let current_cmd = scenario.script.get(idx);
    let text = current_cmd
        .and_then(|c| c.text.as_ref())
        .map(|s| resolve_localized(s, lang));
    let speaker = current_cmd
        .and_then(|c| c.speaker.as_ref())
        .map(|s| resolve_localized(s, lang));

    let choices = current_cmd
        .and_then(|c| c.choices.as_ref())
        .map(|choices| {
            choices
                .iter()
                .map(|c| ChoiceInfo {
                    label: resolve_localized(&c.label, lang),
                    jump: c.jump.clone(),
                })
                .collect()
        })
        .unwrap_or_default();

    PreviewState {
        title: scenario.title.clone(),
        command_index: idx,
        total_commands: total,
        text,
        speaker,
        background,
        character,
        char_pos,
        choices,
        variables,
        labels,
        current_label,
        nvl_mode,
    }
}
