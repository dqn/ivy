//! In-game handler.
//!
//! Handles the main gameplay loop including text display, choices,
//! animations, and transitions.

mod command_update;
mod display;
mod visual;

use macroquad::prelude::*;

use crate::game::{GameContext, GameMode, InputDetector, PlayerAction, QUICK_SAVE_PATH, SCENARIO_PATH};
use crate::render::{calculate_camera_transform, draw_achievement, draw_debug};
use crate::runtime::{DisplayState, GameState, SaveData};
use crate::scenario::load_scenario;

use super::HandlerResult;

pub use visual::draw_visual;

/// Save a screenshot to the screenshots directory.
pub fn save_screenshot() {
    use std::fs;

    // Ensure screenshots directory exists
    if let Err(e) = fs::create_dir_all("screenshots") {
        eprintln!("Failed to create screenshots directory: {}", e);
        return;
    }

    // Get current timestamp for filename
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let filename = format!("screenshots/screenshot_{}.png", timestamp);

    // Capture screen and save
    let image = get_screen_data();
    image.export_png(&filename);
    eprintln!("Screenshot saved: {}", filename);
}

/// Save game state to a specific path.
fn save_game_to(game_state: &GameState, path: &str) {
    let save_data = game_state.to_save_data(SCENARIO_PATH);
    match save_data.save(path) {
        Ok(()) => eprintln!("Game saved to {}", path),
        Err(e) => eprintln!("Failed to save game: {}", e),
    }
}

/// Save game state to quick save slot.
fn save_game(game_state: &GameState) {
    save_game_to(game_state, QUICK_SAVE_PATH);
}

/// Save game state to numbered slot (1-10).
fn save_to_slot(game_state: &GameState, slot: u8) {
    let path = SaveData::slot_path(slot);
    save_game_to(game_state, &path);
}

/// Load game state from a specific path.
fn load_game_from(path: &str) -> Option<GameState> {
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
    Some(GameState::from_save_data(&save_data, scenario))
}

/// Load game state from quick save slot.
fn load_game() -> Option<GameState> {
    load_game_from(QUICK_SAVE_PATH)
}

/// Load game state from numbered slot (1-10).
fn load_from_slot(slot: u8) -> Option<GameState> {
    let path = SaveData::slot_path(slot);
    load_game_from(&path)
}

/// Handle in-game mode.
pub async fn handle_ingame(ctx: &mut GameContext, font: Option<&Font>) -> HandlerResult {
    // Check for hot reload
    if let Some(ref mut reloader) = ctx.hot_reloader
        && reloader.poll()
        && ctx.game_state.is_some()
    {
        match load_scenario(SCENARIO_PATH) {
            Ok(new_scenario) => {
                ctx.reload_scenario(new_scenario);
                eprintln!("[Hot Reload] Scenario reloaded");
            }
            Err(e) => {
                eprintln!("[Hot Reload] Failed to reload: {}", e);
            }
        }
    }

    // Take game state out of ctx to avoid borrow conflicts
    let mut state = match ctx.game_state.take() {
        Some(s) => s,
        None => {
            return HandlerResult::Transition(GameMode::Title);
        }
    };

    // Apply accessibility settings to text config
    let text_config = ctx.text_config();

    // Capture all input for this frame (this drops the borrow on ctx)
    let input = {
        let detector = InputDetector::new(&ctx.settings.keybinds, &ctx.gamepad_state);
        detector.capture(ctx.last_mouse_pos, ctx.choice_nav_state.stick_debounce)
    };

    // Process detected actions
    let mut return_to_title = false;
    let mut transition_to_flowchart = false;

    for action in &input.actions {
        match *action {
            PlayerAction::QuickSave => save_game(&state),
            PlayerAction::QuickLoad => {
                if let Some(loaded_state) = load_game() {
                    state = loaded_state;
                    ctx.last_index = None;
                }
            }
            PlayerAction::SaveToSlot(slot) => save_to_slot(&state, slot),
            PlayerAction::LoadFromSlot(slot) => {
                if SaveData::slot_exists(slot) {
                    if let Some(loaded_state) = load_from_slot(slot) {
                        state = loaded_state;
                        ctx.last_index = None;
                    }
                } else {
                    eprintln!("Slot {} is empty", slot);
                }
            }
            PlayerAction::ToggleBacklog => {
                ctx.show_backlog = !ctx.show_backlog;
                ctx.backlog_state = Default::default();
            }
            PlayerAction::ToggleAuto => {
                ctx.auto_mode = !ctx.auto_mode;
                ctx.auto_timer = 0.0;
                eprintln!("Auto mode {}", if ctx.auto_mode { "ON" } else { "OFF" });
            }
            PlayerAction::ToggleSkip => {
                ctx.skip_mode = !ctx.skip_mode;
                eprintln!("Skip mode {}", if ctx.skip_mode { "ON" } else { "OFF" });
            }
            PlayerAction::ToggleDebug => ctx.debug_state.toggle(),
            PlayerAction::OpenFlowchart => {
                transition_to_flowchart = true;
                ctx.flowchart_state.dirty = true;
            }
            PlayerAction::Rollback => {
                if !ctx.show_backlog && state.can_rollback() {
                    state.rollback();
                }
            }
            PlayerAction::Screenshot => save_screenshot(),
            PlayerAction::ReturnToTitle => {
                if !state.is_ended() {
                    return_to_title = true;
                }
            }
            _ => {}
        }
    }

    if transition_to_flowchart {
        ctx.game_state = Some(state);
        return HandlerResult::Transition(GameMode::Flowchart);
    }

    // Update audio, transition, and unlock images when command changes
    let current_index = state.current_index();
    if ctx.last_index != Some(current_index) {
        command_update::on_command_change(ctx, &mut state).await;
        ctx.last_index = Some(current_index);
    }

    // Update animation states
    command_update::update_animations(ctx);

    // Get shake offset for visual rendering
    let shake_offset = ctx.shake_state.offset();

    // Calculate camera transform
    let camera_transform =
        calculate_camera_transform(&ctx.camera_state, screen_width(), screen_height());

    // Handle display state
    let display_state = state.display_state();
    match &display_state {
        DisplayState::Text { speaker, text, visual } => {
            display::handle_text(
                ctx,
                &mut state,
                speaker.clone(),
                text.clone(),
                visual.clone(),
                &text_config,
                &input,
                &camera_transform,
                shake_offset,
                font,
            ).await;
        }
        DisplayState::Choices {
            speaker,
            text,
            choices,
            visual,
            timeout,
            default_choice,
        } => {
            display::handle_choices(
                ctx,
                &mut state,
                speaker.clone(),
                text.clone(),
                choices.clone(),
                visual.clone(),
                *timeout,
                *default_choice,
                &text_config,
                &input,
                &camera_transform,
                shake_offset,
                font,
            ).await;
        }
        DisplayState::Wait { duration, visual } => {
            display::handle_wait(
                ctx,
                &mut state,
                *duration,
                visual.clone(),
                &input,
                &camera_transform,
                shake_offset,
            ).await;
        }
        DisplayState::Input { input: input_cmd, visual } => {
            display::handle_input(
                ctx,
                &mut state,
                input_cmd.clone(),
                visual.clone(),
                &camera_transform,
                shake_offset,
                font,
            ).await;
        }
        DisplayState::Video {
            path,
            skippable,
            loop_video,
            ..
        } => {
            display::handle_video(
                ctx,
                &mut state,
                path.clone(),
                *skippable,
                *loop_video,
                &input,
            ).await;
        }
        DisplayState::End => {
            if display::handle_end(ctx, &mut state, &text_config, &input, font) {
                return_to_title = true;
            }
        }
    }

    // Update and draw particles
    ctx.particle_state.update_and_draw();

    // Update and draw cinematic bars
    ctx.cinematic_state.update();
    ctx.cinematic_state.draw();

    // Update and draw achievement notification
    ctx.achievement_notifier.update(get_frame_time());
    draw_achievement(&ctx.achievement_config, &ctx.achievement_notifier, font);

    // Draw debug overlay
    draw_debug(&ctx.debug_config, &ctx.debug_state, &state, font);

    // Draw transition overlay
    ctx.transition_state.draw();

    // Return to title on Escape (instead of exiting)
    if is_key_pressed(KeyCode::Escape) && !state.is_ended() {
        return_to_title = true;
    }

    if return_to_title {
        // Don't restore state when returning to title
        return HandlerResult::Transition(GameMode::Title);
    }

    // Restore game state back to ctx
    ctx.game_state = Some(state);
    HandlerResult::Continue
}
