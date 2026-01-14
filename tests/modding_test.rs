use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use ivy::modding::{ModInfo, ModLoader, ModType};

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn create_test_mod_dir() -> PathBuf {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let test_dir = PathBuf::from(format!("tests/fixtures/mods_test_{}", id));
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).ok();
    }
    fs::create_dir_all(&test_dir).unwrap();
    test_dir
}

fn cleanup_test_dir(path: &PathBuf) {
    if path.exists() {
        fs::remove_dir_all(path).ok();
    }
}

#[test]
fn test_mod_loader_empty_dir() {
    let test_dir = create_test_mod_dir();

    let mut loader = ModLoader::new();
    loader.discover(&test_dir).unwrap();

    assert_eq!(loader.count(), 0);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_mod_loader_nonexistent_dir() {
    let mut loader = ModLoader::new();
    let result = loader.discover(&PathBuf::from("nonexistent_mods_dir"));

    assert!(result.is_ok());
    assert_eq!(loader.count(), 0);
}

#[test]
fn test_mod_loader_single_mod() {
    let test_dir = create_test_mod_dir();
    let mod_dir = test_dir.join("test_mod");
    fs::create_dir_all(&mod_dir).unwrap();

    let mod_yaml = r#"
name: "Test Mod"
version: "1.0.0"
author: "Tester"
description: "A test mod for testing"
type: scenario
files:
  - scenario/test.yaml
"#;
    fs::write(mod_dir.join("mod.yaml"), mod_yaml).unwrap();

    let mut loader = ModLoader::new();
    loader.discover(&test_dir).unwrap();

    assert_eq!(loader.count(), 1);
    assert_eq!(loader.enabled_count(), 1);

    let mod_info = loader.get_mod("test_mod").unwrap();
    assert_eq!(mod_info.name, "Test Mod");
    assert_eq!(mod_info.version, "1.0.0");
    assert_eq!(mod_info.author, "Tester");
    assert_eq!(mod_info.mod_type, ModType::Scenario);
    assert!(mod_info.enabled);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_mod_loader_multiple_mods() {
    let test_dir = create_test_mod_dir();

    // Create first mod
    let mod1_dir = test_dir.join("mod_a");
    fs::create_dir_all(&mod1_dir).unwrap();
    fs::write(
        mod1_dir.join("mod.yaml"),
        r#"
name: "Mod A"
priority: 10
"#,
    )
    .unwrap();

    // Create second mod
    let mod2_dir = test_dir.join("mod_b");
    fs::create_dir_all(&mod2_dir).unwrap();
    fs::write(
        mod2_dir.join("mod.yaml"),
        r#"
name: "Mod B"
priority: 5
"#,
    )
    .unwrap();

    let mut loader = ModLoader::new();
    loader.discover(&test_dir).unwrap();

    assert_eq!(loader.count(), 2);

    // Check load order (sorted by priority)
    let mods: Vec<_> = loader.load_order().collect();
    assert_eq!(mods[0].name, "Mod B"); // priority 5
    assert_eq!(mods[1].name, "Mod A"); // priority 10

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_mod_loader_disable_mod() {
    let test_dir = create_test_mod_dir();
    let mod_dir = test_dir.join("my_mod");
    fs::create_dir_all(&mod_dir).unwrap();
    fs::write(mod_dir.join("mod.yaml"), "name: \"My Mod\"").unwrap();

    let mut loader = ModLoader::new();
    loader.discover(&test_dir).unwrap();

    assert_eq!(loader.enabled_count(), 1);

    loader.set_enabled("my_mod", false);
    assert_eq!(loader.enabled_count(), 0);

    loader.set_enabled("my_mod", true);
    assert_eq!(loader.enabled_count(), 1);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_mod_loader_mods_of_type() {
    let test_dir = create_test_mod_dir();

    // Create scenario mod
    let scenario_mod = test_dir.join("scenario_mod");
    fs::create_dir_all(&scenario_mod).unwrap();
    fs::write(
        scenario_mod.join("mod.yaml"),
        r#"
name: "Scenario Mod"
type: scenario
"#,
    )
    .unwrap();

    // Create assets mod
    let assets_mod = test_dir.join("assets_mod");
    fs::create_dir_all(&assets_mod).unwrap();
    fs::write(
        assets_mod.join("mod.yaml"),
        r#"
name: "Assets Mod"
type: assets
"#,
    )
    .unwrap();

    // Create translation mod
    let translation_mod = test_dir.join("translation_mod");
    fs::create_dir_all(&translation_mod).unwrap();
    fs::write(
        translation_mod.join("mod.yaml"),
        r#"
name: "Translation Mod"
type: translation
"#,
    )
    .unwrap();

    let mut loader = ModLoader::new();
    loader.discover(&test_dir).unwrap();

    let scenarios: Vec<_> = loader.mods_of_type(ModType::Scenario).collect();
    assert_eq!(scenarios.len(), 1);
    assert_eq!(scenarios[0].name, "Scenario Mod");

    let assets: Vec<_> = loader.mods_of_type(ModType::Assets).collect();
    assert_eq!(assets.len(), 1);
    assert_eq!(assets[0].name, "Assets Mod");

    let translations: Vec<_> = loader.mods_of_type(ModType::Translation).collect();
    assert_eq!(translations.len(), 1);
    assert_eq!(translations[0].name, "Translation Mod");

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_mod_loader_collect_scenarios() {
    let test_dir = create_test_mod_dir();
    let mod_dir = test_dir.join("story_mod");
    let scenario_dir = mod_dir.join("scenario");
    fs::create_dir_all(&scenario_dir).unwrap();

    fs::write(
        mod_dir.join("mod.yaml"),
        r#"
name: "Story Mod"
type: scenario
files:
  - scenario/chapter1.yaml
"#,
    )
    .unwrap();

    // Create scenario files
    fs::write(scenario_dir.join("chapter1.yaml"), "title: Chapter 1\nscript: []").unwrap();
    fs::write(scenario_dir.join("chapter2.yaml"), "title: Chapter 2\nscript: []").unwrap();

    let mut loader = ModLoader::new();
    loader.discover(&test_dir).unwrap();

    let scenarios = loader.collect_scenarios();

    // Should find both files (one from files list, one from directory scan)
    assert!(scenarios.len() >= 1);
    assert!(scenarios
        .iter()
        .any(|p| p.to_string_lossy().contains("chapter1.yaml")));

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_mod_loader_resolve_path() {
    let test_dir = create_test_mod_dir();
    let mod_dir = test_dir.join("path_mod");
    let assets_dir = mod_dir.join("assets");
    fs::create_dir_all(&assets_dir).unwrap();

    fs::write(mod_dir.join("mod.yaml"), "name: \"Path Mod\"").unwrap();
    fs::write(assets_dir.join("image.png"), "fake image data").unwrap();

    let mut loader = ModLoader::new();
    loader.discover(&test_dir).unwrap();

    // Should resolve existing file
    let resolved = loader.resolve_path("path_mod", "assets/image.png");
    assert!(resolved.is_some());
    assert!(resolved.unwrap().exists());

    // Should return None for non-existent file
    let not_found = loader.resolve_path("path_mod", "assets/missing.png");
    assert!(not_found.is_none());

    // Should return None for non-existent mod
    let no_mod = loader.resolve_path("nonexistent_mod", "file.txt");
    assert!(no_mod.is_none());

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_mod_info_parse_minimal() {
    let yaml = r#"name: "Minimal""#;
    let info: ModInfo = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(info.name, "Minimal");
    assert_eq!(info.version, "1.0.0");
    assert_eq!(info.mod_type, ModType::Scenario);
    assert!(info.enabled);
    assert_eq!(info.priority, 0);
    assert!(info.files.is_empty());
}

#[test]
fn test_mod_info_parse_full() {
    let yaml = r#"
name: "Full Mod"
version: "2.0.0"
author: "Developer"
description: "A fully configured mod"
type: patch
priority: 100
enabled: false
requires:
  ivy: ">=1.0.0"
  base_game: ">=1.5.0"
files:
  - patches/fix.yaml
  - patches/balance.yaml
"#;
    let info: ModInfo = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(info.name, "Full Mod");
    assert_eq!(info.version, "2.0.0");
    assert_eq!(info.author, "Developer");
    assert_eq!(info.description, "A fully configured mod");
    assert_eq!(info.mod_type, ModType::Patch);
    assert_eq!(info.priority, 100);
    assert!(!info.enabled);
    assert!(info.requires.is_some());
    let requires = info.requires.unwrap();
    assert_eq!(requires.ivy, Some(">=1.0.0".to_string()));
    assert_eq!(requires.base_game, Some(">=1.5.0".to_string()));
    assert_eq!(info.files.len(), 2);
}

#[test]
fn test_mod_type_variants() {
    let types = [
        ("scenario", ModType::Scenario),
        ("characters", ModType::Characters),
        ("translation", ModType::Translation),
        ("assets", ModType::Assets),
        ("patch", ModType::Patch),
    ];

    for (yaml, expected) in types {
        let parsed: ModType = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(parsed, expected);
    }
}
