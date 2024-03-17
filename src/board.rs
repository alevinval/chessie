use crate::bitboard::Bits;
use crate::defs::BitBoard;

use crate::moves::Generator;
use crate::moves::Move;
use crate::piece::Piece;
use crate::Color;
use crate::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Castling {
    None,
    Some(bool, bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    mover: Color,
    pub white: [BitBoard; 6],
    pub black: [BitBoard; 6],
    white_side: BitBoard,
    black_side: BitBoard,
    occupancy: BitBoard,
    white_rights: Castling,
    black_rights: Castling,
    n: usize,
}

impl Board {
    #[must_use]
    pub fn mover(&self) -> Color {
        self.mover
    }

    #[must_use]
    pub fn castling_rights(&self, color: Color) -> Castling {
        match color {
            Color::B => self.black_rights,
            Color::W => self.white_rights,
        }
    }

    pub fn set_rights(&mut self, color: Color, rights: Castling) {
        match color {
            Color::B => self.black_rights = rights,
            Color::W => self.white_rights = rights,
        }
    }

    #[must_use]
    pub fn get_piece(&self, color: Color, piece: Piece) -> BitBoard {
        match color {
            Color::B => self.black[piece.idx()],
            Color::W => self.white[piece.idx()],
        }
    }

    #[must_use]
    pub fn side(&self, color: Color) -> BitBoard {
        match color {
            Color::B => self.black_side,
            Color::W => self.white_side,
        }
    }

    #[must_use]
    pub fn occupancy(&self) -> BitBoard {
        self.occupancy
    }

    fn calc_side(bbs: [BitBoard; 6]) -> BitBoard {
        let mut side: BitBoard = 0;
        for b in bbs {
            side |= b;
        }
        side
    }

    pub fn pieces_iter(&self, color: Color) -> impl Iterator<Item = (Piece, BitBoard)> + '_ {
        match color {
            Color::B => self.black,
            Color::W => self.white,
        }
        .into_iter()
        .enumerate()
        .map(|(idx, bb)| (Piece::from_idx(idx), bb))
    }

    pub fn pieces_mut(&mut self) -> &mut [BitBoard; 6] {
        match self.mover {
            Color::B => &mut self.black,
            Color::W => &mut self.white,
        }
    }

    pub fn apply_promo(&mut self, pos: Pos, piece: Piece) {
        match self.mover {
            Color::B => Bits::set(&mut self.black[piece.idx()], pos),
            Color::W => Bits::set(&mut self.white[piece.idx()], pos),
        }
    }

    pub fn at<P: Into<Pos>>(&self, pos: P) -> Option<(Color, Piece, &BitBoard)> {
        let pos = pos.into();
        self.white
            .iter()
            .enumerate()
            .find(|(_, bb)| Bits::has_piece(**bb, pos))
            .map(|(idx, bb)| (Color::W, idx, bb))
            .or_else(|| {
                self.black
                    .iter()
                    .enumerate()
                    .find(|(_, bb)| Bits::has_piece(**bb, pos))
                    .map(|(idx, bb)| (Color::B, idx, bb))
            })
            .map(|(c, idx, bb)| (c, Piece::from_idx(idx), bb))
    }

    pub fn at_mut<P: Into<Pos>>(&mut self, pos: P) -> Option<(Color, Piece, &mut BitBoard)> {
        let pos = pos.into();
        self.white
            .iter_mut()
            .enumerate()
            .find(|(_, bb)| Bits::has_piece(**bb, pos))
            .map(|(idx, bb)| (Color::W, idx, bb))
            .or_else(|| {
                self.black
                    .iter_mut()
                    .enumerate()
                    .find(|(_, bb)| Bits::has_piece(**bb, pos))
                    .map(|(idx, bb)| (Color::B, idx, bb))
            })
            .map(|(c, idx, bb)| (c, Piece::from_idx(idx), bb))
    }

    pub fn next_turn(&mut self) {
        self.white_side = Self::calc_side(self.white);
        self.black_side = Self::calc_side(self.black);
        self.occupancy = self.white_side | self.black_side;
        self.mover = self.mover.flip();
        self.n += 1;
    }

    #[must_use]
    pub fn n(&self) -> usize {
        self.n
    }

    #[must_use]
    pub fn movements(&self, color: Color) -> Vec<Move> {
        self.pieces_iter(color)
            .flat_map(|(_, bb)| {
                let moves: Vec<_> = Bits::pos(bb)
                    .iter()
                    .map(|p| Generator::from_board(self, *p, true).generate())
                    .collect();
                moves
            })
            .flatten()
            .collect()
    }

    #[must_use]
    pub fn pseudo_movements(&self, color: Color) -> Vec<Move> {
        self.pieces_iter(color)
            .flat_map(|(_, bb)| {
                let moves: Vec<_> = Bits::pos(bb)
                    .iter()
                    .map(|p| Generator::from_board(self, *p, false).generate())
                    .collect();
                moves
            })
            .flatten()
            .collect()
    }

    #[must_use]
    pub fn piece_count(&self) -> usize {
        let w: usize = self
            .pieces_iter(Color::W)
            .filter(|(p, _)| *p != Piece::Pawn)
            .map(|(_, bb)| Bits::count(bb))
            .sum();

        let b: usize = self
            .pieces_iter(Color::B)
            .filter(|(p, _)| *p != Piece::Pawn)
            .map(|(_, bb)| Bits::count(bb))
            .sum();

        w + b
    }

    #[must_use]
    pub fn in_check(&self, color: Color) -> bool {
        let king = Bits::pos(self.get_piece(color, Piece::King));
        let king = king.first();

        match king {
            Some(king) => self.pseudo_movements(color.flip()).iter().any(|m| m.to() == *king),
            None => true,
        }
    }

    fn gen_pieces(color: Color) -> [BitBoard; 6] {
        [
            Bits::init(Piece::Pawn, color),
            Bits::init(Piece::Knight, color),
            Bits::init(Piece::Bishop, color),
            Bits::init(Piece::Rook, color),
            Bits::init(Piece::Queen, color),
            Bits::init(Piece::King, color),
        ]
    }

    pub fn clear(&mut self) {
        self.white.iter_mut().for_each(|bb| *bb = 0);
        self.black.iter_mut().for_each(|bb| *bb = 0);
        self.white_rights = Castling::None;
        self.black_rights = Castling::None;
    }
}

impl Default for Board {
    fn default() -> Self {
        let white = Self::gen_pieces(Color::W);
        let black = Self::gen_pieces(Color::B);
        let white_side = Self::calc_side(white);
        let black_side = Self::calc_side(black);
        Self {
            mover: Color::W,
            white,
            black,
            white_side,
            black_side,
            occupancy: white_side | black_side,
            white_rights: Castling::Some(true, true),
            black_rights: Castling::Some(true, true),
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
    fn piece_count() {
        let sut = Board::default();
        assert_eq!(16, sut.piece_count());
    }

    #[test]
    fn at_white_king() {
        let sut = Board::default();
        let king = sut.at((0, 4));

        assert!(king.is_some());

        if let Some((color, piece, _)) = king {
            assert_eq!(Color::W, color);
            assert_eq!(Piece::King, piece);
        }
    }

    #[test]
    fn at_black_king() {
        let sut = Board::default();
        let king = sut.at((7, 4));

        assert!(king.is_some());

        if let Some((color, piece, _)) = king {
            assert_eq!(Color::B, color);
            assert_eq!(Piece::King, piece);
        }
    }

    #[test]
    fn mut_at_white() {
        let pos = (0, 0);

        assert_eq!(Board::default().at(pos).unwrap().1, Board::default().at_mut(pos).unwrap().1);
    }

    #[test]
    fn mut_at_black() {
        let pos = (7, 7);

        assert_eq!(Board::default().at(pos).unwrap().1, Board::default().at_mut(pos).unwrap().1);
    }

    #[test]
    fn size() {
        assert_eq!(128, mem::size_of::<Board>());
        assert_eq!(8, mem::size_of::<&Board>());
    }
}
