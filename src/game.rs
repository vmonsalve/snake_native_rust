use crate::board::Board;
use crate::snake::{Direction, Snake};
use crate::utils::{
    clear_screen,
    enter_alternate_screen,
    exit_alternate_screen,
};

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
        let height = self.board.height();
        
        enter_alternate_screen();

        loop {
            clear_screen();

            let head = self.snake.head();

            println!("Cabeza: ({}, {} Ancho: {})", head.x, head.y, width);

            self.board.draw(&self.snake);

            if head.x == 15 {
                self.snake.set_direction(Direction::Down);
            }

            self.snake.move_forward();

            thread::sleep(Duration::from_millis(250));
        }

        exit_alternate_screen();
    }
}