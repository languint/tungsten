#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}
impl Position {
    pub const ZERO: Position = Position { x: 0, y: 0 };

    pub fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }
}

impl Position {
    pub fn distance(&self, other: &Position) -> i64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2)).isqrt()
    }

    pub fn neighbors(&self) -> Vec<Position> {
        vec![
            Position {
                x: self.x - 1,
                y: self.y,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    pub fn manhattan_distance(&self, other: &Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
