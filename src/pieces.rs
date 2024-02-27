use std::slice::{Iter, IterMut};

pub use self::color::Color;
pub use bitboard::BitBoard;
pub use piece::Piece;

mod bitboard;
mod color;
mod piece;

#[derive(Debug, Clone)]
pub struct Pieces {
    pieces: [BitBoard; 6],
}

impl Pieces {
    pub fn new(color: Color) -> Self {
        Self {
            pieces: [
                BitBoard::new(Piece::Pawn(color)),
                BitBoard::new(Piece::Knight(color)),
                BitBoard::new(Piece::Bishop(color)),
                BitBoard::new(Piece::Rook(color, false, false)),
                BitBoard::new(Piece::Queen(color)),
                BitBoard::new(Piece::King(color, false)),
            ],
        }
    }

    pub fn get(&self, piece: Piece) -> &BitBoard {
        &self.pieces[Self::offset(piece)]
    }

    pub fn iter(&self) -> Iter<'_, BitBoard> {
        self.pieces.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, BitBoard> {
        self.pieces.iter_mut()
    }

    fn offset(piece: Piece) -> usize {
        match piece {
            Piece::Pawn(_) => 0,
            Piece::Knight(_) => 1,
            Piece::Bishop(_) => 2,
            Piece::Rook(_, _, _) => 3,
            Piece::Queen(_) => 4,
            Piece::King(_, _) => 5,
        }
    }
}
