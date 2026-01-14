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

#[test]
fn test_parse_char_idle() {
    use ivy::scenario::{CharIdleType, Easing};

    let yaml = r#"
title: Char Idle Parse Test

script:
  - character: "char.png"
    char_idle:
      type: "sway"
      duration: 3.0
      intensity: 0.5
      easing: ease_in_out
    text: "Swaying"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let idle = scenario.script[0].char_idle.as_ref().unwrap();

    assert!(matches!(idle.idle_type, CharIdleType::Sway));
    assert_eq!(idle.duration, 3.0);
    assert_eq!(idle.intensity, 0.5);
    assert!(matches!(idle.easing, Easing::EaseInOut));
}

#[test]
fn test_parse_char_idle_all_types() {
    use ivy::scenario::CharIdleType;

    let yaml = r#"
title: All Idle Types

script:
  - character: "char.png"
    char_idle:
      type: "breath"
    text: "Breath"

  - char_idle:
      type: "bob"
    text: "Bob"

  - char_idle:
      type: "sway"
    text: "Sway"

  - char_idle:
      type: "pulse"
    text: "Pulse"
"#;

    let scenario = parse_scenario(yaml).unwrap();

    assert!(matches!(
        scenario.script[0].char_idle.as_ref().unwrap().idle_type,
        CharIdleType::Breath
    ));
    assert!(matches!(
        scenario.script[1].char_idle.as_ref().unwrap().idle_type,
        CharIdleType::Bob
    ));
    assert!(matches!(
        scenario.script[2].char_idle.as_ref().unwrap().idle_type,
        CharIdleType::Sway
    ));
    assert!(matches!(
        scenario.script[3].char_idle.as_ref().unwrap().idle_type,
        CharIdleType::Pulse
    ));
}

#[test]
fn test_parse_nvl_mode() {
    let yaml = r#"
title: NVL Mode Test

script:
  - text: "ADV mode text"

  - nvl: true
    text: "NVL mode text"

  - nvl_clear: true
    text: "New page in NVL"

  - nvl: false
    text: "Back to ADV"
"#;

    let scenario = parse_scenario(yaml).unwrap();

    // First command has no NVL setting
    assert!(scenario.script[0].nvl.is_none());
    assert!(!scenario.script[0].nvl_clear);

    // Second command switches to NVL mode
    assert_eq!(scenario.script[1].nvl, Some(true));
    assert!(!scenario.script[1].nvl_clear);

    // Third command clears NVL buffer
    assert!(scenario.script[2].nvl.is_none());
    assert!(scenario.script[2].nvl_clear);

    // Fourth command switches back to ADV
    assert_eq!(scenario.script[3].nvl, Some(false));
    assert!(!scenario.script[3].nvl_clear);
}

#[test]
fn test_parse_modular_characters() {
    let yaml = r#"
title: Modular Character Test

modular_characters:
  sakura:
    base: "assets/sakura/base.png"
    layers:
      - name: "hair"
        images:
          - "assets/sakura/hair_a.png"
          - "assets/sakura/hair_b.png"
      - name: "expression"
        images:
          - "assets/sakura/expr_neutral.png"
          - "assets/sakura/expr_smile.png"

script:
  - text: "Hello"
    modular_char:
      name: sakura
      expression: 1
"#;

    let scenario = parse_scenario(yaml).unwrap();

    // Check modular character definition
    assert!(scenario.modular_characters.contains_key("sakura"));
    let sakura = &scenario.modular_characters["sakura"];
    assert_eq!(sakura.base, "assets/sakura/base.png");
    assert_eq!(sakura.layers.len(), 2);
    assert_eq!(sakura.layers[0].name, "hair");
    assert_eq!(sakura.layers[0].images.len(), 2);
    assert_eq!(sakura.layers[1].name, "expression");
    assert_eq!(sakura.layers[1].images.len(), 2);

    // Check command reference
    let modular_ref = scenario.script[0].modular_char.as_ref().unwrap();
    assert_eq!(modular_ref.name, "sakura");
    assert_eq!(modular_ref.variants.get("expression"), Some(&1));
}

#[test]
fn test_parse_camera_command() {
    use ivy::scenario::{CameraFocus, Easing};

    let yaml = r#"
title: Camera Test

script:
  - camera:
      zoom: 1.5
      duration: 1.0
    text: "Zooming in..."

  - camera:
      pan:
        x: 100
        y: -50
      zoom: 1.2
      tilt: 5
      focus: top_left
      duration: 2.0
      easing: ease_in_out_cubic
    text: "Combined effects"
"#;

    let scenario = parse_scenario(yaml).unwrap();

    // First camera command
    let camera1 = scenario.script[0].camera.as_ref().unwrap();
    assert!(camera1.pan.is_none());
    assert_eq!(camera1.zoom, Some(1.5));
    assert!(camera1.tilt.is_none());
    assert_eq!(camera1.duration, 1.0);

    // Second camera command with all options
    let camera2 = scenario.script[1].camera.as_ref().unwrap();
    let pan = camera2.pan.as_ref().unwrap();
    assert_eq!(pan.x, 100.0);
    assert_eq!(pan.y, -50.0);
    assert_eq!(camera2.zoom, Some(1.2));
    assert_eq!(camera2.tilt, Some(5.0));
    assert!(matches!(camera2.focus, CameraFocus::TopLeft));
    assert_eq!(camera2.duration, 2.0);
    assert!(matches!(camera2.easing, Easing::EaseInOutCubic));
}

#[test]
fn test_parse_ambient_audio() {
    let yaml = r#"
title: Ambient Test

script:
  - ambient:
      - id: rain
        path: "assets/audio/rain.ogg"
        volume: 0.6
        looped: true
        fade_in: 0.5
    text: "Rain starts..."

  - ambient:
      - id: wind
        path: "assets/audio/wind.ogg"
    text: "Wind joins in..."

  - ambient_stop:
      - id: rain
        fade_out: 1.0
    text: "Rain stops."
"#;

    let scenario = parse_scenario(yaml).unwrap();

    // First command with ambient track
    let ambient1 = &scenario.script[0].ambient;
    assert_eq!(ambient1.len(), 1);
    assert_eq!(ambient1[0].id, "rain");
    assert_eq!(ambient1[0].path, "assets/audio/rain.ogg");
    assert_eq!(ambient1[0].volume, 0.6);
    assert!(ambient1[0].looped);
    assert_eq!(ambient1[0].fade_in, 0.5);

    // Second command with default values
    let ambient2 = &scenario.script[1].ambient;
    assert_eq!(ambient2.len(), 1);
    assert_eq!(ambient2[0].id, "wind");
    assert_eq!(ambient2[0].volume, 0.5); // default
    assert!(ambient2[0].looped); // default true

    // Third command with ambient stop
    let ambient_stop = &scenario.script[2].ambient_stop;
    assert_eq!(ambient_stop.len(), 1);
    assert_eq!(ambient_stop[0].id, "rain");
    assert_eq!(ambient_stop[0].fade_out, 1.0);
}

#[test]
fn test_parse_video_background() {
    let yaml = r#"
title: Video Background Test

script:
  - video_bg:
      path: "assets/videos/forest.webm"
      looped: true
    text: "Video background plays..."

  - video_bg:
      path: "assets/videos/sunset.webm"
      looped: false
      on_end: ending
    text: "Non-looped video"

  - video_bg:
      path: ""
    text: "Video stopped"
"#;

    let scenario = parse_scenario(yaml).unwrap();

    // First command with looped video background
    let video_bg1 = scenario.script[0].video_bg.as_ref().unwrap();
    assert_eq!(video_bg1.path, "assets/videos/forest.webm");
    assert!(video_bg1.looped);
    assert!(video_bg1.on_end.is_none());

    // Second command with non-looped video and on_end jump
    let video_bg2 = scenario.script[1].video_bg.as_ref().unwrap();
    assert_eq!(video_bg2.path, "assets/videos/sunset.webm");
    assert!(!video_bg2.looped);
    assert_eq!(video_bg2.on_end.as_ref().unwrap(), "ending");

    // Third command stops video (empty path)
    let video_bg3 = scenario.script[2].video_bg.as_ref().unwrap();
    assert!(video_bg3.path.is_empty());
}
