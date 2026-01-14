use serde::{Deserialize, Serialize};

use crate::scenario::CharPosition;

/// Single character state for multi-character support.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterState {
    pub path: String,
    pub position: CharPosition,
}

/// Visual state (background and character sprite).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualState {
    pub background: Option<String>,
    pub character: Option<String>,
    pub char_pos: CharPosition,
    /// Multiple characters (used when `characters` field is set in command).
    #[serde(default)]
    pub characters: Vec<CharacterState>,
}
