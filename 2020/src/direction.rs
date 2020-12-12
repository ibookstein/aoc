#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Turn {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn(self, to: Turn) -> Self {
        match (self, to) {
            (Direction::Up, Turn::Right) | (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Up, Turn::Left) | (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Left, Turn::Right) | (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Left, Turn::Left) | (Direction::Right, Turn::Right) => Direction::Down,
        }
    }
}
