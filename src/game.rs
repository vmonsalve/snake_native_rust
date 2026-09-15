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

            // Obtenemos las coordenadas de la cabeza y las copiamos.
            // `head` solamente existe dentro de este bloque.
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

            // Prueba temporal:
            // cuando llegue a x = 15 hacemos que baje.
            if head_x == 15 {
                self.snake.set_direction(Direction::Down);
            }

            // Comprobamos la pared correspondiente
            // a la dirección actual.
            let collision = match self.snake.direction() {
                Direction::Right => head_x >= width - 1,
                Direction::Left => head_x == 0,
                Direction::Down => head_y >= height - 1,
                Direction::Up => head_y == 0,
            };

            if collision {
                break;
            }

            // Sólo nos movemos si no existe una colisión.
            self.snake.move_forward();

            thread::sleep(Duration::from_millis(250));

        }

        exit_alternate_screen();
    }
}