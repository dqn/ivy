use std::path::Path;

use anyhow::Result;

use crate::scenario::types::Scenario;

/// Load a scenario from a YAML file.
pub fn load_scenario(path: impl AsRef<Path>) -> Result<Scenario> {
    let content = std::fs::read_to_string(path)?;
    let scenario: Scenario = serde_yaml::from_str(&content)?;
    Ok(scenario)
}
