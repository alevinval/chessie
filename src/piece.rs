use crate::{Color, Piece};

impl Piece {
    pub fn to_str(&self) -> &str {
        match self {
            Piece::Pawn(c) => match c {
                Color::Black => "♟",
                Color::White => "♙",
            },
            Piece::Rook(c) => match c {
                Color::Black => "♜",
                Color::White => "♖",
            },
            Piece::Knight(c) => match c {
                Color::Black => "♞",
                Color::White => "♘",
            },
            Piece::Bishop(c) => match c {
                Color::Black => "♝",
                Color::White => "♗",
            },
            Piece::Queen(c) => match c {
                Color::Black => "♛",
                Color::White => "♕",
            },
            Piece::King(c) => match c {
                Color::Black => "♚",
                Color::White => "♔",
            },
        }
    }

    pub fn is_pawn(&self) -> bool {
        match self {
            Piece::Pawn(_) => true,
            _ => false,
        }
    }

    pub fn color(&self) -> &Color {
        match self {
            Piece::Pawn(c)
            | Piece::Rook(c)
            | Piece::Knight(c)
            | Piece::Bishop(c)
            | Piece::Queen(c)
            | Piece::King(c) => c,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_pawn() {
        let sut = Piece::Pawn(Color::Black);
        assert!(sut.is_pawn(), "should be pawn");

        let sut = Piece::Rook(Color::Black);
        assert!(!sut.is_pawn(), "should not be pawn");
    }

    #[test]
    fn color() {
        let sut = Piece::Bishop(Color::Black);
        assert!(sut.color() == &Color::Black, "should be pawn");

        let sut = Piece::Bishop(Color::White);
        assert!(sut.color() == &Color::White, "should not be pawn");
    }
}
