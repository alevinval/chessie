use std::fmt::Display;

use crate::board::Board;
use crate::pos::Pos;

use super::generator::Generator;
use super::movement;
use super::BitBoard;
use super::Color;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub fn movements(&self, board: &Board, pos: Pos) -> BitBoard {
        let g = Generator::new(board, pos);
        match self {
            Piece::Pawn(c) => match c {
                Color::Black => movement::black_pawn(g),
                Color::White => movement::white_pawn(g),
            },
            Piece::Rook(_) => movement::rook(g),
            Piece::Bishop(_) => movement::bishop(g),
            Piece::Queen(_) => movement::queen(g),
            Piece::Knight(_) => movement::knight(g),
            Piece::King(_) => movement::king(g),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Piece::Pawn(c)
            | Piece::Rook(c)
            | Piece::Knight(c)
            | Piece::Bishop(c)
            | Piece::Queen(c)
            | Piece::King(c) => *c,
        }
    }

    #[cfg(test)]
    pub fn is_pawn(&self) -> bool {
        matches!(self, Piece::Pawn(_))
    }

    pub fn score(&self) -> f32 {
        match self {
            Piece::Pawn(_) => 1.0,
            Piece::Rook(_) => 5.0,
            Piece::Knight(_) => 2.5,
            Piece::Bishop(_) => 3.0,
            Piece::Queen(_) => 9.0,
            Piece::King(_) => 25.0,
        }
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
        assert!(sut.color() == Color::Black, "should be pawn");

        let sut = Piece::Bishop(Color::White);
        assert!(sut.color() == Color::White, "should not be pawn");
    }
}
