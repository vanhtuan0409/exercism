#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(x: i32, y: i32) -> Option<Self> {
        if !(0..8).contains(&x) || !(0..8).contains(&y) {
            return None;
        }
        Some(Self { x, y })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.pos.x == other.pos.x {
            return true;
        }
        if self.pos.y == other.pos.y {
            return true;
        }
        if (self.pos.x - other.pos.x).abs() == (self.pos.y - other.pos.y).abs() {
            return true;
        }
        false
    }
}
