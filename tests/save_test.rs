mod common;

use ivy::runtime::{SaveData, Value, Variables, VisualState};
use ivy::scenario::CharPosition;

#[test]
fn test_save_data_serialization_roundtrip() {
    let save = SaveData {
        scenario_path: "test.yaml".to_string(),
        current_index: 5,
        visual: VisualState::default(),
        timestamp: 1234567890,
        variables: Variables::new(),
    };

    let json = serde_json::to_string(&save).unwrap();
    let restored: SaveData = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.scenario_path, "test.yaml");
    assert_eq!(restored.current_index, 5);
    assert_eq!(restored.timestamp, 1234567890);
}

#[test]
fn test_save_data_with_variables() {
    let mut variables = Variables::new();
    variables.set("name", Value::String("Alice".to_string()));
    variables.set("score", Value::Int(100));
    variables.set("has_key", Value::Bool(true));

    let save = SaveData {
        scenario_path: "test.yaml".to_string(),
        current_index: 0,
        visual: VisualState::default(),
        timestamp: 0,
        variables,
    };

    let json = serde_json::to_string(&save).unwrap();
    let restored: SaveData = serde_json::from_str(&json).unwrap();

    assert_eq!(
        restored.variables.get("name"),
        Some(&Value::String("Alice".to_string()))
    );
    assert_eq!(restored.variables.get("score"), Some(&Value::Int(100)));
    assert_eq!(restored.variables.get("has_key"), Some(&Value::Bool(true)));
}

#[test]
fn test_save_data_with_visual_state() {
    let visual = VisualState {
        background: Some("bg.png".to_string()),
        character: Some("char.png".to_string()),
        char_pos: CharPosition::Center,
        characters: vec![],
        nvl_mode: false,
        modular_char: None,
    };

    let save = SaveData {
        scenario_path: "test.yaml".to_string(),
        current_index: 0,
        visual,
        timestamp: 0,
        variables: Variables::new(),
    };

    let json = serde_json::to_string(&save).unwrap();
    let restored: SaveData = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.visual.background, Some("bg.png".to_string()));
    assert_eq!(restored.visual.character, Some("char.png".to_string()));
    assert!(matches!(restored.visual.char_pos, CharPosition::Center));
}

#[test]
fn test_save_data_with_characters() {
    use ivy::runtime::CharacterState;

    let visual = VisualState {
        background: None,
        character: None,
        char_pos: CharPosition::Center,
        characters: vec![
            CharacterState {
                path: "char_a.png".to_string(),
                position: CharPosition::Left,
                enter: None,
                exit: None,
                idle: None,
            },
            CharacterState {
                path: "char_b.png".to_string(),
                position: CharPosition::Right,
                enter: None,
                exit: None,
                idle: None,
            },
        ],
        nvl_mode: false,
        modular_char: None,
    };

    let save = SaveData {
        scenario_path: "test.yaml".to_string(),
        current_index: 0,
        visual,
        timestamp: 0,
        variables: Variables::new(),
    };

    let json = serde_json::to_string(&save).unwrap();
    let restored: SaveData = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.visual.characters.len(), 2);
    assert_eq!(restored.visual.characters[0].path, "char_a.png");
    assert!(matches!(
        restored.visual.characters[0].position,
        CharPosition::Left
    ));
    assert_eq!(restored.visual.characters[1].path, "char_b.png");
    assert!(matches!(
        restored.visual.characters[1].position,
        CharPosition::Right
    ));
}

#[test]
fn test_save_data_timestamp() {
    let save = SaveData {
        scenario_path: "test.yaml".to_string(),
        current_index: 0,
        visual: VisualState::default(),
        timestamp: 1700000000,
        variables: Variables::new(),
    };

    let json = serde_json::to_string(&save).unwrap();
    let restored: SaveData = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.timestamp, 1700000000);
}

#[test]
fn test_slot_path_generation() {
    let path = SaveData::slot_path(1);
    assert_eq!(path, "saves/slot_1.json");
}

#[test]
fn test_slot_path_for_all_slots() {
    for slot in 1..=10 {
        let path = SaveData::slot_path(slot);
        assert_eq!(path, format!("saves/slot_{}.json", slot));
    }
}

#[test]
fn test_deserialize_minimal() {
    let json = r#"{
        "scenario_path": "minimal.yaml",
        "current_index": 0,
        "visual": {}
    }"#;

    let save: SaveData = serde_json::from_str(json).unwrap();

    assert_eq!(save.scenario_path, "minimal.yaml");
    assert_eq!(save.current_index, 0);
    assert_eq!(save.timestamp, 0); // default
    assert!(save.variables.all().is_empty()); // default
}

#[test]
fn test_deserialize_with_defaults() {
    let json = r#"{
        "scenario_path": "test.yaml",
        "current_index": 10,
        "visual": {
            "background": "bg.png"
        }
    }"#;

    let save: SaveData = serde_json::from_str(json).unwrap();

    assert_eq!(save.scenario_path, "test.yaml");
    assert_eq!(save.current_index, 10);
    assert_eq!(save.visual.background, Some("bg.png".to_string()));
    assert!(save.visual.character.is_none());
    assert!(save.visual.characters.is_empty());
    assert_eq!(save.timestamp, 0);
}

#[test]
fn test_save_data_from_game_state() {
    let yaml = r#"
title: Test

script:
  - text: "First"
  - text: "Second"
"#;

    let mut state = common::create_game_state(yaml);
    state.advance();

    let save = state.to_save_data("test.yaml");

    assert_eq!(save.scenario_path, "test.yaml");
    assert_eq!(save.current_index, 1);
    assert!(save.timestamp > 0);
}

#[test]
fn test_restore_game_state_from_save_data() {
    use ivy::runtime::{DisplayState, GameState};
    use ivy::scenario::parse_scenario;

    let yaml = r#"
title: Test

script:
  - text: "First"
  - text: "Second"
  - text: "Third"
"#;

    let save = SaveData {
        scenario_path: "test.yaml".to_string(),
        current_index: 2,
        visual: VisualState::default(),
        timestamp: 0,
        variables: Variables::new(),
    };

    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::from_save_data(&save, scenario);

    if let DisplayState::Text { text, .. } = state.display_state() {
        assert_eq!(text, "Third");
    } else {
        panic!("Expected Text display state");
    }
}

#[test]
fn test_save_data_preserves_index() {
    let save = SaveData {
        scenario_path: "test.yaml".to_string(),
        current_index: 42,
        visual: VisualState::default(),
        timestamp: 0,
        variables: Variables::new(),
    };

    let json = serde_json::to_string(&save).unwrap();
    let restored: SaveData = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.current_index, 42);
}
