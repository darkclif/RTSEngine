use std::ops;

enum PosOffset {
    // Hexagonal next field
    UpLeft,     // y++
    UpRight,    // z++
    Left,       // x--
    Right,      // x++
    DownLeft,   // z--
    DownRight   // y--
}

#[derive(Clone, Copy)]
pub struct Position2 {
    pub x: i64,
    pub y: i64
}

impl Position2 {
    pub fn origin() -> Position2 {
        Position2 { x: (0), y: (0) }
    }

    pub fn create(x: i64, y: i64) -> Position2 {
        Position2 { x, y }
    }

    fn next_pos(&self, side: PosOffset) -> Position2 {
        match side {
            PosOffset::UpLeft => Position2{y: self.y + 1, x: self.x - 1 + (self.y % 2)},
            PosOffset::UpRight => Position2{y: self.y + 1, x: self.x + (self.y % 2)},
            PosOffset::Left => Position2{x: self.x - 1, ..*self},
            PosOffset::Right => Position2{x: self.x + 1, ..*self},
            PosOffset::DownLeft => Position2{y: self.y - 1, x: self.x - 1 + (self.y % 2)},
            PosOffset::DownRight => Position2{y: self.y - 1, x: self.x + (self.y % 2)},
        }
    }
}

impl ops::Add<&Position2> for Position2 {
    type Output = Position2;

    fn add(self, _rhs: &Position2) -> Position2{
        return Position2{
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Sub<&Position2> for Position2 {
    type Output = Position2;

    fn sub(self, _rhs: &Position2) -> Self::Output {
        Position2{
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}