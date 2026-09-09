mod board;
mod snake;

use board::Board;
use snake::Snake;

fn main() {
    let board = Board::new(20, 10);
    let mut snake = Snake::new();

    snake.move_right();
    board.draw(&snake);
}