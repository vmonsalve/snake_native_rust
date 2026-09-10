mod board;
mod snake;
mod utils;

use board::Board;
use snake::Snake;
use utils::clear_screen;

use std::thread;
use std::time::Duration;


fn main() {
    let board = Board::new(20, 10);
    let mut snake = Snake::new();

    loop {
        clear_screen();

        board.draw(&snake);

        snake.move_right();

        thread::sleep(Duration::from_millis(250));
    }
}