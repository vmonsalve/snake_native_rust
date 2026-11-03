pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: Vec<Position>,
    direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![
                Position { x: 2, y: 5 },
                Position { x: 3, y: 5 },
                Position { x: 4, y: 5 },
                Position { x: 5, y: 5 },
                Position { x: 6, y: 5 },
                Position { x: 7, y: 5 },
                Position { x: 8, y: 5 },
                Position { x: 9, y: 5 },
                Position { x: 10, y: 5 },
            ],
            direction: Direction::Right,
        }
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
    
    pub fn head(&self) -> &Position {
        self.body.last().unwrap()
    }
    pub fn next_position(&self) -> Position {
        let head = self.body.last().unwrap();

        match self.direction {
            Direction::Right => Position {
                x: head.x + 1,
                y: head.y,
            },

            Direction::Left => Position {
                x: head.x - 1,
                y: head.y,
            },

            Direction::Up => Position {
                x: head.x,
                y: head.y - 1,
            },

            Direction::Down => Position {
                x: head.x,
                y: head.y + 1,
            },
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

    pub fn will_collide_with_self(&self) -> bool {
        let next = self.next_position();
        for position in &self.body[1..] {
            if position.x == next.x && position.y == next.y {
                return true;
            }
        }
        false
    }
    
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn move_forward(&mut self) {

        let new_head = self.next_position();

        self.body.push(new_head);
        self.body.remove(0);
    }
}
