//! Localized string type with custom deserialization.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

use super::Translations;

/// A string that can be localized.
///
/// Supports three formats:
/// - `Plain(String)`: Traditional single-language string.
/// - `Localized(HashMap)`: Inline localized strings (lang -> text).
/// - `Key(String)`: Reference to translation key (starts with "@").
#[derive(Debug, Clone, PartialEq)]
pub enum LocalizedString {
    /// Plain string (single language, backward compatible).
    Plain(String),
    /// Localized strings keyed by language code.
    Localized(HashMap<String, String>),
    /// Translation key reference (starts with @).
    Key(String),
}

impl Default for LocalizedString {
    fn default() -> Self {
        LocalizedString::Plain(String::new())
    }
}

impl LocalizedString {
    /// Create a plain (non-localized) string.
    pub fn plain(s: impl Into<String>) -> Self {
        LocalizedString::Plain(s.into())
    }

    /// Create a translation key reference.
    pub fn key(s: impl Into<String>) -> Self {
        LocalizedString::Key(s.into())
    }

    /// Create a localized string with multiple languages.
    pub fn localized(map: HashMap<String, String>) -> Self {
        LocalizedString::Localized(map)
    }

    /// Resolve to a string using the given language code.
    pub fn resolve(&self, lang: &str, translations: &Translations) -> String {
        match self {
            LocalizedString::Plain(s) => s.clone(),
            LocalizedString::Localized(map) => {
                // Try the requested language first
                map.get(lang)
                    // Fall back to English
                    .or_else(|| map.get("en"))
                    // Fall back to the first available language
                    .or_else(|| map.values().next())
                    .cloned()
                    .unwrap_or_default()
            }
            LocalizedString::Key(key) => translations.get(lang, key),
        }
    }

    /// Check if this is an empty string.
    pub fn is_empty(&self) -> bool {
        match self {
            LocalizedString::Plain(s) => s.is_empty(),
            LocalizedString::Localized(map) => map.is_empty(),
            LocalizedString::Key(k) => k.is_empty(),
        }
    }

    /// Get the plain string value if this is a Plain variant.
    pub fn as_plain(&self) -> Option<&str> {
        match self {
            LocalizedString::Plain(s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for LocalizedString {
    fn from(s: String) -> Self {
        if let Some(stripped) = s.strip_prefix('@') {
            LocalizedString::Key(stripped.to_string())
        } else {
            LocalizedString::Plain(s)
        }
    }
}

impl From<&str> for LocalizedString {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl PartialEq<str> for LocalizedString {
    fn eq(&self, other: &str) -> bool {
        match self {
            LocalizedString::Plain(s) => s == other,
            LocalizedString::Key(k) => k == other,
            LocalizedString::Localized(map) => {
                // Check if any language matches
                map.values().any(|v| v == other)
            }
        }
    }
}

impl PartialEq<&str> for LocalizedString {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

/// Custom deserializer that accepts either a string or a map.
impl<'de> Deserialize<'de> for LocalizedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};

        struct LocalizedStringVisitor;

        impl<'de> Visitor<'de> for LocalizedStringVisitor {
            type Value = LocalizedString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or a map of language codes to strings")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v.into())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v.into())
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::new();
                while let Some((key, value)) = access.next_entry::<String, String>()? {
                    map.insert(key, value);
                }
                Ok(LocalizedString::Localized(map))
            }
        }

        deserializer.deserialize_any(LocalizedStringVisitor)
    }
}

/// Serialize LocalizedString.
impl Serialize for LocalizedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LocalizedString::Plain(s) => serializer.serialize_str(s),
            LocalizedString::Key(k) => {
                let key_with_prefix = format!("@{}", k);
                serializer.serialize_str(&key_with_prefix)
            }
            LocalizedString::Localized(map) => map.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_plain_string() {
        let yaml = r#""Hello World""#;
        let result: LocalizedString = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, LocalizedString::Plain("Hello World".to_string()));
    }

    #[test]
    fn test_deserialize_key_reference() {
        let yaml = r#""@intro.welcome""#;
        let result: LocalizedString = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, LocalizedString::Key("intro.welcome".to_string()));
    }

    #[test]
    fn test_deserialize_localized_map() {
        let yaml = r#"
en: "Hello"
ja: "こんにちは"
"#;
        let result: LocalizedString = serde_yaml::from_str(yaml).unwrap();
        if let LocalizedString::Localized(map) = result {
            assert_eq!(map.get("en"), Some(&"Hello".to_string()));
            assert_eq!(map.get("ja"), Some(&"こんにちは".to_string()));
        } else {
            panic!("Expected Localized variant");
        }
    }

    #[test]
    fn test_resolve_plain() {
        let text = LocalizedString::Plain("Hello".to_string());
        let translations = Translations::new();
        assert_eq!(text.resolve("en", &translations), "Hello");
        assert_eq!(text.resolve("ja", &translations), "Hello");
    }

    #[test]
    fn test_resolve_localized() {
        let mut map = HashMap::new();
        map.insert("en".to_string(), "Hello".to_string());
        map.insert("ja".to_string(), "こんにちは".to_string());
        let text = LocalizedString::Localized(map);
        let translations = Translations::new();

        assert_eq!(text.resolve("en", &translations), "Hello");
        assert_eq!(text.resolve("ja", &translations), "こんにちは");
        // Falls back to English for unknown language
        assert_eq!(text.resolve("fr", &translations), "Hello");
    }

    #[test]
    fn test_resolve_key() {
        let text = LocalizedString::Key("intro.welcome".to_string());
        let mut translations = Translations::new();

        let mut en = HashMap::new();
        en.insert("intro.welcome".to_string(), "Welcome!".to_string());
        translations.add_language("en", en);

        let mut ja = HashMap::new();
        ja.insert("intro.welcome".to_string(), "ようこそ！".to_string());
        translations.add_language("ja", ja);

        assert_eq!(text.resolve("en", &translations), "Welcome!");
        assert_eq!(text.resolve("ja", &translations), "ようこそ！");
    }

    #[test]
    fn test_serialize_plain() {
        let text = LocalizedString::Plain("Hello".to_string());
        let yaml = serde_yaml::to_string(&text).unwrap();
        assert_eq!(yaml.trim(), "Hello");
    }

    #[test]
    fn test_serialize_key() {
        let text = LocalizedString::Key("intro.welcome".to_string());
        let yaml = serde_yaml::to_string(&text).unwrap();
        assert_eq!(yaml.trim(), "'@intro.welcome'");
    }
}
