pub struct Snake {
    x: usize,
    y: usize,
}

impl Snake {
    pub fn new(x: usize, y: usize) -> Snake {
        Snake { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}