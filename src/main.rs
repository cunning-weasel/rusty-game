use rand::prelude::*;
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

// glaobal road speed - check out game logic
const ROAD_SPEED: f32 = 400.0;

fn main() {
    // mut as we need to keep track of game-state
    let mut game = Game::new();

    // audio
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.1);

    // setup game
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation = Vec2::new(-500.0, 0.0);
    player1.rotation = RIGHT;
    player1.collision = true;
    player1.layer = 10.0;

    // road line moves left to create illusion of
    // of car moving down the road
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    // obstacles - more == harder
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
    ];

    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

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

    // move road objects
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
        } else if sprite.translation.x < -675.0 {
            sprite.translation.x += 1500.0;
        }
        // obstacles appearing on the road as they move
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
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
