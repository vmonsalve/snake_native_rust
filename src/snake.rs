pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Snake {
    body: Vec<Position>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![
                Position { x: 7, y: 5 },
                Position { x: 8, y: 5 },
                Position { x: 9, y: 5 },
                Position { x: 10, y: 5 },
            ],
        }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        for position in &self.body {
            if position.x == x && position.y == y {
                return true;
            }
        }
        false
    }

    pub fn move_right(&mut self) {
        let head = self.body.last().unwrap();
        
        let new_head = Position {
            x: head.x + 1,
            y: head.y,
        };

        self.body.push(new_head);
        self.body.remove(0);
    }
}