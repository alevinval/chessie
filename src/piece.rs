use std::fmt::Display;

use super::Color;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

pub type Idx = usize;

impl Piece {
    pub const P: Idx = 0;
    pub const N: Idx = 1;
    pub const B: Idx = 2;
    pub const R: Idx = 3;
    pub const Q: Idx = 4;
    pub const K: Idx = 5;

    pub const fn color(self) -> Color {
        match self {
            Piece::Pawn(c)
            | Piece::Rook(c)
            | Piece::Knight(c)
            | Piece::Bishop(c)
            | Piece::Queen(c)
            | Piece::King(c) => c,
        }
    }

    pub const fn is_king(self) -> bool {
        matches!(self, Piece::King(_))
    }

    pub const fn is_pawn(self) -> bool {
        matches!(self, Piece::Pawn(_))
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Piece::Pawn(c) => match c {
                Color::B => "♟",
                Color::W => "♙",
            },
            Piece::Rook(c) => match c {
                Color::B => "♜",
                Color::W => "♖",
            },
            Piece::Knight(c) => match c {
                Color::B => "♞",
                Color::W => "♘",
            },
            Piece::Bishop(c) => match c {
                Color::B => "♝",
                Color::W => "♗",
            },
            Piece::Queen(c) => match c {
                Color::B => "♛",
                Color::W => "♕",
            },
            Piece::King(c) => match c {
                Color::B => "♚",
                Color::W => "♔",
            },
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod test {

    use std::mem;

    use crate::color::{B, W};

    use super::*;

    #[test]
    fn is_pawn() {
        let sut = Piece::Pawn(B);
        assert!(sut.is_pawn());
        assert!(!sut.is_king());
    }

    #[test]
    fn is_king() {
        let sut = Piece::King(B);
        assert!(sut.is_king());
        assert!(!sut.is_pawn());
    }

    #[test]
    fn color() {
        let sut = Piece::Bishop(B);
        assert!(sut.color() == B);

        let sut = Piece::Bishop(W);
        assert!(sut.color() == W);
    }

    #[test]
    fn as_str() {
        assert_eq!("♙", Piece::Pawn(W).as_str());
        assert_eq!("♟", Piece::Pawn(B).as_str());

        assert_eq!("♗", Piece::Bishop(W).as_str());
        assert_eq!("♝", Piece::Bishop(B).as_str());

        assert_eq!("♘", Piece::Knight(W).as_str());
        assert_eq!("♞", Piece::Knight(B).as_str());

        assert_eq!("♖", Piece::Rook(W).as_str());
        assert_eq!("♜", Piece::Rook(B).as_str());

        assert_eq!("♕", Piece::Queen(W).as_str());
        assert_eq!("♛", Piece::Queen(B).as_str());

        assert_eq!("♔", Piece::King(W).as_str());
        assert_eq!("♚", Piece::King(B).as_str());
    }

    #[test]
    fn size() {
        assert_eq!(2, mem::size_of::<Piece>());
        assert_eq!(8, mem::size_of::<&Piece>());
    }
}
