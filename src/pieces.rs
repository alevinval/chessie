use std::slice::{Iter, IterMut};

pub use self::color::Color;
pub use bitboard::BitBoard;
pub use piece::Piece;
pub use pieceset::PieceSet;

mod bitboard;
mod color;
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
                PieceSet::new(Piece::Rook(color)),
                PieceSet::new(Piece::Knight(color)),
                PieceSet::new(Piece::Bishop(color)),
                PieceSet::new(Piece::Queen(color)),
                PieceSet::new(Piece::King(color)),
            ],
        }
    }

    pub fn iter(&self) -> Iter<'_, PieceSet> {
        self.pieces.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, PieceSet> {
        self.pieces.iter_mut()
    }
}
