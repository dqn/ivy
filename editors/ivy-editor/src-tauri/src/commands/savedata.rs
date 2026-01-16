use chrono::{DateTime, Local, TimeZone};
use ivy::runtime::save::SaveData;
use ivy::scenario::parse_scenario;
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SaveDataVisualState {
    pub background: Option<String>,
    pub character: Option<String>,
    pub char_pos: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SaveDataSummary {
    pub scenario_path: String,
    pub current_index: usize,
    pub total_commands: Option<usize>,
    pub timestamp: i64,
    pub formatted_time: String,
    pub variable_count: usize,
    pub visual: SaveDataVisualState,
}

#[derive(Debug, Clone, Serialize)]
pub struct SaveDataValidationResult {
    pub valid: bool,
    pub file_path: String,
    pub summary: Option<SaveDataSummary>,
    pub issues: Vec<ValidationIssue>,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct SaveDataInfo {
    pub file_name: String,
    pub file_path: String,
    pub slot: Option<u8>,
    pub timestamp: i64,
    pub formatted_time: String,
    pub scenario_path: Option<String>,
    pub size_bytes: u64,
}

fn format_timestamp(timestamp: i64) -> String {
    Local
        .timestamp_opt(timestamp, 0)
        .single()
        .map(|dt: DateTime<Local>| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn extract_slot_number(file_name: &str) -> Option<u8> {
    // Match patterns like "slot_1.json", "slot_10.json"
    if file_name.starts_with("slot_") && file_name.ends_with(".json") {
        let num_part = &file_name[5..file_name.len() - 5];
        num_part.parse().ok()
    } else {
        None
    }
}

#[tauri::command]
pub fn list_save_data(base_dir: &str) -> Result<Vec<SaveDataInfo>, String> {
    let saves_dir = Path::new(base_dir).join("saves");

    if !saves_dir.exists() {
        return Ok(vec![]);
    }

    let entries =
        fs::read_dir(&saves_dir).map_err(|e| format!("Failed to read saves directory: {}", e))?;

    let mut save_files: Vec<SaveDataInfo> = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();

        if path.extension().map(|e| e == "json").unwrap_or(false) {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            let metadata = fs::metadata(&path).ok();
            let size_bytes = metadata.map(|m| m.len()).unwrap_or(0);

            // Try to parse the save data to get metadata
            let (timestamp, scenario_path) = match fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str::<SaveData>(&content) {
                    Ok(save) => (save.timestamp, Some(save.scenario_path)),
                    Err(_) => (0, None),
                },
                Err(_) => (0, None),
            };

            save_files.push(SaveDataInfo {
                file_name: file_name.clone(),
                file_path: path.to_string_lossy().to_string(),
                slot: extract_slot_number(&file_name),
                timestamp,
                formatted_time: format_timestamp(timestamp),
                scenario_path,
                size_bytes,
            });
        }
    }

    // Sort by slot number first, then by timestamp (newest first)
    save_files.sort_by(|a, b| match (a.slot, b.slot) {
        (Some(slot_a), Some(slot_b)) => slot_a.cmp(&slot_b),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => b.timestamp.cmp(&a.timestamp),
    });

    Ok(save_files)
}

#[tauri::command]
pub fn validate_save_data(save_path: &str, base_dir: Option<&str>) -> SaveDataValidationResult {
    let mut issues: Vec<ValidationIssue> = Vec::new();
    let file_path = save_path.to_string();

    // Read the file
    let content = match fs::read_to_string(save_path) {
        Ok(c) => c,
        Err(e) => {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                code: "FILE_READ_ERROR".to_string(),
                message: "Failed to read save file".to_string(),
                details: Some(e.to_string()),
            });
            return SaveDataValidationResult {
                valid: false,
                file_path,
                summary: None,
                issues,
                error_count: 1,
                warning_count: 0,
                info_count: 0,
            };
        }
    };

    // Parse JSON
    let save_data: SaveData = match serde_json::from_str(&content) {
        Ok(s) => s,
        Err(e) => {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                code: "JSON_PARSE_ERROR".to_string(),
                message: "Invalid JSON format".to_string(),
                details: Some(e.to_string()),
            });
            return SaveDataValidationResult {
                valid: false,
                file_path,
                summary: None,
                issues,
                error_count: 1,
                warning_count: 0,
                info_count: 0,
            };
        }
    };

    // Validate required fields
    if save_data.scenario_path.is_empty() {
        issues.push(ValidationIssue {
            severity: IssueSeverity::Error,
            code: "MISSING_SCENARIO_PATH".to_string(),
            message: "Missing required field: scenario_path".to_string(),
            details: None,
        });
    }

    // Check scenario file exists
    let scenario_full_path = if let Some(base) = base_dir {
        Path::new(base).join(&save_data.scenario_path)
    } else {
        Path::new(&save_data.scenario_path).to_path_buf()
    };

    let mut total_commands: Option<usize> = None;
    let mut scenario_var_refs: HashSet<String> = HashSet::new();

    if scenario_full_path.exists() {
        // Load and parse scenario to validate index and variables
        if let Ok(scenario_content) = fs::read_to_string(&scenario_full_path) {
            match parse_scenario(&scenario_content) {
                Ok(scenario) => {
                    total_commands = Some(scenario.script.len());

                    // Check index is within range
                    if save_data.current_index >= scenario.script.len() {
                        issues.push(ValidationIssue {
                            severity: IssueSeverity::Error,
                            code: "INDEX_OUT_OF_RANGE".to_string(),
                            message: format!(
                                "current_index {} is out of range (0-{})",
                                save_data.current_index,
                                scenario.script.len().saturating_sub(1)
                            ),
                            details: None,
                        });
                    }

                    // Collect variable references from scenario
                    for cmd in &scenario.script {
                        // Variables from set commands
                        if let Some(set) = &cmd.set {
                            scenario_var_refs.insert(set.name.clone());
                        }
                        // Variables from if conditions
                        if let Some(if_cond) = &cmd.if_cond {
                            scenario_var_refs.insert(if_cond.var.clone());
                        }
                        // Variables from input commands
                        if let Some(input) = &cmd.input {
                            scenario_var_refs.insert(input.var.clone());
                        }
                    }
                }
                Err(e) => {
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::Warning,
                        code: "SCENARIO_PARSE_ERROR".to_string(),
                        message: "Failed to parse scenario file".to_string(),
                        details: Some(e.to_string()),
                    });
                }
            }
        }
    } else if !save_data.scenario_path.is_empty() {
        issues.push(ValidationIssue {
            severity: IssueSeverity::Error,
            code: "SCENARIO_NOT_FOUND".to_string(),
            message: format!("Scenario file not found: {}", save_data.scenario_path),
            details: None,
        });
    }

    // Check visual state assets
    if let Some(base) = base_dir {
        // Check background
        if let Some(bg) = &save_data.visual.background {
            let bg_path = Path::new(base).join(bg);
            if !bg_path.exists() {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    code: "BACKGROUND_NOT_FOUND".to_string(),
                    message: format!("Background image not found: {}", bg),
                    details: None,
                });
            }
        }

        // Check character
        if let Some(char_path) = &save_data.visual.character {
            let char_full_path = Path::new(base).join(char_path);
            if !char_full_path.exists() {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    code: "CHARACTER_NOT_FOUND".to_string(),
                    message: format!("Character image not found: {}", char_path),
                    details: None,
                });
            }
        }
    }

    // Variable validation
    let save_vars: HashSet<String> = save_data.variables.all().keys().cloned().collect();

    // Unused variables (in save but not referenced in scenario)
    if !scenario_var_refs.is_empty() {
        for var in save_vars.difference(&scenario_var_refs) {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                code: "UNUSED_VARIABLE".to_string(),
                message: format!(
                    "Variable '{}' in save data is not referenced in scenario",
                    var
                ),
                details: None,
            });
        }
    }

    // Missing variables (referenced in scenario but not in save)
    // This is just info, not an error, as variables might be set later
    for var in scenario_var_refs.difference(&save_vars) {
        issues.push(ValidationIssue {
            severity: IssueSeverity::Info,
            code: "UNDEFINED_VARIABLE".to_string(),
            message: format!(
                "Variable '{}' referenced in scenario but not in save data",
                var
            ),
            details: Some("This variable may be set after the current position".to_string()),
        });
    }

    // Add info about save data
    issues.push(ValidationIssue {
        severity: IssueSeverity::Info,
        code: "SAVE_INFO".to_string(),
        message: format!(
            "Position: {} | Variables: {} | Timestamp: {}",
            save_data.current_index,
            save_data.variables.all().len(),
            format_timestamp(save_data.timestamp)
        ),
        details: None,
    });

    // Count issues by severity
    let error_count = issues
        .iter()
        .filter(|i| matches!(i.severity, IssueSeverity::Error))
        .count();
    let warning_count = issues
        .iter()
        .filter(|i| matches!(i.severity, IssueSeverity::Warning))
        .count();
    let info_count = issues
        .iter()
        .filter(|i| matches!(i.severity, IssueSeverity::Info))
        .count();

    // Build summary
    let summary = SaveDataSummary {
        scenario_path: save_data.scenario_path,
        current_index: save_data.current_index,
        total_commands,
        timestamp: save_data.timestamp,
        formatted_time: format_timestamp(save_data.timestamp),
        variable_count: save_data.variables.all().len(),
        visual: SaveDataVisualState {
            background: save_data.visual.background,
            character: save_data.visual.character,
            char_pos: Some(format!("{:?}", save_data.visual.char_pos)),
        },
    };

    SaveDataValidationResult {
        valid: error_count == 0,
        file_path,
        summary: Some(summary),
        issues,
        error_count,
        warning_count,
        info_count,
    }
}
