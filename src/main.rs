// Many public APIs are not used in main but are provided for external use or future features.
#![allow(dead_code)]

mod accessibility;
mod audio;
mod cache;
mod flowchart;
mod game;
mod hotreload;
mod i18n;
mod input;
mod platform;
mod render;
mod runtime;
mod scenario;
mod types;
mod video;

use std::collections::HashMap;

use macroquad::prelude::*;

use cache::TextureCache;
use flowchart::{build_flowchart, calculate_layout};
use game::{GameContext, GameMode, QUICK_SAVE_PATH, SCENARIO_PATH, window_conf};
use input::{GamepadAxis, GamepadButton, STICK_THRESHOLD};
use render::{
    ChapterSelectState, CharAnimationState, CharIdleState, InputSource, ParticleType,
    TitleMenuItem, VideoBackgroundState, calculate_camera_transform, count_nvl_chars,
    count_visible_chars, draw_achievement, draw_background_with_offset, draw_backlog,
    draw_chapter_select, draw_character_animated, draw_choices_with_timer,
    draw_continue_indicator_with_font, draw_debug, draw_flowchart, draw_gallery, draw_input,
    draw_modular_char, draw_nvl_text_box, draw_settings_screen, draw_speaker_name,
    draw_text_box_typewriter, draw_text_box_with_font, draw_title_screen, interpolate_variables,
    pop_camera_transform, push_camera_transform,
};
use runtime::{Action, CameraState, DisplayState, GameState, SaveData, VisualState};
use scenario::{CharPosition, ModularCharDef, load_scenario};

/// Draw visual elements (background and character) with shake offset and character animation.
async fn draw_visual(
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

/// Save a screenshot to the screenshots directory.
fn save_screenshot() {
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

#[macroquad::main(window_conf)]
async fn main() {
    // Initialize game context and font (font is separate to avoid borrow conflicts)
    let (mut ctx, custom_font) = match GameContext::new().await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to initialize game: {}", e);
            return;
        }
    };

    // Start with title screen
    let mut game_mode = GameMode::Title;

    // Font reference is separate from ctx to avoid borrow conflicts
    let font_ref = custom_font.as_ref();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        // Apply accessibility settings to text config
        let text_config = ctx.text_config();

        // Poll gamepad events at the start of each frame
        ctx.gamepad_state.poll();

        match game_mode {
            GameMode::Title => {
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
                    font_ref,
                );

                if let Some(item) = result.selected {
                    match item {
                        TitleMenuItem::NewGame => {
                            // Start new game
                            if let Err(e) = ctx.start_new_game() {
                                eprintln!("Failed to start new game: {}", e);
                            } else {
                                game_mode = GameMode::InGame;
                            }
                        }
                        TitleMenuItem::Continue => {
                            // Try to load from quick save first, then from slots
                            if let Some(loaded_state) = load_game() {
                                ctx.game_state = Some(loaded_state);
                                ctx.reset_game_state();
                                game_mode = GameMode::InGame;
                            } else {
                                // Try slots 1-3
                                for slot in 1..=3 {
                                    if let Some(loaded_state) = load_from_slot(slot) {
                                        ctx.game_state = Some(loaded_state);
                                        ctx.reset_game_state();
                                        game_mode = GameMode::InGame;
                                        break;
                                    }
                                }
                            }
                        }
                        TitleMenuItem::Chapters => {
                            game_mode = GameMode::Chapters;
                            ctx.chapter_select_state = ChapterSelectState::default();
                        }
                        TitleMenuItem::Gallery => {
                            game_mode = GameMode::Gallery;
                            ctx.gallery_state = Default::default();
                        }
                        TitleMenuItem::Settings => {
                            game_mode = GameMode::Settings;
                        }
                        TitleMenuItem::Quit => {
                            break;
                        }
                    }
                }

                // Exit on Escape
                if is_key_pressed(KeyCode::Escape) {
                    break;
                }

                // Screenshot
                if ctx.settings.keybinds.is_pressed(Action::Screenshot) {
                    save_screenshot();
                }

                next_frame().await;
                continue;
            }
            GameMode::Settings => {
                let result =
                    draw_settings_screen(&ctx.settings_config, &mut ctx.settings, font_ref);

                if result.back_pressed {
                    // Save settings when leaving
                    ctx.settings.save();
                    eprintln!("Settings saved");
                    game_mode = GameMode::Title;
                }

                // Apply volume settings to audio manager
                ctx.audio_manager.set_bgm_volume(ctx.settings.bgm_volume);
                ctx.audio_manager.set_se_volume(ctx.settings.se_volume);
                ctx.audio_manager.set_voice_volume(ctx.settings.voice_volume);

                // Screenshot
                if ctx.settings.keybinds.is_pressed(Action::Screenshot) {
                    save_screenshot();
                }

                next_frame().await;
                continue;
            }
            GameMode::Gallery => {
                let images = ctx.unlocks.unlocked_images();
                let result = draw_gallery(
                    &ctx.gallery_config,
                    &mut ctx.gallery_state,
                    &images,
                    ctx.texture_cache.as_map(),
                    font_ref,
                );

                if result.back_pressed {
                    game_mode = GameMode::Title;
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
                    save_screenshot();
                }

                next_frame().await;
                continue;
            }
            GameMode::Chapters => {
                let result = draw_chapter_select(
                    &ctx.chapter_select_config,
                    &mut ctx.chapter_select_state,
                    &ctx.chapter_manager,
                    font_ref,
                );

                if result.back_pressed {
                    game_mode = GameMode::Title;
                }

                if let Some(chapter_id) = result.selected {
                    // Start game from the chapter's start label
                    if let Some(chapter) = ctx.chapter_manager.get_chapter(&chapter_id) {
                        let start_label = chapter.start_label.clone();
                        if let Err(e) = ctx.start_from_chapter(&start_label) {
                            eprintln!("Failed to start chapter: {}", e);
                        } else {
                            game_mode = GameMode::InGame;
                        }
                    }
                }

                // Screenshot
                if ctx.settings.keybinds.is_pressed(Action::Screenshot) {
                    save_screenshot();
                }

                next_frame().await;
                continue;
            }
            GameMode::Flowchart => {
                // Build flowchart if dirty or not cached
                if ctx.flowchart_state.dirty || ctx.flowchart_cache.is_none() {
                    let fc = build_flowchart(&ctx.scenario);
                    let layouts = calculate_layout(&fc, &ctx.layout_config);
                    ctx.flowchart_cache = Some((fc, layouts));
                    ctx.flowchart_state.dirty = false;
                }

                let (fc, layouts) = ctx.flowchart_cache.as_ref().unwrap();
                let current_idx = ctx.game_state.as_ref().map(|s| s.current_index());

                let result = draw_flowchart(
                    &ctx.flowchart_config,
                    &mut ctx.flowchart_state,
                    fc,
                    layouts,
                    current_idx,
                    font_ref,
                );

                if result.back_pressed {
                    game_mode = if ctx.game_state.is_some() {
                        GameMode::InGame
                    } else {
                        GameMode::Title
                    };
                }

                // Screenshot
                if ctx.settings.keybinds.is_pressed(Action::Screenshot) {
                    save_screenshot();
                }

                next_frame().await;
                continue;
            }
            GameMode::InGame => {
                // In-game logic follows
            }
        }

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

        // Get mutable reference to game state
        let state = match ctx.game_state.as_mut() {
            Some(s) => s,
            None => {
                game_mode = GameMode::Title;
                continue;
            }
        };

        // Flag to return to title screen (set later, processed at end of loop)
        let mut return_to_title = false;

        // Handle save/load
        // QuickSave / QuickLoad keybinds
        // Shift+1-0 = save to slot, 1-0 = load from slot
        if ctx.settings
            .keybinds
            .is_pressed_with_gamepad(Action::QuickSave, &ctx.gamepad_state)
        {
            save_game(state);
        }
        if ctx.settings
            .keybinds
            .is_pressed_with_gamepad(Action::QuickLoad, &ctx.gamepad_state)
            && let Some(loaded_state) = load_game()
        {
            *state = loaded_state;
            ctx.last_index = None; // Force audio/transition update
        }

        // Slot save/load (1-9, 0=10)
        let shift_held = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let slot_keys = [
            (KeyCode::Key1, 1),
            (KeyCode::Key2, 2),
            (KeyCode::Key3, 3),
            (KeyCode::Key4, 4),
            (KeyCode::Key5, 5),
            (KeyCode::Key6, 6),
            (KeyCode::Key7, 7),
            (KeyCode::Key8, 8),
            (KeyCode::Key9, 9),
            (KeyCode::Key0, 10),
        ];

        for (key, slot) in slot_keys {
            if is_key_pressed(key) {
                if shift_held {
                    save_to_slot(state, slot);
                } else if SaveData::slot_exists(slot) {
                    if let Some(loaded_state) = load_from_slot(slot) {
                        *state = loaded_state;
                        ctx.last_index = None; // Force audio/transition update
                    }
                } else {
                    eprintln!("Slot {} is empty", slot);
                }
            }
        }

        // Toggle backlog
        if ctx.settings
            .keybinds
            .is_pressed_with_gamepad(Action::Backlog, &ctx.gamepad_state)
        {
            ctx.show_backlog = !ctx.show_backlog;
            ctx.backlog_state = Default::default();
        }

        // Toggle auto mode
        if ctx.settings
            .keybinds
            .is_pressed_with_gamepad(Action::AutoMode, &ctx.gamepad_state)
        {
            ctx.auto_mode = !ctx.auto_mode;
            ctx.auto_timer = 0.0;
            if ctx.auto_mode {
                eprintln!("Auto mode ON");
            } else {
                eprintln!("Auto mode OFF");
            }
        }

        // Toggle skip mode
        if ctx.settings
            .keybinds
            .is_pressed_with_gamepad(Action::SkipMode, &ctx.gamepad_state)
        {
            ctx.skip_mode = !ctx.skip_mode;
            if ctx.skip_mode {
                eprintln!("Skip mode ON");
            } else {
                eprintln!("Skip mode OFF");
            }
        }

        // Toggle debug console
        if ctx.settings
            .keybinds
            .is_pressed_with_gamepad(Action::Debug, &ctx.gamepad_state)
        {
            ctx.debug_state.toggle();
        }

        // Open flowchart with F key (debug feature)
        if is_key_pressed(KeyCode::F) {
            game_mode = GameMode::Flowchart;
            ctx.flowchart_state.dirty = true;
        }

        // Handle rollback (keybind or mouse wheel up) - only when backlog is not shown
        if !ctx.show_backlog {
            let wheel = mouse_wheel();
            if (ctx.settings
                .keybinds
                .is_pressed_with_gamepad(Action::Rollback, &ctx.gamepad_state)
                || wheel.1 > 0.0)
                && state.can_rollback()
            {
                state.rollback();
            }
        }

        // Update audio, transition, and unlock images when command changes
        let current_index = state.current_index();
        if ctx.last_index != Some(current_index) {
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
                        ctx.char_anim_states.get_mut(&pos).unwrap().start_enter(enter);
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
                    pan_x: camera_cmd.pan.as_ref().map(|p| p.x).unwrap_or(ctx.camera_state.pan_x),
                    pan_y: camera_cmd.pan.as_ref().map(|p| p.y).unwrap_or(ctx.camera_state.pan_y),
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

            ctx.last_index = Some(current_index);
        }

        // Update transition state
        ctx.transition_state.update();

        // Update shake state
        ctx.shake_state.update();

        // Update character animation state
        ctx.char_anim_state.update();

        // Check if enter animation just completed and start pending idle
        if !ctx.char_anim_state.is_active()
            && ctx.char_anim_state.direction() == Some(render::character::AnimationDirection::Enter)
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
        let completed_positions: Vec<CharPosition> = ctx.char_anim_states
            .iter()
            .filter(|(_, anim_state)| {
                !anim_state.is_active()
                    && anim_state.direction() == Some(render::character::AnimationDirection::Enter)
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

        // Get shake offset for visual rendering
        let shake_offset = ctx.shake_state.offset();

        // Calculate camera transform
        let camera_transform = calculate_camera_transform(
            &ctx.camera_state,
            screen_width(),
            screen_height(),
        );

        match state.display_state() {
            DisplayState::Text {
                speaker,
                text,
                visual,
            } => {
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
                push_camera_transform(&camera_transform);
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
                pop_camera_transform(&camera_transform);

                // Resolve localized text
                let resolved_text = ctx.language_config.resolve(&text);

                // Interpolate variables in text
                let interpolated_text = interpolate_variables(&resolved_text, state.variables());

                // Resolve speaker name
                let resolved_speaker = speaker
                    .as_ref()
                    .map(|name| interpolate_variables(&ctx.language_config.resolve(name), state.variables()));

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
                        font_ref,
                        char_limit,
                    );
                } else {
                    // ADV mode: draw speaker name if present
                    if let Some(ref name) = resolved_speaker {
                        draw_speaker_name(&text_config, name, font_ref);
                    }

                    // Draw text box with typewriter effect
                    draw_text_box_typewriter(&text_config, &interpolated_text, font_ref, char_limit);

                    // Only show continue indicator when text is complete
                    if ctx.typewriter_state.is_complete() {
                        draw_continue_indicator_with_font(&text_config, font_ref);
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
                    let skip_active = ctx.skip_mode
                        || is_key_down(KeyCode::LeftControl)
                        || is_key_down(KeyCode::RightControl);

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
                    let input_pressed = is_mouse_button_pressed(MouseButton::Left)
                        || ctx.settings
                            .keybinds
                            .is_pressed_with_gamepad(Action::Advance, &ctx.gamepad_state);

                    if skip_active || auto_advance {
                        // Check if we can skip (skip_unread=true or text is read)
                        let can_skip = ctx.settings.skip_unread
                            || ctx.read_state.is_read(SCENARIO_PATH, state.current_index());

                        if can_skip || auto_advance {
                            // Skip mode and auto mode bypass typewriter
                            ctx.typewriter_state.complete();
                            // In NVL mode, add completed text to buffer before advancing
                            if is_nvl_mode {
                                ctx.nvl_state.push(resolved_speaker.clone(), interpolated_text.clone());
                            }
                            ctx.read_state.mark_read(SCENARIO_PATH, state.current_index());
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
                                ctx.nvl_state.push(resolved_speaker.clone(), interpolated_text.clone());
                            }
                            ctx.read_state.mark_read(SCENARIO_PATH, state.current_index());
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
                    draw_text("NVL", 750.0, indicator_y + 22.0, 20.0, Color::new(0.5, 1.0, 0.5, 1.0));
                }
            }
            DisplayState::Choices {
                speaker,
                text,
                choices,
                visual,
                timeout,
                default_choice,
            } => {
                // Auto-stop skip mode at choices
                if ctx.skip_mode {
                    ctx.skip_mode = false;
                    eprintln!("Skip mode OFF (reached choices)");
                }

                // Draw visuals first with shake offset and camera
                push_camera_transform(&camera_transform);
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
                pop_camera_transform(&camera_transform);

                // Resolve localized text
                let resolved_text = ctx.language_config.resolve(&text);

                // Interpolate variables in text
                let interpolated_text = interpolate_variables(&resolved_text, state.variables());

                // Draw speaker name if present (also interpolate variables)
                if let Some(ref name) = speaker {
                    let resolved_name = ctx.language_config.resolve(name);
                    let interpolated_name =
                        interpolate_variables(&resolved_name, state.variables());
                    draw_speaker_name(&text_config, &interpolated_name, font_ref);
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
                draw_text_box_typewriter(&text_config, &interpolated_text, font_ref, char_limit);

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
                                    ctx.read_state.mark_read(SCENARIO_PATH, state.current_index());
                                    state.select_choice(idx);
                                    ctx.choice_timer = None;
                                    ctx.choice_total_time = None;
                                    ctx.choice_nav_state = Default::default();
                                }
                            }
                        }

                        // --- Input mode switching ---
                        let mouse_pos = mouse_position();
                        if mouse_pos != ctx.last_mouse_pos {
                            // Mouse moved, switch to mouse mode
                            ctx.choice_nav_state.input_source = InputSource::Mouse;
                            ctx.choice_nav_state.focus_index = None;
                            ctx.last_mouse_pos = mouse_pos;
                        }

                        // --- Gamepad input ---
                        let dpad_up = ctx.gamepad_state.is_button_pressed(GamepadButton::DPadUp);
                        let dpad_down = ctx.gamepad_state.is_button_pressed(GamepadButton::DPadDown);
                        let stick_y = ctx.gamepad_state.axis(GamepadAxis::LeftY);

                        // Stick debounce processing
                        ctx.choice_nav_state.stick_debounce -= get_frame_time();
                        let stick_up =
                            stick_y < -STICK_THRESHOLD && ctx.choice_nav_state.stick_debounce <= 0.0;
                        let stick_down =
                            stick_y > STICK_THRESHOLD && ctx.choice_nav_state.stick_debounce <= 0.0;

                        if dpad_up || dpad_down || stick_up || stick_down {
                            // Switch to gamepad mode
                            ctx.choice_nav_state.input_source = InputSource::Gamepad;

                            // Initialize focus if not set
                            if ctx.choice_nav_state.focus_index.is_none() {
                                ctx.choice_nav_state.focus_index = Some(0);
                            }

                            // Move focus
                            if let Some(idx) = ctx.choice_nav_state.focus_index {
                                if dpad_up || stick_up {
                                    ctx.choice_nav_state.focus_index = Some(idx.saturating_sub(1));
                                    ctx.choice_nav_state.stick_debounce = 0.2;
                                } else if dpad_down || stick_down {
                                    ctx.choice_nav_state.focus_index =
                                        Some((idx + 1).min(choice_count - 1));
                                    ctx.choice_nav_state.stick_debounce = 0.2;
                                }
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
                        // Mouse click
                        if let Some(index) = result.selected {
                            ctx.read_state.mark_read(SCENARIO_PATH, state.current_index());
                            state.select_choice(index);
                            ctx.choice_timer = None;
                            ctx.choice_total_time = None;
                            ctx.choice_nav_state = Default::default();
                        } else if ctx.choice_nav_state.input_source == InputSource::Gamepad {
                            // Gamepad A button
                            if ctx.gamepad_state.is_button_pressed(GamepadButton::A)
                                && let Some(idx) = ctx.choice_nav_state.focus_index
                            {
                                ctx.read_state.mark_read(SCENARIO_PATH, state.current_index());
                                state.select_choice(idx);
                                ctx.choice_timer = None;
                                ctx.choice_total_time = None;
                                ctx.choice_nav_state = Default::default();
                            }
                        }
                    } else {
                        // Click to complete text
                        if is_mouse_button_pressed(MouseButton::Left)
                            || ctx.settings
                                .keybinds
                                .is_pressed_with_gamepad(Action::Advance, &ctx.gamepad_state)
                        {
                            ctx.typewriter_state.complete();
                        }
                    }
                }
            }
            DisplayState::Wait { duration, visual } => {
                // Draw visuals with shake offset and camera
                push_camera_transform(&camera_transform);
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
                pop_camera_transform(&camera_transform);

                // Reset wait timer if just started waiting
                if !ctx.in_wait {
                    ctx.in_wait = true;
                    ctx.wait_timer = 0.0;
                }

                // Update wait timer
                ctx.wait_timer += get_frame_time();

                // Check if wait is complete or skipped
                let skip_active = ctx.skip_mode
                    || is_key_down(KeyCode::LeftControl)
                    || is_key_down(KeyCode::RightControl);

                if ctx.wait_timer >= duration
                    || skip_active
                    || is_mouse_button_pressed(MouseButton::Left)
                    || ctx.settings
                        .keybinds
                        .is_pressed_with_gamepad(Action::Advance, &ctx.gamepad_state)
                {
                    ctx.in_wait = false;
                    ctx.wait_timer = 0.0;
                    ctx.read_state.mark_read(SCENARIO_PATH, state.current_index());
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
            DisplayState::Input { input, visual } => {
                // Draw visuals with shake offset and camera
                push_camera_transform(&camera_transform);
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
                pop_camera_transform(&camera_transform);

                // Initialize input state if this is a new input command
                if ctx.awaiting_input.as_ref() != Some(&input.var) {
                    ctx.awaiting_input = Some(input.var.clone());
                    ctx.input_state.reset(input.default.as_deref());
                }

                // Draw input dialog
                let result = draw_input(
                    &ctx.input_config,
                    &mut ctx.input_state,
                    input.prompt.as_deref(),
                    font_ref,
                );

                if result.submitted {
                    // Store input value as variable
                    let value = runtime::Value::String(ctx.input_state.text.clone());
                    state.set_variable(&input.var, value);
                    ctx.awaiting_input = None;
                    ctx.read_state.mark_read(SCENARIO_PATH, state.current_index());
                    state.advance();
                } else if result.cancelled {
                    // Use default value or empty string
                    let default_value = input.default.clone().unwrap_or_default();
                    let value = runtime::Value::String(default_value);
                    state.set_variable(&input.var, value);
                    ctx.awaiting_input = None;
                    ctx.read_state.mark_read(SCENARIO_PATH, state.current_index());
                    state.advance();
                }
            }
            DisplayState::Video {
                path,
                skippable,
                loop_video,
                ..
            } => {
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
                let skip_pressed = is_mouse_button_pressed(MouseButton::Left)
                    || ctx.settings
                        .keybinds
                        .is_pressed_with_gamepad(Action::Advance, &ctx.gamepad_state)
                    || is_key_pressed(KeyCode::Escape);

                // Advance when video finishes or is skipped
                if ctx.video_state.is_finished() || (skip_pressed && ctx.video_state.can_skip()) {
                    ctx.video_state.stop();
                    ctx.read_state.mark_read(SCENARIO_PATH, state.current_index());
                    state.advance();
                }
            }
            DisplayState::End => {
                draw_text_box_with_font(&text_config, "[ End ]", font_ref);

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

                // Return to title on click or Advance, or exit on Escape
                if is_mouse_button_pressed(MouseButton::Left)
                    || ctx.settings
                        .keybinds
                        .is_pressed_with_gamepad(Action::Advance, &ctx.gamepad_state)
                {
                    return_to_title = true;
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
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
        draw_achievement(&ctx.achievement_config, &ctx.achievement_notifier, font_ref);

        // Draw debug overlay
        draw_debug(&ctx.debug_config, &ctx.debug_state, state, font_ref);

        // Draw transition overlay
        ctx.transition_state.draw();

        // Return to title on Escape (instead of exiting)
        if is_key_pressed(KeyCode::Escape) && !state.is_ended() {
            return_to_title = true;
        }

        // Screenshot
        if ctx.settings.keybinds.is_pressed(Action::Screenshot) {
            save_screenshot();
        }

        next_frame().await;

        // Process return to title (after frame, when state borrow is dropped)
        if return_to_title {
            game_mode = GameMode::Title;
            ctx.game_state = None;
        }
    }
}
