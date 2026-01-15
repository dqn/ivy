mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
