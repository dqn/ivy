mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::flowchart::get_flowchart,
            commands::scenario::load_scenario,
            commands::scenario::save_scenario,
            commands::scenario::validate,
            commands::scenario::scenario_to_yaml,
            commands::scenario::create_empty_scenario,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
