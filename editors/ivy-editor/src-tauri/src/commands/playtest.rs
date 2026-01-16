//! Tauri commands for playtest mode.

use crate::playtest::{PlaytestSession, PlaytestState};
use ivy::scenario::Scenario;
use ivy::types::Value;
use tauri::State;

#[tauri::command]
pub fn playtest_start(
    scenario: Scenario,
    language: Option<String>,
    scenario_path: Option<String>,
    base_dir: Option<String>,
    session: State<'_, PlaytestSession>,
) -> Result<PlaytestState, String> {
    Ok(session.start(scenario, language, scenario_path, base_dir))
}

#[tauri::command]
pub fn playtest_stop(session: State<'_, PlaytestSession>) -> Result<(), String> {
    session.stop();
    Ok(())
}

#[tauri::command]
pub fn playtest_get_state(
    session: State<'_, PlaytestSession>,
) -> Result<Option<PlaytestState>, String> {
    Ok(session.get_state())
}

#[tauri::command]
pub fn playtest_advance(session: State<'_, PlaytestSession>) -> Result<PlaytestState, String> {
    session
        .advance()
        .ok_or_else(|| "No active playtest session".to_string())
}

#[tauri::command]
pub fn playtest_select_choice(
    choice_index: usize,
    session: State<'_, PlaytestSession>,
) -> Result<PlaytestState, String> {
    session
        .select_choice(choice_index)
        .ok_or_else(|| "No active playtest session".to_string())
}

#[tauri::command]
pub fn playtest_rollback(session: State<'_, PlaytestSession>) -> Result<PlaytestState, String> {
    session
        .rollback()
        .ok_or_else(|| "No active playtest session".to_string())
}

#[tauri::command]
pub fn playtest_jump_to_label(
    label: String,
    session: State<'_, PlaytestSession>,
) -> Result<PlaytestState, String> {
    session
        .jump_to_label(&label)
        .ok_or_else(|| "No active playtest session".to_string())
}

#[tauri::command]
pub fn playtest_set_variable(
    name: String,
    value: Value,
    session: State<'_, PlaytestSession>,
) -> Result<PlaytestState, String> {
    session
        .set_variable(&name, value)
        .ok_or_else(|| "No active playtest session".to_string())
}

#[tauri::command]
pub fn playtest_restart(session: State<'_, PlaytestSession>) -> Result<PlaytestState, String> {
    session
        .restart()
        .ok_or_else(|| "No active playtest session".to_string())
}

#[tauri::command]
pub fn playtest_reload_scenario(
    scenario: Scenario,
    session: State<'_, PlaytestSession>,
) -> Result<PlaytestState, String> {
    session
        .reload_scenario(scenario)
        .ok_or_else(|| "No active playtest session".to_string())
}

#[tauri::command]
pub fn playtest_set_language(
    language: String,
    session: State<'_, PlaytestSession>,
) -> Result<PlaytestState, String> {
    session
        .set_language(language)
        .ok_or_else(|| "No active playtest session".to_string())
}

#[tauri::command]
pub fn playtest_submit_input(
    value: String,
    session: State<'_, PlaytestSession>,
) -> Result<PlaytestState, String> {
    session
        .submit_input(value)
        .ok_or_else(|| "No active playtest session".to_string())
}

#[tauri::command]
pub fn playtest_save(slot: u8, session: State<'_, PlaytestSession>) -> Result<(), String> {
    session.save(slot)
}

#[tauri::command]
pub fn playtest_load(
    slot: u8,
    session: State<'_, PlaytestSession>,
) -> Result<PlaytestState, String> {
    session.load(slot)
}
