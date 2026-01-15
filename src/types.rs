use serde::{Deserialize, Serialize};

/// Variable value types used across scenario and runtime modules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "editor-types", derive(ts_rs::TS))]
#[serde(untagged)]
pub enum Value {
    Bool(bool),
    Int(i64),
    String(String),
}

impl Value {
    /// Get as boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Get as integer.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Get as string.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_bool() {
        let value = Value::Bool(true);
        assert_eq!(value.as_bool(), Some(true));
        assert_eq!(value.as_int(), None);
        assert_eq!(value.as_string(), None);
    }

    #[test]
    fn test_value_int() {
        let value = Value::Int(42);
        assert_eq!(value.as_bool(), None);
        assert_eq!(value.as_int(), Some(42));
        assert_eq!(value.as_string(), None);
    }

    #[test]
    fn test_value_string() {
        let value = Value::String("hello".to_string());
        assert_eq!(value.as_bool(), None);
        assert_eq!(value.as_int(), None);
        assert_eq!(value.as_string(), Some("hello"));
    }

    #[test]
    fn test_value_equality() {
        assert_eq!(Value::Bool(true), Value::Bool(true));
        assert_ne!(Value::Bool(true), Value::Bool(false));
        assert_eq!(Value::Int(10), Value::Int(10));
        assert_ne!(Value::Int(10), Value::Int(20));
        assert_eq!(
            Value::String("test".to_string()),
            Value::String("test".to_string())
        );
    }

    #[test]
    fn test_value_serialization() {
        let bool_val = Value::Bool(true);
        let int_val = Value::Int(123);
        let str_val = Value::String("test".to_string());

        assert_eq!(serde_json::to_string(&bool_val).unwrap(), "true");
        assert_eq!(serde_json::to_string(&int_val).unwrap(), "123");
        assert_eq!(serde_json::to_string(&str_val).unwrap(), "\"test\"");
    }

    #[test]
    fn test_value_deserialization() {
        let bool_val: Value = serde_json::from_str("true").unwrap();
        let int_val: Value = serde_json::from_str("42").unwrap();
        let str_val: Value = serde_json::from_str("\"hello\"").unwrap();

        assert_eq!(bool_val, Value::Bool(true));
        assert_eq!(int_val, Value::Int(42));
        assert_eq!(str_val, Value::String("hello".to_string()));
    }
}
