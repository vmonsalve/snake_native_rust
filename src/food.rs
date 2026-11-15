use crate::snake::Position;

pub struct Food {
    position: Position,
}

impl Food {
    pub fn new(x: usize, y: usize) -> Food {
        Food {
            position: Position { x, y },
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}