use crate::pos::Pos;
use crate::Color;

pub use bitboard::BitBoard;
pub use piece::Piece;

mod bitboard;
mod piece;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pieces {
    pub pieces: [BitBoard; 6],
}

impl Pieces {
    pub const P: usize = 0;
    pub const N: usize = 1;
    pub const B: usize = 2;
    pub const R: usize = 3;
    pub const Q: usize = 4;
    pub const K: usize = 5;

    pub fn new(color: Color) -> Self {
        Self {
            pieces: [
                Piece::Pawn(color).into(),
                Piece::Knight(color).into(),
                Piece::Bishop(color).into(),
                Piece::Rook(color, false, false).into(),
                Piece::Queen(color).into(),
                Piece::King(color, false).into(),
            ],
        }
    }

    pub fn at<P: Into<Pos>>(&self, pos: P) -> Option<&BitBoard> {
        let pos = pos.into();
        self.pieces.iter().find(|bb| bb.has_piece(pos))
    }

    pub fn at_mut<P: Into<Pos>>(&mut self, pos: P) -> Option<&mut BitBoard> {
        let pos = pos.into();
        self.pieces.iter_mut().find(|bb| bb.has_piece(pos))
    }

    pub fn iter(&self) -> impl Iterator<Item = &BitBoard> {
        self.pieces.iter()
    }
}

#[cfg(test)]
mod test {
    use std::mem;

    use super::*;

    #[test]
    fn size() {
        assert_eq!(96, mem::size_of::<Pieces>());
        assert_eq!(8, mem::size_of::<&Pieces>());
    }
}
