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
    pub white: [BitBoard; 6],
    pub black: [BitBoard; 6],
    white_rights: Castling,
    black_rights: Castling,
    n: usize,
}

impl Board {
    pub fn mover(&self) -> Color {
        self.mover
    }

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
        .map(|(idx, bb)| (Piece::from_idx(idx), bb))
    }

    pub fn pieces_mut(&mut self) -> &mut [BitBoard; 6] {
        match self.mover {
            Color::B => &mut self.black,
            Color::W => &mut self.white,
        }
    }

    pub fn apply_promo<P: Into<Pos>>(&mut self, pos: P, piece: Piece) {
        match self.mover {
            Color::B => self.black[piece.idx()].set(pos.into()),
            Color::W => self.white[piece.idx()].set(pos.into()),
        }
    }

    pub fn at<P: Into<Pos>>(&self, pos: P) -> Option<(Color, Piece, &BitBoard)> {
        let pos = pos.into();

        self.white
            .iter()
            .enumerate()
            .find(|(_, bb)| bb.has_piece(pos))
            .map(|(idx, bb)| (Color::W, idx, bb))
            .or_else(|| {
                self.black
                    .iter()
                    .enumerate()
                    .find(|(_, bb)| bb.has_piece(pos))
                    .map(|(idx, bb)| (Color::B, idx, bb))
            })
            .map(|(c, idx, bb)| (c, Piece::from_idx(idx), bb))
    }

    pub fn at_mut<P: Into<Pos>>(&mut self, pos: P) -> Option<(Color, Piece, &mut BitBoard)> {
        let pos = pos.into();

        self.white
            .iter_mut()
            .enumerate()
            .find(|(_, bb)| bb.has_piece(pos))
            .map(|(idx, bb)| (Color::W, idx, bb))
            .or_else(|| {
                self.black
                    .iter_mut()
                    .enumerate()
                    .find(|(_, bb)| bb.has_piece(pos))
                    .map(|(idx, bb)| (Color::B, idx, bb))
            })
            .map(|(c, idx, bb)| (c, Piece::from_idx(idx), bb))
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
        self.pieces_iter(color)
            .flat_map(|(_, bb)| {
                let moves: Vec<_> = bb
                    .iter_pos()
                    .map(|p| MoveGen::new(self, p).generate(true))
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
                let moves: Vec<_> = bb
                    .iter_pos()
                    .map(|p| MoveGen::new(self, p).generate(false))
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
            .map(|(_, bb)| bb.count())
            .sum();

        let b: usize = self
            .pieces_iter(Color::B)
            .filter(|(p, _)| *p != Piece::Pawn)
            .map(|(_, bb)| bb.count())
            .sum();

        w + b
    }

    pub fn in_check(&self, color: Color) -> bool {
        let king = self.get_piece(color, Piece::King).iter_pos().next();

        match king {
            Some(king) => self
                .pseudo_movements(color.opposite())
                .iter()
                .any(|m| m.to() == king),
            None => true,
        }
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
            white: Self::gen_pieces(Color::W),
            black: Self::gen_pieces(Color::B),
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

        assert_eq!(
            Board::default().at(pos).unwrap().1,
            Board::default().at_mut(pos).unwrap().1
        );
    }

    #[test]
    fn mut_at_black() {
        let pos = (7, 7);

        assert_eq!(
            Board::default().at(pos).unwrap().1,
            Board::default().at_mut(pos).unwrap().1
        );
    }

    #[test]
    fn size() {
        assert_eq!(112, mem::size_of::<Board>());
        assert_eq!(8, mem::size_of::<&Board>());
    }
}
