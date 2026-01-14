use insta::assert_yaml_snapshot;
use ivy::runtime::GameState;
use ivy::scenario::parse_scenario;

/// Test DisplayState snapshot for minimal scenario.
#[test]
fn test_display_state_minimal() {
    let yaml = include_str!("fixtures/minimal.yaml");
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    let display = state.display_state();
    assert_yaml_snapshot!("display_minimal", display);
}

/// Test DisplayState snapshot for visual elements.
#[test]
fn test_display_state_visual() {
    let yaml = include_str!("fixtures/visual.yaml");
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First command with background
    let display1 = state.display_state();
    assert_yaml_snapshot!("display_visual_1", display1);

    // Advance and check character
    state.advance();
    let display2 = state.display_state();
    assert_yaml_snapshot!("display_visual_2", display2);

    // Advance and check position change
    state.advance();
    let display3 = state.display_state();
    assert_yaml_snapshot!("display_visual_3", display3);
}

/// Test DisplayState snapshot for choices.
#[test]
fn test_display_state_choices() {
    let yaml = include_str!("fixtures/choices.yaml");
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Advance to choices
    state.advance();
    let display = state.display_state();
    assert_yaml_snapshot!("display_choices", display);
}

/// Test DisplayState snapshot for text formatting.
#[test]
fn test_display_state_text_formatting() {
    let yaml = include_str!("fixtures/text_formatting.yaml");
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Basic colored text
    let display1 = state.display_state();
    assert_yaml_snapshot!("display_text_color", display1);

    // Advance to nested colors
    state.advance();
    let display2 = state.display_state();
    assert_yaml_snapshot!("display_text_nested_color", display2);

    // Advance to ruby
    state.advance();
    let display3 = state.display_state();
    assert_yaml_snapshot!("display_text_ruby", display3);
}

/// Test DisplayState snapshot for variables.
#[test]
fn test_display_state_variables() {
    let yaml = include_str!("fixtures/variables.yaml");
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    let display = state.display_state();
    assert_yaml_snapshot!("display_variables", display);
}

/// Test DisplayState snapshot for timed choices.
#[test]
fn test_display_state_timed_choices() {
    let yaml = include_str!("fixtures/timed_choices.yaml");
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // First regular choice
    let display1 = state.display_state();
    assert_yaml_snapshot!("display_timed_regular", display1);

    // Select option A and advance to timed section
    state.select_choice(0);
    state.advance();

    let display2 = state.display_state();
    assert_yaml_snapshot!("display_timed_with_timeout", display2);
}

/// Test DisplayState snapshot for multiple characters.
#[test]
fn test_display_state_multiple_chars() {
    let yaml = include_str!("fixtures/multiple_chars.yaml");
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Single character center
    let display1 = state.display_state();
    assert_yaml_snapshot!("display_char_single_center", display1);

    // Advance through positions
    state.advance();
    let display2 = state.display_state();
    assert_yaml_snapshot!("display_char_left", display2);

    state.advance();
    let display3 = state.display_state();
    assert_yaml_snapshot!("display_char_right", display3);

    // Advance to different character
    state.advance();
    let display4 = state.display_state();
    assert_yaml_snapshot!("display_char_bob", display4);

    // Advance to multiple characters
    state.advance();
    let display5 = state.display_state();
    assert_yaml_snapshot!("display_chars_multiple", display5);

    // Three characters
    state.advance();
    let display6 = state.display_state();
    assert_yaml_snapshot!("display_chars_three", display6);
}

/// Test DisplayState snapshot for effects.
#[test]
fn test_display_state_effects() {
    let yaml = include_str!("fixtures/effects.yaml");
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Fade transition
    let display1 = state.display_state();
    assert_yaml_snapshot!("display_effect_fade", display1);

    // Advance through effects
    state.advance();
    state.advance();
    state.advance();

    // Horizontal shake
    let display2 = state.display_state();
    assert_yaml_snapshot!("display_effect_shake", display2);
}

/// Test DisplayState snapshot for audio commands.
#[test]
fn test_display_state_audio() {
    let yaml = include_str!("fixtures/audio.yaml");
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // BGM start
    let bgm1 = state.current_bgm();
    assert_yaml_snapshot!("audio_bgm_start", bgm1);

    // Advance and check BGM continues
    state.advance();
    let bgm2 = state.current_bgm();
    assert_yaml_snapshot!("audio_bgm_continue", bgm2);

    // Advance to SE
    state.advance();
    let se = state.current_se();
    assert_yaml_snapshot!("audio_se", se);

    // Advance to voice
    state.advance();
    let voice = state.current_voice();
    assert_yaml_snapshot!("audio_voice", voice);
}

/// Test DisplayState snapshot for full scenario.
#[test]
fn test_display_state_full_scenario() {
    let yaml = include_str!("fixtures/full_scenario.yaml");
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Opening with transition
    let display1 = state.display_state();
    assert_yaml_snapshot!("full_opening", display1);

    // Advance to input command
    state.advance();
    let display2 = state.display_state();
    assert_yaml_snapshot!("full_input", display2);
}

/// Test state after choice selection.
#[test]
fn test_state_after_choice() {
    let yaml = include_str!("fixtures/choices.yaml");
    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Advance to choices
    state.advance();

    // Select first choice
    state.select_choice(0);
    let display = state.display_state();
    assert_yaml_snapshot!("state_after_choice_0", display);
}

/// Test visual state persistence.
#[test]
fn test_visual_state_persistence() {
    let yaml = r#"
title: Persistence Test

script:
  - background: "assets/bg.png"
    character: "assets/char.png"
    char_pos: center
    text: "Initial state"

  - text: "No visual changes (should persist)"

  - character: ""
    text: "Character cleared, background persists"

  - background: ""
    text: "All cleared"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Initial state
    let display1 = state.display_state();
    assert_yaml_snapshot!("persist_initial", display1);

    // No changes - should persist
    state.advance();
    let display2 = state.display_state();
    assert_yaml_snapshot!("persist_unchanged", display2);

    // Character cleared
    state.advance();
    let display3 = state.display_state();
    assert_yaml_snapshot!("persist_char_cleared", display3);

    // All cleared
    state.advance();
    let display4 = state.display_state();
    assert_yaml_snapshot!("persist_all_cleared", display4);
}

/// Test rollback state.
#[test]
fn test_rollback_state() {
    let yaml = r#"
title: Rollback Test

script:
  - background: "assets/bg1.png"
    text: "State 1"

  - background: "assets/bg2.png"
    text: "State 2"

  - background: "assets/bg3.png"
    text: "State 3"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let mut state = GameState::new(scenario);

    // Advance to state 2
    state.advance();
    let display1 = state.display_state();
    assert_yaml_snapshot!("rollback_state2", display1);

    // Advance to state 3
    state.advance();
    let display2 = state.display_state();
    assert_yaml_snapshot!("rollback_state3", display2);

    // Rollback to state 2
    state.rollback();
    let display3 = state.display_state();
    assert_yaml_snapshot!("rollback_back_to_state2", display3);

    // Rollback to state 1
    state.rollback();
    let display4 = state.display_state();
    assert_yaml_snapshot!("rollback_back_to_state1", display4);
}
