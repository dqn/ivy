mod audio;
mod render;
mod runtime;
mod scenario;

use std::collections::HashMap;

use macroquad::prelude::*;

use audio::AudioManager;
use render::{
    draw_backlog, draw_background, draw_character, draw_choices, draw_continue_indicator,
    draw_text_box, BacklogConfig, BacklogState, ChoiceButtonConfig, TextBoxConfig,
};
use runtime::{DisplayState, GameState, SaveData, VisualState};
use scenario::load_scenario;

const SCENARIO_PATH: &str = "assets/sample.yaml";
const SAVE_PATH: &str = "saves/save.json";

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

/// Save game state.
fn save_game(game_state: &GameState) {
    let save_data = game_state.to_save_data(SCENARIO_PATH);
    match save_data.save(SAVE_PATH) {
        Ok(()) => eprintln!("Game saved to {}", SAVE_PATH),
        Err(e) => eprintln!("Failed to save game: {}", e),
    }
}

/// Load game state.
fn load_game() -> Option<GameState> {
    let save_data = match SaveData::load(SAVE_PATH) {
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

    eprintln!("Game loaded from {}", SAVE_PATH);
    Some(GameState::from_save_data(&save_data, scenario))
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

    eprintln!("Loaded scenario: {}", scenario.title);

    let mut game_state = GameState::new(scenario);
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

    loop {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        // Handle save/load
        if is_key_pressed(KeyCode::F5) {
            save_game(&game_state);
        }
        if is_key_pressed(KeyCode::F9) {
            if let Some(loaded_state) = load_game() {
                game_state = loaded_state;
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
                if game_state.can_rollback() {
                    game_state.rollback();
                }
            }
        }

        // Update audio when command changes
        let current_index = game_state.current_index();
        if last_index != Some(current_index) {
            // Update BGM
            audio_manager
                .update_bgm(game_state.current_bgm())
                .await;

            // Play SE
            audio_manager.play_se(game_state.current_se()).await;

            // Play voice
            audio_manager.play_voice(game_state.current_voice()).await;

            // Reset auto timer on command change
            auto_timer = 0.0;

            last_index = Some(current_index);
        }

        match game_state.display_state() {
            DisplayState::Text { text, visual } => {
                // Draw visuals first (background, then character)
                draw_visual(&visual, &mut texture_cache).await;

                // Draw text box on top
                draw_text_box(&text_config, &text);
                draw_continue_indicator(&text_config);

                // Draw backlog overlay if enabled
                if show_backlog {
                    let history: Vec<_> = game_state.history().iter().cloned().collect();
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
                        game_state.advance();
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
                draw_text_box(&text_config, &text);

                // Draw backlog overlay if enabled
                if show_backlog {
                    let history: Vec<_> = game_state.history().iter().cloned().collect();
                    draw_backlog(&backlog_config, &mut backlog_state, &history);
                } else {
                    let result = draw_choices(&choice_config, &choices);
                    if let Some(index) = result.selected {
                        game_state.select_choice(index);
                    }
                }
            }
            DisplayState::End => {
                draw_text_box(&text_config, "[ End ]");

                // Draw backlog overlay if enabled
                if show_backlog {
                    let history: Vec<_> = game_state.history().iter().cloned().collect();
                    draw_backlog(&backlog_config, &mut backlog_state, &history);
                }

                // Exit on Escape
                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }

        // Global exit on Escape
        if is_key_pressed(KeyCode::Escape) && !game_state.is_ended() {
            break;
        }

        next_frame().await
    }
}
