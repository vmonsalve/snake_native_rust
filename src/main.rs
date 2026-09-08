mod board;
mod snake;

use board::Board;
use snake::Snake;

fn main() {
    let board = Board::new(20, 10);
    let snake = Snake::new();

    board.draw(&snake);
}