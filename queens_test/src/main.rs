fn main() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());

    println!(
        "Is it possible for the queens to attack each other? => {}",
        white_queen.can_attack(&black_queen)
    );

    let white_queen = Queen::new(ChessPosition::new(1, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());

    println!(
        "Is it possible for the queens to attack each other? => {}",
        white_queen.can_attack(&black_queen)
    );
}


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
        if (0..7).contains(&rank) && (0..7).contains(&file) {
            return Some(ChessPosition{
                rank: rank,
                file: file
            })
        } else {
            return None;
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

        x1 == x2 || y1 == y2 || (x1 - x2).abs() == (y1 - y2).abs()
    }
}

