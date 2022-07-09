use rusty_engine::prelude::*;

// game state struct
struct GameState {
    health_amount: u8,
    lost: bool,
}

// default game state
impl Default for GameState {
    fn default() -> Self {
        Self {
            health_amount: 0,
            lost: false,

        }
    }
}

fn main() {
    // mut as we need to keep track of game-state
    let mut game = Game::new();

    // audio
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // setup game
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation = Vec2::new(-500.0, 0.0);
    player1.rotation = RIGHT;
    player1.collision = true;
    player1.layer = 20.0;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic
    


    // game_state.current_score += 1;
    // print!("current score: {}", game_state.current_score);
}
