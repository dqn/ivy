use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::scenario::{CharAnimation, CharIdleAnimation, CharPosition};

/// Single character state for multi-character support.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterState {
    pub path: String,
    pub position: CharPosition,
    /// Enter animation for this character (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter: Option<CharAnimation>,
    /// Exit animation for this character (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<CharAnimation>,
    /// Idle animation for this character (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idle: Option<CharIdleAnimation>,
}

/// Modular character state for layered sprite compositing.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModularCharState {
    /// Character definition name.
    pub name: String,
    /// Screen position.
    pub position: CharPosition,
    /// Layer variant selections (layer_name -> variant_index).
    #[serde(default)]
    pub variants: HashMap<String, usize>,
}

/// Visual state (background and character sprite).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualState {
    pub background: Option<String>,
    pub character: Option<String>,
    #[serde(default)]
    pub char_pos: CharPosition,
    /// Multiple characters (used when `characters` field is set in command).
    #[serde(default)]
    pub characters: Vec<CharacterState>,
    /// NVL mode (full-screen text display).
    #[serde(default)]
    pub nvl_mode: bool,
    /// Modular character (layered sprite compositing).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modular_char: Option<ModularCharState>,
}
