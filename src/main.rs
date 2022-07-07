use rusty_engine::prelude::*;

// game state struct
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

// default game state
impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, false),
        }
    }
}

fn main() {
    // mut as we need to keep track of game-state
    let mut game = Game::new();

    // setup game
    game.run(GameState::default());
}
