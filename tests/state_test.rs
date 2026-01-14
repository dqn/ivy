use ivy::runtime::{DisplayState, GameState, Value};
use ivy::scenario::parse_scenario;

fn create_minimal_state() -> GameState {
    let yaml = r#"
title: Test

script:
  - text: "First"
  - text: "Second"
  - text: "Third"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    GameState::new(scenario)
}

#[test]
fn test_new_state_starts_at_first_command() {
    let state = create_minimal_state();
    assert_eq!(state.current_index(), 0);
}

#[test]
fn test_display_state_returns_text() {
    let mut state = create_minimal_state();
    let display = state.display_state();

    match display {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "First");
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_advance_moves_to_next_command() {
    let mut state = create_minimal_state();
    state.advance();

    assert_eq!(state.current_index(), 1);
}

#[test]
fn test_advance_updates_display_state() {
    let mut state = create_minimal_state();
    state.advance();

    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "Second");
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_advance_to_end() {
    let mut state = create_minimal_state();
    state.advance();
    state.advance();
    state.advance();

    assert!(matches!(state.display_state(), DisplayState::End));
    assert!(state.is_ended());
}

#[test]
fn test_advance_past_end_does_nothing() {
    let mut state = create_minimal_state();
    state.advance();
    state.advance();
    state.advance();
    state.advance(); // Extra advance

    assert!(state.is_ended());
}

#[test]
fn test_can_rollback_after_advance() {
    let mut state = create_minimal_state();
    assert!(!state.can_rollback());

    state.advance();
    assert!(state.can_rollback());
}

#[test]
fn test_rollback_returns_to_previous_state() {
    let mut state = create_minimal_state();
    state.advance();
    state.advance();

    assert_eq!(state.current_index(), 2);

    let result = state.rollback();
    assert!(result);
    assert_eq!(state.current_index(), 1);
}

#[test]
fn test_rollback_when_empty_returns_false() {
    let mut state = create_minimal_state();
    let result = state.rollback();

    assert!(!result);
    assert_eq!(state.current_index(), 0);
}

#[test]
fn test_history_grows_with_advances() {
    let mut state = create_minimal_state();
    assert!(state.history().is_empty());

    state.advance();
    assert_eq!(state.history().len(), 1);

    state.advance();
    assert_eq!(state.history().len(), 2);
}

#[test]
fn test_select_choice_jumps_to_label() {
    let yaml = r#"
title: Choices Test

script:
  - text: "Choose"
    choices:
      - label: "A"
        jump: label_a
      - label: "B"
        jump: label_b

  - label: label_a
    text: "You chose A"

  - label: label_b
    text: "You chose B"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Verify we start at choices
    assert!(matches!(state.display_state(), DisplayState::Choices { .. }));

    // Select second choice (B)
    state.select_choice(1);

    // Should now be at label_b
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "You chose B");
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_display_state_choices() {
    let yaml = r#"
title: Choices Test

script:
  - text: "What do you want?"
    choices:
      - label: "Option A"
        jump: a
      - label: "Option B"
        jump: b
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Choices { text, choices, .. } => {
            assert_eq!(text, "What do you want?");
            assert_eq!(choices.len(), 2);
            assert_eq!(choices[0].label, "Option A");
            assert_eq!(choices[1].label, "Option B");
        }
        _ => panic!("Expected DisplayState::Choices"),
    }
}

#[test]
fn test_unconditional_jump() {
    let yaml = r#"
title: Jump Test

script:
  - text: "Start"
  - jump: ending
  - text: "This is skipped"
  - label: ending
    text: "End"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First command
    assert!(matches!(state.display_state(), DisplayState::Text { text, .. } if text == "Start"));

    // Advance should follow the jump
    state.advance();

    // Should skip "This is skipped" and go to "End"
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "End");
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_conditional_jump_when_true() {
    let yaml = r#"
title: Condition Test

script:
  - set:
      name: flag
      value: true
    text: "Setting flag"

  - if:
      var: flag
      is: true
      jump: success
    text: "This is skipped when flag is true"

  - label: success
    text: "Success!"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First command sets flag and displays text
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "Setting flag");
        }
        _ => panic!("Expected DisplayState::Text"),
    }

    // Advance - should evaluate condition and jump
    state.advance();

    // Should be at "Success!" due to conditional jump
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "Success!");
        }
        _ => panic!("Expected DisplayState::Text at success"),
    }
}

#[test]
fn test_conditional_jump_when_false() {
    let yaml = r#"
title: Condition Test

script:
  - set:
      name: flag
      value: false
    text: "Setting flag to false"

  - if:
      var: flag
      is: true
      jump: success
    text: "Flag is false"

  - label: success
    text: "Success!"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First command
    state.advance();

    // Condition is false, so we show "Flag is false"
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "Flag is false");
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_set_variable_during_advance() {
    let yaml = r#"
title: Variable Test

script:
  - set:
      name: player_name
      value: "Alice"
    text: "Welcome"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Trigger display_state to process set command
    let _ = state.display_state();

    // Variable should be set
    let value = state.variables().get("player_name");
    assert!(value.is_some());
}

#[test]
fn test_set_variable_via_api() {
    let mut state = create_minimal_state();

    state.set_variable("custom_var", Value::Int(42));

    let value = state.variables().get("custom_var");
    assert_eq!(value.unwrap().as_int(), Some(42));
}

#[test]
fn test_display_state_wait() {
    let yaml = r#"
title: Wait Test

script:
  - wait: 2.5
  - text: "After wait"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Wait { duration, .. } => {
            assert_eq!(duration, 2.5);
        }
        _ => panic!("Expected DisplayState::Wait"),
    }
}

#[test]
fn test_display_state_input() {
    let yaml = r#"
title: Input Test

script:
  - input:
      var: player_name
      prompt: "Enter your name"
      default: "Player"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Input { input, .. } => {
            assert_eq!(input.var, "player_name");
            assert_eq!(input.prompt.as_ref().unwrap(), "Enter your name");
        }
        _ => panic!("Expected DisplayState::Input"),
    }
}

#[test]
fn test_visual_state_background() {
    let yaml = r#"
title: Visual Test

script:
  - background: "bg.png"
    text: "With background"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.background.as_ref().unwrap(), "bg.png");
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_visual_state_persists() {
    let yaml = r#"
title: Visual Persistence Test

script:
  - background: "bg.png"
    text: "First"
  - text: "Second (background should persist)"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First command sets background
    let _ = state.display_state();
    state.advance();

    // Second command should still have the background
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.background.as_ref().unwrap(), "bg.png");
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_visual_state_clear() {
    let yaml = r#"
title: Visual Clear Test

script:
  - background: "bg.png"
    text: "First"
  - background: ""
    text: "Background cleared"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First command sets background
    let _ = state.display_state();
    state.advance();

    // Second command clears background
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert!(visual.background.is_none());
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_speaker_in_display_state() {
    let yaml = r#"
title: Speaker Test

script:
  - speaker: "Alice"
    text: "Hello!"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Text { speaker, .. } => {
            assert_eq!(speaker.as_ref().unwrap(), "Alice");
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_jump_to_label() {
    let yaml = r#"
title: Jump Label Test

script:
  - text: "First"
  - label: target
    text: "Target"
  - text: "After target"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    state.jump_to_label("target");

    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "Target");
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_jump_to_nonexistent_label_goes_to_end() {
    let mut state = create_minimal_state();
    state.jump_to_label("nonexistent");

    assert!(state.is_ended());
}

#[test]
fn test_multiple_rollbacks() {
    let mut state = create_minimal_state();

    state.advance();
    state.advance();

    assert_eq!(state.current_index(), 2);

    state.rollback();
    assert_eq!(state.current_index(), 1);

    state.rollback();
    assert_eq!(state.current_index(), 0);

    // Can't rollback anymore
    assert!(!state.rollback());
}

#[test]
fn test_current_bgm() {
    let yaml = r#"
title: BGM Test

script:
  - bgm: "music.ogg"
    text: "Playing music"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    assert_eq!(state.current_bgm().unwrap(), "music.ogg");
}

#[test]
fn test_current_se() {
    let yaml = r#"
title: SE Test

script:
  - se: "sound.ogg"
    text: "Playing sound"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    assert_eq!(state.current_se().unwrap(), "sound.ogg");
}

#[test]
fn test_current_voice() {
    let yaml = r#"
title: Voice Test

script:
  - voice: "voice.ogg"
    text: "Speaking"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    assert_eq!(state.current_voice().unwrap(), "voice.ogg");
}

#[test]
fn test_empty_scenario() {
    let yaml = r#"
title: Empty

script: []
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    assert!(matches!(state.display_state(), DisplayState::End));
    assert!(state.is_ended());
}

#[test]
fn test_skip_label_only_commands() {
    let yaml = r#"
title: Label Only Test

script:
  - label: start
  - label: middle
  - text: "Finally some text"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Should skip label-only commands and land on text
    match state.display_state() {
        DisplayState::Text { text, .. } => {
            assert_eq!(text, "Finally some text");
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_multiple_characters() {
    let yaml = r#"
title: Multiple Characters Test

script:
  - characters:
      - image: "alice.png"
        pos: left
      - image: "bob.png"
        pos: right
    text: "Two characters"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.characters.len(), 2);
            assert_eq!(visual.characters[0].path, "alice.png");
            assert_eq!(visual.characters[1].path, "bob.png");
            // Single character should be cleared when using multiple
            assert!(visual.character.is_none());
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_single_character_clears_multiple() {
    let yaml = r#"
title: Single Clears Multiple Test

script:
  - characters:
      - image: "alice.png"
        pos: left
      - image: "bob.png"
        pos: right
    text: "Two characters"
  - character: "charlie.png"
    char_pos: center
    text: "Single character"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First command has multiple characters
    let _ = state.display_state();
    state.advance();

    // Second command has single character, multiple should be cleared
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert!(visual.characters.is_empty());
            assert_eq!(visual.character.as_ref().unwrap(), "charlie.png");
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_character_positions() {
    use ivy::scenario::CharPosition;

    let yaml = r#"
title: Position Test

script:
  - character: "char.png"
    char_pos: left
    text: "Left"
  - char_pos: center
    text: "Center"
  - char_pos: right
    text: "Right"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Left position
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert!(matches!(visual.char_pos, CharPosition::Left));
        }
        _ => panic!("Expected DisplayState::Text"),
    }

    state.advance();

    // Center position (character persists)
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert!(matches!(visual.char_pos, CharPosition::Center));
            assert_eq!(visual.character.as_ref().unwrap(), "char.png");
        }
        _ => panic!("Expected DisplayState::Text"),
    }

    state.advance();

    // Right position
    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert!(matches!(visual.char_pos, CharPosition::Right));
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_three_characters() {
    let yaml = r#"
title: Three Characters Test

script:
  - characters:
      - image: "alice.png"
        pos: left
      - image: "bob.png"
        pos: center
      - image: "charlie.png"
        pos: right
    text: "Three characters"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Text { visual, .. } => {
            assert_eq!(visual.characters.len(), 3);
        }
        _ => panic!("Expected DisplayState::Text"),
    }
}

#[test]
fn test_default_choice_second() {
    let yaml = r#"
title: Default Choice Test

script:
  - text: "Choose"
    timeout: 5.0
    choices:
      - label: "Option A"
        jump: a
      - label: "Option B"
        jump: b
        default: true
      - label: "Option C"
        jump: c
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    match state.display_state() {
        DisplayState::Choices { default_choice, .. } => {
            assert_eq!(default_choice, Some(1)); // Second choice (index 1) is default
        }
        _ => panic!("Expected DisplayState::Choices"),
    }
}

#[test]
fn test_transition_command() {
    let yaml = r#"
title: Transition Test

script:
  - transition:
      type: fade
      duration: 0.5
    text: "Fading"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    let transition = state.current_transition();
    assert!(transition.is_some());
    let t = transition.unwrap();
    assert_eq!(t.duration, 0.5);
}

#[test]
fn test_shake_command() {
    let yaml = r#"
title: Shake Test

script:
  - shake:
      type: horizontal
      intensity: 10.0
      duration: 0.3
    text: "Shaking"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    let shake = state.current_shake();
    assert!(shake.is_some());
    let s = shake.unwrap();
    assert_eq!(s.intensity, 10.0);
    assert_eq!(s.duration, 0.3);
}

#[test]
fn test_particles_command() {
    let yaml = r#"
title: Particles Test

script:
  - particles: "snow"
    particle_intensity: 0.7
    text: "Snowing"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    let particles = state.current_particles();
    assert!(particles.is_some());
    let (particle_type, intensity) = particles.unwrap();
    assert_eq!(particle_type, "snow");
    assert_eq!(intensity, 0.7);
}

#[test]
fn test_cinematic_command() {
    let yaml = r#"
title: Cinematic Test

script:
  - cinematic: true
    cinematic_duration: 0.5
    text: "Dramatic"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    let cinematic = state.current_cinematic();
    assert!(cinematic.is_some());
    let (enabled, duration) = cinematic.unwrap();
    assert!(enabled);
    assert_eq!(duration, 0.5);
}

#[test]
fn test_char_enter_animation() {
    use ivy::scenario::CharAnimationType;

    let yaml = r#"
title: Char Enter Test

script:
  - character: "char.png"
    char_enter:
      type: slide_left
      duration: 0.3
    text: "Entering"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    let char_enter = state.current_char_enter();
    assert!(char_enter.is_some());
    let anim = char_enter.unwrap();
    assert!(matches!(anim.animation_type, CharAnimationType::SlideLeft));
    assert_eq!(anim.duration, 0.3);
}

#[test]
fn test_char_exit_animation() {
    use ivy::scenario::CharAnimationType;

    let yaml = r#"
title: Char Exit Test

script:
  - character: ""
    char_exit:
      type: fade
      duration: 0.4
    text: "Exiting"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    let char_exit = state.current_char_exit();
    assert!(char_exit.is_some());
    let anim = char_exit.unwrap();
    assert!(matches!(anim.animation_type, CharAnimationType::Fade));
    assert_eq!(anim.duration, 0.4);
}

#[test]
fn test_achievement_command() {
    let yaml = r#"
title: Achievement Test

script:
  - achievement:
      id: first_win
      name: "First Victory"
      description: "Win your first battle"
    text: "Achievement unlocked!"
"#;
    let scenario = parse_scenario(yaml).unwrap();
    let state = GameState::new(scenario);

    let achievement = state.current_achievement();
    assert!(achievement.is_some());
    let a = achievement.unwrap();
    assert_eq!(a.id, "first_win");
    assert_eq!(a.name, "First Victory");
    assert_eq!(a.description, "Win your first battle");
}
