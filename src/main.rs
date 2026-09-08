mod board;
mod snake;

use board::Board;
use snake::Snake;

fn main() {
    let board = Board::new(20, 10);
    let snake = Snake::new(10, 5);

    board.draw(&snake);
}