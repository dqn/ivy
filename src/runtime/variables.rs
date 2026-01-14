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
            self.data.insert(name.to_string(), Value::Int(current + amount));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variables_set_and_get() {
        let mut vars = Variables::new();

        vars.set("name", Value::String("test".to_string()));
        vars.set("count", Value::Int(42));
        vars.set("flag", Value::Bool(true));

        assert_eq!(vars.get("name"), Some(&Value::String("test".to_string())));
        assert_eq!(vars.get("count"), Some(&Value::Int(42)));
        assert_eq!(vars.get("flag"), Some(&Value::Bool(true)));
        assert_eq!(vars.get("nonexistent"), None);
    }

    #[test]
    fn test_variables_equals() {
        let mut vars = Variables::new();
        vars.set("count", Value::Int(10));

        assert!(vars.equals("count", &Value::Int(10)));
        assert!(!vars.equals("count", &Value::Int(20)));
        assert!(!vars.equals("nonexistent", &Value::Int(10)));
    }

    #[test]
    fn test_variables_increment() {
        let mut vars = Variables::new();
        vars.set("count", Value::Int(10));

        vars.increment("count", 5);
        assert_eq!(vars.get("count"), Some(&Value::Int(15)));

        vars.increment("count", -3);
        assert_eq!(vars.get("count"), Some(&Value::Int(12)));

        // Incrementing non-existent variable does nothing
        vars.increment("nonexistent", 10);
        assert_eq!(vars.get("nonexistent"), None);

        // Incrementing non-integer variable does nothing
        vars.set("name", Value::String("test".to_string()));
        vars.increment("name", 5);
        assert_eq!(vars.get("name"), Some(&Value::String("test".to_string())));
    }

    #[test]
    fn test_variables_overwrite() {
        let mut vars = Variables::new();
        vars.set("key", Value::Int(1));
        vars.set("key", Value::Int(2));
        assert_eq!(vars.get("key"), Some(&Value::Int(2)));
    }

    #[test]
    fn test_variables_serialization() {
        let mut vars = Variables::new();
        vars.set("name", Value::String("test".to_string()));
        vars.set("count", Value::Int(42));

        let json = serde_json::to_string(&vars).unwrap();
        let restored: Variables = serde_json::from_str(&json).unwrap();

        assert_eq!(
            restored.get("name"),
            Some(&Value::String("test".to_string()))
        );
        assert_eq!(restored.get("count"), Some(&Value::Int(42)));
    }
}
