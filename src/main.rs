mod board;
mod snake;

use board::Board;
use snake::Snake;

use std::thread;
use std::time::Duration;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

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