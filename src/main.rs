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
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(200.0, -300.0);
    player.rotation = RIGHT;
    player.scale = 1.0;
    player.layer = 999.0;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic



    // game_state.current_score += 1;
    // print!("current score: {}", game_state.current_score);
}
