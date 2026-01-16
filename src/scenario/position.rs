//! Position utilities for mapping command indices to line numbers.

use std::collections::HashMap;

/// Line position information for a YAML element.
#[derive(Debug, Clone, Copy, Default)]
pub struct LinePosition {
    /// 0-indexed line number.
    pub line: u32,
    /// 0-indexed column number.
    pub column: u32,
}

impl LinePosition {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// Maps command indices to their line positions in the YAML source.
pub struct PositionMap {
    /// Command index -> line position.
    pub commands: HashMap<usize, LinePosition>,
    /// Label name -> line position.
    pub labels: HashMap<String, LinePosition>,
    /// Label references (jump targets) -> list of line positions.
    pub label_references: HashMap<String, Vec<LinePosition>>,
}

impl PositionMap {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            labels: HashMap::new(),
            label_references: HashMap::new(),
        }
    }

    /// Build a position map from YAML text.
    pub fn from_yaml(yaml: &str) -> Self {
        let mut map = Self::new();
        let lines: Vec<&str> = yaml.lines().collect();

        let mut in_script = false;
        let mut command_index = 0;

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Detect script section start.
            if trimmed == "script:" {
                in_script = true;
                continue;
            }

            // Exit script section when encountering another top-level key.
            if in_script
                && !line.starts_with(' ')
                && !line.starts_with('\t')
                && !trimmed.is_empty()
                && !trimmed.starts_with('-')
                && !trimmed.starts_with('#')
            {
                in_script = false;
                continue;
            }

            if !in_script {
                continue;
            }

            // Detect command start (list item).
            if trimmed.starts_with("- ") || trimmed == "-" {
                let column = line.find('-').unwrap_or(0) as u32;
                map.commands
                    .insert(command_index, LinePosition::new(line_num as u32, column));
                command_index += 1;
            }

            // Detect label definition.
            if let Some(label) = extract_label_definition(trimmed) {
                let column = line.find("label:").unwrap_or(0) as u32;
                map.labels.insert(
                    label.to_string(),
                    LinePosition::new(line_num as u32, column),
                );
            }

            // Detect jump references.
            if let Some(target) = extract_jump_target(trimmed) {
                let column = line.find("jump:").unwrap_or(0) as u32;
                map.label_references
                    .entry(target.to_string())
                    .or_default()
                    .push(LinePosition::new(line_num as u32, column));
            }

            // Detect choice jump references.
            if let Some(target) = extract_choice_jump(trimmed) {
                let column = line.find("jump:").unwrap_or(0) as u32;
                map.label_references
                    .entry(target.to_string())
                    .or_default()
                    .push(LinePosition::new(line_num as u32, column));
            }
        }

        map
    }

    /// Get the line position for a command index.
    pub fn get_command_position(&self, index: usize) -> Option<LinePosition> {
        self.commands.get(&index).copied()
    }

    /// Get the line position for a label definition.
    pub fn get_label_position(&self, label: &str) -> Option<LinePosition> {
        self.labels.get(label).copied()
    }

    /// Get all reference positions for a label.
    pub fn get_label_references(&self, label: &str) -> Option<&Vec<LinePosition>> {
        self.label_references.get(label)
    }
}

impl Default for PositionMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract label name from a line like "label: foo" or "- label: foo".
fn extract_label_definition(line: &str) -> Option<&str> {
    let trimmed = line.trim_start_matches("- ").trim();
    if trimmed.starts_with("label:") {
        let value = trimmed.strip_prefix("label:")?.trim();
        // Handle quoted and unquoted values.
        Some(value.trim_matches(|c| c == '"' || c == '\''))
    } else {
        None
    }
}

/// Extract jump target from a line like "jump: foo".
fn extract_jump_target(line: &str) -> Option<&str> {
    let trimmed = line.trim_start_matches("- ").trim();
    // Skip if this is inside a choices block (indented jump).
    if line.starts_with("        ") || line.starts_with("\t\t") {
        return None;
    }
    if trimmed.starts_with("jump:") {
        let value = trimmed.strip_prefix("jump:")?.trim();
        Some(value.trim_matches(|c| c == '"' || c == '\''))
    } else {
        None
    }
}

/// Extract choice jump target from an indented line like "  jump: foo".
fn extract_choice_jump(line: &str) -> Option<&str> {
    // Choice jumps are typically more indented.
    if !line.starts_with("        ") && !line.starts_with("\t\t") {
        return None;
    }
    let trimmed = line.trim();
    if trimmed.starts_with("jump:") {
        let value = trimmed.strip_prefix("jump:")?.trim();
        Some(value.trim_matches(|c| c == '"' || c == '\''))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_map_basic() {
        let yaml = r#"
title: Test

script:
  - label: start
    text: "Hello"
  - text: "World"
    jump: ending
  - label: ending
    text: "Goodbye"
"#;

        let map = PositionMap::from_yaml(yaml);

        // Check command positions.
        assert!(map.commands.contains_key(&0));
        assert!(map.commands.contains_key(&1));
        assert!(map.commands.contains_key(&2));

        // Check label positions.
        assert!(map.labels.contains_key("start"));
        assert!(map.labels.contains_key("ending"));

        // Check jump references.
        assert!(map.label_references.contains_key("ending"));
    }

    #[test]
    fn test_extract_label_definition() {
        assert_eq!(extract_label_definition("label: start"), Some("start"));
        assert_eq!(extract_label_definition("- label: start"), Some("start"));
        assert_eq!(extract_label_definition("label: \"start\""), Some("start"));
        assert_eq!(extract_label_definition("text: hello"), None);
    }

    #[test]
    fn test_extract_jump_target() {
        assert_eq!(extract_jump_target("jump: ending"), Some("ending"));
        assert_eq!(extract_jump_target("- jump: ending"), Some("ending"));
        assert_eq!(extract_jump_target("label: start"), None);
    }
}
