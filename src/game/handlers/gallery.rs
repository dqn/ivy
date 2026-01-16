//! Gallery screen handler.

use macroquad::prelude::*;

use crate::game::{GameContext, GameMode};
use crate::render::draw_gallery;
use crate::runtime::Action;

use super::HandlerResult;

/// Handle gallery screen mode.
pub async fn handle_gallery(ctx: &mut GameContext, font: Option<&Font>) -> HandlerResult {
    let images = ctx.unlocks.unlocked_images();
    let result = draw_gallery(
        &ctx.gallery_config,
        &mut ctx.gallery_state,
        &images,
        ctx.texture_cache.as_map(),
        font,
    );

    if result.back_pressed {
        return HandlerResult::Transition(GameMode::Title);
    }

    // Load textures for gallery images (async)
    for path in &images {
        if !ctx.texture_cache.contains(path)
            && let Ok(texture) = load_texture(path).await
        {
            texture.set_filter(FilterMode::Linear);
            ctx.texture_cache.insert(path.clone(), texture);
        }
    }

    // Screenshot
    if ctx.settings.keybinds.is_pressed(Action::Screenshot) {
        crate::game::handlers::ingame::save_screenshot();
    }

    HandlerResult::Continue
}
