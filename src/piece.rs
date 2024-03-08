use super::Color;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    pub const fn is_king(self) -> bool {
        matches!(self, Piece::King)
    }

    pub const fn is_pawn(self) -> bool {
        matches!(self, Piece::Pawn)
    }

    pub fn as_str(self, c: Color) -> &'static str {
        match self {
            Piece::Pawn => match c {
                Color::B => "♟",
                Color::W => "♙",
            },
            Piece::Rook => match c {
                Color::B => "♜",
                Color::W => "♖",
            },
            Piece::Knight => match c {
                Color::B => "♞",
                Color::W => "♘",
            },
            Piece::Bishop => match c {
                Color::B => "♝",
                Color::W => "♗",
            },
            Piece::Queen => match c {
                Color::B => "♛",
                Color::W => "♕",
            },
            Piece::King => match c {
                Color::B => "♚",
                Color::W => "♔",
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
    fn is_pawn() {
        let sut = Piece::Pawn;
        assert!(sut.is_pawn());
        assert!(!sut.is_king());
    }

    #[test]
    fn is_king() {
        let sut = Piece::King;
        assert!(sut.is_king());
        assert!(!sut.is_pawn());
    }

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
