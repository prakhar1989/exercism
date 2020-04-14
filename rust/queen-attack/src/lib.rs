#[derive(Debug)]
pub struct ChessPosition {
    pos: (i32, i32),
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 7 && file < 7 && rank >= 0 && file >= 0 {
            return Some(ChessPosition { pos: (rank, file) });
        }
        None
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (r1, f1) = self.position.pos;
        let (r2, f2) = other.position.pos;

        r1 == r2 || f1 == f2 || (r1 - r2).abs() == (f1 - f2).abs()
    }
}
