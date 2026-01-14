use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub use crate::types::Value;

/// Variable storage for game state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Variables {
    data: HashMap<String, Value>,
}

impl Variables {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Set a variable.
    pub fn set(&mut self, name: impl Into<String>, value: Value) {
        self.data.insert(name.into(), value);
    }

    /// Get a variable.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.data.get(name)
    }

    /// Check if a variable equals a value.
    pub fn equals(&self, name: &str, value: &Value) -> bool {
        self.data.get(name) == Some(value)
    }

    /// Increment an integer variable.
    pub fn increment(&mut self, name: &str, amount: i64) {
        if let Some(Value::Int(current)) = self.data.get(name) {
            self.data
                .insert(name.to_string(), Value::Int(current + amount));
        }
    }

    /// Get all variables for serialization.
    pub fn all(&self) -> &HashMap<String, Value> {
        &self.data
    }

    /// Iterate over variables.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.data.iter()
    }

    /// Restore variables from serialized data.
    pub fn restore(&mut self, data: HashMap<String, Value>) {
        self.data = data;
    }
}
