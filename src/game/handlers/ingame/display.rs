//! Display state handlers for different game states.

use macroquad::prelude::*;

use crate::game::{ChoiceNavAction, DetectedInput, GameContext, SCENARIO_PATH};
use crate::i18n::LocalizedString;
use crate::render::{
    CameraTransform, InputSource, TextBoxConfig, count_nvl_chars, count_visible_chars,
    draw_backlog, draw_choices_with_timer, draw_continue_indicator_with_font, draw_input,
    draw_nvl_text_box, draw_speaker_name, draw_text_box_typewriter, draw_text_box_with_font,
    interpolate_variables, pop_camera_transform, push_camera_transform,
};
use crate::runtime::{GameState, Value, VisualState};
use crate::scenario::{Choice, Input};

use super::draw_visual;

/// Handle DisplayState::Text
#[allow(clippy::too_many_arguments)]
pub async fn handle_text(
    ctx: &mut GameContext,
    state: &mut GameState,
    speaker: Option<LocalizedString>,
    text: LocalizedString,
    visual: VisualState,
    text_config: &TextBoxConfig,
    input: &DetectedInput,
    camera_transform: &CameraTransform,
    shake_offset: (f32, f32),
    font: Option<&Font>,
) {
    // Update NVL state based on visual state
    let is_nvl_mode = visual.nvl_mode;
    if ctx.nvl_state.active != is_nvl_mode {
        ctx.nvl_state.set_active(is_nvl_mode);
    }

    // Check for NVL clear command
    if is_nvl_mode && state.current_nvl_clear() {
        ctx.nvl_state.clear();
    }

    // Draw visuals first (background, then character) with shake offset and camera
    push_camera_transform(camera_transform);
    draw_visual(
        &visual,
        &mut ctx.texture_cache,
        shake_offset,
        &ctx.char_anim_state,
        &ctx.char_idle_state,
        &ctx.char_anim_states,
        &ctx.char_idle_states,
        &ctx.modular_char_defs,
        &ctx.video_bg_state,
    )
    .await;
    pop_camera_transform(camera_transform);

    // Resolve localized text
    let resolved_text = ctx.language_config.resolve(&text);

    // Interpolate variables in text
    let interpolated_text = interpolate_variables(&resolved_text, state.variables());

    // Resolve speaker name
    let resolved_speaker = speaker.as_ref().map(|name| {
        interpolate_variables(&ctx.language_config.resolve(name), state.variables())
    });

    // Reset typewriter if text changed
    if ctx.last_text.as_ref() != Some(&interpolated_text) {
        if is_nvl_mode {
            // In NVL mode, count all accumulated chars plus current text
            let total_chars = count_nvl_chars(&ctx.nvl_state, &interpolated_text);
            ctx.typewriter_state.reset(total_chars);
        } else {
            // In ADV mode, count visible characters (excluding color tags)
            let total_chars = count_visible_chars(&interpolated_text);
            ctx.typewriter_state.reset(total_chars);
        }
        ctx.last_text = Some(interpolated_text.clone());
    }

    // Update typewriter state
    let char_limit = ctx.typewriter_state.update(ctx.settings.text_speed);

    if is_nvl_mode {
        // NVL mode: draw full-screen text box
        draw_nvl_text_box(
            &ctx.nvl_config,
            &ctx.nvl_state,
            resolved_speaker.as_deref(),
            &interpolated_text,
            font,
            char_limit,
        );
    } else {
        // ADV mode: draw speaker name if present
        if let Some(ref name) = resolved_speaker {
            draw_speaker_name(text_config, name, font);
        }

        // Draw text box with typewriter effect
        draw_text_box_typewriter(text_config, &interpolated_text, font, char_limit);

        // Only show continue indicator when text is complete
        if ctx.typewriter_state.is_complete() {
            draw_continue_indicator_with_font(text_config, font);
        }
    }

    // Draw backlog overlay if enabled
    if ctx.show_backlog {
        let history: Vec<_> = state.history().iter().cloned().collect();
        draw_backlog(
            &ctx.backlog_config,
            &mut ctx.backlog_state,
            &history,
            &ctx.language_config,
        );
    } else {
        // Skip mode: S key toggle or Ctrl key held down
        let skip_active = input.is_skip_active(ctx.skip_mode);

        // Auto mode timer (only counts when text is complete)
        let mut auto_advance = false;
        if ctx.auto_mode && ctx.typewriter_state.is_complete() {
            ctx.auto_timer += get_frame_time() as f64;
            // Wait time based on text length, adjusted by auto speed setting
            // Higher speed = shorter wait time
            let base_wait = 2.0 + resolved_text.len() as f64 * 0.05;
            let wait_time = base_wait / ctx.settings.auto_speed as f64;
            if ctx.auto_timer >= wait_time {
                auto_advance = true;
                ctx.auto_timer = 0.0;
            }
        }

        // Handle click/Advance keybind
        let input_pressed = input.advance_pressed;

        if skip_active || auto_advance {
            // Check if we can skip (skip_unread=true or text is read)
            let can_skip = ctx.settings.skip_unread
                || ctx.read_state.is_read(SCENARIO_PATH, state.current_index());

            if can_skip || auto_advance {
                // Skip mode and auto mode bypass typewriter
                ctx.typewriter_state.complete();
                // In NVL mode, add completed text to buffer before advancing
                if is_nvl_mode {
                    ctx.nvl_state
                        .push(resolved_speaker.clone(), interpolated_text.clone());
                }
                ctx.read_state
                    .mark_read(SCENARIO_PATH, state.current_index());
                state.advance();
                ctx.auto_timer = 0.0;
            } else {
                // Stop skip mode on unread text
                ctx.skip_mode = false;
                eprintln!("Skip mode stopped (unread text)");
            }
        } else if input_pressed {
            if ctx.typewriter_state.is_complete() {
                // Text is complete, advance to next
                // In NVL mode, add completed text to buffer before advancing
                if is_nvl_mode {
                    ctx.nvl_state
                        .push(resolved_speaker.clone(), interpolated_text.clone());
                }
                ctx.read_state
                    .mark_read(SCENARIO_PATH, state.current_index());
                state.advance();
                ctx.auto_timer = 0.0;
            } else {
                // Text is still animating, complete it instantly
                ctx.typewriter_state.complete();
            }
        }
    }

    // Draw mode indicators
    let mut indicator_y = 20.0;
    if ctx.skip_mode {
        draw_text(
            "SKIP",
            750.0,
            indicator_y,
            20.0,
            Color::new(1.0, 0.5, 0.5, 1.0),
        );
        indicator_y += 22.0;
    }
    if ctx.auto_mode {
        draw_text("AUTO", 750.0, indicator_y, 20.0, YELLOW);
    }
    if is_nvl_mode {
        draw_text(
            "NVL",
            750.0,
            indicator_y + 22.0,
            20.0,
            Color::new(0.5, 1.0, 0.5, 1.0),
        );
    }
}

/// Handle DisplayState::Choices
#[allow(clippy::too_many_arguments)]
pub async fn handle_choices(
    ctx: &mut GameContext,
    state: &mut GameState,
    speaker: Option<LocalizedString>,
    text: LocalizedString,
    choices: Vec<Choice>,
    visual: VisualState,
    timeout: Option<f32>,
    default_choice: Option<usize>,
    text_config: &TextBoxConfig,
    input: &DetectedInput,
    camera_transform: &CameraTransform,
    shake_offset: (f32, f32),
    font: Option<&Font>,
) {
    // Auto-stop skip mode at choices
    if ctx.skip_mode {
        ctx.skip_mode = false;
        eprintln!("Skip mode OFF (reached choices)");
    }

    // Draw visuals first with shake offset and camera
    push_camera_transform(camera_transform);
    draw_visual(
        &visual,
        &mut ctx.texture_cache,
        shake_offset,
        &ctx.char_anim_state,
        &ctx.char_idle_state,
        &ctx.char_anim_states,
        &ctx.char_idle_states,
        &ctx.modular_char_defs,
        &ctx.video_bg_state,
    )
    .await;
    pop_camera_transform(camera_transform);

    // Resolve localized text
    let resolved_text = ctx.language_config.resolve(&text);

    // Interpolate variables in text
    let interpolated_text = interpolate_variables(&resolved_text, state.variables());

    // Draw speaker name if present (also interpolate variables)
    if let Some(ref name) = speaker {
        let resolved_name = ctx.language_config.resolve(name);
        let interpolated_name = interpolate_variables(&resolved_name, state.variables());
        draw_speaker_name(text_config, &interpolated_name, font);
    }

    // Reset typewriter if text changed
    if ctx.last_text.as_ref() != Some(&interpolated_text) {
        let total_chars = count_visible_chars(&interpolated_text);
        ctx.typewriter_state.reset(total_chars);
        ctx.last_text = Some(interpolated_text.clone());
        // Reset choice timer and navigation state when text changes
        ctx.choice_timer = timeout;
        ctx.choice_total_time = timeout;
        ctx.choice_nav_state = Default::default();
    }

    // Update typewriter state
    let char_limit = ctx.typewriter_state.update(ctx.settings.text_speed);

    // Draw text box with typewriter effect
    draw_text_box_typewriter(text_config, &interpolated_text, font, char_limit);

    // Draw backlog overlay if enabled
    if ctx.show_backlog {
        let history: Vec<_> = state.history().iter().cloned().collect();
        draw_backlog(
            &ctx.backlog_config,
            &mut ctx.backlog_state,
            &history,
            &ctx.language_config,
        );
    } else {
        // Only show choices when text is complete
        if ctx.typewriter_state.is_complete() {
            let choice_count = choices.len();

            // Update choice timer
            if let Some(ref mut remaining) = ctx.choice_timer {
                *remaining -= get_frame_time();

                // Check if timer expired
                if *remaining <= 0.0 {
                    // Auto-select default choice
                    if let Some(idx) = default_choice {
                        ctx.read_state
                            .mark_read(SCENARIO_PATH, state.current_index());
                        state.select_choice(idx);
                        ctx.choice_timer = None;
                        ctx.choice_total_time = None;
                        ctx.choice_nav_state = Default::default();
                    }
                }
            }

            // --- Input mode switching and navigation ---
            ctx.choice_nav_state.stick_debounce -= get_frame_time();

            if let Some(nav_action) = &input.choice_nav {
                match nav_action {
                    ChoiceNavAction::MouseMoved => {
                        ctx.choice_nav_state.input_source = InputSource::Mouse;
                        ctx.choice_nav_state.focus_index = None;
                        ctx.last_mouse_pos = mouse_position();
                    }
                    ChoiceNavAction::Up | ChoiceNavAction::Down => {
                        ctx.choice_nav_state.input_source = InputSource::Gamepad;
                        if ctx.choice_nav_state.focus_index.is_none() {
                            ctx.choice_nav_state.focus_index = Some(0);
                        }
                        if let Some(idx) = ctx.choice_nav_state.focus_index {
                            if *nav_action == ChoiceNavAction::Up {
                                ctx.choice_nav_state.focus_index = Some(idx.saturating_sub(1));
                            } else {
                                ctx.choice_nav_state.focus_index =
                                    Some((idx + 1).min(choice_count - 1));
                            }
                            ctx.choice_nav_state.stick_debounce = 0.2;
                        }
                    }
                    ChoiceNavAction::Confirm => {}
                }
            }

            // Calculate remaining time relative to total for progress bar
            let remaining_time = ctx.choice_timer.map(|t| t.max(0.0));

            let result = draw_choices_with_timer(
                &ctx.choice_config,
                &choices,
                remaining_time,
                default_choice,
                &ctx.language_config,
                &ctx.choice_nav_state,
            );

            // --- Selection confirmation ---
            let selected_index = if let Some(index) = result.selected {
                // Mouse click
                Some(index)
            } else if ctx.choice_nav_state.input_source == InputSource::Gamepad
                && input.choice_nav == Some(ChoiceNavAction::Confirm)
                && let Some(idx) = ctx.choice_nav_state.focus_index
            {
                // Gamepad A button
                Some(idx)
            } else {
                None
            };

            if let Some(index) = selected_index {
                ctx.read_state
                    .mark_read(SCENARIO_PATH, state.current_index());
                state.select_choice(index);
                ctx.choice_timer = None;
                ctx.choice_total_time = None;
                ctx.choice_nav_state = Default::default();
            }
        } else {
            // Click to complete text
            if input.advance_pressed {
                ctx.typewriter_state.complete();
            }
        }
    }
}

/// Handle DisplayState::Wait
pub async fn handle_wait(
    ctx: &mut GameContext,
    state: &mut GameState,
    duration: f32,
    visual: VisualState,
    input: &DetectedInput,
    camera_transform: &CameraTransform,
    shake_offset: (f32, f32),
) {
    // Draw visuals with shake offset and camera
    push_camera_transform(camera_transform);
    draw_visual(
        &visual,
        &mut ctx.texture_cache,
        shake_offset,
        &ctx.char_anim_state,
        &ctx.char_idle_state,
        &ctx.char_anim_states,
        &ctx.char_idle_states,
        &ctx.modular_char_defs,
        &ctx.video_bg_state,
    )
    .await;
    pop_camera_transform(camera_transform);

    // Reset wait timer if just started waiting
    if !ctx.in_wait {
        ctx.in_wait = true;
        ctx.wait_timer = 0.0;
    }

    // Update wait timer
    ctx.wait_timer += get_frame_time();

    // Check if wait is complete or skipped
    let skip_active = input.is_skip_active(ctx.skip_mode);

    if ctx.wait_timer >= duration || skip_active || input.advance_pressed {
        ctx.in_wait = false;
        ctx.wait_timer = 0.0;
        ctx.read_state
            .mark_read(SCENARIO_PATH, state.current_index());
        state.advance();
    }

    // Draw mode indicators
    let mut indicator_y = 20.0;
    if ctx.skip_mode {
        draw_text(
            "SKIP",
            750.0,
            indicator_y,
            20.0,
            Color::new(1.0, 0.5, 0.5, 1.0),
        );
        indicator_y += 22.0;
    }
    if ctx.auto_mode {
        draw_text("AUTO", 750.0, indicator_y, 20.0, YELLOW);
    }
}

/// Handle DisplayState::Input
pub async fn handle_input(
    ctx: &mut GameContext,
    state: &mut GameState,
    input_cmd: Input,
    visual: VisualState,
    camera_transform: &CameraTransform,
    shake_offset: (f32, f32),
    font: Option<&Font>,
) {
    // Draw visuals with shake offset and camera
    push_camera_transform(camera_transform);
    draw_visual(
        &visual,
        &mut ctx.texture_cache,
        shake_offset,
        &ctx.char_anim_state,
        &ctx.char_idle_state,
        &ctx.char_anim_states,
        &ctx.char_idle_states,
        &ctx.modular_char_defs,
        &ctx.video_bg_state,
    )
    .await;
    pop_camera_transform(camera_transform);

    // Initialize input state if this is a new input command
    if ctx.awaiting_input.as_ref() != Some(&input_cmd.var) {
        ctx.awaiting_input = Some(input_cmd.var.clone());
        ctx.input_state.reset(input_cmd.default.as_deref());
    }

    // Draw input dialog
    let result = draw_input(
        &ctx.input_config,
        &mut ctx.input_state,
        input_cmd.prompt.as_deref(),
        font,
    );

    if result.submitted {
        // Store input value as variable
        let value = Value::String(ctx.input_state.text.clone());
        state.set_variable(&input_cmd.var, value);
        ctx.awaiting_input = None;
        ctx.read_state
            .mark_read(SCENARIO_PATH, state.current_index());
        state.advance();
    } else if result.cancelled {
        // Use default value or empty string
        let default_value = input_cmd.default.clone().unwrap_or_default();
        let value = Value::String(default_value);
        state.set_variable(&input_cmd.var, value);
        ctx.awaiting_input = None;
        ctx.read_state
            .mark_read(SCENARIO_PATH, state.current_index());
        state.advance();
    }
}

/// Handle DisplayState::Video
pub async fn handle_video(
    ctx: &mut GameContext,
    state: &mut GameState,
    path: String,
    skippable: bool,
    loop_video: bool,
    input: &DetectedInput,
) {
    // Start video playback if not already playing
    if !ctx.video_state.is_playing() && !ctx.video_state.is_finished() {
        // Fade out BGM before video starts
        ctx.audio_manager.stop_bgm_fade(0.5).await;

        if let Err(e) = ctx.video_state.start(&path, skippable, loop_video) {
            eprintln!("Failed to start video: {}", e);
            // Skip to next command on error
            state.advance();
        }
    }

    // Update and draw video
    ctx.video_state.update();
    ctx.video_state.draw();

    // Check for skip input
    let skip_pressed = input.advance_pressed || is_key_pressed(KeyCode::Escape);

    // Advance when video finishes or is skipped
    if ctx.video_state.is_finished() || (skip_pressed && ctx.video_state.can_skip()) {
        ctx.video_state.stop();
        ctx.read_state
            .mark_read(SCENARIO_PATH, state.current_index());
        state.advance();
    }
}

/// Handle DisplayState::End
/// Returns true if should return to title.
pub fn handle_end(
    ctx: &mut GameContext,
    state: &mut GameState,
    text_config: &TextBoxConfig,
    input: &DetectedInput,
    font: Option<&Font>,
) -> bool {
    draw_text_box_with_font(text_config, "[ End ]", font);

    // Draw backlog overlay if enabled
    if ctx.show_backlog {
        let history: Vec<_> = state.history().iter().cloned().collect();
        draw_backlog(
            &ctx.backlog_config,
            &mut ctx.backlog_state,
            &history,
            &ctx.language_config,
        );
    }

    // Return to title on click or Advance
    input.advance_pressed
}
