//! Settings screen handler.

use macroquad::prelude::*;

use crate::game::{GameContext, GameMode};
use crate::render::draw_settings_screen;
use crate::runtime::Action;

use super::HandlerResult;

/// Handle settings screen mode.
pub fn handle_settings(ctx: &mut GameContext, font: Option<&Font>) -> HandlerResult {
    let result = draw_settings_screen(&ctx.settings_config, &mut ctx.settings, font);

    if result.back_pressed {
        // Save settings when leaving
        ctx.settings.save();
        eprintln!("Settings saved");
        return HandlerResult::Transition(GameMode::Title);
    }

    // Apply volume settings to audio manager
    ctx.audio_manager.set_bgm_volume(ctx.settings.bgm_volume);
    ctx.audio_manager.set_se_volume(ctx.settings.se_volume);
    ctx.audio_manager
        .set_voice_volume(ctx.settings.voice_volume);

    // Screenshot
    if ctx.settings.keybinds.is_pressed(Action::Screenshot) {
        crate::game::handlers::ingame::save_screenshot();
    }

    HandlerResult::Continue
}
