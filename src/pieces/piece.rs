use std::fmt::Display;

use crate::pos::Pos;

use super::movement;
use super::BitBoard;
use super::Color;

#[derive(PartialEq, Clone, Copy)]
pub enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    // TODO: It will need access to full current board and past bitboards for castling or
    // en-passant rules, etc.
    pub fn movements(&self, pos: Pos) -> BitBoard {
        match self {
            Piece::Pawn(c) => match c {
                Color::Black => movement::black_pawn(pos),
                Color::White => movement::white_pawn(pos),
            },
            Piece::Rook(_) => movement::rook(pos),
            Piece::Bishop(_) => movement::bishop(pos),
            Piece::Queen(_) => movement::queen(pos),
            Piece::Knight(_) => movement::knight(pos),
            Piece::King(_) => movement::king(pos),
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

    pub fn is_pawn(&self) -> bool {
        matches!(self, Piece::Pawn(_))
    }

    pub fn as_str(&self) -> &'static str {
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
