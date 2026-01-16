//! Visual rendering utilities.

use std::collections::HashMap;

use crate::cache::TextureCache;
use crate::render::{
    CharAnimationState, CharIdleState, VideoBackgroundState, draw_background_with_offset,
    draw_character_animated, draw_modular_char,
};
use crate::runtime::VisualState;
use crate::scenario::{CharPosition, ModularCharDef};

/// Draw visual elements (background and character) with shake offset and character animation.
#[allow(clippy::too_many_arguments)]
pub async fn draw_visual(
    visual: &VisualState,
    cache: &mut TextureCache,
    offset: (f32, f32),
    char_anim: &CharAnimationState,
    char_idle: &CharIdleState,
    char_anim_states: &HashMap<CharPosition, CharAnimationState>,
    char_idle_states: &HashMap<CharPosition, CharIdleState>,
    modular_char_defs: &HashMap<String, ModularCharDef>,
    video_bg_state: &VideoBackgroundState,
) {
    // Draw video background if active
    if video_bg_state.is_playing() {
        video_bg_state.draw();
    } else if let Some(bg_path) = &visual.background
        && let Some(texture) = cache.get(bg_path).await
    {
        // Draw static background if no video background
        draw_background_with_offset(&texture, offset);
    }

    // Draw modular character (if specified)
    if let Some(modular) = &visual.modular_char {
        if let Some(def) = modular_char_defs.get(&modular.name) {
            draw_modular_char(modular, def, cache, offset).await;
        }
    } else if !visual.characters.is_empty() {
        // Draw multiple characters (if specified)
        for char_state in &visual.characters {
            if let Some(texture) = cache.get(&char_state.path).await {
                let pos = char_state.position;
                let anim = char_anim_states.get(&pos).cloned().unwrap_or_default();
                let idle = char_idle_states.get(&pos).cloned().unwrap_or_default();
                draw_character_animated(&texture, pos, offset, &anim, &idle);
            }
        }
    } else if let Some(char_path) = &visual.character {
        // Draw single character with animation
        if let Some(texture) = cache.get(char_path).await {
            draw_character_animated(&texture, visual.char_pos, offset, char_anim, char_idle);
        }
    }
}
