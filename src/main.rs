mod board;
mod game;
mod snake;
mod utils;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}