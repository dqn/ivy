use ivy::scenario::parse_scenario;

#[test]
fn test_parse_minimal_scenario() {
    let yaml = r#"
title: Minimal Test

script:
  - text: "Hello, world!"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    assert_eq!(scenario.title, "Minimal Test");
    assert_eq!(scenario.script.len(), 1);
    assert_eq!(scenario.script[0].text.as_ref().unwrap(), "Hello, world!");
}

#[test]
fn test_parse_multiple_commands() {
    let yaml = r#"
title: Multiple Commands

script:
  - text: "First"
  - text: "Second"
  - text: "Third"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    assert_eq!(scenario.script.len(), 3);
}

#[test]
fn test_parse_choices() {
    let yaml = r#"
title: Choices Test

script:
  - text: "What do you want?"
    choices:
      - label: "Option A"
        jump: label_a
      - label: "Option B"
        jump: label_b
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let choices = scenario.script[0].choices.as_ref().unwrap();

    assert_eq!(choices.len(), 2);
    assert_eq!(choices[0].label, "Option A");
    assert_eq!(choices[0].jump, "label_a");
    assert_eq!(choices[1].label, "Option B");
    assert_eq!(choices[1].jump, "label_b");
}

#[test]
fn test_parse_labels_and_jumps() {
    let yaml = r#"
title: Labels Test

script:
  - label: start
    text: "Start"
  - jump: ending
  - label: ending
    text: "End"
"#;

    let scenario = parse_scenario(yaml).unwrap();

    assert_eq!(scenario.script[0].label.as_ref().unwrap(), "start");
    assert_eq!(scenario.script[1].jump.as_ref().unwrap(), "ending");
    assert_eq!(scenario.script[2].label.as_ref().unwrap(), "ending");
}

#[test]
fn test_parse_background_and_character() {
    let yaml = r#"
title: Visual Test

script:
  - background: "bg.png"
    character: "char.png"
    char_pos: left
    text: "Hello"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let cmd = &scenario.script[0];

    assert_eq!(cmd.background.as_ref().unwrap(), "bg.png");
    assert_eq!(cmd.character.as_ref().unwrap(), "char.png");
}

#[test]
fn test_parse_set_variable() {
    let yaml = r#"
title: Variable Test

script:
  - set:
      name: player_name
      value: "Alice"
    text: "Hello"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let set = scenario.script[0].set.as_ref().unwrap();

    assert_eq!(set.name, "player_name");
}

#[test]
fn test_parse_if_condition() {
    let yaml = r#"
title: Condition Test

script:
  - if:
      var: has_key
      is: true
      jump: with_key
    text: "Checking..."
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let if_cond = scenario.script[0].if_cond.as_ref().unwrap();

    assert_eq!(if_cond.var, "has_key");
    assert_eq!(if_cond.jump, "with_key");
}

#[test]
fn test_parse_speaker() {
    let yaml = r#"
title: Speaker Test

script:
  - speaker: "Alice"
    text: "Hello!"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    assert_eq!(scenario.script[0].speaker.as_ref().unwrap(), "Alice");
}

#[test]
fn test_parse_audio() {
    let yaml = r#"
title: Audio Test

script:
  - bgm: "music.ogg"
    se: "sound.ogg"
    voice: "voice.ogg"
    text: "Playing audio"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let cmd = &scenario.script[0];

    assert_eq!(cmd.bgm.as_ref().unwrap(), "music.ogg");
    assert_eq!(cmd.se.as_ref().unwrap(), "sound.ogg");
    assert_eq!(cmd.voice.as_ref().unwrap(), "voice.ogg");
}

#[test]
fn test_parse_wait() {
    let yaml = r#"
title: Wait Test

script:
  - wait: 2.5
"#;

    let scenario = parse_scenario(yaml).unwrap();
    assert_eq!(scenario.script[0].wait, Some(2.5));
}

#[test]
fn test_parse_input() {
    let yaml = r#"
title: Input Test

script:
  - input:
      var: player_name
      prompt: "Enter your name"
      default: "Player"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let input = scenario.script[0].input.as_ref().unwrap();

    assert_eq!(input.var, "player_name");
    assert_eq!(input.prompt.as_ref().unwrap(), "Enter your name");
    assert_eq!(input.default.as_ref().unwrap(), "Player");
}

#[test]
fn test_parse_timeout_choices() {
    let yaml = r#"
title: Timeout Test

script:
  - text: "Quick!"
    timeout: 5.0
    choices:
      - label: "Yes"
        jump: yes
        default: true
      - label: "No"
        jump: no
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let cmd = &scenario.script[0];

    assert_eq!(cmd.timeout, Some(5.0));
    assert!(cmd.choices.as_ref().unwrap()[0].default);
}

#[test]
fn test_parse_chapters() {
    let yaml = r#"
title: Chapter Test

chapters:
  - id: ch1
    title: "Chapter 1"
    start_label: chapter1_start
  - id: ch2
    title: "Chapter 2"
    start_label: chapter2_start

script:
  - label: chapter1_start
    text: "Chapter 1 begins"
"#;

    let scenario = parse_scenario(yaml).unwrap();

    assert_eq!(scenario.chapters.len(), 2);
    assert_eq!(scenario.chapters[0].id, "ch1");
    assert_eq!(scenario.chapters[0].title, "Chapter 1");
    assert_eq!(scenario.chapters[1].start_label, "chapter2_start");
}

#[test]
fn test_parse_transition() {
    let yaml = r#"
title: Transition Test

script:
  - text: "Fading..."
    transition:
      type: fade
      duration: 1.0
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let transition = scenario.script[0].transition.as_ref().unwrap();

    assert_eq!(transition.duration, 1.0);
}

#[test]
fn test_parse_shake() {
    let yaml = r#"
title: Shake Test

script:
  - text: "Earthquake!"
    shake:
      type: both
      intensity: 15.0
      duration: 0.8
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let shake = scenario.script[0].shake.as_ref().unwrap();

    assert_eq!(shake.intensity, 15.0);
    assert_eq!(shake.duration, 0.8);
}

#[test]
fn test_parse_particles() {
    let yaml = r#"
title: Particles Test

script:
  - text: "It's snowing!"
    particles: "snow"
    particle_intensity: 0.8
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let cmd = &scenario.script[0];

    assert_eq!(cmd.particles.as_ref().unwrap(), "snow");
    assert_eq!(cmd.particle_intensity, 0.8);
}

#[test]
fn test_parse_cinematic() {
    let yaml = r#"
title: Cinematic Test

script:
  - text: "Dramatic scene"
    cinematic: true
    cinematic_duration: 0.8
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let cmd = &scenario.script[0];

    assert_eq!(cmd.cinematic, Some(true));
    assert_eq!(cmd.cinematic_duration, 0.8);
}

#[test]
fn test_parse_achievement() {
    let yaml = r#"
title: Achievement Test

script:
  - text: "You did it!"
    achievement:
      id: first_win
      name: "First Victory"
      description: "Win your first game"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let achievement = scenario.script[0].achievement.as_ref().unwrap();

    assert_eq!(achievement.id, "first_win");
    assert_eq!(achievement.name, "First Victory");
    assert_eq!(achievement.description, "Win your first game");
}

#[test]
fn test_parse_invalid_yaml_returns_error() {
    let yaml = "invalid: [unclosed";

    let result = parse_scenario(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_title_returns_error() {
    let yaml = r#"
script:
  - text: "Hello"
"#;

    let result = parse_scenario(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_script_returns_error() {
    let yaml = r#"
title: No Script
"#;

    let result = parse_scenario(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_script() {
    let yaml = r#"
title: Empty Script

script: []
"#;

    let scenario = parse_scenario(yaml).unwrap();
    assert!(scenario.script.is_empty());
}
