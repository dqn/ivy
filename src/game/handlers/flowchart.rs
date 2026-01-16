//! Flowchart screen handler.

use macroquad::prelude::*;

use crate::flowchart::{build_flowchart, calculate_layout};
use crate::game::{GameContext, GameMode};
use crate::render::draw_flowchart;
use crate::runtime::Action;

use super::HandlerResult;

/// Handle flowchart screen mode.
pub fn handle_flowchart(ctx: &mut GameContext, font: Option<&Font>) -> HandlerResult {
    // Build flowchart if dirty or not cached
    if ctx.flowchart_state.dirty || ctx.flowchart_cache.is_none() {
        let fc = build_flowchart(&ctx.scenario);
        let layouts = calculate_layout(&fc, &ctx.layout_config);
        ctx.flowchart_cache = Some((fc, layouts));
        ctx.flowchart_state.dirty = false;
    }

    let Some((fc, layouts)) = ctx.flowchart_cache.as_ref() else {
        eprintln!("Flowchart cache unexpectedly empty");
        return HandlerResult::Transition(GameMode::Title);
    };
    let current_idx = ctx.game_state.as_ref().map(|s| s.current_index());

    let result = draw_flowchart(
        &ctx.flowchart_config,
        &mut ctx.flowchart_state,
        fc,
        layouts,
        current_idx,
        font,
    );

    if result.back_pressed {
        let next_mode = if ctx.game_state.is_some() {
            GameMode::InGame
        } else {
            GameMode::Title
        };
        return HandlerResult::Transition(next_mode);
    }

    // Screenshot
    if ctx.settings.keybinds.is_pressed(Action::Screenshot) {
        crate::game::handlers::ingame::save_screenshot();
    }

    HandlerResult::Continue
}
