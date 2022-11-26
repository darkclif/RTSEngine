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
pub struct Position3 {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl Position3 {
    pub fn origin() -> Position3 {
        Position3 { x: (0), y: (0), z: (0) }
    }

    pub fn create(x: i64, y: i64, z: i64) -> Position3 {
        Position3 { x, y, z}
    }

    fn next_pos(&self, side: PosOffset) -> Position3 {
        match side {
            PosOffset::UpLeft => Position3{y: self.y + 1, ..*self},
            PosOffset::UpRight => Position3{z: self.z + 1, ..*self},
            PosOffset::Left => Position3{x: self.x - 1, ..*self},
            PosOffset::Right => Position3{x: self.x + 1, ..*self},
            PosOffset::DownLeft => Position3{z: self.z - 1, ..*self},
            PosOffset::DownRight => Position3{y: self.y - 1, ..*self},
        }
    }
}

impl ops::Add<&Position3> for Position3 {
    type Output = Position3;

    fn add(self, _rhs: &Position3) -> Position3{
        return Position3{
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub<&Position3> for Position3 {
    type Output = Position3;

    fn sub(self, _rhs: &Position3) -> Self::Output {
        Position3{
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}