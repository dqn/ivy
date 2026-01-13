mod audio;
mod render;
mod runtime;
mod scenario;

use std::collections::HashMap;

use macroquad::prelude::*;

use audio::AudioManager;
use render::{
    draw_backlog, draw_background, draw_character, draw_choices,
    draw_continue_indicator_with_font, draw_text_box_with_font, draw_title_screen,
    BacklogConfig, BacklogState, ChoiceButtonConfig, TextBoxConfig, TitleConfig, TitleMenuItem,
    TransitionState,
};
use runtime::{DisplayState, GameState, SaveData, VisualState};
use scenario::load_scenario;

/// Game mode: title screen or in-game.
#[derive(Debug, Clone, Copy, PartialEq)]
enum GameMode {
    Title,
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

/// Load a texture, using cache if available.
async fn get_texture(path: &str, cache: &mut HashMap<String, Texture2D>) -> Option<Texture2D> {
    if let Some(texture) = cache.get(path) {
        return Some(texture.clone());
    }

    match load_texture(path).await {
        Ok(texture) => {
            texture.set_filter(FilterMode::Linear);
            cache.insert(path.to_string(), texture.clone());
            Some(texture)
        }
        Err(e) => {
            eprintln!("Failed to load texture '{}': {}", path, e);
            None
        }
    }
}

/// Draw visual elements (background and character).
async fn draw_visual(visual: &VisualState, cache: &mut HashMap<String, Texture2D>) {
    // Draw background
    if let Some(bg_path) = &visual.background {
        if let Some(texture) = get_texture(bg_path, cache).await {
            draw_background(&texture);
        }
    }

    // Draw character
    if let Some(char_path) = &visual.character {
        if let Some(texture) = get_texture(char_path, cache).await {
            draw_character(&texture, visual.char_pos);
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

    let scenario_title = scenario.title.clone();
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

    // Start with title screen
    let mut game_mode = GameMode::Title;
    let mut game_state: Option<GameState> = None;
    let title_config = TitleConfig::default();
    let text_config = TextBoxConfig::default();
    let choice_config = ChoiceButtonConfig::default();
    let backlog_config = BacklogConfig::default();
    let mut backlog_state = BacklogState::default();
    let mut show_backlog = false;
    let mut texture_cache: HashMap<String, Texture2D> = HashMap::new();
    let mut audio_manager = AudioManager::new();
    let mut last_index: Option<usize> = None;
    let mut auto_mode = false;
    let mut auto_timer = 0.0;
    let mut transition_state = TransitionState::default();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        match game_mode {
            GameMode::Title => {
                // Check if any save exists
                let has_save = SaveData::slot_exists(1)
                    || SaveData::slot_exists(2)
                    || SaveData::slot_exists(3)
                    || std::path::Path::new(QUICK_SAVE_PATH).exists();

                let result = draw_title_screen(&title_config, &scenario_title, has_save, font_ref);

                if let Some(item) = result.selected {
                    match item {
                        TitleMenuItem::NewGame => {
                            // Start new game
                            let new_scenario = load_scenario(SCENARIO_PATH).unwrap();
                            game_state = Some(GameState::new(new_scenario));
                            game_mode = GameMode::InGame;
                            last_index = None;
                            auto_mode = false;
                            show_backlog = false;
                        }
                        TitleMenuItem::Continue => {
                            // Try to load from quick save first, then from slots
                            if let Some(loaded_state) = load_game() {
                                game_state = Some(loaded_state);
                                game_mode = GameMode::InGame;
                                last_index = None;
                                auto_mode = false;
                                show_backlog = false;
                            } else {
                                // Try slots 1-3
                                for slot in 1..=3 {
                                    if let Some(loaded_state) = load_from_slot(slot) {
                                        game_state = Some(loaded_state);
                                        game_mode = GameMode::InGame;
                                        last_index = None;
                                        auto_mode = false;
                                        show_backlog = false;
                                        break;
                                    }
                                }
                            }
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

                next_frame().await;
                continue;
            }
            GameMode::InGame => {
                // In-game logic follows
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
        // F5 = quick save, F9 = quick load
        // Shift+1-0 = save to slot, 1-0 = load from slot
        if is_key_pressed(KeyCode::F5) {
            save_game(state);
        }
        if is_key_pressed(KeyCode::F9) {
            if let Some(loaded_state) = load_game() {
                *state = loaded_state;
                last_index = None; // Force audio/transition update
            }
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

        // Toggle backlog with L key
        if is_key_pressed(KeyCode::L) {
            show_backlog = !show_backlog;
            backlog_state = BacklogState::default();
        }

        // Toggle auto mode with A key
        if is_key_pressed(KeyCode::A) {
            auto_mode = !auto_mode;
            auto_timer = 0.0;
            if auto_mode {
                eprintln!("Auto mode ON");
            } else {
                eprintln!("Auto mode OFF");
            }
        }

        // Handle rollback (Up arrow or mouse wheel up) - only when backlog is not shown
        if !show_backlog {
            let wheel = mouse_wheel();
            if is_key_pressed(KeyCode::Up) || wheel.1 > 0.0 {
                if state.can_rollback() {
                    state.rollback();
                }
            }
        }

        // Update audio and transition when command changes
        let current_index = state.current_index();
        if last_index != Some(current_index) {
            // Update BGM
            audio_manager
                .update_bgm(state.current_bgm())
                .await;

            // Play SE
            audio_manager.play_se(state.current_se()).await;

            // Play voice
            audio_manager.play_voice(state.current_voice()).await;

            // Start transition if specified
            if let Some(transition) = state.current_transition() {
                transition_state.start(transition.transition_type, transition.duration);
            }

            // Reset auto timer on command change
            auto_timer = 0.0;

            last_index = Some(current_index);
        }

        // Update transition state
        transition_state.update();

        match state.display_state() {
            DisplayState::Text { text, visual } => {
                // Draw visuals first (background, then character)
                draw_visual(&visual, &mut texture_cache).await;

                // Draw text box on top
                draw_text_box_with_font(&text_config, &text, font_ref);
                draw_continue_indicator_with_font(&text_config, font_ref);

                // Draw backlog overlay if enabled
                if show_backlog {
                    let history: Vec<_> = state.history().iter().cloned().collect();
                    draw_backlog(&backlog_config, &mut backlog_state, &history);
                } else {
                    // Skip mode: Ctrl key held down
                    let skip_mode =
                        is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);

                    // Auto mode timer
                    let mut auto_advance = false;
                    if auto_mode {
                        auto_timer += get_frame_time() as f64;
                        // Wait time based on text length (base 2s + 0.05s per character)
                        let wait_time = 2.0 + text.len() as f64 * 0.05;
                        if auto_timer >= wait_time {
                            auto_advance = true;
                            auto_timer = 0.0;
                        }
                    }

                    // Advance on click, Enter key, skip mode, or auto mode
                    if is_mouse_button_pressed(MouseButton::Left)
                        || is_key_pressed(KeyCode::Enter)
                        || skip_mode
                        || auto_advance
                    {
                        state.advance();
                        auto_timer = 0.0; // Reset timer on manual advance
                    }
                }

                // Draw auto mode indicator
                if auto_mode {
                    draw_text("AUTO", 750.0, 20.0, 20.0, YELLOW);
                }
            }
            DisplayState::Choices {
                text,
                choices,
                visual,
            } => {
                // Draw visuals first
                draw_visual(&visual, &mut texture_cache).await;

                // Draw text box and choices on top
                draw_text_box_with_font(&text_config, &text, font_ref);

                // Draw backlog overlay if enabled
                if show_backlog {
                    let history: Vec<_> = state.history().iter().cloned().collect();
                    draw_backlog(&backlog_config, &mut backlog_state, &history);
                } else {
                    let result = draw_choices(&choice_config, &choices);
                    if let Some(index) = result.selected {
                        state.select_choice(index);
                    }
                }
            }
            DisplayState::End => {
                draw_text_box_with_font(&text_config, "[ End ]", font_ref);

                // Draw backlog overlay if enabled
                if show_backlog {
                    let history: Vec<_> = state.history().iter().cloned().collect();
                    draw_backlog(&backlog_config, &mut backlog_state, &history);
                }

                // Return to title on click or Enter, or exit on Escape
                if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Enter) {
                    return_to_title = true;
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }

        // Draw transition overlay
        transition_state.draw();

        // Return to title on Escape (instead of exiting)
        if is_key_pressed(KeyCode::Escape) && !state.is_ended() {
            return_to_title = true;
        }

        next_frame().await;

        // Process return to title (after frame, when state borrow is dropped)
        if return_to_title {
            game_mode = GameMode::Title;
            game_state = None;
        }
    }
}
