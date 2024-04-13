use std::fmt;

use super::Color;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub(crate) type Idx = usize;

impl Piece {
    pub(crate) const PROMO: [Piece; 4] = [Piece::Bishop, Piece::Knight, Piece::Queen, Piece::Rook];

    pub(crate) const P: Idx = 0;
    pub(crate) const N: Idx = 1;
    pub(crate) const B: Idx = 2;
    pub(crate) const R: Idx = 3;
    pub(crate) const Q: Idx = 4;
    pub(crate) const K: Idx = 5;

    #[must_use]
    pub(crate) const fn from_idx(idx: Idx) -> Self {
        match idx {
            Self::P => Self::Pawn,
            Self::N => Self::Knight,
            Self::B => Self::Bishop,
            Self::R => Self::Rook,
            Self::Q => Self::Queen,
            Self::K => Self::King,
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub(crate) const fn idx(self) -> usize {
        match self {
            Self::Bishop => Self::B,
            Self::King => Self::K,
            Self::Knight => Self::N,
            Self::Pawn => Self::P,
            Self::Queen => Self::Q,
            Self::Rook => Self::R,
        }
    }

    #[must_use]
    pub(crate) const fn as_str(self, c: Color) -> &'static str {
        match self {
            Self::Bishop => match c {
                Color::B => "♝",
                Color::W => "♗",
            },
            Self::King => match c {
                Color::B => "♚",
                Color::W => "♔",
            },
            Self::Knight => match c {
                Color::B => "♞",
                Color::W => "♘",
            },
            Self::Pawn => match c {
                Color::B => "♟",
                Color::W => "♙",
            },
            Self::Queen => match c {
                Color::B => "♛",
                Color::W => "♕",
            },
            Self::Rook => match c {
                Color::B => "♜",
                Color::W => "♖",
            },
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str(Color::W))
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case(Piece::Pawn, Color::W, "♙")]
    #[test_case(Piece::Pawn, Color::B, "♟")]
    #[test_case(Piece::Bishop, Color::W, "♗")]
    #[test_case(Piece::Bishop, Color::B, "♝")]
    #[test_case(Piece::Knight, Color::W, "♘")]
    #[test_case(Piece::Knight, Color::B, "♞")]
    #[test_case(Piece::Rook, Color::W, "♖")]
    #[test_case(Piece::Rook, Color::B, "♜")]
    #[test_case(Piece::Queen, Color::W, "♕")]
    #[test_case(Piece::Queen, Color::B, "♛")]
    #[test_case(Piece::King, Color::W, "♔")]
    #[test_case(Piece::King, Color::B, "♚")]
    fn as_str(piece: Piece, color: Color, expected: &str) {
        assert_eq!(expected, piece.as_str(color));
    }

    #[test_case(Piece::Pawn)]
    #[test_case(Piece::Bishop)]
    #[test_case(Piece::Knight)]
    #[test_case(Piece::Rook)]
    #[test_case(Piece::Queen)]
    #[test_case(Piece::King)]
    fn index_symmetry(piece: Piece) {
        assert_eq!(piece, Piece::from_idx(piece.idx()));
    }
}
