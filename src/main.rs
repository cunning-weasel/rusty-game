// extra challenges + docs at the bottom
use rand::prelude::*;
use rusty_engine::prelude::*;

// glaobal road speed - check out game logic
const ROAD_SPEED: f32 = 400.0;
// speed moving up and down in px/ second
const PLAYER_SPEED: f32 = 250.0;

// game state struct
struct GameState {
    health_amount: u8,
    lost: bool,
}

// default game state
// impl Default for GameState {
//     fn default() -> Self {
//         Self {
//             health_amount: 0,
//             lost: false,
//         }
//     }
// }

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
        SpritePreset::RollingBlockCorner,
        SpritePreset::RollingBlockSquare,
        SpritePreset::RollingBlockSmall,
    ];

    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    // health
    let health_message = game.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Don't run any more game logic if the game has ended
    if game_state.lost {
        return;
    }

    // game logic - called once every frame
    let mut direction = 0.0;

    // handle movement & player inputs
    let mut direction = 0.0;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W, KeyCode::Comma])
    {
        direction += 1.0;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S, KeyCode::O])
    {
        direction -= 1.0;
    }

    // move player sprite
    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;

    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;
    if player1.translation.y < -360.0 || player1.translation.y > 360.0 {
        game_state.health_amount = 0;
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

    // handling health with collisions
    let health_message = engine.texts.get_mut("health_message").unwrap();

    for event in engine.collision_events.drain(..) {
        // We don't care if obstacles collide with each other or collisions end
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
    }

    // player dies
    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}

/* 
Challenges
Add a second player!
Variant A: The two players can overlap, harmlessly
Variant B: The two players are separated into their own lanes, and cannot cross to the same lane
Variant C: The two players crash if they touch each other. Requires implementing the "cars can move forward and backward a little".
Powerups! All powerups wear off after a short time, so you'll need to use Timers
Powerup A: Boost car maneuverability
Powerup B: Armor - car can withstand more obstacle hits
Powerup C: Phase shift - car can move through obstacles
Powerup D: Explosion - all visible obstacles are cleared
Hazards! All hazards wear off after a short time, so you'll need to use Timers
Hazard A: Oil Slick - car is unable to control movement
Hazard B: Anti-Powerup - hitting the anti-powerup causes a new type of obstacle to appear
Hazard C: Afterburners - road speed increases
Polish
Make the car turn (rotate) smoothly instead of suddenly, and have the speed the car moves in the y direction vary proportianally to how much the car is rotated.
Randomize the rotation of the obstacles
Add support for driving the car via mouse input
Add controls to change or turn off the music
Add text indicators of things that happen (collisions, powerups, etc.)
Add the ability to pause and resume the game
Collect points for every obstacle that you pass, display the score in the corner during the game, add an end screen showing the final score.
Instead of ignoring obstacles that collide with each other, take one of the colliding items and set it to a new random starting position.
Make it so you can't hit the same obstacle twice (currently if you wiggle back and forth you can run into the same obstacle multiple times)

Challegenee to self - after the above, get this thing deployed and accessible on herkoku / own home server/ lab
*/