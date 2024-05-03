#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        // Check if rank and file are within the valid range (0-7)
        if (0..=7).contains(&rank) && (0..=7).contains(&file) {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let x1 = self.position.rank;
        let y1 = self.position.file;
        let x2 = other.position.rank;
        let y2 = other.position.file;

        // Queens can attack each other if they are on the same rank, file, or diagonal
        x1 == x2 || y1 == y2 || (x1 - x2).abs() == (y1 - y2).abs()
    }
}