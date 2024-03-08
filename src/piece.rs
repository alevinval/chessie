use std::fmt::Display;

use super::Color;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Piece {
    Pawn(Color),
    Rook(Color, bool, bool),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color, bool),
}

impl Piece {
    pub const fn color(self) -> Color {
        match self {
            Piece::Pawn(c)
            | Piece::Rook(c, _, _)
            | Piece::Knight(c)
            | Piece::Bishop(c)
            | Piece::Queen(c)
            | Piece::King(c, _) => c,
        }
    }

    pub const fn is_king(self) -> bool {
        matches!(self, Piece::King(_, _))
    }

    pub const fn is_pawn(self) -> bool {
        matches!(self, Piece::Pawn(_))
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Piece::Pawn(c) => match c {
                Color::B => "♟",
                Color::W => "♙",
            },
            Piece::Rook(c, _, _) => match c {
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
            Piece::King(c, _) => match c {
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
        let sut = Piece::King(B, false);
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

        assert_eq!("♖", Piece::Rook(W, false, false).as_str());
        assert_eq!("♜", Piece::Rook(B, false, false).as_str());

        assert_eq!("♕", Piece::Queen(W).as_str());
        assert_eq!("♛", Piece::Queen(B).as_str());

        assert_eq!("♔", Piece::King(W, false).as_str());
        assert_eq!("♚", Piece::King(B, false).as_str());
    }

    #[test]
    fn size() {
        assert_eq!(3, mem::size_of::<Piece>());
        assert_eq!(8, mem::size_of::<&Piece>());
    }
}
