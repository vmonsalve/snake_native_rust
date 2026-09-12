use crate::board::Board;
use crate::snake::Snake;
use crate::utils::clear_screen;

use std::thread;
use std::time::Duration;

pub struct Game {
    board: Board,
    snake: Snake,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(20, 10),
            snake: Snake::new(),
        }
    }

    pub fn run(&mut self) {
        let width = self.board.width();
        loop {
            clear_screen();
            let head = self.snake.head();
            println!("Cabeza: ({}, {} Ancho: {})", head.x, head.y, width);
            
            self.board.draw(&self.snake);
            
            if head.x >= width - 1 {
                break;
            }

            self.snake.move_right();

            thread::sleep(Duration::from_millis(250));
        }
    }
}