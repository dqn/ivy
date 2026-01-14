use ivy::runtime::{DisplayState, GameState, Variables};
use ivy::scenario::parse_scenario;
use ivy::types::Value;

/// Test a complete game playthrough with multiple paths.
#[test]
fn test_complete_playthrough_path_a() {
    let yaml = r#"
title: Adventure Game

script:
  - text: "Welcome to the adventure!"

  - text: "You stand at a crossroads."
    choices:
      - label: "Go left"
        jump: left_path
      - label: "Go right"
        jump: right_path

  - label: left_path
    text: "You chose the left path."
    jump: ending

  - label: right_path
    text: "You chose the right path."
    jump: ending

  - label: ending
    text: "Your journey ends here."
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First text
    assert!(
        matches!(state.display_state(), DisplayState::Text { text, .. } if text == "Welcome to the adventure!")
    );
    state.advance();

    // Second text with choices
    assert!(matches!(
        state.display_state(),
        DisplayState::Choices { .. }
    ));

    // Select left path (index 0)
    state.select_choice(0);

    // Should be at left_path
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "You chose the left path.");
        }
        _ => panic!("Expected text after choice"),
    }

    state.advance();

    // Should be at ending
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "Your journey ends here.");
        }
        _ => panic!("Expected ending text"),
    }

    state.advance();
    assert!(state.is_ended());
}

/// Test the alternative path.
#[test]
fn test_complete_playthrough_path_b() {
    let yaml = r#"
title: Adventure Game

script:
  - text: "Welcome to the adventure!"

  - text: "You stand at a crossroads."
    choices:
      - label: "Go left"
        jump: left_path
      - label: "Go right"
        jump: right_path

  - label: left_path
    text: "You chose the left path."
    jump: ending

  - label: right_path
    text: "You chose the right path."
    jump: ending

  - label: ending
    text: "Your journey ends here."
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    state.advance(); // Skip welcome
    state.select_choice(1); // Select right path

    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "You chose the right path.");
        }
        _ => panic!("Expected right path text"),
    }
}

/// Test variable-based branching.
#[test]
fn test_variable_branching() {
    let yaml = r#"
title: Variable Test

script:
  - set:
      name: visited_cave
      value: false
    text: "You see a cave entrance."

  - text: "Enter the cave?"
    choices:
      - label: "Yes"
        jump: enter_cave
      - label: "No"
        jump: skip_cave

  - label: enter_cave
    set:
      name: visited_cave
      value: true
    text: "You found treasure in the cave!"
    jump: check_treasure

  - label: skip_cave
    text: "You walk past the cave."
    jump: check_treasure

  - label: check_treasure
    if:
      var: visited_cave
      is: true
      jump: rich_ending
    text: "You continue your journey empty-handed."
    jump: ending

  - label: rich_ending
    text: "You continue your journey with treasure!"
    jump: ending

  - label: ending
    text: "The end."
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First text
    let _ = state.display_state();
    state.advance();

    // Choice: enter cave
    state.select_choice(0);

    // Should show treasure message
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "You found treasure in the cave!");
        }
        _ => panic!("Expected treasure text"),
    }

    // Advance to check_treasure
    state.advance();

    // Condition should be true, jump to rich_ending
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "You continue your journey with treasure!");
        }
        _ => panic!("Expected rich ending"),
    }
}

/// Test rollback functionality during gameplay.
#[test]
fn test_rollback_during_gameplay() {
    let yaml = r#"
title: Rollback Test

script:
  - text: "Message 1"
  - text: "Message 2"
  - text: "Message 3"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Advance through all messages
    assert!(
        matches!(state.display_state(), DisplayState::Text { text, .. } if text == "Message 1")
    );
    state.advance();

    assert!(
        matches!(state.display_state(), DisplayState::Text { text, .. } if text == "Message 2")
    );
    state.advance();

    assert!(
        matches!(state.display_state(), DisplayState::Text { text, .. } if text == "Message 3")
    );

    // Rollback to message 2
    assert!(state.rollback());
    assert!(
        matches!(state.display_state(), DisplayState::Text { text, .. } if text == "Message 2")
    );

    // Rollback to message 1
    assert!(state.rollback());
    assert!(
        matches!(state.display_state(), DisplayState::Text { text, .. } if text == "Message 1")
    );

    // Can't rollback further
    assert!(!state.rollback());
}

/// Test wait command.
#[test]
fn test_wait_command() {
    let yaml = r#"
title: Wait Test

script:
  - text: "Before wait"
  - wait: 2.5
  - text: "After wait"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    state.advance(); // Skip first text

    match state.display_state() {
        DisplayState::Wait { duration, .. } => {
            assert_eq!(duration, 2.5);
        }
        _ => panic!("Expected wait state"),
    }

    state.advance(); // Skip wait

    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "After wait");
        }
        _ => panic!("Expected text after wait"),
    }
}

/// Test input command.
#[test]
fn test_input_command() {
    let yaml = r#"
title: Input Test

script:
  - input:
      var: player_name
      prompt: "What is your name?"
      default: "Hero"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Input { input, .. } => {
            assert_eq!(input.var, "player_name");
            assert_eq!(input.prompt.as_ref().unwrap(), "What is your name?");
            assert_eq!(input.default.as_ref().unwrap(), "Hero");
        }
        _ => panic!("Expected input state"),
    }
}

/// Test timed choices with default selection.
#[test]
fn test_timed_choices() {
    let yaml = r#"
title: Timed Choice Test

script:
  - text: "Quick! Choose!"
    timeout: 5.0
    choices:
      - label: "Run"
        jump: run
      - label: "Hide"
        jump: hide
        default: true

  - label: run
    text: "You ran!"

  - label: hide
    text: "You hid!"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Choices {
            timeout,
            default_choice,
            choices,
            ..
        } => {
            assert_eq!(timeout, Some(5.0));
            assert_eq!(default_choice, Some(1)); // "Hide" is default
            assert_eq!(choices.len(), 2);
        }
        _ => panic!("Expected choices with timeout"),
    }
}

/// Test visual state persistence across commands.
#[test]
fn test_visual_state_persistence() {
    let yaml = r#"
title: Visual Persistence

script:
  - background: "bg1.png"
    text: "First scene"

  - character: "char1.png"
    text: "Character appears"

  - text: "Both should persist"

  - character: ""
    text: "Character cleared, background persists"

  - background: ""
    text: "All cleared"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First: background set
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.background.as_ref().unwrap(), "bg1.png");
            assert!(visual.character.is_none());
        }
        _ => panic!("Expected text"),
    }
    state.advance();

    // Second: character added
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.background.as_ref().unwrap(), "bg1.png");
            assert_eq!(visual.character.as_ref().unwrap(), "char1.png");
        }
        _ => panic!("Expected text"),
    }
    state.advance();

    // Third: both persist
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.background.as_ref().unwrap(), "bg1.png");
            assert_eq!(visual.character.as_ref().unwrap(), "char1.png");
        }
        _ => panic!("Expected text"),
    }
    state.advance();

    // Fourth: character cleared
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.background.as_ref().unwrap(), "bg1.png");
            assert!(visual.character.is_none());
        }
        _ => panic!("Expected text"),
    }
    state.advance();

    // Fifth: all cleared
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert!(visual.background.is_none());
            assert!(visual.character.is_none());
        }
        _ => panic!("Expected text"),
    }
}

/// Test audio command retrieval.
#[test]
fn test_audio_commands() {
    let yaml = r#"
title: Audio Test

script:
  - bgm: "music.ogg"
    se: "click.ogg"
    voice: "hello.ogg"
    text: "Audio playing"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    assert_eq!(state.current_bgm().unwrap(), "music.ogg");
    assert_eq!(state.current_se().unwrap(), "click.ogg");
    assert_eq!(state.current_voice().unwrap(), "hello.ogg");
}

/// Test complex scenario with multiple features.
#[test]
fn test_complex_scenario() {
    let yaml = r#"
title: Complex Game

chapters:
  - id: prologue
    title: "Prologue"
    start_label: start
  - id: chapter1
    title: "Chapter 1"
    start_label: ch1

script:
  - label: start
    background: "intro.png"
    text: "The story begins..."

  - speaker: "Narrator"
    text: "You wake up in a strange place."

  - set:
      name: has_sword
      value: false
    text: "Your adventure begins."

  - text: "What do you do?"
    choices:
      - label: "Look around"
        jump: look
      - label: "Call for help"
        jump: call

  - label: look
    set:
      name: has_sword
      value: true
    text: "You found a sword!"
    jump: ch1

  - label: call
    text: "No one answers..."
    jump: ch1

  - label: ch1
    background: "forest.png"
    text: "Chapter 1 begins."

  - if:
      var: has_sword
      is: true
      jump: armed
    text: "You feel vulnerable."
    jump: end

  - label: armed
    text: "You feel ready for battle!"
    jump: end

  - label: end
    text: "To be continued..."
"#;
    let scenario = parse_scenario(yaml).unwrap();

    // Chapters should be parsed
    assert_eq!(scenario.chapters.len(), 2);
    assert_eq!(scenario.chapters[0].id, "prologue");

    let mut state = GameState::new(scenario);

    // Play through with "Look around" choice
    let _ = state.display_state(); // intro
    state.advance();
    let _ = state.display_state(); // narrator
    state.advance();
    let _ = state.display_state(); // stats set
    state.advance();

    // At choices
    assert!(matches!(
        state.display_state(),
        DisplayState::Choices { .. }
    ));

    // Choose "Look around"
    state.select_choice(0);

    // Should have sword message
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "You found a sword!");
        }
        _ => panic!("Expected sword text"),
    }

    state.advance(); // To ch1

    match state.display_state() {
        DisplayState::Text { text, visual, .. } => {
            assert_eq!(text, "Chapter 1 begins.");
            assert_eq!(visual.background.as_ref().unwrap(), "forest.png");
        }
        _ => panic!("Expected ch1 text"),
    }

    state.advance();

    // Condition check - should jump to armed
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "You feel ready for battle!");
        }
        _ => panic!("Expected armed text"),
    }
}

// Multiple characters tests

/// Test multiple characters display.
#[test]
fn test_multiple_characters_display() {
    let yaml = r#"
title: Multi Character Test

script:
  - characters:
      - image: "char_a.png"
        pos: left
      - image: "char_b.png"
        pos: center
      - image: "char_c.png"
        pos: right
    text: "Three characters appear"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.characters.len(), 3);
            assert_eq!(visual.characters[0].path, "char_a.png");
            assert_eq!(visual.characters[1].path, "char_b.png");
            assert_eq!(visual.characters[2].path, "char_c.png");
        }
        _ => panic!("Expected text state"),
    }
}

/// Test clearing multiple characters.
#[test]
fn test_multiple_characters_clear() {
    let yaml = r#"
title: Clear Characters Test

script:
  - characters:
      - image: "char_a.png"
        pos: left
      - image: "char_b.png"
        pos: right
    text: "Two characters"

  - characters: []
    text: "Characters cleared"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First: two characters
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.characters.len(), 2);
        }
        _ => panic!("Expected text"),
    }
    state.advance();

    // Second: cleared
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert!(visual.characters.is_empty());
        }
        _ => panic!("Expected text"),
    }
}

/// Test mixing single and multiple characters.
#[test]
fn test_mixed_single_and_multiple_characters() {
    let yaml = r#"
title: Mixed Characters Test

script:
  - character: "solo.png"
    char_pos: center
    text: "Single character"

  - characters:
      - image: "a.png"
        pos: left
      - image: "b.png"
        pos: right
    text: "Switch to multiple"

  - character: "solo2.png"
    text: "Back to single"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Single
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.character.as_ref().unwrap(), "solo.png");
            assert!(visual.characters.is_empty());
        }
        _ => panic!("Expected text"),
    }
    state.advance();

    // Multiple
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.characters.len(), 2);
        }
        _ => panic!("Expected text"),
    }
    state.advance();

    // Back to single
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.character.as_ref().unwrap(), "solo2.png");
        }
        _ => panic!("Expected text"),
    }
}

/// Test save/restore roundtrip with characters.
#[test]
fn test_save_restore_with_characters() {
    use ivy::runtime::{CharacterState, SaveData, VisualState};
    use ivy::scenario::CharPosition;

    let yaml = r#"
title: Save Restore Test

script:
  - text: "First"
  - text: "Second"
  - text: "Third"
"#;
    let scenario = parse_scenario(yaml).unwrap();

    let visual = VisualState {
        background: Some("bg.png".to_string()),
        character: None,
        char_pos: CharPosition::Center,
        characters: vec![
            CharacterState {
                path: "a.png".to_string(),
                position: CharPosition::Left,
            },
            CharacterState {
                path: "b.png".to_string(),
                position: CharPosition::Right,
            },
        ],
    };

    let save = SaveData {
        scenario_path: "test.yaml".to_string(),
        current_index: 1,
        visual,
        timestamp: 0,
        variables: Variables::new(),
    };

    let scenario2 = parse_scenario(yaml).unwrap();
    let mut state = GameState::from_save_data(&save, scenario2);

    match state.display_state() {
        DisplayState::Text { text, visual, .. } => {
            assert_eq!(text, "Second");
            assert_eq!(visual.background, Some("bg.png".to_string()));
            assert_eq!(visual.characters.len(), 2);
            assert_eq!(visual.characters[0].path, "a.png");
        }
        _ => panic!("Expected text"),
    }
}

/// Test input command sets variable.
#[test]
fn test_input_command_sets_variable() {
    let yaml = r#"
title: Input Variable Test

script:
  - input:
      var: player_name
      prompt: "Enter name"
      default: "Player"
  - text: "Hello!"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Get input state
    assert!(matches!(state.display_state(), DisplayState::Input { .. }));

    // Set the variable directly (simulating user input)
    state.set_variable("player_name", Value::String("TestPlayer".to_string()));
    state.advance();

    // Variable should be set
    assert_eq!(
        state.variables().get("player_name"),
        Some(&Value::String("TestPlayer".to_string()))
    );
}

/// Test effects in display state.
#[test]
fn test_effects_in_display_state() {
    let yaml = r#"
title: Effects Test

script:
  - background: "bg.png"
    transition:
      type: fade
      duration: 1.0
    shake:
      type: both
      intensity: 10.0
      duration: 0.5
    particles: rain
    particle_intensity: 0.7
    text: "Dramatic scene"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    // Check transition
    let transition = state.current_transition();
    assert!(transition.is_some());
    assert_eq!(transition.unwrap().duration, 1.0);

    // Check shake
    let shake = state.current_shake();
    assert!(shake.is_some());
    assert_eq!(shake.unwrap().intensity, 10.0);

    // Check particles
    if let Some((particle_type, intensity)) = state.current_particles() {
        assert_eq!(particle_type, "rain");
        assert!((intensity - 0.7).abs() < 0.01);
    } else {
        panic!("Expected particles to be set");
    }
}

/// Test timed choice default selection path.
#[test]
fn test_timed_choice_default_path() {
    let yaml = r#"
title: Timed Default Test

script:
  - text: "Choose fast!"
    timeout: 3.0
    choices:
      - label: "Option A"
        jump: a
      - label: "Option B"
        jump: b
        default: true

  - label: a
    text: "You chose A"

  - label: b
    text: "Default B selected"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Choices {
            default_choice,
            timeout,
            ..
        } => {
            assert_eq!(default_choice, Some(1)); // "Option B"
            assert_eq!(timeout, Some(3.0));

            // Simulate timeout - select default
            state.select_choice(1);
        }
        _ => panic!("Expected choices"),
    }

    // Should be at "Default B selected"
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "Default B selected");
        }
        _ => panic!("Expected text"),
    }
}

/// Test chapter definitions are parsed.
#[test]
fn test_chapter_definitions_parsed() {
    let yaml = r#"
title: Chapter Test

chapters:
  - id: prologue
    title: "Prologue"
    start_label: prologue_start
    description: "The beginning"
  - id: chapter1
    title: "Chapter 1"
    start_label: ch1_start
  - id: chapter2
    title: "Chapter 2"
    start_label: ch2_start

script:
  - label: prologue_start
    text: "Prologue begins"
  - label: ch1_start
    text: "Chapter 1 begins"
  - label: ch2_start
    text: "Chapter 2 begins"
"#;
    let scenario = parse_scenario(yaml).unwrap();

    assert_eq!(scenario.chapters.len(), 3);

    assert_eq!(scenario.chapters[0].id, "prologue");
    assert_eq!(scenario.chapters[0].title, "Prologue");
    assert_eq!(scenario.chapters[0].start_label, "prologue_start");
    assert_eq!(scenario.chapters[0].description, "The beginning");

    assert_eq!(scenario.chapters[1].id, "chapter1");
    assert!(scenario.chapters[1].description.is_empty()); // default
}

/// Test achievement command in scenario.
#[test]
fn test_achievement_in_scenario() {
    let yaml = r#"
title: Achievement Test

script:
  - achievement:
      id: first_achievement
      name: "First Steps"
      description: "Begin your journey"
    text: "Achievement unlocked!"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    let ach = state.current_achievement();
    assert!(ach.is_some());

    let a = ach.unwrap();
    assert_eq!(a.id, "first_achievement");
    assert_eq!(a.name, "First Steps");
    assert_eq!(a.description, "Begin your journey");
}
