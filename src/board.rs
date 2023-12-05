use std::{fs::File, io::Write};

use crate::piece::{Color, Piece};

fn pos(value: u64, row: usize, col: usize) -> u64 {
    (value << row * 8) << col
}

pub struct Board {
    white: [Piece; 6],
    black: [Piece; 6],
}

impl Board {
    pub fn new() -> Self {
        Self {
            white: [
                Piece::Pawn(Color::White, pos(0b11111111, 1, 0) + pos(0b1, 5, 3)),
                Piece::Rook(Color::White, pos(0b10000001, 0, 0)),
                Piece::Knight(Color::White, pos(0b01000010, 0, 0)),
                Piece::Bishop(Color::White, pos(0b00100100, 0, 0)),
                Piece::Queen(Color::White, pos(1, 0, 3)),
                Piece::King(Color::White, pos(1, 0, 4)),
            ],

            black: [
                Piece::Pawn(Color::Black, pos(0b11111111, 6, 0)),
                Piece::Rook(Color::White, pos(0b10000001, 7, 0)),
                Piece::Knight(Color::White, pos(0b01000010, 7, 0)),
                Piece::Bishop(Color::White, pos(0b00100100, 7, 0)),
                Piece::Queen(Color::White, pos(1, 7, 3)),
                Piece::King(Color::White, pos(1, 7, 4)),
            ],
        }
    }

    pub fn at(&self, row: usize, col: usize) -> Option<&Piece> {
        self.white
            .iter()
            .chain(self.black.iter())
            .find(|piece| piece.at(row, col) == 1)
    }

    pub fn save(&self, fname: &str) {
        let mut w = File::create(fname).unwrap();
        self.white.iter().for_each(|p| {
            w.write(&p.nle()).unwrap();
        });
        self.black.iter().for_each(|p| {
            w.write(&p.nle()).unwrap();
        });
    }
}
