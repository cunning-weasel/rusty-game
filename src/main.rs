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
    game.audio_manager
        .play_music(MusicPreset::Classy8Bit, 0.1);

    // setup game
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation = Vec2::new(-500.0, 0.0);
    player1.rotation = RIGHT;
    player1.collision = true;
    player1.layer = 10.0;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic - called once every frame
    let mut direction = 0.0;
    // speed moving up and down in px/ second
    const PLAYER_SPEED: f32 = 250.0;
    // handle movement & player inputs
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    } else if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;

    // player dies
    if player1.translation.y > 360.0 || player1.translation.y < -360.0 {
        game_state.health_amount = 0;
    }
    // game_state.current_score += 1;
    // print!("current score: {}", game_state.current_score);
}
