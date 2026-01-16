mod commands;
mod playtest;

use playtest::PlaytestSession;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(PlaytestSession::new())
        .invoke_handler(tauri::generate_handler![
            commands::assets::get_relative_path,
            commands::assets::read_asset_base64,
            commands::assets::list_assets,
            commands::assets::get_asset_info,
            commands::assets::find_asset_usages,
            commands::assets::find_unused_assets,
            commands::character::load_characters,
            commands::character::save_characters,
            commands::character::add_character,
            commands::character::update_character,
            commands::character::remove_character,
            commands::character::extract_speakers,
            commands::character::find_character_usages,
            commands::character::get_merged_characters,
            commands::character::find_character_by_speaker,
            commands::flowchart::get_flowchart,
            commands::playtest::playtest_start,
            commands::playtest::playtest_stop,
            commands::playtest::playtest_get_state,
            commands::playtest::playtest_advance,
            commands::playtest::playtest_select_choice,
            commands::playtest::playtest_rollback,
            commands::playtest::playtest_jump_to_label,
            commands::playtest::playtest_set_variable,
            commands::playtest::playtest_restart,
            commands::playtest::playtest_reload_scenario,
            commands::playtest::playtest_set_language,
            commands::playtest::playtest_submit_input,
            commands::playtest::playtest_save,
            commands::playtest::playtest_load,
            commands::preview::get_preview_state,
            commands::project::create_project,
            commands::project::load_project,
            commands::project::save_project,
            commands::project::add_scenario_to_project,
            commands::project::remove_scenario_from_project,
            commands::project::is_project_directory,
            commands::scenario::load_scenario,
            commands::scenario::save_scenario,
            commands::scenario::validate,
            commands::scenario::scenario_to_yaml,
            commands::scenario::create_empty_scenario,
            commands::savedata::list_save_data,
            commands::savedata::validate_save_data,
            commands::export::check_build_environment,
            commands::export::start_export,
            commands::export::cancel_export,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
