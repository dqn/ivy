use ivy::scenario::types::{LayerDef, ModularCharDef, Scenario};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Character definition with aliases for speaker name mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDef {
    /// Base image path (body silhouette).
    pub base: String,
    /// Speaker name aliases for this character.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Ordered list of layers (rendered from first to last).
    #[serde(default)]
    pub layers: Vec<LayerDef>,
}

impl From<ModularCharDef> for CharacterDef {
    fn from(def: ModularCharDef) -> Self {
        Self {
            base: def.base,
            aliases: Vec::new(),
            layers: def.layers,
        }
    }
}

impl From<CharacterDef> for ModularCharDef {
    fn from(def: CharacterDef) -> Self {
        Self {
            base: def.base,
            layers: def.layers,
        }
    }
}

/// Character database stored in characters.yaml.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterDatabase {
    #[serde(default)]
    pub characters: HashMap<String, CharacterDef>,
}

/// Load characters from project's characters.yaml file.
#[tauri::command]
pub fn load_characters(project_path: &str) -> Result<CharacterDatabase, String> {
    let path = Path::new(project_path).join("characters.yaml");

    if !path.exists() {
        return Ok(CharacterDatabase::default());
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read characters.yaml: {}", e))?;

    serde_yaml::from_str(&content).map_err(|e| format!("Failed to parse characters.yaml: {}", e))
}

/// Save characters to project's characters.yaml file.
#[tauri::command]
pub fn save_characters(project_path: &str, database: CharacterDatabase) -> Result<(), String> {
    let path = Path::new(project_path).join("characters.yaml");

    let content = serde_yaml::to_string(&database)
        .map_err(|e| format!("Failed to serialize characters: {}", e))?;

    std::fs::write(&path, content).map_err(|e| format!("Failed to write characters.yaml: {}", e))
}

/// Add a new character to the database.
#[tauri::command]
pub fn add_character(
    database: CharacterDatabase,
    name: String,
    definition: CharacterDef,
) -> Result<CharacterDatabase, String> {
    if database.characters.contains_key(&name) {
        return Err(format!("Character '{}' already exists", name));
    }

    let mut new_db = database;
    new_db.characters.insert(name, definition);
    Ok(new_db)
}

/// Update an existing character in the database.
#[tauri::command]
pub fn update_character(
    database: CharacterDatabase,
    name: String,
    definition: CharacterDef,
) -> Result<CharacterDatabase, String> {
    let mut new_db = database;
    new_db.characters.insert(name, definition);
    Ok(new_db)
}

/// Remove a character from the database.
#[tauri::command]
pub fn remove_character(
    database: CharacterDatabase,
    name: String,
) -> Result<CharacterDatabase, String> {
    let mut new_db = database;
    new_db.characters.remove(&name);
    Ok(new_db)
}

/// Extract all unique speaker names from a scenario.
#[tauri::command]
pub fn extract_speakers(scenario: Scenario) -> Vec<String> {
    let mut speakers: Vec<String> = scenario
        .script
        .iter()
        .filter_map(|cmd| cmd.speaker.as_ref())
        .map(|s| match s {
            ivy::i18n::LocalizedString::Plain(text) => text.clone(),
            ivy::i18n::LocalizedString::Localized(map) => map
                .get("en")
                .or_else(|| map.values().next())
                .cloned()
                .unwrap_or_default(),
            ivy::i18n::LocalizedString::Key(key) => key.clone(),
        })
        .collect();

    speakers.sort();
    speakers.dedup();
    speakers
}

/// Find command indices where a character is used.
#[tauri::command]
pub fn find_character_usages(scenario: Scenario, character_name: String) -> Vec<usize> {
    scenario
        .script
        .iter()
        .enumerate()
        .filter_map(|(i, cmd)| {
            if let Some(ref modular_char) = cmd.modular_char {
                if modular_char.name == character_name {
                    return Some(i);
                }
            }
            None
        })
        .collect()
}

/// Get merged character definitions from project database and scenario.
/// Project-level definitions take precedence over scenario-level ones.
#[tauri::command]
pub fn get_merged_characters(
    database: CharacterDatabase,
    scenario: Scenario,
) -> HashMap<String, CharacterDef> {
    let mut merged = HashMap::new();

    // First, add scenario-level modular_characters
    for (name, def) in scenario.modular_characters {
        merged.insert(name, CharacterDef::from(def));
    }

    // Then, override with project-level characters (higher priority)
    for (name, def) in database.characters {
        merged.insert(name, def);
    }

    merged
}

/// Find character by speaker alias.
#[tauri::command]
pub fn find_character_by_speaker(database: CharacterDatabase, speaker: String) -> Option<String> {
    for (name, def) in &database.characters {
        if def.aliases.iter().any(|alias| alias == &speaker) {
            return Some(name.clone());
        }
    }
    None
}
