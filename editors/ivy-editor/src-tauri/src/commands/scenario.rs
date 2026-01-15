use ivy::scenario::{parse_scenario, validate_scenario, Scenario, ValidationResult};
use std::fs;

#[tauri::command]
pub fn load_scenario(path: &str) -> Result<Scenario, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
    parse_scenario(&content).map_err(|e| format!("{}", e))
}

#[tauri::command]
pub fn save_scenario(path: &str, scenario: Scenario) -> Result<(), String> {
    let yaml =
        serde_yaml::to_string(&scenario).map_err(|e| format!("Failed to serialize: {}", e))?;
    fs::write(path, yaml).map_err(|e| format!("Failed to write file: {}", e))
}

#[tauri::command]
pub fn validate(scenario: Scenario) -> ValidationResult {
    validate_scenario(&scenario)
}

#[tauri::command]
pub fn scenario_to_yaml(scenario: Scenario) -> Result<String, String> {
    serde_yaml::to_string(&scenario).map_err(|e| format!("Failed to serialize: {}", e))
}

#[tauri::command]
pub fn create_empty_scenario(title: String) -> Scenario {
    Scenario {
        title,
        chapters: vec![],
        modular_characters: std::collections::HashMap::new(),
        script: vec![],
    }
}
