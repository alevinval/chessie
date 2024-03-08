use crate::bitboard::BitBoard;
use crate::moves::{Move, MoveGen};
use crate::piece::Piece;
use crate::pos::Pos;
use crate::Color;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    mover: Color,
    white: [BitBoard; 6],
    black: [BitBoard; 6],
    n: usize,
}

impl Board {
    pub const P: usize = 0;
    pub const N: usize = 1;
    pub const B: usize = 2;
    pub const R: usize = 3;
    pub const Q: usize = 4;
    pub const K: usize = 5;

    fn gen_pieces(color: Color) -> [BitBoard; 6] {
        [
            Piece::Pawn(color).into(),
            Piece::Knight(color).into(),
            Piece::Bishop(color).into(),
            Piece::Rook(color, false, false).into(),
            Piece::Queen(color).into(),
            Piece::King(color, false).into(),
        ]
    }

    pub fn mover(&self) -> Color {
        self.mover
    }

    pub fn pieces(&self, color: Color) -> &[BitBoard; 6] {
        match color {
            Color::B => &self.black,
            Color::W => &self.white,
        }
    }

    pub fn pieces_mut(&mut self) -> &mut [BitBoard; 6] {
        match self.mover {
            Color::B => &mut self.black,
            Color::W => &mut self.white,
        }
    }

    pub fn at<P: Into<Pos>>(&self, pos: P) -> Option<&BitBoard> {
        let pos = pos.into();

        self.white
            .iter()
            .find(|bb| bb.has_piece(pos))
            .or_else(|| self.black.iter().find(|bb| bb.has_piece(pos)))
    }

    pub fn at_mut<P: Into<Pos>>(&mut self, pos: P) -> Option<&mut BitBoard> {
        let pos = pos.into();

        self.white
            .iter_mut()
            .find(|bb| bb.has_piece(pos))
            .or_else(|| self.black.iter_mut().find(|bb| bb.has_piece(pos)))
    }

    pub fn next_turn(&mut self) {
        self.mover = self.mover.opposite();
        self.n += 1;
    }

    pub fn n(&self) -> usize {
        self.n
    }

    #[must_use]
    pub fn movements(&self, color: Color) -> Vec<Move> {
        self.pieces(color)
            .iter()
            .flat_map(BitBoard::iter_pos)
            .flat_map(|p| MoveGen::new(self, p).generate(true))
            .collect()
    }

    #[must_use]
    pub fn pseudo_movements(&self, color: Color) -> Vec<Move> {
        self.pieces(color)
            .iter()
            .flat_map(BitBoard::iter_pos)
            .flat_map(|p| MoveGen::new(self, p).generate(false))
            .collect()
    }

    #[must_use]
    pub fn piece_count(&self) -> usize {
        let w: usize = self
            .pieces(Color::W)
            .iter()
            .filter(|bb| !bb.piece().is_pawn())
            .map(|bb| bb.iter_pos().count())
            .sum();

        let b: usize = self
            .pieces(Color::B)
            .iter()
            .filter(|bb| !bb.piece().is_pawn())
            .map(|bb| bb.iter_pos().count())
            .sum();

        w + b
    }

    pub fn in_check(&self, color: Color) -> bool {
        let king = self.pieces(color)[Board::K].iter_pos().next();

        match king {
            Some(king) => self
                .pseudo_movements(color.opposite())
                .iter()
                .any(|m| m.to() == king),
            None => true,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            mover: Color::W,
            white: Self::gen_pieces(Color::W),
            black: Self::gen_pieces(Color::B),
            n: 0,
        }
    }
}

#[cfg(test)]
mod test {

    use std::mem;

    use super::*;

    #[test]
    fn mover() {
        let sut = Board::default();
        assert_eq!(Color::W, sut.mover());
    }

    #[test]
    fn pieces_for() {
        let sut = Board::default();
        assert_eq!(&sut.white, sut.pieces(Color::W));
        assert_eq!(&sut.black, sut.pieces(Color::B));
    }

    #[test]
    fn at_white_king() {
        let sut = Board::default();
        let king = sut.at((0, 4));

        assert!(king.is_some());

        if let Some(king) = king {
            assert_eq!(Color::W, king.color());
            assert_eq!(Piece::King(Color::W, false), king.piece());
        }
    }

    #[test]
    fn at_black_king() {
        let sut = Board::default();
        let king = sut.at((7, 4));

        assert!(king.is_some());

        if let Some(king) = king {
            assert_eq!(Color::B, king.color());
            assert_eq!(Piece::King(Color::B, false), king.piece());
        }
    }

    #[test]
    fn mut_at_white() {
        let pos = (0, 0);

        assert_eq!(
            Board::default().at(pos).unwrap(),
            Board::default().at_mut(pos).unwrap()
        );
    }

    #[test]
    fn mut_at_black() {
        let pos = (7, 7);

        assert_eq!(
            Board::default().at(pos).unwrap(),
            Board::default().at_mut(pos).unwrap()
        );
    }

    #[test]
    fn size() {
        assert_eq!(208, mem::size_of::<Board>());
        assert_eq!(8, mem::size_of::<&Board>());
    }
}
