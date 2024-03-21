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

    #[must_use]
    pub fn get_piece(&self, color: Color, piece: Piece) -> BitBoard {
        match color {
            Color::B => self.black[piece.idx()],
            Color::W => self.white[piece.idx()],
        }
    }

    pub fn pieces_iter(&self, color: Color) -> impl Iterator<Item = (Piece, BitBoard)> + '_ {
        match color {
            Color::B => self.black,
            Color::W => self.white,
        }
        .into_iter()
        .enumerate()
        .map(|(i, bb)| (Piece::from_idx(i), bb))
    }

    pub fn pieces_mut(&mut self) -> &mut [BitBoard; 6] {
        match self.mover {
            Color::B => &mut self.black,
            Color::W => &mut self.white,
        }
    }

    pub fn at<P: Into<Pos>>(&self, pos: P) -> Option<(Color, Piece, BitBoard)> {
        let pos = pos.into();
        self.white
            .into_iter()
            .position(|bb| bb.has_piece(pos))
            .map(|i| (Color::W, Piece::from_idx(i), self.white[i]))
            .or_else(|| {
                self.black
                    .into_iter()
                    .position(|bb| bb.has_piece(pos))
                    .map(|i| (Color::B, Piece::from_idx(i), self.black[i]))
            })
    }

    pub fn at_mut<P: Into<Pos>>(&mut self, pos: P) -> Option<(Color, Piece, &mut BitBoard)> {
        let pos = pos.into();
        self.white
            .iter()
            .position(|bb| bb.has_piece(pos))
            .map(|i| (Color::W, Piece::from_idx(i), &mut self.white[i]))
            .or_else(|| {
                self.black
                    .into_iter()
                    .position(|bb| bb.has_piece(pos))
                    .map(|i| (Color::B, Piece::from_idx(i), &mut self.black[i]))
            })
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
        self.generate_movements(color, true)
    }

    #[must_use]
    pub fn pseudo_movements(&self, color: Color) -> Vec<Move> {
        self.generate_movements(color, false)
    }

    #[must_use]
    pub fn piece_count(&self) -> usize {
        let w: usize = self
            .pieces_iter(Color::W)
            .filter(|(p, _)| *p != Piece::Pawn)
            .map(|(_, bb)| bb.iter_pos(Color::W).count())
            .sum();

        let b: usize = self
            .pieces_iter(Color::B)
            .filter(|(p, _)| *p != Piece::Pawn)
            .map(|(_, bb)| bb.iter_pos(Color::B).count())
            .sum();

        w + b
    }

    pub fn in_check(&self, color: Color) -> bool {
        let king = self.get_piece(color, Piece::King).iter_pos(color).next();

        match king {
            Some(king) => self.pseudo_movements(color.flip()).iter().any(|m| m.to() == king),
            None => true,
        }
    }

    fn generate_movements(&self, color: Color, legal_only: bool) -> Vec<Move> {
        self.pieces_iter(color)
            .flat_map(|(_, bb)| bb.iter_pos(color))
            .flat_map(|p| MoveGen::new(self, p).generate(legal_only))
            .collect()
    }

    fn gen_pieces(color: Color) -> [BitBoard; 6] {
        [
            BitBoard::new(Piece::Pawn, color),
            BitBoard::new(Piece::Knight, color),
            BitBoard::new(Piece::Bishop, color),
            BitBoard::new(Piece::Rook, color),
            BitBoard::new(Piece::Queen, color),
            BitBoard::new(Piece::King, color),
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
    fn at_white_king() {
        let sut = Board::default();
        let king = sut.at((0, 4));
        assert!(king.is_some());

        if let Some((color, piece, _bb)) = king {
            assert_eq!(Color::W, color);
            assert_eq!(Piece::King, piece);
        }
    }

    #[test]
    fn at_black_king() {
        let sut = Board::default();
        let king = sut.at((7, 4));

        assert!(king.is_some());

        if let Some((color, piece, _bb)) = king {
            assert_eq!(Color::B, color);
            assert_eq!(Piece::King, piece);
        }
    }

    #[test]
    fn mut_at_white() {
        let pos = (0, 0);

        assert_eq!(Board::default().at(pos).unwrap().2, *Board::default().at_mut(pos).unwrap().2);
    }

    #[test]
    fn mut_at_black() {
        let pos = (7, 7);

        assert_eq!(Board::default().at(pos).unwrap().2, *Board::default().at_mut(pos).unwrap().2);
    }

    #[test]
    fn size() {
        assert_eq!(208, mem::size_of::<Board>());
        assert_eq!(8, mem::size_of::<&Board>());
    }
}
