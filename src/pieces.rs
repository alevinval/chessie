pub use self::color::Color;
pub use bitboard::BitBoard;
pub use piece::Piece;
pub use pieceset::PieceSet;

mod bitboard;
mod color;
mod movement;
mod piece;
mod pieceset;

pub struct Pieces(pub [PieceSet; 6]);

impl Pieces {
    pub fn new(color: Color) -> Self {
        Self([
            PieceSet::new(Piece::Pawn(color)),
            PieceSet::new(Piece::Rook(color)),
            PieceSet::new(Piece::Knight(color)),
            PieceSet::new(Piece::Bishop(color)),
            PieceSet::new(Piece::Queen(color)),
            PieceSet::new(Piece::King(color)),
        ])
    }

    pub fn clear(&mut self) {
        self.0.iter_mut().for_each(|pset| pset.clear());
    }
}
