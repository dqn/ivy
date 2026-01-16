//! Title screen handler.

use macroquad::prelude::*;

use crate::game::{GameContext, GameMode, QUICK_SAVE_PATH};
use crate::render::{ChapterSelectState, TitleMenuItem, draw_title_screen};
use crate::runtime::{Action, SaveData};

use super::HandlerResult;

/// Load game state from a specific path.
fn load_game_from(path: &str) -> Option<crate::runtime::GameState> {
    use crate::scenario::load_scenario;

    let save_data = match SaveData::load(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to load save: {}", e);
            return None;
        }
    };

    let scenario = match load_scenario(&save_data.scenario_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load scenario: {}", e);
            return None;
        }
    };

    eprintln!("Game loaded from {}", path);
    Some(crate::runtime::GameState::from_save_data(&save_data, scenario))
}

/// Load game state from quick save slot.
fn load_game() -> Option<crate::runtime::GameState> {
    load_game_from(QUICK_SAVE_PATH)
}

/// Load game state from numbered slot (1-10).
fn load_from_slot(slot: u8) -> Option<crate::runtime::GameState> {
    let path = SaveData::slot_path(slot);
    load_game_from(&path)
}

/// Handle title screen mode.
pub fn handle_title(ctx: &mut GameContext, font: Option<&Font>) -> HandlerResult {
    // Check if any save exists
    let has_save = SaveData::slot_exists(1)
        || SaveData::slot_exists(2)
        || SaveData::slot_exists(3)
        || std::path::Path::new(QUICK_SAVE_PATH).exists();

    // Check if gallery has any unlocked images
    let has_gallery = ctx.unlocks.image_count() > 0;

    // Check if chapters are defined
    let has_chapters = ctx.chapter_manager.has_chapters();

    let result = draw_title_screen(
        &ctx.title_config,
        &ctx.scenario_title,
        has_save,
        has_chapters,
        has_gallery,
        font,
    );

    if let Some(item) = result.selected {
        match item {
            TitleMenuItem::NewGame => {
                // Start new game
                if let Err(e) = ctx.start_new_game() {
                    eprintln!("Failed to start new game: {}", e);
                } else {
                    return HandlerResult::Transition(GameMode::InGame);
                }
            }
            TitleMenuItem::Continue => {
                // Try to load from quick save first, then from slots
                if let Some(loaded_state) = load_game() {
                    ctx.game_state = Some(loaded_state);
                    ctx.reset_game_state();
                    return HandlerResult::Transition(GameMode::InGame);
                } else {
                    // Try slots 1-3
                    for slot in 1..=3 {
                        if let Some(loaded_state) = load_from_slot(slot) {
                            ctx.game_state = Some(loaded_state);
                            ctx.reset_game_state();
                            return HandlerResult::Transition(GameMode::InGame);
                        }
                    }
                }
            }
            TitleMenuItem::Chapters => {
                ctx.chapter_select_state = ChapterSelectState::default();
                return HandlerResult::Transition(GameMode::Chapters);
            }
            TitleMenuItem::Gallery => {
                ctx.gallery_state = Default::default();
                return HandlerResult::Transition(GameMode::Gallery);
            }
            TitleMenuItem::Settings => {
                return HandlerResult::Transition(GameMode::Settings);
            }
            TitleMenuItem::Quit => {
                return HandlerResult::Exit;
            }
        }
    }

    // Exit on Escape
    if is_key_pressed(KeyCode::Escape) {
        return HandlerResult::Exit;
    }

    // Screenshot
    if ctx.settings.keybinds.is_pressed(Action::Screenshot) {
        crate::game::handlers::ingame::save_screenshot();
    }

    HandlerResult::Continue
}
