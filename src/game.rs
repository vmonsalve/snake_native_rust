use crate::board::Board;
use crate::snake::{Direction, Snake};
use crate::utils::{
    clear_screen,
    enter_alternate_screen,
    exit_alternate_screen,
    enable_raw_mode,
    disable_raw_mode,
};

use std::io::{self, Read};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;


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
        let (tx, rx) = mpsc::channel::<Direction>();

        thread::spawn(move || {    
            let stdin = io::stdin();
            for byte in stdin.lock().bytes() {
                let direction = match byte.unwrap() {
                    b'w' => Some(Direction::Up),
                    b's' => Some(Direction::Down),
                    b'a' => Some(Direction::Left),
                    b'd' => Some(Direction::Right),
                    b'x' => continue,
                    _ => None,
                };

              
                if let Some(dir) = direction {
                    tx.send(dir).unwrap();
                }
         
            }
        });
    
        enter_alternate_screen();
        enable_raw_mode();

        loop {

            clear_screen();

            let (head_x, head_y) = {
                let head = self.snake.head();
                (head.x, head.y)
            };

            println!(
                "Cabeza: ({}, {} Ancho: {})",
                head_x,
                head_y,
                width
            );

            self.board.draw(&self.snake);

            if let Ok(direction) = rx.try_recv() {
                self.snake.set_direction(direction);
            }

            let collision = match self.snake.direction() {
                Direction::Right => head_x >= width - 1,
                Direction::Left => head_x == 0,
                Direction::Down => head_y >= height - 1,
                Direction::Up => head_y == 0,
            };

            if collision {
                break;
            }

            self.snake.move_forward();

            thread::sleep(Duration::from_millis(250));

        }
        
        disable_raw_mode();
        exit_alternate_screen();

    }
}