use crate::bitboard::BitBoard;
use crate::moves::{Move, MoveGen};
use crate::piece::Piece;
use crate::pos::Pos;
use crate::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Castling {
    None,
    Some(bool, bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    mover: Color,
    white: [BitBoard; 6],
    black: [BitBoard; 6],
    white_castling: Castling,
    black_castling: Castling,
    n: usize,
}

impl Board {
    pub fn mover(&self) -> Color {
        self.mover
    }

    #[must_use]
    pub const fn castling(&self, color: Color) -> Castling {
        match color {
            Color::B => self.black_castling,
            Color::W => self.white_castling,
        }
    }

    pub fn set_castling(&mut self, color: Color, rights: Castling) {
        match color {
            Color::B => self.black_castling = rights,
            Color::W => self.white_castling = rights,
        }
    }

    pub fn pieces(&self) -> &[BitBoard; 6] {
        self.pieces_for(self.mover)
    }

    pub fn pieces_for(&self, color: Color) -> &[BitBoard; 6] {
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

    pub fn at<P: Into<Pos>>(&self, pos: P) -> Option<BitBoard> {
        let pos = pos.into();
        self.white
            .into_iter()
            .find(|b| b.has_piece(pos))
            .or_else(|| self.black.into_iter().find(|b| b.has_piece(pos)))
    }

    pub fn at_mut<P: Into<Pos>>(&mut self, pos: P) -> Option<&mut BitBoard> {
        let pos = pos.into();
        self.white
            .iter_mut()
            .find(|b| b.has_piece(pos))
            .or_else(|| self.black.iter_mut().find(|b| b.has_piece(pos)))
    }

    pub fn next_turn(&mut self) {
        self.mover = self.mover.flip();
        self.n += 1;
    }

    pub fn n(&self) -> usize {
        self.n
    }

    #[must_use]
    pub fn movements(&self, color: Color) -> Vec<Move> {
        self.pieces_for(color)
            .iter()
            .flat_map(BitBoard::iter_pos)
            .flat_map(|p| MoveGen::new(self, p).generate(true))
            .collect()
    }

    #[must_use]
    pub fn pseudo_movements(&self, color: Color) -> Vec<Move> {
        self.pieces_for(color)
            .iter()
            .flat_map(BitBoard::iter_pos)
            .flat_map(|p| MoveGen::new(self, p).generate(false))
            .collect()
    }

    #[must_use]
    pub fn piece_count(&self) -> usize {
        let w: usize = self
            .pieces_for(Color::W)
            .iter()
            .filter(|bb| !bb.piece().is_pawn())
            .map(|bb| bb.iter_pos().count())
            .sum();

        let b: usize = self
            .pieces_for(Color::B)
            .iter()
            .filter(|bb| !bb.piece().is_pawn())
            .map(|bb| bb.iter_pos().count())
            .sum();

        w + b
    }

    pub fn in_check(&self, color: Color) -> bool {
        let king = self.pieces_for(color)[Piece::K].iter_pos().next();

        match king {
            Some(king) => self.pseudo_movements(color.flip()).iter().any(|m| m.to() == king),
            None => true,
        }
    }

    fn gen_pieces(color: Color) -> [BitBoard; 6] {
        [
            Piece::Pawn(color).into(),
            Piece::Knight(color).into(),
            Piece::Bishop(color).into(),
            Piece::Rook(color).into(),
            Piece::Queen(color).into(),
            Piece::King(color).into(),
        ]
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            mover: Color::W,
            white: Board::gen_pieces(Color::W),
            black: Board::gen_pieces(Color::B),
            white_castling: Castling::Some(true, true),
            black_castling: Castling::Some(true, true),
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
    fn pieces() {
        let sut = Board::default();
        assert_eq!(&sut.white, sut.pieces());
    }

    #[test]
    fn pieces_for() {
        let sut = Board::default();
        assert_eq!(&sut.black, sut.pieces_for(Color::B));
    }

    #[test]
    fn at_white_king() {
        let sut = Board::default();
        let king = sut.at((0, 4));

        assert!(king.is_some());

        if let Some(king) = king {
            assert_eq!(Color::W, king.color());
            assert_eq!(Piece::King(Color::W), king.piece());
        }
    }

    #[test]
    fn at_black_king() {
        let sut = Board::default();
        let king = sut.at((7, 4));

        assert!(king.is_some());

        if let Some(king) = king {
            assert_eq!(Color::B, king.color());
            assert_eq!(Piece::King(Color::B), king.piece());
        }
    }

    #[test]
    fn mut_at_white() {
        let pos = (0, 0);

        assert_eq!(Board::default().at(pos).unwrap(), *Board::default().at_mut(pos).unwrap());
    }

    #[test]
    fn mut_at_black() {
        let pos = (7, 7);

        assert_eq!(Board::default().at(pos).unwrap(), *Board::default().at_mut(pos).unwrap());
    }

    #[test]
    fn size() {
        assert_eq!(208, mem::size_of::<Board>());
        assert_eq!(8, mem::size_of::<&Board>());
    }
}
