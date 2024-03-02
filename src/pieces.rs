use crate::pos::Pos;
use crate::Color;

pub use bitboard::BitBoard;
pub use piece::Piece;

mod bitboard;
mod piece;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pieces {
    pub pawns: BitBoard,
    pub knights: BitBoard,
    pub bishops: BitBoard,
    pub rooks: BitBoard,
    pub queen: BitBoard,
    pub king: BitBoard,
}

impl Pieces {
    pub fn new(color: Color) -> Self {
        Self {
            pawns: Piece::Pawn(color).into(),
            knights: Piece::Knight(color).into(),
            bishops: Piece::Bishop(color).into(),
            rooks: Piece::Rook(color, false, false).into(),
            queen: Piece::Queen(color).into(),
            king: Piece::King(color, false).into(),
        }
    }

    pub fn at<P: Into<Pos>>(&self, pos: P) -> Option<&BitBoard> {
        let pos = pos.into();
        if self.pawns.has_piece(pos) {
            return Some(&self.pawns);
        } else if self.knights.has_piece(pos) {
            return Some(&self.knights);
        } else if self.bishops.has_piece(pos) {
            return Some(&self.bishops);
        } else if self.rooks.has_piece(pos) {
            return Some(&self.rooks);
        } else if self.queen.has_piece(pos) {
            return Some(&self.queen);
        } else if self.king.has_piece(pos) {
            return Some(&self.king);
        }
        None
    }

    pub fn at_mut<P: Into<Pos>>(&mut self, pos: P) -> Option<&mut BitBoard> {
        let pos = pos.into();
        if self.pawns.has_piece(pos) {
            return Some(&mut self.pawns);
        } else if self.knights.has_piece(pos) {
            return Some(&mut self.knights);
        } else if self.bishops.has_piece(pos) {
            return Some(&mut self.bishops);
        } else if self.rooks.has_piece(pos) {
            return Some(&mut self.rooks);
        } else if self.queen.has_piece(pos) {
            return Some(&mut self.queen);
        } else if self.king.has_piece(pos) {
            return Some(&mut self.king);
        }
        None
    }

    pub fn iter(&self) -> impl Iterator<Item = &BitBoard> {
        [
            &self.pawns,
            &self.knights,
            &self.bishops,
            &self.rooks,
            &self.queen,
            &self.king,
        ]
        .into_iter()
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
