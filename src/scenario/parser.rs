use anyhow::{anyhow, Result};

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

            message.push_str(&format!("\nCause: {}", err));
            anyhow!(message)
        }
        None => anyhow!("YAML parse error: {}", err),
    }
}
