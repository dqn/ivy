use anyhow::{Result, anyhow};

use crate::platform;
use crate::scenario::types::Scenario;

/// Load a scenario from a YAML file (or localStorage on WASM).
pub fn load_scenario(path: &str) -> Result<Scenario> {
    let content = platform::read_file(path)?;
    parse_scenario(&content)
}

/// Parse a scenario from a YAML string.
pub fn parse_scenario(yaml: &str) -> Result<Scenario> {
    serde_yaml::from_str(yaml).map_err(|e| format_yaml_error(yaml, e))
}

/// Common error patterns and their helpful hints.
struct ErrorHint {
    /// Pattern to search for in the error line.
    pattern: &'static str,
    /// Hint message to display.
    hint: &'static str,
}

/// List of common error patterns with helpful hints.
const ERROR_HINTS: &[ErrorHint] = &[
    ErrorHint {
        pattern: "choice:",
        hint: "Did you mean 'choices:' (plural)? Choices must be a list.",
    },
    ErrorHint {
        pattern: "pos:",
        hint: "Did you mean 'char_pos:'? Use 'char_pos: left/center/right' for character position.",
    },
    ErrorHint {
        pattern: "position:",
        hint: "Did you mean 'char_pos:'? Use 'char_pos: left/center/right' for character position.",
    },
    ErrorHint {
        pattern: "bg:",
        hint: "Did you mean 'background:'? Use 'background:' to set the background image.",
    },
    ErrorHint {
        pattern: "char:",
        hint: "Did you mean 'character:'? Use 'character:' to set the character sprite.",
    },
    ErrorHint {
        pattern: "sprite:",
        hint: "Did you mean 'character:'? Use 'character:' to set the character sprite.",
    },
    ErrorHint {
        pattern: "music:",
        hint: "Did you mean 'bgm:'? Use 'bgm:' to set background music.",
    },
    ErrorHint {
        pattern: "sound:",
        hint: "Did you mean 'se:'? Use 'se:' for sound effects or 'bgm:' for background music.",
    },
    ErrorHint {
        pattern: "sfx:",
        hint: "Did you mean 'se:'? Use 'se:' for sound effects.",
    },
    ErrorHint {
        pattern: "goto:",
        hint: "Did you mean 'jump:'? Use 'jump: label_name' to jump to a label.",
    },
    ErrorHint {
        pattern: "branch:",
        hint: "Did you mean 'jump:' or 'choices:'? Use 'jump:' for unconditional jumps or 'choices:' for player decisions.",
    },
    ErrorHint {
        pattern: "option:",
        hint: "Did you mean 'choices:'? Use 'choices:' with 'label:' and 'jump:' for each option.",
    },
    ErrorHint {
        pattern: "options:",
        hint: "Did you mean 'choices:'? Use 'choices:' with 'label:' and 'jump:' for each option.",
    },
    ErrorHint {
        pattern: "condition:",
        hint: "Did you mean 'if:'? Use 'if: { var: name, is: value, jump: label }' for conditionals.",
    },
    ErrorHint {
        pattern: "when:",
        hint: "Did you mean 'if:'? Use 'if: { var: name, is: value, jump: label }' for conditionals.",
    },
    ErrorHint {
        pattern: "delay:",
        hint: "Did you mean 'wait:'? Use 'wait: 2.0' for a pause in seconds.",
    },
    ErrorHint {
        pattern: "pause:",
        hint: "Did you mean 'wait:'? Use 'wait: 2.0' for a pause in seconds.",
    },
    ErrorHint {
        pattern: "name:",
        hint: "Did you mean 'speaker:'? Use 'speaker:' to set who is speaking.",
    },
    ErrorHint {
        pattern: "dialogue:",
        hint: "Did you mean 'text:'? Use 'text:' for the dialogue content.",
    },
    ErrorHint {
        pattern: "message:",
        hint: "Did you mean 'text:'? Use 'text:' for the dialogue content.",
    },
    ErrorHint {
        pattern: "fade:",
        hint: "Did you mean 'transition:'? Use 'transition: { type: fade, duration: 1.0 }'.",
    },
    ErrorHint {
        pattern: "effect:",
        hint: "Did you mean 'particles:', 'shake:', or 'transition:'? See documentation for effect types.",
    },
    ErrorHint {
        pattern: "var:",
        hint: "If setting a variable, use 'set: { name: var_name, value: value }'. If checking, use 'if: { var: name, is: value, jump: label }'.",
    },
    ErrorHint {
        pattern: "variable:",
        hint: "If setting a variable, use 'set: { name: var_name, value: value }'. If checking, use 'if: { var: name, is: value, jump: label }'.",
    },
];

/// Detect common YAML patterns and provide helpful hints.
fn detect_error_hints(yaml: &str, error_line: usize) -> Option<String> {
    let lines: Vec<&str> = yaml.lines().collect();

    // Check the error line and surrounding lines
    let start = error_line.saturating_sub(2);
    let end = (error_line + 1).min(lines.len());

    for idx in start..end {
        if let Some(line) = lines.get(idx) {
            let line_lower = line.to_lowercase();
            for hint in ERROR_HINTS {
                if line_lower.contains(hint.pattern) {
                    return Some(hint.hint.to_string());
                }
            }

            // Check for common formatting issues
            if line.contains("text:") && !line.contains('"') && !line.contains('\'') {
                let trimmed = line.trim();
                if trimmed.starts_with("- text:") || trimmed.starts_with("text:") {
                    let after_colon = trimmed.split(':').nth(1).unwrap_or("").trim();
                    // If there's content after colon that's not quoted and not empty
                    if !after_colon.is_empty()
                        && !after_colon.starts_with('"')
                        && !after_colon.starts_with('\'')
                        && !after_colon.starts_with('|')
                        && !after_colon.starts_with('>')
                    {
                        return Some(
                            "Text values should be quoted. Use 'text: \"your text here\"'."
                                .to_string(),
                        );
                    }
                }
            }

            // Check for tab characters
            if line.contains('\t') {
                return Some(
                    "YAML does not allow tabs for indentation. Use spaces instead.".to_string(),
                );
            }
        }
    }

    None
}

/// Format a YAML parse error with line context for better readability.
fn format_yaml_error(yaml: &str, err: serde_yaml::Error) -> anyhow::Error {
    let location = err.location();

    match location {
        Some(loc) => {
            let line = loc.line();
            let col = loc.column();
            let lines: Vec<&str> = yaml.lines().collect();

            let mut message = format!("YAML parse error at line {}, column {}:\n", line, col);

            // Show context: 2 lines before, the error line, and 2 lines after
            let start = line.saturating_sub(3);
            let end = (line + 2).min(lines.len());

            for (idx, line_content) in lines.iter().enumerate().skip(start).take(end - start) {
                let line_num = idx + 1;
                let prefix = if line_num == line { ">> " } else { "   " };
                message.push_str(&format!("{}{:4} | {}\n", prefix, line_num, line_content));

                // Add column indicator for the error line
                if line_num == line && col > 0 {
                    let spaces = " ".repeat(col + 7); // Account for prefix and line number
                    message.push_str(&format!("{}^\n", spaces));
                }
            }

            // Add helpful hint if detected
            if let Some(hint) = detect_error_hints(yaml, line) {
                message.push_str(&format!("\nHint: {}", hint));
            }

            message.push_str(&format!("\nCause: {}", err));
            anyhow!(message)
        }
        None => anyhow!("YAML parse error: {}", err),
    }
}
