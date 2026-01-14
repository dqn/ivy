//! Internationalization (i18n) support.
//!
//! This module provides localization support while maintaining backwards compatibility
//! with single-language scenarios.
//!
//! # Design Goals
//!
//! - **Backwards compatible**: Single-language scenarios work exactly as before.
//! - **Simple single-language**: No extra syntax when using only one language.
//! - **Flexible multi-language**: Two approaches for multi-language support:
//!   - Inline: Multiple languages in the same YAML file.
//!   - Separate: Language files in a dedicated directory.
//!
//! # Examples
//!
//! ## Single Language (Traditional)
//! ```yaml
//! - text: "Welcome to the story!"
//! ```
//!
//! ## Inline Multi-Language
//! ```yaml
//! - text:
//!     en: "Welcome to the story!"
//!     ja: "ストーリーへようこそ！"
//! ```
//!
//! ## Separate Language Files
//! ```yaml
//! # main.yaml
//! - text: "@intro.welcome"
//!
//! # i18n/en.yaml
//! intro:
//!   welcome: "Welcome to the story!"
//!
//! # i18n/ja.yaml
//! intro:
//!   welcome: "ストーリーへようこそ！"
//! ```

mod localized;

pub use localized::LocalizedString;

use std::collections::HashMap;

/// Translation storage for key-based localization.
#[derive(Debug, Clone, Default)]
pub struct Translations {
    /// Language code -> (key path -> translated text)
    data: HashMap<String, HashMap<String, String>>,
    /// Fallback language code (default: "en")
    fallback: String,
}

impl Translations {
    /// Create a new empty translations store.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            fallback: "en".to_string(),
        }
    }

    /// Set the fallback language.
    pub fn set_fallback(&mut self, lang: &str) {
        self.fallback = lang.to_string();
    }

    /// Add translations for a language.
    pub fn add_language(&mut self, lang: &str, translations: HashMap<String, String>) {
        self.data.insert(lang.to_string(), translations);
    }

    /// Get a translation by key for a specific language.
    /// Falls back to the fallback language if not found.
    pub fn get(&self, lang: &str, key: &str) -> String {
        // Try the requested language first
        if let Some(lang_data) = self.data.get(lang)
            && let Some(text) = lang_data.get(key)
        {
            return text.clone();
        }

        // Try fallback language
        if lang != self.fallback
            && let Some(fallback_data) = self.data.get(&self.fallback)
            && let Some(text) = fallback_data.get(key)
        {
            return text.clone();
        }

        // Return the key itself as a last resort
        key.to_string()
    }

    /// Check if any translations are loaded.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get available language codes.
    pub fn languages(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

/// Language configuration for the game.
#[derive(Debug, Clone)]
pub struct LanguageConfig {
    /// Current language code.
    pub current: String,
    /// Available languages.
    pub available: Vec<String>,
    /// Translation data.
    pub translations: Translations,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            current: "en".to_string(),
            available: vec!["en".to_string()],
            translations: Translations::new(),
        }
    }
}

impl LanguageConfig {
    /// Create a new language configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current language.
    pub fn set_language(&mut self, lang: &str) {
        if self.available.contains(&lang.to_string()) {
            self.current = lang.to_string();
        }
    }

    /// Resolve a LocalizedString to a plain string using the current language.
    pub fn resolve(&self, text: &LocalizedString) -> String {
        text.resolve(&self.current, &self.translations)
    }
}
