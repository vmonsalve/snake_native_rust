pub struct Board {
    width: usize,
    height: usize,
}

use crate::snake::Snake;

impl Board {
    
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
        }
    }

    pub fn draw(&self, snake: &Snake) {
        println!("+{}+", "-".repeat(self.width));

        for y in 0..self.height {
            print!("|");

            for x in 0..self.width {
                if snake.contains(x, y) {
                    print!("O");
                } else {
                    print!(" ");
                }
            }

            println!("|");
        }

        println!("+{}+", "-".repeat(self.width));
    }
}