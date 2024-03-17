use super::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

pub type PieceIdx = usize;

impl Piece {
    pub const PROMO: [Piece; 4] = [Piece::Bishop, Piece::Knight, Piece::Queen, Piece::Rook];

    pub const P: PieceIdx = 0;
    pub const N: PieceIdx = 1;
    pub const B: PieceIdx = 2;
    pub const R: PieceIdx = 3;
    pub const Q: PieceIdx = 4;
    pub const K: PieceIdx = 5;

    pub const fn from_idx(idx: PieceIdx) -> Self {
        match idx {
            Self::P => Self::Pawn,
            Self::N => Self::Knight,
            Self::B => Self::Bishop,
            Self::R => Self::Rook,
            Self::Q => Self::Queen,
            Self::K => Self::King,
            _ => panic!("incorrect piece index"),
        }
    }

    pub const fn idx(self) -> usize {
        match self {
            Self::Bishop => Self::B,
            Self::King => Self::K,
            Self::Knight => Self::N,
            Self::Pawn => Self::P,
            Self::Queen => Self::Q,
            Self::Rook => Self::R,
        }
    }

    pub const fn as_str(self, c: Color) -> &'static str {
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

#[cfg(test)]
mod test {

    use std::mem;

    use crate::color::{B, W};

    use super::*;

    #[test]
    fn as_str() {
        assert_eq!("♙", Piece::Pawn.as_str(W));
        assert_eq!("♟", Piece::Pawn.as_str(B));

        assert_eq!("♗", Piece::Bishop.as_str(W));
        assert_eq!("♝", Piece::Bishop.as_str(B));

        assert_eq!("♘", Piece::Knight.as_str(W));
        assert_eq!("♞", Piece::Knight.as_str(B));

        assert_eq!("♖", Piece::Rook.as_str(W));
        assert_eq!("♜", Piece::Rook.as_str(B));

        assert_eq!("♕", Piece::Queen.as_str(W));
        assert_eq!("♛", Piece::Queen.as_str(B));

        assert_eq!("♔", Piece::King.as_str(W));
        assert_eq!("♚", Piece::King.as_str(B));
    }

    #[test]
    fn size() {
        assert_eq!(1, mem::size_of::<Piece>());
        assert_eq!(8, mem::size_of::<&Piece>());
    }
}
