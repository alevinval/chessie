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
    pub fn color(self) -> Color {
        match self {
            Piece::Pawn(c)
            | Piece::Rook(c, _, _)
            | Piece::Knight(c)
            | Piece::Bishop(c)
            | Piece::Queen(c)
            | Piece::King(c, _) => c,
        }
    }

    pub fn is_king(self) -> bool {
        matches!(self, Piece::King(_, _))
    }

    pub fn is_pawn(self) -> bool {
        matches!(self, Piece::Pawn(_))
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Piece::Pawn(c) => match c {
                Color::Black => "♟",
                Color::White => "♙",
            },
            Piece::Rook(c, _, _) => match c {
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
            Piece::King(c, _) => match c {
                Color::Black => "♚",
                Color::White => "♔",
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
    use super::*;

    #[test]
    fn is_pawn() {
        let sut = Piece::Pawn(Color::Black);
        assert!(sut.is_pawn(), "should be pawn");
    }

    #[test]
    fn color() {
        let sut = Piece::Bishop(Color::Black);
        assert!(sut.color() == Color::Black, "should be black");

        let sut = Piece::Bishop(Color::White);
        assert!(sut.color() == Color::White, "should be white");
    }
}
