mod render;
mod runtime;
mod scenario;

use std::collections::HashMap;

use macroquad::prelude::*;

use render::{
    draw_background, draw_character, draw_choices, draw_continue_indicator, draw_text_box,
    ChoiceButtonConfig, TextBoxConfig,
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

    let mut game_state = GameState::new(scenario);
    let text_config = TextBoxConfig::default();
    let choice_config = ChoiceButtonConfig::default();
    let mut texture_cache: HashMap<String, Texture2D> = HashMap::new();

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

        match game_state.display_state() {
            DisplayState::Text { text, visual } => {
                // Draw visuals first (background, then character)
                draw_visual(&visual, &mut texture_cache).await;

                // Draw text box on top
                draw_text_box(&text_config, &text);
                draw_continue_indicator(&text_config);

                // Advance on click or Enter key
                if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Enter) {
                    game_state.advance();
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

                let result = draw_choices(&choice_config, &choices);
                if let Some(index) = result.selected {
                    game_state.select_choice(index);
                }
            }
            DisplayState::End => {
                draw_text_box(&text_config, "[ End ]");

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
