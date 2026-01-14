// Many public APIs are not used in main but are provided for external use or future features.
#![allow(dead_code)]

mod audio;
mod cache;
mod flowchart;
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
use i18n::LanguageConfig;
use input::{GamepadAxis, GamepadButton, GamepadState, STICK_THRESHOLD};

use audio::AudioManager;
use flowchart::{Flowchart, LayoutConfig, NodeId, NodeLayout, build_flowchart, calculate_layout};
use hotreload::HotReloader;
use render::{
    AchievementConfig, BacklogConfig, BacklogState, ChapterSelectConfig, ChapterSelectState,
    CharAnimationState, CharIdleState, ChoiceButtonConfig, ChoiceNavState, CinematicState,
    DebugConfig, DebugState, FlowchartConfig, FlowchartState, GalleryConfig, GalleryState,
    GameSettings, InputConfig, InputSource, InputState, NvlConfig, NvlState, ParticleState,
    ParticleType, SettingsConfig, ShakeState, TextBoxConfig, TitleConfig, TitleMenuItem,
    TransitionState, TypewriterState, VideoBackgroundState, VideoState, calculate_camera_transform,
    count_nvl_chars, count_visible_chars, draw_achievement, draw_background_with_offset,
    draw_backlog, draw_chapter_select, draw_character_animated, draw_choices_with_timer,
    draw_continue_indicator_with_font, draw_debug, draw_flowchart, draw_gallery, draw_input,
    draw_modular_char, draw_nvl_text_box, draw_settings_screen, draw_speaker_name,
    draw_text_box_typewriter, draw_text_box_with_font, draw_title_screen, interpolate_variables,
    pop_camera_transform, push_camera_transform,
};
use runtime::{
    AchievementNotifier, Achievements, Action, CameraAnimationState, CameraState, Chapter,
    ChapterManager, DisplayState, GameState, ReadState, SaveData, Unlocks, VisualState,
};
use scenario::{CharPosition, ModularCharDef, load_scenario};

/// Game mode: title screen, settings, gallery, chapters, flowchart, or in-game.
#[derive(Debug, Clone, Copy, PartialEq)]
enum GameMode {
    Title,
    Settings,
    Gallery,
    Chapters,
    Flowchart,
    InGame,
}

const SCENARIO_PATH: &str = "assets/sample.yaml";
const QUICK_SAVE_PATH: &str = "saves/save.json";
const FONT_PATH: &str = "assets/fonts/NotoSansJP-Regular.ttf";

fn window_conf() -> Conf {
    Conf {
        window_title: "ivy".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

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
    // Load scenario
    let scenario = match load_scenario(SCENARIO_PATH) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load scenario: {}", e);
            return;
        }
    };

    // Initialize hot reloader for development
    let mut hot_reloader = match HotReloader::new() {
        Ok(mut reloader) => {
            if let Err(e) = reloader.watch(SCENARIO_PATH) {
                eprintln!("Failed to watch scenario file: {}", e);
            } else {
                eprintln!("[Hot Reload] Watching {}", SCENARIO_PATH);
            }
            Some(reloader)
        }
        Err(e) => {
            eprintln!("Hot reload disabled: {}", e);
            None
        }
    };

    let scenario_title = scenario.title.clone();
    let scenario_chapters: Vec<Chapter> = scenario
        .chapters
        .iter()
        .map(|c| Chapter {
            id: c.id.clone(),
            title: c.title.clone(),
            start_label: c.start_label.clone(),
            description: c.description.clone(),
        })
        .collect();
    let mut modular_char_defs = scenario.modular_characters.clone();
    eprintln!("Loaded scenario: {}", scenario_title);

    // Load custom font for Japanese text support
    let custom_font = match load_ttf_font(FONT_PATH).await {
        Ok(font) => {
            eprintln!("Loaded custom font from {}", FONT_PATH);
            Some(font)
        }
        Err(e) => {
            eprintln!("Custom font not found ({}), using default font", e);
            None
        }
    };
    let font_ref = custom_font.as_ref();

    // Load settings
    let mut settings = GameSettings::load();
    eprintln!(
        "Loaded settings: BGM={:.0}%, SE={:.0}%, Voice={:.0}%, Auto={:.1}x",
        settings.bgm_volume * 100.0,
        settings.se_volume * 100.0,
        settings.voice_volume * 100.0,
        settings.auto_speed
    );

    // Start with title screen
    let mut game_mode = GameMode::Title;
    let mut game_state: Option<GameState> = None;
    let title_config = TitleConfig::default();
    let settings_config = SettingsConfig::default();
    let text_config = TextBoxConfig::default();
    let choice_config = ChoiceButtonConfig::default();
    let backlog_config = BacklogConfig::default();
    let input_config = InputConfig::default();
    let gallery_config = GalleryConfig::default();
    let achievement_config = AchievementConfig::default();
    let debug_config = DebugConfig::default();
    let chapter_select_config = ChapterSelectConfig::default();
    let flowchart_config = FlowchartConfig::default();
    let layout_config = LayoutConfig::default();
    let mut backlog_state = BacklogState::default();
    let mut debug_state = DebugState::default();
    let mut input_state = InputState::default();
    let mut gallery_state = GalleryState::default();
    let mut chapter_select_state = ChapterSelectState::default();
    let mut flowchart_state = FlowchartState::new();
    let mut flowchart_cache: Option<(Flowchart, HashMap<NodeId, NodeLayout>)> = None;
    let mut chapter_manager = ChapterManager::new();
    chapter_manager.set_chapters(scenario_chapters);
    let mut unlocks = Unlocks::load();
    let mut achievements = Achievements::load();
    let mut read_state = ReadState::load();
    let mut achievement_notifier = AchievementNotifier::default();
    let mut awaiting_input: Option<String> = None; // Variable name waiting for input
    let language_config = LanguageConfig::default();
    let mut show_backlog = false;
    let mut texture_cache = TextureCache::new();
    let mut audio_manager = AudioManager::new();
    let mut gamepad_state = GamepadState::new();
    let mut last_index: Option<usize> = None;
    let mut auto_mode = false;
    let mut auto_timer = 0.0;
    let mut skip_mode = false;
    let mut transition_state = TransitionState::default();
    let mut shake_state = ShakeState::default();
    let mut typewriter_state = TypewriterState::default();
    let mut last_text: Option<String> = None;
    let mut wait_timer: f32 = 0.0;
    let mut in_wait: bool = false;
    let mut char_anim_state = CharAnimationState::default();
    let mut char_idle_state = CharIdleState::default();
    let mut pending_idle: Option<scenario::CharIdleAnimation> = None;
    // Position-based animation states for multi-character support
    let mut char_anim_states: HashMap<CharPosition, CharAnimationState> = HashMap::new();
    let mut char_idle_states: HashMap<CharPosition, CharIdleState> = HashMap::new();
    let mut pending_idles: HashMap<CharPosition, scenario::CharIdleAnimation> = HashMap::new();
    let mut particle_state = ParticleState::default();
    let mut cinematic_state = CinematicState::default();
    let mut video_state = VideoState::new();
    let mut video_bg_state = VideoBackgroundState::new();
    let mut nvl_state = NvlState::new();
    let nvl_config = NvlConfig::default();
    let mut choice_timer: Option<f32> = None;
    let mut _choice_total_time: Option<f32> = None;
    let mut choice_nav_state = ChoiceNavState::default();
    let mut last_mouse_pos: (f32, f32) = (0.0, 0.0);
    let mut camera_state = CameraState::default();
    let mut camera_anim_state = CameraAnimationState::default();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        // Poll gamepad events at the start of each frame
        gamepad_state.poll();

        match game_mode {
            GameMode::Title => {
                // Check if any save exists
                let has_save = SaveData::slot_exists(1)
                    || SaveData::slot_exists(2)
                    || SaveData::slot_exists(3)
                    || std::path::Path::new(QUICK_SAVE_PATH).exists();

                // Check if gallery has any unlocked images
                let has_gallery = unlocks.image_count() > 0;

                // Check if chapters are defined
                let has_chapters = chapter_manager.has_chapters();

                let result = draw_title_screen(
                    &title_config,
                    &scenario_title,
                    has_save,
                    has_chapters,
                    has_gallery,
                    font_ref,
                );

                if let Some(item) = result.selected {
                    match item {
                        TitleMenuItem::NewGame => {
                            // Start new game
                            let new_scenario = load_scenario(SCENARIO_PATH).unwrap();
                            game_state = Some(GameState::new(new_scenario));
                            game_mode = GameMode::InGame;
                            last_index = None;
                            auto_mode = false;
                            skip_mode = false;
                            show_backlog = false;
                        }
                        TitleMenuItem::Continue => {
                            // Try to load from quick save first, then from slots
                            if let Some(loaded_state) = load_game() {
                                game_state = Some(loaded_state);
                                game_mode = GameMode::InGame;
                                last_index = None;
                                auto_mode = false;
                                skip_mode = false;
                                show_backlog = false;
                            } else {
                                // Try slots 1-3
                                for slot in 1..=3 {
                                    if let Some(loaded_state) = load_from_slot(slot) {
                                        game_state = Some(loaded_state);
                                        game_mode = GameMode::InGame;
                                        last_index = None;
                                        auto_mode = false;
                                        skip_mode = false;
                                        show_backlog = false;
                                        break;
                                    }
                                }
                            }
                        }
                        TitleMenuItem::Chapters => {
                            game_mode = GameMode::Chapters;
                            chapter_select_state = ChapterSelectState::default();
                        }
                        TitleMenuItem::Gallery => {
                            game_mode = GameMode::Gallery;
                            gallery_state = GalleryState::default();
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
                if settings.keybinds.is_pressed(Action::Screenshot) {
                    save_screenshot();
                }

                next_frame().await;
                continue;
            }
            GameMode::Settings => {
                let result = draw_settings_screen(&settings_config, &mut settings, font_ref);

                if result.back_pressed {
                    // Save settings when leaving
                    settings.save();
                    eprintln!("Settings saved");
                    game_mode = GameMode::Title;
                }

                // Apply volume settings to audio manager
                audio_manager.set_bgm_volume(settings.bgm_volume);
                audio_manager.set_se_volume(settings.se_volume);
                audio_manager.set_voice_volume(settings.voice_volume);

                // Screenshot
                if settings.keybinds.is_pressed(Action::Screenshot) {
                    save_screenshot();
                }

                next_frame().await;
                continue;
            }
            GameMode::Gallery => {
                let images = unlocks.unlocked_images();
                let result = draw_gallery(
                    &gallery_config,
                    &mut gallery_state,
                    &images,
                    texture_cache.as_map(),
                    font_ref,
                );

                if result.back_pressed {
                    game_mode = GameMode::Title;
                }

                // Load textures for gallery images (async)
                for path in &images {
                    if !texture_cache.contains(path)
                        && let Ok(texture) = load_texture(path).await
                    {
                        texture.set_filter(FilterMode::Linear);
                        texture_cache.insert(path.clone(), texture);
                    }
                }

                // Screenshot
                if settings.keybinds.is_pressed(Action::Screenshot) {
                    save_screenshot();
                }

                next_frame().await;
                continue;
            }
            GameMode::Chapters => {
                let result = draw_chapter_select(
                    &chapter_select_config,
                    &mut chapter_select_state,
                    &chapter_manager,
                    font_ref,
                );

                if result.back_pressed {
                    game_mode = GameMode::Title;
                }

                if let Some(chapter_id) = result.selected {
                    // Start game from the chapter's start label
                    if let Some(chapter) = chapter_manager.get_chapter(&chapter_id) {
                        let new_scenario = load_scenario(SCENARIO_PATH).unwrap();
                        let mut new_state = GameState::new(new_scenario);
                        // Jump to the chapter's start label
                        new_state.jump_to_label(&chapter.start_label);
                        game_state = Some(new_state);
                        game_mode = GameMode::InGame;
                        last_index = None;
                        auto_mode = false;
                        skip_mode = false;
                        show_backlog = false;
                    }
                }

                // Screenshot
                if settings.keybinds.is_pressed(Action::Screenshot) {
                    save_screenshot();
                }

                next_frame().await;
                continue;
            }
            GameMode::Flowchart => {
                // Build flowchart if dirty or not cached
                if flowchart_state.dirty || flowchart_cache.is_none() {
                    let fc = build_flowchart(&scenario);
                    let layouts = calculate_layout(&fc, &layout_config);
                    flowchart_cache = Some((fc, layouts));
                    flowchart_state.dirty = false;
                }

                let (fc, layouts) = flowchart_cache.as_ref().unwrap();
                let current_idx = game_state.as_ref().map(|s| s.current_index());

                let result = draw_flowchart(
                    &flowchart_config,
                    &mut flowchart_state,
                    fc,
                    layouts,
                    current_idx,
                    font_ref,
                );

                if result.back_pressed {
                    game_mode = if game_state.is_some() {
                        GameMode::InGame
                    } else {
                        GameMode::Title
                    };
                }

                // Screenshot
                if settings.keybinds.is_pressed(Action::Screenshot) {
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
        if let Some(ref mut reloader) = hot_reloader
            && reloader.poll()
            && let Some(ref mut state) = game_state
        {
            match load_scenario(SCENARIO_PATH) {
                Ok(new_scenario) => {
                    modular_char_defs = new_scenario.modular_characters.clone();
                    state.reload_scenario(new_scenario);
                    last_index = None; // Force audio/transition update
                    flowchart_state.dirty = true; // Invalidate flowchart cache
                    eprintln!("[Hot Reload] Scenario reloaded");
                }
                Err(e) => {
                    eprintln!("[Hot Reload] Failed to reload: {}", e);
                }
            }
        }

        // Get mutable reference to game state
        let state = match game_state.as_mut() {
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
        if settings
            .keybinds
            .is_pressed_with_gamepad(Action::QuickSave, &gamepad_state)
        {
            save_game(state);
        }
        if settings
            .keybinds
            .is_pressed_with_gamepad(Action::QuickLoad, &gamepad_state)
            && let Some(loaded_state) = load_game()
        {
            *state = loaded_state;
            last_index = None; // Force audio/transition update
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
                        last_index = None; // Force audio/transition update
                    }
                } else {
                    eprintln!("Slot {} is empty", slot);
                }
            }
        }

        // Toggle backlog
        if settings
            .keybinds
            .is_pressed_with_gamepad(Action::Backlog, &gamepad_state)
        {
            show_backlog = !show_backlog;
            backlog_state = BacklogState::default();
        }

        // Toggle auto mode
        if settings
            .keybinds
            .is_pressed_with_gamepad(Action::AutoMode, &gamepad_state)
        {
            auto_mode = !auto_mode;
            auto_timer = 0.0;
            if auto_mode {
                eprintln!("Auto mode ON");
            } else {
                eprintln!("Auto mode OFF");
            }
        }

        // Toggle skip mode
        if settings
            .keybinds
            .is_pressed_with_gamepad(Action::SkipMode, &gamepad_state)
        {
            skip_mode = !skip_mode;
            if skip_mode {
                eprintln!("Skip mode ON");
            } else {
                eprintln!("Skip mode OFF");
            }
        }

        // Toggle debug console
        if settings
            .keybinds
            .is_pressed_with_gamepad(Action::Debug, &gamepad_state)
        {
            debug_state.toggle();
        }

        // Open flowchart with F key (debug feature)
        if is_key_pressed(KeyCode::F) {
            game_mode = GameMode::Flowchart;
            flowchart_state.dirty = true;
        }

        // Handle rollback (keybind or mouse wheel up) - only when backlog is not shown
        if !show_backlog {
            let wheel = mouse_wheel();
            if (settings
                .keybinds
                .is_pressed_with_gamepad(Action::Rollback, &gamepad_state)
                || wheel.1 > 0.0)
                && state.can_rollback()
            {
                state.rollback();
            }
        }

        // Update audio, transition, and unlock images when command changes
        let current_index = state.current_index();
        if last_index != Some(current_index) {
            // Unlock images that are displayed (for CG gallery)
            let display_state = state.display_state();
            match &display_state {
                DisplayState::Text { visual, .. }
                | DisplayState::Choices { visual, .. }
                | DisplayState::Wait { visual, .. }
                | DisplayState::Input { visual, .. }
                | DisplayState::Video { visual, .. } => {
                    if let Some(bg) = &visual.background {
                        unlocks.unlock_image(bg);
                    }
                    if let Some(ch) = &visual.character {
                        unlocks.unlock_image(ch);
                    }
                    for char_state in &visual.characters {
                        unlocks.unlock_image(&char_state.path);
                    }
                }
                DisplayState::End => {}
            }

            // Update BGM
            audio_manager.update_bgm(state.current_bgm()).await;

            // Play SE
            audio_manager.play_se(state.current_se()).await;

            // Play voice
            audio_manager.play_voice(state.current_voice()).await;

            // Start ambient tracks
            for track in state.current_ambient() {
                audio_manager.start_ambient(track).await;
            }

            // Stop ambient tracks
            for stop in state.current_ambient_stop() {
                audio_manager.stop_ambient(stop);
            }

            // Start transition if specified
            if let Some(transition) = state.current_transition() {
                transition_state.start_with_config(
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
                shake_state.start(shake);
            }

            // Start character enter animation if specified
            if let Some(char_enter) = state.current_char_enter() {
                char_anim_state.start_enter(char_enter);
                // Store pending idle animation to start after enter completes
                pending_idle = state.current_char_idle().cloned();
                char_idle_state.stop(); // Stop current idle during enter animation
            } else if let Some(char_idle) = state.current_char_idle() {
                // No enter animation, start idle directly
                char_idle_state.start(char_idle);
            }

            // Start character exit animation if specified
            if let Some(char_exit) = state.current_char_exit() {
                char_anim_state.start_exit(char_exit);
                char_idle_state.stop(); // Stop idle during exit animation
                pending_idle = None;
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
                char_idle_state.stop();
                pending_idle = None;
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
                        if let Some(anim) = char_anim_states.get_mut(&pos) {
                            anim.reset();
                        }
                        if let Some(idle) = char_idle_states.get_mut(&pos) {
                            idle.stop();
                        }
                        pending_idles.remove(&pos);
                    }
                }

                // Start animations for each character
                for char_state in &visual.characters {
                    let pos = char_state.position;

                    // Initialize state if not exists
                    char_anim_states.entry(pos).or_default();
                    char_idle_states.entry(pos).or_default();

                    // Start enter animation if specified
                    if let Some(enter) = &char_state.enter {
                        char_anim_states.get_mut(&pos).unwrap().start_enter(enter);
                        // Store pending idle to start after enter completes
                        if let Some(idle) = &char_state.idle {
                            pending_idles.insert(pos, idle.clone());
                        }
                        char_idle_states.get_mut(&pos).unwrap().stop();
                    } else if let Some(idle) = &char_state.idle {
                        // No enter animation, start idle directly
                        char_idle_states.get_mut(&pos).unwrap().start(idle);
                    }

                    // Start exit animation if specified
                    if let Some(exit) = &char_state.exit {
                        char_anim_states.get_mut(&pos).unwrap().start_exit(exit);
                        char_idle_states.get_mut(&pos).unwrap().stop();
                        pending_idles.remove(&pos);
                    }
                }
            }

            // Update particles if specified
            if let Some((particles, intensity)) = state.current_particles() {
                if particles.is_empty() {
                    particle_state.stop();
                } else {
                    let particle_type = ParticleType::from_str(particles);
                    particle_state.set(particle_type, intensity);
                }
            }

            // Update cinematic bars if specified
            if let Some((enabled, duration)) = state.current_cinematic() {
                cinematic_state.set(enabled, duration);
            }

            // Unlock achievement if specified
            if let Some(achievement) = state.current_achievement()
                && achievements.unlock(&achievement.id)
            {
                achievement_notifier.notify(
                    &achievement.id,
                    &achievement.name,
                    &achievement.description,
                );
                eprintln!("Achievement unlocked: {}", achievement.name);
            }

            // Start camera animation if specified
            if let Some(camera_cmd) = state.current_camera() {
                let target = CameraState {
                    pan_x: camera_cmd.pan.as_ref().map(|p| p.x).unwrap_or(camera_state.pan_x),
                    pan_y: camera_cmd.pan.as_ref().map(|p| p.y).unwrap_or(camera_state.pan_y),
                    zoom: camera_cmd.zoom.unwrap_or(camera_state.zoom),
                    tilt: camera_cmd.tilt.unwrap_or(camera_state.tilt),
                    focus: camera_cmd.focus,
                };
                camera_anim_state.start(
                    camera_state.clone(),
                    target,
                    camera_cmd.duration,
                    camera_cmd.easing,
                );
            }

            // Start or stop video background
            if let Some(video_bg) = state.current_video_bg() {
                if video_bg.path.is_empty() {
                    video_bg_state.stop();
                } else if let Err(e) = video_bg_state.start(&video_bg.path, video_bg.looped) {
                    eprintln!("Failed to start video background: {}", e);
                }
            }

            // Reset auto timer on command change
            auto_timer = 0.0;

            last_index = Some(current_index);
        }

        // Update transition state
        transition_state.update();

        // Update shake state
        shake_state.update();

        // Update character animation state
        char_anim_state.update();

        // Check if enter animation just completed and start pending idle
        if !char_anim_state.is_active()
            && char_anim_state.direction() == Some(render::character::AnimationDirection::Enter)
            && let Some(idle) = pending_idle.take()
        {
            char_idle_state.start(&idle);
        }

        // Update idle animation state
        char_idle_state.update();

        // Update multiple character animation states
        for anim_state in char_anim_states.values_mut() {
            anim_state.update();
        }

        // Check if enter animations completed and start pending idles
        let completed_positions: Vec<CharPosition> = char_anim_states
            .iter()
            .filter(|(_, anim_state)| {
                !anim_state.is_active()
                    && anim_state.direction() == Some(render::character::AnimationDirection::Enter)
            })
            .map(|(pos, _)| *pos)
            .collect();

        for pos in completed_positions {
            if let Some(idle) = pending_idles.remove(&pos)
                && let Some(idle_state) = char_idle_states.get_mut(&pos)
            {
                idle_state.start(&idle);
            }
        }

        // Update multiple character idle animation states
        for idle_state in char_idle_states.values_mut() {
            idle_state.update();
        }

        // Update camera animation state
        camera_anim_state.update(get_frame_time());
        camera_state = camera_anim_state.current();

        // Update video background state
        video_bg_state.update();

        // Get shake offset for visual rendering
        let shake_offset = shake_state.offset();

        // Calculate camera transform
        let camera_transform = calculate_camera_transform(
            &camera_state,
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
                if nvl_state.active != is_nvl_mode {
                    nvl_state.set_active(is_nvl_mode);
                }

                // Check for NVL clear command
                if is_nvl_mode && state.current_nvl_clear() {
                    nvl_state.clear();
                }

                // Draw visuals first (background, then character) with shake offset and camera
                push_camera_transform(&camera_transform);
                draw_visual(
                    &visual,
                    &mut texture_cache,
                    shake_offset,
                    &char_anim_state,
                    &char_idle_state,
                    &char_anim_states,
                    &char_idle_states,
                    &modular_char_defs,
                    &video_bg_state,
                )
                .await;
                pop_camera_transform(&camera_transform);

                // Resolve localized text
                let resolved_text = language_config.resolve(&text);

                // Interpolate variables in text
                let interpolated_text = interpolate_variables(&resolved_text, state.variables());

                // Resolve speaker name
                let resolved_speaker = speaker
                    .as_ref()
                    .map(|name| interpolate_variables(&language_config.resolve(name), state.variables()));

                // Reset typewriter if text changed
                if last_text.as_ref() != Some(&interpolated_text) {
                    if is_nvl_mode {
                        // In NVL mode, count all accumulated chars plus current text
                        let total_chars = count_nvl_chars(&nvl_state, &interpolated_text);
                        typewriter_state.reset(total_chars);
                    } else {
                        // In ADV mode, count visible characters (excluding color tags)
                        let total_chars = count_visible_chars(&interpolated_text);
                        typewriter_state.reset(total_chars);
                    }
                    last_text = Some(interpolated_text.clone());
                }

                // Update typewriter state
                let char_limit = typewriter_state.update(settings.text_speed);

                if is_nvl_mode {
                    // NVL mode: draw full-screen text box
                    draw_nvl_text_box(
                        &nvl_config,
                        &nvl_state,
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
                    if typewriter_state.is_complete() {
                        draw_continue_indicator_with_font(&text_config, font_ref);
                    }
                }

                // Draw backlog overlay if enabled
                if show_backlog {
                    let history: Vec<_> = state.history().iter().cloned().collect();
                    draw_backlog(
                        &backlog_config,
                        &mut backlog_state,
                        &history,
                        &language_config,
                    );
                } else {
                    // Skip mode: S key toggle or Ctrl key held down
                    let skip_active = skip_mode
                        || is_key_down(KeyCode::LeftControl)
                        || is_key_down(KeyCode::RightControl);

                    // Auto mode timer (only counts when text is complete)
                    let mut auto_advance = false;
                    if auto_mode && typewriter_state.is_complete() {
                        auto_timer += get_frame_time() as f64;
                        // Wait time based on text length, adjusted by auto speed setting
                        // Higher speed = shorter wait time
                        let base_wait = 2.0 + resolved_text.len() as f64 * 0.05;
                        let wait_time = base_wait / settings.auto_speed as f64;
                        if auto_timer >= wait_time {
                            auto_advance = true;
                            auto_timer = 0.0;
                        }
                    }

                    // Handle click/Advance keybind
                    let input_pressed = is_mouse_button_pressed(MouseButton::Left)
                        || settings
                            .keybinds
                            .is_pressed_with_gamepad(Action::Advance, &gamepad_state);

                    if skip_active || auto_advance {
                        // Check if we can skip (skip_unread=true or text is read)
                        let can_skip = settings.skip_unread
                            || read_state.is_read(SCENARIO_PATH, state.current_index());

                        if can_skip || auto_advance {
                            // Skip mode and auto mode bypass typewriter
                            typewriter_state.complete();
                            // In NVL mode, add completed text to buffer before advancing
                            if is_nvl_mode {
                                nvl_state.push(resolved_speaker.clone(), interpolated_text.clone());
                            }
                            read_state.mark_read(SCENARIO_PATH, state.current_index());
                            state.advance();
                            auto_timer = 0.0;
                        } else {
                            // Stop skip mode on unread text
                            skip_mode = false;
                            eprintln!("Skip mode stopped (unread text)");
                        }
                    } else if input_pressed {
                        if typewriter_state.is_complete() {
                            // Text is complete, advance to next
                            // In NVL mode, add completed text to buffer before advancing
                            if is_nvl_mode {
                                nvl_state.push(resolved_speaker.clone(), interpolated_text.clone());
                            }
                            read_state.mark_read(SCENARIO_PATH, state.current_index());
                            state.advance();
                            auto_timer = 0.0;
                        } else {
                            // Text is still animating, complete it instantly
                            typewriter_state.complete();
                        }
                    }
                }

                // Draw mode indicators
                let mut indicator_y = 20.0;
                if skip_mode {
                    draw_text(
                        "SKIP",
                        750.0,
                        indicator_y,
                        20.0,
                        Color::new(1.0, 0.5, 0.5, 1.0),
                    );
                    indicator_y += 22.0;
                }
                if auto_mode {
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
                if skip_mode {
                    skip_mode = false;
                    eprintln!("Skip mode OFF (reached choices)");
                }

                // Draw visuals first with shake offset and camera
                push_camera_transform(&camera_transform);
                draw_visual(
                    &visual,
                    &mut texture_cache,
                    shake_offset,
                    &char_anim_state,
                    &char_idle_state,
                    &char_anim_states,
                    &char_idle_states,
                    &modular_char_defs,
                    &video_bg_state,
                )
                .await;
                pop_camera_transform(&camera_transform);

                // Resolve localized text
                let resolved_text = language_config.resolve(&text);

                // Interpolate variables in text
                let interpolated_text = interpolate_variables(&resolved_text, state.variables());

                // Draw speaker name if present (also interpolate variables)
                if let Some(ref name) = speaker {
                    let resolved_name = language_config.resolve(name);
                    let interpolated_name =
                        interpolate_variables(&resolved_name, state.variables());
                    draw_speaker_name(&text_config, &interpolated_name, font_ref);
                }

                // Reset typewriter if text changed
                if last_text.as_ref() != Some(&interpolated_text) {
                    let total_chars = count_visible_chars(&interpolated_text);
                    typewriter_state.reset(total_chars);
                    last_text = Some(interpolated_text.clone());
                    // Reset choice timer and navigation state when text changes
                    choice_timer = timeout;
                    _choice_total_time = timeout;
                    choice_nav_state = ChoiceNavState::default();
                }

                // Update typewriter state
                let char_limit = typewriter_state.update(settings.text_speed);

                // Draw text box with typewriter effect
                draw_text_box_typewriter(&text_config, &interpolated_text, font_ref, char_limit);

                // Draw backlog overlay if enabled
                if show_backlog {
                    let history: Vec<_> = state.history().iter().cloned().collect();
                    draw_backlog(
                        &backlog_config,
                        &mut backlog_state,
                        &history,
                        &language_config,
                    );
                } else {
                    // Only show choices when text is complete
                    if typewriter_state.is_complete() {
                        let choice_count = choices.len();

                        // Update choice timer
                        if let Some(ref mut remaining) = choice_timer {
                            *remaining -= get_frame_time();

                            // Check if timer expired
                            if *remaining <= 0.0 {
                                // Auto-select default choice
                                if let Some(idx) = default_choice {
                                    read_state.mark_read(SCENARIO_PATH, state.current_index());
                                    state.select_choice(idx);
                                    choice_timer = None;
                                    _choice_total_time = None;
                                    choice_nav_state = ChoiceNavState::default();
                                }
                            }
                        }

                        // --- Input mode switching ---
                        let mouse_pos = mouse_position();
                        if mouse_pos != last_mouse_pos {
                            // Mouse moved, switch to mouse mode
                            choice_nav_state.input_source = InputSource::Mouse;
                            choice_nav_state.focus_index = None;
                            last_mouse_pos = mouse_pos;
                        }

                        // --- Gamepad input ---
                        let dpad_up = gamepad_state.is_button_pressed(GamepadButton::DPadUp);
                        let dpad_down = gamepad_state.is_button_pressed(GamepadButton::DPadDown);
                        let stick_y = gamepad_state.axis(GamepadAxis::LeftY);

                        // Stick debounce processing
                        choice_nav_state.stick_debounce -= get_frame_time();
                        let stick_up =
                            stick_y < -STICK_THRESHOLD && choice_nav_state.stick_debounce <= 0.0;
                        let stick_down =
                            stick_y > STICK_THRESHOLD && choice_nav_state.stick_debounce <= 0.0;

                        if dpad_up || dpad_down || stick_up || stick_down {
                            // Switch to gamepad mode
                            choice_nav_state.input_source = InputSource::Gamepad;

                            // Initialize focus if not set
                            if choice_nav_state.focus_index.is_none() {
                                choice_nav_state.focus_index = Some(0);
                            }

                            // Move focus
                            if let Some(idx) = choice_nav_state.focus_index {
                                if dpad_up || stick_up {
                                    choice_nav_state.focus_index = Some(idx.saturating_sub(1));
                                    choice_nav_state.stick_debounce = 0.2;
                                } else if dpad_down || stick_down {
                                    choice_nav_state.focus_index =
                                        Some((idx + 1).min(choice_count - 1));
                                    choice_nav_state.stick_debounce = 0.2;
                                }
                            }
                        }

                        // Calculate remaining time relative to total for progress bar
                        let remaining_time = choice_timer.map(|t| t.max(0.0));

                        let result = draw_choices_with_timer(
                            &choice_config,
                            &choices,
                            remaining_time,
                            default_choice,
                            &language_config,
                            &choice_nav_state,
                        );

                        // --- Selection confirmation ---
                        // Mouse click
                        if let Some(index) = result.selected {
                            read_state.mark_read(SCENARIO_PATH, state.current_index());
                            state.select_choice(index);
                            choice_timer = None;
                            _choice_total_time = None;
                            choice_nav_state = ChoiceNavState::default();
                        } else if choice_nav_state.input_source == InputSource::Gamepad {
                            // Gamepad A button
                            if gamepad_state.is_button_pressed(GamepadButton::A)
                                && let Some(idx) = choice_nav_state.focus_index
                            {
                                read_state.mark_read(SCENARIO_PATH, state.current_index());
                                state.select_choice(idx);
                                choice_timer = None;
                                _choice_total_time = None;
                                choice_nav_state = ChoiceNavState::default();
                            }
                        }
                    } else {
                        // Click to complete text
                        if is_mouse_button_pressed(MouseButton::Left)
                            || settings
                                .keybinds
                                .is_pressed_with_gamepad(Action::Advance, &gamepad_state)
                        {
                            typewriter_state.complete();
                        }
                    }
                }
            }
            DisplayState::Wait { duration, visual } => {
                // Draw visuals with shake offset and camera
                push_camera_transform(&camera_transform);
                draw_visual(
                    &visual,
                    &mut texture_cache,
                    shake_offset,
                    &char_anim_state,
                    &char_idle_state,
                    &char_anim_states,
                    &char_idle_states,
                    &modular_char_defs,
                    &video_bg_state,
                )
                .await;
                pop_camera_transform(&camera_transform);

                // Reset wait timer if just started waiting
                if !in_wait {
                    in_wait = true;
                    wait_timer = 0.0;
                }

                // Update wait timer
                wait_timer += get_frame_time();

                // Check if wait is complete or skipped
                let skip_active = skip_mode
                    || is_key_down(KeyCode::LeftControl)
                    || is_key_down(KeyCode::RightControl);

                if wait_timer >= duration
                    || skip_active
                    || is_mouse_button_pressed(MouseButton::Left)
                    || settings
                        .keybinds
                        .is_pressed_with_gamepad(Action::Advance, &gamepad_state)
                {
                    in_wait = false;
                    wait_timer = 0.0;
                    read_state.mark_read(SCENARIO_PATH, state.current_index());
                    state.advance();
                }

                // Draw mode indicators
                let mut indicator_y = 20.0;
                if skip_mode {
                    draw_text(
                        "SKIP",
                        750.0,
                        indicator_y,
                        20.0,
                        Color::new(1.0, 0.5, 0.5, 1.0),
                    );
                    indicator_y += 22.0;
                }
                if auto_mode {
                    draw_text("AUTO", 750.0, indicator_y, 20.0, YELLOW);
                }
            }
            DisplayState::Input { input, visual } => {
                // Draw visuals with shake offset and camera
                push_camera_transform(&camera_transform);
                draw_visual(
                    &visual,
                    &mut texture_cache,
                    shake_offset,
                    &char_anim_state,
                    &char_idle_state,
                    &char_anim_states,
                    &char_idle_states,
                    &modular_char_defs,
                    &video_bg_state,
                )
                .await;
                pop_camera_transform(&camera_transform);

                // Initialize input state if this is a new input command
                if awaiting_input.as_ref() != Some(&input.var) {
                    awaiting_input = Some(input.var.clone());
                    input_state.reset(input.default.as_deref());
                }

                // Draw input dialog
                let result = draw_input(
                    &input_config,
                    &mut input_state,
                    input.prompt.as_deref(),
                    font_ref,
                );

                if result.submitted {
                    // Store input value as variable
                    let value = runtime::Value::String(input_state.text.clone());
                    state.set_variable(&input.var, value);
                    awaiting_input = None;
                    read_state.mark_read(SCENARIO_PATH, state.current_index());
                    state.advance();
                } else if result.cancelled {
                    // Use default value or empty string
                    let default_value = input.default.clone().unwrap_or_default();
                    let value = runtime::Value::String(default_value);
                    state.set_variable(&input.var, value);
                    awaiting_input = None;
                    read_state.mark_read(SCENARIO_PATH, state.current_index());
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
                if !video_state.is_playing() && !video_state.is_finished() {
                    // Fade out BGM before video starts
                    audio_manager.stop_bgm_fade(0.5).await;

                    if let Err(e) = video_state.start(&path, skippable, loop_video) {
                        eprintln!("Failed to start video: {}", e);
                        // Skip to next command on error
                        state.advance();
                    }
                }

                // Update and draw video
                video_state.update();
                video_state.draw();

                // Check for skip input
                let skip_pressed = is_mouse_button_pressed(MouseButton::Left)
                    || settings
                        .keybinds
                        .is_pressed_with_gamepad(Action::Advance, &gamepad_state)
                    || is_key_pressed(KeyCode::Escape);

                // Advance when video finishes or is skipped
                if video_state.is_finished() || (skip_pressed && video_state.can_skip()) {
                    video_state.stop();
                    read_state.mark_read(SCENARIO_PATH, state.current_index());
                    state.advance();
                }
            }
            DisplayState::End => {
                draw_text_box_with_font(&text_config, "[ End ]", font_ref);

                // Draw backlog overlay if enabled
                if show_backlog {
                    let history: Vec<_> = state.history().iter().cloned().collect();
                    draw_backlog(
                        &backlog_config,
                        &mut backlog_state,
                        &history,
                        &language_config,
                    );
                }

                // Return to title on click or Advance, or exit on Escape
                if is_mouse_button_pressed(MouseButton::Left)
                    || settings
                        .keybinds
                        .is_pressed_with_gamepad(Action::Advance, &gamepad_state)
                {
                    return_to_title = true;
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }

        // Update and draw particles
        particle_state.update_and_draw();

        // Update and draw cinematic bars
        cinematic_state.update();
        cinematic_state.draw();

        // Update and draw achievement notification
        achievement_notifier.update(get_frame_time());
        draw_achievement(&achievement_config, &achievement_notifier, font_ref);

        // Draw debug overlay
        draw_debug(&debug_config, &debug_state, state, font_ref);

        // Draw transition overlay
        transition_state.draw();

        // Return to title on Escape (instead of exiting)
        if is_key_pressed(KeyCode::Escape) && !state.is_ended() {
            return_to_title = true;
        }

        // Screenshot
        if settings.keybinds.is_pressed(Action::Screenshot) {
            save_screenshot();
        }

        next_frame().await;

        // Process return to title (after frame, when state borrow is dropped)
        if return_to_title {
            game_mode = GameMode::Title;
            game_state = None;
        }
    }
}
