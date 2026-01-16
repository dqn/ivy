//! Chapter select screen handler.

use macroquad::prelude::*;

use crate::game::{GameContext, GameMode};
use crate::render::draw_chapter_select;
use crate::runtime::Action;

use super::HandlerResult;

/// Handle chapter select screen mode.
pub fn handle_chapters(ctx: &mut GameContext, font: Option<&Font>) -> HandlerResult {
    let result = draw_chapter_select(
        &ctx.chapter_select_config,
        &mut ctx.chapter_select_state,
        &ctx.chapter_manager,
        font,
    );

    if result.back_pressed {
        return HandlerResult::Transition(GameMode::Title);
    }

    if let Some(chapter_id) = result.selected {
        // Start game from the chapter's start label
        if let Some(chapter) = ctx.chapter_manager.get_chapter(&chapter_id) {
            let start_label = chapter.start_label.clone();
            if let Err(e) = ctx.start_from_chapter(&start_label) {
                eprintln!("Failed to start chapter: {}", e);
            } else {
                return HandlerResult::Transition(GameMode::InGame);
            }
        }
    }

    // Screenshot
    if ctx.settings.keybinds.is_pressed(Action::Screenshot) {
        crate::game::handlers::ingame::save_screenshot();
    }

    HandlerResult::Continue
}
