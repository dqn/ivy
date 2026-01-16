//! Command change handling and animation updates.

use macroquad::prelude::*;

use crate::game::GameContext;
use crate::render::{character::AnimationDirection, ParticleType};
use crate::runtime::{CameraState, DisplayState, GameState};
use crate::scenario::CharPosition;

/// Called when the current command index changes.
pub async fn on_command_change(ctx: &mut GameContext, state: &mut GameState) {
    // Unlock images that are displayed (for CG gallery)
    let display_state = state.display_state();
    match &display_state {
        DisplayState::Text { visual, .. }
        | DisplayState::Choices { visual, .. }
        | DisplayState::Wait { visual, .. }
        | DisplayState::Input { visual, .. }
        | DisplayState::Video { visual, .. } => {
            if let Some(bg) = &visual.background {
                ctx.unlocks.unlock_image(bg);
            }
            if let Some(ch) = &visual.character {
                ctx.unlocks.unlock_image(ch);
            }
            for char_state in &visual.characters {
                ctx.unlocks.unlock_image(&char_state.path);
            }
        }
        DisplayState::End => {}
    }

    // Update BGM
    ctx.audio_manager.update_bgm(state.current_bgm()).await;

    // Play SE
    ctx.audio_manager.play_se(state.current_se()).await;

    // Play voice
    ctx.audio_manager.play_voice(state.current_voice()).await;

    // Start ambient tracks
    for track in state.current_ambient() {
        ctx.audio_manager.start_ambient(track).await;
    }

    // Stop ambient tracks
    for stop in state.current_ambient_stop() {
        ctx.audio_manager.stop_ambient(stop);
    }

    // Start transition if specified
    if let Some(transition) = state.current_transition() {
        ctx.transition_state.start_with_config(
            transition.transition_type,
            transition.duration,
            transition.easing,
            transition.direction,
            transition.blinds_count,
            transition.max_pixel_size,
        );
    }

    // Start shake if specified
    if let Some(shake) = state.current_shake() {
        ctx.shake_state.start(shake);
    }

    // Start character enter animation if specified
    if let Some(char_enter) = state.current_char_enter() {
        ctx.char_anim_state.start_enter(char_enter);
        // Store pending idle animation to start after enter completes
        ctx.pending_idle = state.current_char_idle().cloned();
        ctx.char_idle_state.stop(); // Stop current idle during enter animation
    } else if let Some(char_idle) = state.current_char_idle() {
        // No enter animation, start idle directly
        ctx.char_idle_state.start(char_idle);
    }

    // Start character exit animation if specified
    if let Some(char_exit) = state.current_char_exit() {
        ctx.char_anim_state.start_exit(char_exit);
        ctx.char_idle_state.stop(); // Stop idle during exit animation
        ctx.pending_idle = None;
    }

    // Stop idle animation when character is cleared
    let display = state.display_state();
    let visual = match &display {
        DisplayState::Text { visual, .. } => Some(visual),
        DisplayState::Choices { visual, .. } => Some(visual),
        _ => None,
    };
    if let Some(visual) = visual
        && visual.character.is_none()
    {
        ctx.char_idle_state.stop();
        ctx.pending_idle = None;
    }

    // Handle multiple character animations
    if let Some(visual) = visual {
        // Track which positions are currently active
        let active_positions: std::collections::HashSet<_> =
            visual.characters.iter().map(|c| c.position).collect();

        // Clear animations for positions no longer in use
        for pos in [
            CharPosition::Left,
            CharPosition::Center,
            CharPosition::Right,
        ] {
            if !active_positions.contains(&pos) {
                if let Some(anim) = ctx.char_anim_states.get_mut(&pos) {
                    anim.reset();
                }
                if let Some(idle) = ctx.char_idle_states.get_mut(&pos) {
                    idle.stop();
                }
                ctx.pending_idles.remove(&pos);
            }
        }

        // Start animations for each character
        for char_state in &visual.characters {
            let pos = char_state.position;

            // Initialize state if not exists
            ctx.char_anim_states.entry(pos).or_default();
            ctx.char_idle_states.entry(pos).or_default();

            // Start enter animation if specified
            if let Some(enter) = &char_state.enter {
                ctx.char_anim_states
                    .get_mut(&pos)
                    .unwrap()
                    .start_enter(enter);
                // Store pending idle to start after enter completes
                if let Some(idle) = &char_state.idle {
                    ctx.pending_idles.insert(pos, idle.clone());
                }
                ctx.char_idle_states.get_mut(&pos).unwrap().stop();
            } else if let Some(idle) = &char_state.idle {
                // No enter animation, start idle directly
                ctx.char_idle_states.get_mut(&pos).unwrap().start(idle);
            }

            // Start exit animation if specified
            if let Some(exit) = &char_state.exit {
                ctx.char_anim_states.get_mut(&pos).unwrap().start_exit(exit);
                ctx.char_idle_states.get_mut(&pos).unwrap().stop();
                ctx.pending_idles.remove(&pos);
            }
        }
    }

    // Update particles if specified
    if let Some((particles, intensity)) = state.current_particles() {
        if particles.is_empty() {
            ctx.particle_state.stop();
        } else {
            let particle_type = ParticleType::from_str(particles);
            ctx.particle_state.set(particle_type, intensity);
        }
    }

    // Update cinematic bars if specified
    if let Some((enabled, duration)) = state.current_cinematic() {
        ctx.cinematic_state.set(enabled, duration);
    }

    // Unlock achievement if specified
    if let Some(achievement) = state.current_achievement()
        && ctx.achievements.unlock(&achievement.id)
    {
        ctx.achievement_notifier.notify(
            &achievement.id,
            &achievement.name,
            &achievement.description,
        );
        eprintln!("Achievement unlocked: {}", achievement.name);
    }

    // Start camera animation if specified
    if let Some(camera_cmd) = state.current_camera() {
        let target = CameraState {
            pan_x: camera_cmd
                .pan
                .as_ref()
                .map(|p| p.x)
                .unwrap_or(ctx.camera_state.pan_x),
            pan_y: camera_cmd
                .pan
                .as_ref()
                .map(|p| p.y)
                .unwrap_or(ctx.camera_state.pan_y),
            zoom: camera_cmd.zoom.unwrap_or(ctx.camera_state.zoom),
            tilt: camera_cmd.tilt.unwrap_or(ctx.camera_state.tilt),
            focus: camera_cmd.focus,
        };
        ctx.camera_anim_state.start(
            ctx.camera_state.clone(),
            target,
            camera_cmd.duration,
            camera_cmd.easing,
        );
    }

    // Start or stop video background
    if let Some(video_bg) = state.current_video_bg() {
        if video_bg.path.is_empty() {
            ctx.video_bg_state.stop();
        } else if let Err(e) = ctx.video_bg_state.start(&video_bg.path, video_bg.looped) {
            eprintln!("Failed to start video background: {}", e);
        }
    }

    // Reset auto timer on command change
    ctx.auto_timer = 0.0;
}

/// Update all animation states each frame.
pub fn update_animations(ctx: &mut GameContext) {
    // Update transition state
    ctx.transition_state.update();

    // Update shake state
    ctx.shake_state.update();

    // Update character animation state
    ctx.char_anim_state.update();

    // Check if enter animation just completed and start pending idle
    if !ctx.char_anim_state.is_active()
        && ctx.char_anim_state.direction() == Some(AnimationDirection::Enter)
        && let Some(idle) = ctx.pending_idle.take()
    {
        ctx.char_idle_state.start(&idle);
    }

    // Update idle animation state
    ctx.char_idle_state.update();

    // Update multiple character animation states
    for anim_state in ctx.char_anim_states.values_mut() {
        anim_state.update();
    }

    // Check if enter animations completed and start pending idles
    let completed_positions: Vec<CharPosition> = ctx
        .char_anim_states
        .iter()
        .filter(|(_, anim_state)| {
            !anim_state.is_active()
                && anim_state.direction() == Some(AnimationDirection::Enter)
        })
        .map(|(pos, _)| *pos)
        .collect();

    for pos in completed_positions {
        if let Some(idle) = ctx.pending_idles.remove(&pos)
            && let Some(idle_state) = ctx.char_idle_states.get_mut(&pos)
        {
            idle_state.start(&idle);
        }
    }

    // Update multiple character idle animation states
    for idle_state in ctx.char_idle_states.values_mut() {
        idle_state.update();
    }

    // Update camera animation state
    ctx.camera_anim_state.update(get_frame_time());
    ctx.camera_state = ctx.camera_anim_state.current();

    // Update video background state
    ctx.video_bg_state.update();
}
