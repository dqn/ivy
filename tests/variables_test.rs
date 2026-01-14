use ivy::runtime::{Value, Variables};

#[test]
fn test_new_variables_is_empty() {
    let vars = Variables::new();
    assert!(vars.all().is_empty());
}

#[test]
fn test_set_and_get_string() {
    let mut vars = Variables::new();
    vars.set("name", Value::String("Alice".to_string()));

    let value = vars.get("name");
    assert!(value.is_some());
    assert_eq!(value.unwrap().as_string(), Some("Alice"));
}

#[test]
fn test_set_and_get_int() {
    let mut vars = Variables::new();
    vars.set("score", Value::Int(100));

    let value = vars.get("score");
    assert!(value.is_some());
    assert_eq!(value.unwrap().as_int(), Some(100));
}

#[test]
fn test_set_and_get_bool() {
    let mut vars = Variables::new();
    vars.set("has_key", Value::Bool(true));

    let value = vars.get("has_key");
    assert!(value.is_some());
    assert_eq!(value.unwrap().as_bool(), Some(true));
}

#[test]
fn test_get_nonexistent_returns_none() {
    let vars = Variables::new();
    assert!(vars.get("nonexistent").is_none());
}

#[test]
fn test_equals_true() {
    let mut vars = Variables::new();
    vars.set("flag", Value::Bool(true));

    assert!(vars.equals("flag", &Value::Bool(true)));
}

#[test]
fn test_equals_false_different_value() {
    let mut vars = Variables::new();
    vars.set("flag", Value::Bool(true));

    assert!(!vars.equals("flag", &Value::Bool(false)));
}

#[test]
fn test_equals_false_nonexistent() {
    let vars = Variables::new();
    assert!(!vars.equals("nonexistent", &Value::Bool(true)));
}

#[test]
fn test_increment_int() {
    let mut vars = Variables::new();
    vars.set("count", Value::Int(10));
    vars.increment("count", 5);

    assert_eq!(vars.get("count").unwrap().as_int(), Some(15));
}

#[test]
fn test_increment_negative() {
    let mut vars = Variables::new();
    vars.set("count", Value::Int(10));
    vars.increment("count", -3);

    assert_eq!(vars.get("count").unwrap().as_int(), Some(7));
}

#[test]
fn test_increment_nonexistent_does_nothing() {
    let mut vars = Variables::new();
    vars.increment("count", 5);

    assert!(vars.get("count").is_none());
}

#[test]
fn test_increment_non_int_does_nothing() {
    let mut vars = Variables::new();
    vars.set("name", Value::String("Alice".to_string()));
    vars.increment("name", 5);

    // Value should remain unchanged
    assert_eq!(vars.get("name").unwrap().as_string(), Some("Alice"));
}

#[test]
fn test_overwrite_variable() {
    let mut vars = Variables::new();
    vars.set("value", Value::Int(10));
    vars.set("value", Value::Int(20));

    assert_eq!(vars.get("value").unwrap().as_int(), Some(20));
}

#[test]
fn test_iter() {
    let mut vars = Variables::new();
    vars.set("a", Value::Int(1));
    vars.set("b", Value::Int(2));

    let count = vars.iter().count();
    assert_eq!(count, 2);
}

#[test]
fn test_restore() {
    let mut vars = Variables::new();
    vars.set("original", Value::Int(1));

    let mut new_data = std::collections::HashMap::new();
    new_data.insert("restored".to_string(), Value::Int(42));

    vars.restore(new_data);

    assert!(vars.get("original").is_none());
    assert_eq!(vars.get("restored").unwrap().as_int(), Some(42));
}

#[test]
fn test_serialization_roundtrip() {
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
