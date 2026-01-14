use anyhow::Result;

use crate::platform;
use crate::scenario::types::Scenario;

/// Load a scenario from a YAML file (or localStorage on WASM).
pub fn load_scenario(path: &str) -> Result<Scenario> {
    let content = platform::read_file(path)?;
    parse_scenario(&content)
}

/// Parse a scenario from a YAML string.
pub fn parse_scenario(yaml: &str) -> Result<Scenario> {
    let scenario: Scenario = serde_yaml::from_str(yaml)?;
    Ok(scenario)
}
