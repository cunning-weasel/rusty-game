use rusty_engine::prelude::*;

fn main() {
    // mut as we need to keep track of game-state
    let mut game = Game::new();

    // setup game

    game.run(());
}

