use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const PROJECT_FILE_NAME: &str = "ivy-project.yaml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioRef {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub resolution: Resolution,
    #[serde(default)]
    pub entry_scenario: String,
    #[serde(default)]
    pub scenarios: Vec<ScenarioRef>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub config: ProjectConfig,
    pub root_path: String,
}

#[allow(dead_code)] // Will be used in Phase 6: Project Management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentProject {
    pub path: String,
    pub name: String,
    pub last_opened: u64,
}

#[tauri::command]
pub fn create_project(
    root_path: &str,
    name: String,
    author: String,
    description: String,
    resolution: Resolution,
) -> Result<Project, String> {
    let root = Path::new(root_path);

    // Create project directory if it doesn't exist.
    if !root.exists() {
        fs::create_dir_all(root).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Create subdirectories.
    let scenarios_dir = root.join("scenarios");
    let assets_dir = root.join("assets");
    let backgrounds_dir = assets_dir.join("backgrounds");
    let characters_dir = assets_dir.join("characters");
    let audio_dir = assets_dir.join("audio");

    for dir in [
        &scenarios_dir,
        &assets_dir,
        &backgrounds_dir,
        &characters_dir,
        &audio_dir,
    ] {
        if !dir.exists() {
            fs::create_dir_all(dir).map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    // Create initial scenario file.
    let initial_scenario_path = scenarios_dir.join("main.ivy.yaml");
    let initial_scenario_content = format!(
        r#"title: "{}"

script:
  - text: "Welcome to your new visual novel!"
"#,
        name
    );
    fs::write(&initial_scenario_path, initial_scenario_content)
        .map_err(|e| format!("Failed to create scenario file: {}", e))?;

    // Create project config.
    let config = ProjectConfig {
        name: name.clone(),
        version: "1.0.0".to_string(),
        author,
        description,
        resolution,
        entry_scenario: "scenarios/main.ivy.yaml".to_string(),
        scenarios: vec![ScenarioRef {
            path: "scenarios/main.ivy.yaml".to_string(),
            chapter: Some("Main".to_string()),
        }],
    };

    // Save project file.
    let project_file = root.join(PROJECT_FILE_NAME);
    let yaml =
        serde_yaml::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&project_file, yaml).map_err(|e| format!("Failed to write project file: {}", e))?;

    Ok(Project {
        config,
        root_path: root_path.to_string(),
    })
}

#[tauri::command]
pub fn load_project(project_path: &str) -> Result<Project, String> {
    let path = Path::new(project_path);

    // Determine if path is a directory or file.
    let (root_path, config_path) = if path.is_dir() {
        (path.to_path_buf(), path.join(PROJECT_FILE_NAME))
    } else if path
        .file_name()
        .map(|n| n == PROJECT_FILE_NAME)
        .unwrap_or(false)
    {
        (
            path.parent().ok_or("Invalid project path")?.to_path_buf(),
            path.to_path_buf(),
        )
    } else {
        return Err("Invalid project path: expected directory or ivy-project.yaml".to_string());
    };

    if !config_path.exists() {
        return Err(format!("Project file not found: {}", config_path.display()));
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read project file: {}", e))?;
    let config: ProjectConfig = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse project file: {}", e))?;

    Ok(Project {
        config,
        root_path: root_path.to_string_lossy().to_string(),
    })
}

#[tauri::command]
pub fn save_project(root_path: &str, config: ProjectConfig) -> Result<(), String> {
    let project_file = Path::new(root_path).join(PROJECT_FILE_NAME);
    let yaml =
        serde_yaml::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&project_file, yaml).map_err(|e| format!("Failed to write project file: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn add_scenario_to_project(
    root_path: &str,
    scenario_path: &str,
    chapter_name: Option<String>,
) -> Result<ProjectConfig, String> {
    let project = load_project(root_path)?;
    let mut config = project.config;

    // Check if scenario already exists.
    if config.scenarios.iter().any(|s| s.path == scenario_path) {
        return Err("Scenario already exists in project".to_string());
    }

    config.scenarios.push(ScenarioRef {
        path: scenario_path.to_string(),
        chapter: chapter_name,
    });

    save_project(root_path, config.clone())?;
    Ok(config)
}

#[tauri::command]
pub fn remove_scenario_from_project(
    root_path: &str,
    scenario_path: &str,
) -> Result<ProjectConfig, String> {
    let project = load_project(root_path)?;
    let mut config = project.config;

    let original_len = config.scenarios.len();
    config.scenarios.retain(|s| s.path != scenario_path);

    if config.scenarios.len() == original_len {
        return Err("Scenario not found in project".to_string());
    }

    save_project(root_path, config.clone())?;
    Ok(config)
}

#[tauri::command]
pub fn is_project_directory(path: &str) -> bool {
    let project_file = Path::new(path).join(PROJECT_FILE_NAME);
    project_file.exists()
}
