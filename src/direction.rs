#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self, other: Direction) -> bool {
        match self {
            Direction::Up => other == Direction::Down,
            Direction::Down => other == Direction::Up,
            Direction::Left => other == Direction::Right,
            Direction::Right => other == Direction::Left,
        }
    }
}