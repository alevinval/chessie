use std::slice::{Iter, IterMut};

pub use self::color::Color;
pub use bitboard::BitBoard;
pub use generator::Movements;
pub use piece::Piece;
pub use pieceset::PieceSet;

mod bitboard;
mod color;
mod generator;
mod movement;
mod piece;
mod pieceset;

#[derive(Debug, Clone)]
pub struct Pieces {
    pieces: [PieceSet; 6],
}

impl Pieces {
    pub fn new(color: Color) -> Self {
        Self {
            pieces: [
                PieceSet::new(Piece::Pawn(color)),
                PieceSet::new(Piece::Knight(color)),
                PieceSet::new(Piece::Bishop(color)),
                PieceSet::new(Piece::Rook(color)),
                PieceSet::new(Piece::Queen(color)),
                PieceSet::new(Piece::King(color)),
            ],
        }
    }

    pub fn get(&self, piece: Piece) -> &PieceSet {
        &self.pieces[self.offset(piece)]
    }

    pub fn iter(&self) -> Iter<'_, PieceSet> {
        self.pieces.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, PieceSet> {
        self.pieces.iter_mut()
    }

    fn offset(&self, piece: Piece) -> usize {
        match piece {
            Piece::Pawn(_) => 0,
            Piece::Knight(_) => 1,
            Piece::Bishop(_) => 2,
            Piece::Rook(_) => 3,
            Piece::Queen(_) => 4,
            Piece::King(_) => 5,
        }
    }
}
