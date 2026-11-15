mod board;
mod game;
mod snake;
mod utils;
mod food;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}