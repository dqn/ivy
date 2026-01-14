use ivy::runtime::GameState;
use ivy::scenario::parse_scenario;

/// Create a GameState from YAML string.
pub fn create_game_state(yaml: &str) -> GameState {
    let scenario = parse_scenario(yaml).expect("Failed to parse scenario");
    GameState::new(scenario)
}

/// Advance the game state n times.
#[allow(dead_code)]
pub fn advance_n_times(state: &mut GameState, n: usize) {
    for _ in 0..n {
        state.advance();
    }
}
