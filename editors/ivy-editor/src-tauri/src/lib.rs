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
            commands::flowchart::get_flowchart,
            commands::preview::get_preview_state,
            commands::scenario::load_scenario,
            commands::scenario::save_scenario,
            commands::scenario::validate,
            commands::scenario::scenario_to_yaml,
            commands::scenario::create_empty_scenario,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
