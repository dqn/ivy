use ivy::scenario::{parse_scenario, validate_scenario, detect_circular_paths, Severity};

#[test]
fn test_validate_valid_scenario() {
    let yaml = r#"
title: Valid Scenario

script:
  - label: start
    text: "Hello"
  - text: "World"
    choices:
      - label: "Option A"
        jump: ending
  - label: ending
    text: "The End"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    assert!(result.is_valid());
    assert!(!result.has_warnings());
}

#[test]
fn test_validate_undefined_label_in_jump() {
    let yaml = r#"
title: Undefined Label

script:
  - text: "Hello"
    jump: nonexistent
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    assert!(!result.is_valid());
    assert_eq!(result.errors().count(), 1);
    let error = result.errors().next().unwrap();
    assert!(error.message.contains("nonexistent"));
}

#[test]
fn test_validate_undefined_label_in_choice() {
    let yaml = r#"
title: Undefined Choice Label

script:
  - text: "Choose"
    choices:
      - label: "Go"
        jump: nowhere
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    assert!(!result.is_valid());
    assert_eq!(result.errors().count(), 1);
}

#[test]
fn test_validate_undefined_label_in_condition() {
    let yaml = r#"
title: Undefined Conditional Label

script:
  - if:
      var: flag
      is: true
      jump: missing
    text: "Test"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    assert!(!result.is_valid());
    assert_eq!(result.errors().count(), 1);
}

#[test]
fn test_validate_unused_label_warning() {
    let yaml = r#"
title: Unused Label

script:
  - label: start
    text: "Hello"
  - label: unused_section
    text: "Never reached"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    assert!(result.is_valid());
    assert!(result.has_warnings());
    assert_eq!(result.warnings().count(), 1);
    let warning = result.warnings().next().unwrap();
    assert!(warning.message.contains("unused_section"));
}

#[test]
fn test_validate_start_label_not_warning() {
    let yaml = r#"
title: Start Label

script:
  - label: start
    text: "Hello"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    assert!(result.is_valid());
    assert!(!result.has_warnings());
}

#[test]
fn test_validate_duplicate_label() {
    let yaml = r#"
title: Duplicate Label

script:
  - label: scene1
    text: "First"
  - label: scene1
    text: "Duplicate"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    assert!(!result.is_valid());
    assert_eq!(result.errors().count(), 1);
    let error = result.errors().next().unwrap();
    assert!(error.message.contains("Duplicate"));
}

#[test]
fn test_validate_self_referencing_jump() {
    let yaml = r#"
title: Self Reference

script:
  - label: infinite
    text: "This loops forever"
    jump: infinite
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    assert!(!result.is_valid());
    assert_eq!(result.errors().count(), 1);
    let error = result.errors().next().unwrap();
    assert!(error.message.contains("Self-referencing"));
}

#[test]
fn test_validate_empty_scenario_warning() {
    let yaml = r#"
title: Empty

script: []
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    assert!(result.is_valid());
    assert!(result.has_warnings());
}

#[test]
fn test_validate_chapter_undefined_start_label() {
    let yaml = r#"
title: Bad Chapter

chapters:
  - id: ch1
    title: "Chapter 1"
    start_label: nonexistent

script:
  - label: start
    text: "Hello"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    assert!(!result.is_valid());
    let error = result.errors().next().unwrap();
    assert!(error.message.contains("Chapter"));
}

#[test]
fn test_validate_chapter_valid_start_label() {
    let yaml = r#"
title: Good Chapter

chapters:
  - id: ch1
    title: "Chapter 1"
    start_label: chapter1

script:
  - label: start
    text: "Intro"
  - label: chapter1
    text: "Chapter 1 content"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    // chapter1 is referenced by chapter definition, so no unused warning
    assert!(result.is_valid());
}

#[test]
fn test_detect_circular_paths_simple() {
    let yaml = r#"
title: Circular

script:
  - label: a
    text: "A"
    jump: b
  - label: b
    text: "B"
    jump: c
  - label: c
    text: "C"
    jump: a
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let cycles = detect_circular_paths(&scenario);

    assert!(!cycles.is_empty());
    let cycle = &cycles[0];
    assert!(cycle.contains(&"a".to_string()));
    assert!(cycle.contains(&"b".to_string()));
    assert!(cycle.contains(&"c".to_string()));
}

#[test]
fn test_detect_circular_paths_none() {
    let yaml = r#"
title: Linear

script:
  - label: start
    text: "Start"
    jump: middle
  - label: middle
    text: "Middle"
    jump: ending
  - label: ending
    text: "End"
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let cycles = detect_circular_paths(&scenario);

    assert!(cycles.is_empty());
}

#[test]
fn test_validate_multiple_errors() {
    let yaml = r#"
title: Multiple Errors

script:
  - jump: nowhere
  - label: dup
    text: "First"
  - label: dup
    text: "Second"
    jump: also_missing
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    assert!(!result.is_valid());
    assert!(result.errors().count() >= 3);
}

#[test]
fn test_severity_values() {
    let yaml = r#"
title: Mixed

script:
  - label: start
    text: "Hello"
  - label: unused
    text: "Unused"
  - jump: missing
"#;

    let scenario = parse_scenario(yaml).unwrap();
    let result = validate_scenario(&scenario);

    let errors: Vec<_> = result.issues.iter().filter(|i| i.severity == Severity::Error).collect();
    let warnings: Vec<_> = result.issues.iter().filter(|i| i.severity == Severity::Warning).collect();

    assert!(!errors.is_empty());
    assert!(!warnings.is_empty());
}
