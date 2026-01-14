//! Mod types and loader implementation.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Type of mod content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModType {
    /// New scenario or route.
    Scenario,
    /// Character sprites and definitions.
    Characters,
    /// Translation/localization.
    Translation,
    /// Asset replacements (backgrounds, music, UI).
    Assets,
    /// Bug fixes or balance changes.
    Patch,
}

/// Version requirement for dependencies.
#[derive(Debug, Clone, Deserialize)]
pub struct VersionReq {
    /// Minimum ivy version required.
    #[serde(default)]
    pub ivy: Option<String>,
    /// Minimum base game version required.
    #[serde(default)]
    pub base_game: Option<String>,
}

/// Mod metadata loaded from mod.yaml.
#[derive(Debug, Clone, Deserialize)]
pub struct ModInfo {
    /// Display name of the mod.
    pub name: String,
    /// Mod version.
    #[serde(default = "default_version")]
    pub version: String,
    /// Mod author.
    #[serde(default)]
    pub author: String,
    /// Mod description.
    #[serde(default)]
    pub description: String,
    /// Type of mod content.
    #[serde(rename = "type", default = "default_mod_type")]
    pub mod_type: ModType,
    /// Version requirements.
    #[serde(default)]
    pub requires: Option<VersionReq>,
    /// List of files included in this mod.
    #[serde(default)]
    pub files: Vec<String>,
    /// Load priority (higher = loaded later, can override earlier mods).
    #[serde(default)]
    pub priority: i32,
    /// Whether this mod is enabled.
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Directory path where mod was loaded from (set at runtime).
    #[serde(skip)]
    pub path: PathBuf,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

fn default_mod_type() -> ModType {
    ModType::Scenario
}

fn default_enabled() -> bool {
    true
}

/// Error type for mod loading operations.
#[derive(Debug)]
pub enum ModLoadError {
    /// Failed to read mod directory.
    DirectoryRead(std::io::Error),
    /// Failed to read mod.yaml file.
    MetadataRead(std::io::Error),
    /// Failed to parse mod.yaml.
    MetadataParse(serde_yaml::Error),
    /// Mod file not found.
    FileNotFound(PathBuf),
    /// Version requirement not met.
    VersionMismatch { mod_name: String, requirement: String },
}

impl std::fmt::Display for ModLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModLoadError::DirectoryRead(e) => write!(f, "Failed to read mod directory: {}", e),
            ModLoadError::MetadataRead(e) => write!(f, "Failed to read mod.yaml: {}", e),
            ModLoadError::MetadataParse(e) => write!(f, "Failed to parse mod.yaml: {}", e),
            ModLoadError::FileNotFound(path) => write!(f, "Mod file not found: {}", path.display()),
            ModLoadError::VersionMismatch { mod_name, requirement } => {
                write!(f, "Mod '{}' requires version {}", mod_name, requirement)
            }
        }
    }
}

impl std::error::Error for ModLoadError {}

/// Mod loader for discovering and loading mods.
#[derive(Debug, Default)]
pub struct ModLoader {
    /// Loaded mods indexed by directory name.
    mods: HashMap<String, ModInfo>,
    /// Mods sorted by load order (priority).
    load_order: Vec<String>,
}

impl ModLoader {
    /// Create a new mod loader.
    pub fn new() -> Self {
        Self::default()
    }

    /// Discover and load all mods from the specified directory.
    pub fn discover(&mut self, mods_dir: &Path) -> Result<(), ModLoadError> {
        if !mods_dir.exists() {
            return Ok(());
        }

        let entries = std::fs::read_dir(mods_dir).map_err(ModLoadError::DirectoryRead)?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Err(e) = self.load_mod(&path) {
                    eprintln!("Warning: Failed to load mod at {}: {}", path.display(), e);
                }
            }
        }

        self.sort_by_priority();
        Ok(())
    }

    /// Load a single mod from a directory.
    pub fn load_mod(&mut self, mod_dir: &Path) -> Result<(), ModLoadError> {
        let metadata_path = mod_dir.join("mod.yaml");
        if !metadata_path.exists() {
            return Err(ModLoadError::FileNotFound(metadata_path));
        }

        let content = std::fs::read_to_string(&metadata_path).map_err(ModLoadError::MetadataRead)?;

        let mut info: ModInfo =
            serde_yaml::from_str(&content).map_err(ModLoadError::MetadataParse)?;

        info.path = mod_dir.to_path_buf();

        let mod_id = mod_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        self.mods.insert(mod_id.clone(), info);
        self.load_order.push(mod_id);

        Ok(())
    }

    /// Sort mods by priority (lower priority first).
    fn sort_by_priority(&mut self) {
        self.load_order.sort_by(|a, b| {
            let priority_a = self.mods.get(a).map(|m| m.priority).unwrap_or(0);
            let priority_b = self.mods.get(b).map(|m| m.priority).unwrap_or(0);
            priority_a.cmp(&priority_b)
        });
    }

    /// Get all loaded mods.
    pub fn mods(&self) -> &HashMap<String, ModInfo> {
        &self.mods
    }

    /// Get mods in load order.
    pub fn load_order(&self) -> impl Iterator<Item = &ModInfo> {
        self.load_order
            .iter()
            .filter_map(|id| self.mods.get(id))
            .filter(|m| m.enabled)
    }

    /// Get enabled mods of a specific type.
    pub fn mods_of_type(&self, mod_type: ModType) -> impl Iterator<Item = &ModInfo> {
        self.load_order().filter(move |m| m.mod_type == mod_type)
    }

    /// Get a mod by its directory name.
    pub fn get_mod(&self, id: &str) -> Option<&ModInfo> {
        self.mods.get(id)
    }

    /// Enable or disable a mod.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(info) = self.mods.get_mut(id) {
            info.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// Get the full path to a file within a mod.
    pub fn resolve_path(&self, mod_id: &str, relative_path: &str) -> Option<PathBuf> {
        self.mods
            .get(mod_id)
            .map(|m| m.path.join(relative_path))
            .filter(|p| p.exists())
    }

    /// Collect all scenario files from enabled scenario mods.
    pub fn collect_scenarios(&self) -> Vec<PathBuf> {
        let mut scenarios = Vec::new();

        for mod_info in self.mods_of_type(ModType::Scenario) {
            for file in &mod_info.files {
                if file.ends_with(".yaml") || file.ends_with(".yml") {
                    let path = mod_info.path.join(file);
                    if path.exists() {
                        scenarios.push(path);
                    }
                }
            }

            // Also check scenario/ subdirectory
            let scenario_dir = mod_info.path.join("scenario");
            if scenario_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&scenario_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if let Some(ext) = path.extension() {
                            if ext == "yaml" || ext == "yml" {
                                scenarios.push(path);
                            }
                        }
                    }
                }
            }
        }

        scenarios
    }

    /// Collect all asset directories from enabled asset mods.
    pub fn collect_asset_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        for mod_info in self.mods_of_type(ModType::Assets) {
            let assets_dir = mod_info.path.join("assets");
            if assets_dir.exists() {
                dirs.push(assets_dir);
            }
        }

        dirs
    }

    /// Get total number of loaded mods.
    pub fn count(&self) -> usize {
        self.mods.len()
    }

    /// Get number of enabled mods.
    pub fn enabled_count(&self) -> usize {
        self.mods.values().filter(|m| m.enabled).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_info_deserialize() {
        let yaml = r#"
name: "Test Mod"
version: "1.0.0"
author: "Tester"
description: "A test mod"
type: scenario
files:
  - scenario/test.yaml
"#;
        let info: ModInfo = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(info.name, "Test Mod");
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.mod_type, ModType::Scenario);
        assert!(info.enabled);
    }

    #[test]
    fn test_mod_info_defaults() {
        let yaml = r#"
name: "Minimal Mod"
"#;
        let info: ModInfo = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(info.name, "Minimal Mod");
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.mod_type, ModType::Scenario);
        assert!(info.enabled);
        assert_eq!(info.priority, 0);
    }

    #[test]
    fn test_mod_type_deserialize() {
        let yaml = "scenario";
        let t: ModType = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(t, ModType::Scenario);

        let yaml = "translation";
        let t: ModType = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(t, ModType::Translation);
    }

    #[test]
    fn test_mod_loader_new() {
        let loader = ModLoader::new();
        assert_eq!(loader.count(), 0);
        assert_eq!(loader.enabled_count(), 0);
    }
}
