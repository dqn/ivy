//! Scenario validation for detecting errors and warnings.

use std::collections::{HashMap, HashSet};

use crate::scenario::types::Scenario;

/// Severity level for validation issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

/// A validation issue found in the scenario.
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub message: String,
    pub command_index: Option<usize>,
    pub label: Option<String>,
}

impl ValidationIssue {
    fn error(message: impl Into<String>, command_index: Option<usize>) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            command_index,
            label: None,
        }
    }

    fn warning(message: impl Into<String>, command_index: Option<usize>) -> Self {
        Self {
            severity: Severity::Warning,
            message: message.into(),
            command_index,
            label: None,
        }
    }

    fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Result of scenario validation.
#[derive(Debug, Default)]
pub struct ValidationResult {
    pub issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if validation passed (no errors).
    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }

    /// Check if there are any errors.
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    /// Check if there are any warnings.
    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Warning)
    }

    /// Get all errors.
    pub fn errors(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|i| i.severity == Severity::Error)
    }

    /// Get all warnings.
    pub fn warnings(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
    }
}

/// Validate a scenario for common issues.
pub fn validate_scenario(scenario: &Scenario) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Collect all defined labels
    let defined_labels: HashSet<&str> = scenario
        .script
        .iter()
        .filter_map(|cmd| cmd.label.as_deref())
        .collect();

    // Collect all referenced labels (jumps)
    let mut referenced_labels: HashMap<&str, Vec<usize>> = HashMap::new();

    for (index, cmd) in scenario.script.iter().enumerate() {
        // Check jump references
        if let Some(ref jump_target) = cmd.jump {
            referenced_labels
                .entry(jump_target.as_str())
                .or_default()
                .push(index);
        }

        // Check choice jump references
        if let Some(ref choices) = cmd.choices {
            for choice in choices {
                referenced_labels
                    .entry(choice.jump.as_str())
                    .or_default()
                    .push(index);
            }
        }

        // Check conditional jump references
        if let Some(ref if_cond) = cmd.if_cond {
            referenced_labels
                .entry(if_cond.jump.as_str())
                .or_default()
                .push(index);
        }
    }

    // Check for undefined label references
    for (label, indices) in &referenced_labels {
        if !defined_labels.contains(label) {
            for &index in indices {
                result.issues.push(
                    ValidationIssue::error(
                        format!("Jump to undefined label '{}'", label),
                        Some(index),
                    )
                    .with_label(label.to_string()),
                );
            }
        }
    }

    // Check for unused labels
    for (index, cmd) in scenario.script.iter().enumerate() {
        if let Some(ref label) = cmd.label {
            // "start" is special and doesn't need to be referenced
            if label != "start" && !referenced_labels.contains_key(label.as_str()) {
                result.issues.push(
                    ValidationIssue::warning(format!("Unused label '{}'", label), Some(index))
                        .with_label(label.clone()),
                );
            }
        }
    }

    // Check for circular jumps (simple case: self-referencing)
    for (index, cmd) in scenario.script.iter().enumerate() {
        if let Some(ref label) = cmd.label {
            if let Some(ref jump) = cmd.jump {
                if label == jump {
                    result.issues.push(
                        ValidationIssue::error(
                            format!("Self-referencing jump at label '{}'", label),
                            Some(index),
                        )
                        .with_label(label.clone()),
                    );
                }
            }
        }
    }

    // Check for duplicate labels
    let mut seen_labels: HashMap<&str, usize> = HashMap::new();
    for (index, cmd) in scenario.script.iter().enumerate() {
        if let Some(ref label) = cmd.label {
            if let Some(first_index) = seen_labels.get(label.as_str()) {
                result.issues.push(
                    ValidationIssue::error(
                        format!(
                            "Duplicate label '{}' (first defined at command {})",
                            label,
                            first_index + 1
                        ),
                        Some(index),
                    )
                    .with_label(label.clone()),
                );
            } else {
                seen_labels.insert(label.as_str(), index);
            }
        }
    }

    // Check for empty scenario
    if scenario.script.is_empty() {
        result
            .issues
            .push(ValidationIssue::warning("Scenario has no commands", None));
    }

    // Check for commands with choices but no text
    for (index, cmd) in scenario.script.iter().enumerate() {
        if cmd.choices.is_some() && cmd.text.is_none() {
            result.issues.push(ValidationIssue::warning(
                "Choice command without display text",
                Some(index),
            ));
        }
    }

    // Check chapter definitions
    for chapter in &scenario.chapters {
        if !defined_labels.contains(chapter.start_label.as_str()) {
            result.issues.push(ValidationIssue::error(
                format!(
                    "Chapter '{}' references undefined start label '{}'",
                    chapter.id, chapter.start_label
                ),
                None,
            ));
        }
    }

    result
}

/// Detect potential circular jump paths (more thorough analysis).
pub fn detect_circular_paths(scenario: &Scenario) -> Vec<Vec<String>> {
    let mut label_to_index: HashMap<&str, usize> = HashMap::new();
    let mut index_to_label: HashMap<usize, &str> = HashMap::new();

    // Build label index maps
    for (index, cmd) in scenario.script.iter().enumerate() {
        if let Some(ref label) = cmd.label {
            label_to_index.insert(label.as_str(), index);
            index_to_label.insert(index, label.as_str());
        }
    }

    // Build adjacency list (label -> labels it can jump to)
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for cmd in &scenario.script {
        let source_label = cmd.label.as_deref();

        // Get all jump targets from this command
        let mut targets: Vec<&str> = Vec::new();

        if let Some(ref jump) = cmd.jump {
            targets.push(jump.as_str());
        }
        if let Some(ref choices) = cmd.choices {
            for choice in choices {
                targets.push(choice.jump.as_str());
            }
        }
        if let Some(ref if_cond) = cmd.if_cond {
            targets.push(if_cond.jump.as_str());
        }

        // Associate jumps with the most recent label
        if let Some(label) = source_label {
            graph.entry(label).or_default().extend(targets);
        }
    }

    // Find cycles using DFS
    let mut cycles: Vec<Vec<String>> = Vec::new();
    let mut visited: HashSet<&str> = HashSet::new();
    let mut rec_stack: Vec<&str> = Vec::new();

    fn dfs<'a>(
        node: &'a str,
        graph: &HashMap<&'a str, HashSet<&'a str>>,
        visited: &mut HashSet<&'a str>,
        rec_stack: &mut Vec<&'a str>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node);
        rec_stack.push(node);

        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                if !visited.contains(neighbor) {
                    dfs(neighbor, graph, visited, rec_stack, cycles);
                } else if rec_stack.contains(&neighbor) {
                    // Found a cycle
                    let cycle_start = rec_stack.iter().position(|&n| n == neighbor).unwrap();
                    let cycle: Vec<String> = rec_stack[cycle_start..]
                        .iter()
                        .map(|s| s.to_string())
                        .collect();
                    cycles.push(cycle);
                }
            }
        }

        rec_stack.pop();
    }

    for label in graph.keys() {
        if !visited.contains(label) {
            dfs(label, &graph, &mut visited, &mut rec_stack, &mut cycles);
        }
    }

    cycles
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::parse_scenario;

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
    fn test_validate_undefined_label() {
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
    }

    #[test]
    fn test_validate_unused_label() {
        let yaml = r#"
title: Unused Label

script:
  - label: start
    text: "Hello"
  - label: unused
    text: "Never reached"
"#;

        let scenario = parse_scenario(yaml).unwrap();
        let result = validate_scenario(&scenario);

        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert_eq!(result.warnings().count(), 1);
    }

    #[test]
    fn test_validate_duplicate_label() {
        let yaml = r#"
title: Duplicate Label

script:
  - label: start
    text: "First"
  - label: start
    text: "Duplicate"
"#;

        let scenario = parse_scenario(yaml).unwrap();
        let result = validate_scenario(&scenario);

        assert!(!result.is_valid());
        assert_eq!(result.errors().count(), 1);
    }

    #[test]
    fn test_validate_self_referencing_jump() {
        let yaml = r#"
title: Self Reference

script:
  - label: loop
    text: "Infinite loop"
    jump: loop
"#;

        let scenario = parse_scenario(yaml).unwrap();
        let result = validate_scenario(&scenario);

        assert!(!result.is_valid());
        assert_eq!(result.errors().count(), 1);
    }

    #[test]
    fn test_validate_chapter_undefined_label() {
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
        assert_eq!(result.errors().count(), 1);
    }

    #[test]
    fn test_detect_circular_paths() {
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
    }
}
