use std::{fs::File, io::Write};

use crate::{Board, Piece, PieceSet};
use crate::{Color, Pos};

fn pos(value: u64, row: usize, col: usize) -> u64 {
    (value << row * 8) << col
}

impl Board {
    pub fn new() -> Self {
        Self {
            white: [
                PieceSet::new(
                    Piece::Pawn(Color::White),
                    pos(0b11111111, 1, 0) + pos(0b1, 5, 3),
                ),
                PieceSet::new(Piece::Rook(Color::White), pos(0b10000001, 0, 0)),
                PieceSet::new(Piece::Knight(Color::White), pos(0b01000010, 0, 0)),
                PieceSet::new(Piece::Bishop(Color::White), pos(0b00100100, 0, 0)),
                PieceSet::new(Piece::Queen(Color::White), pos(1, 0, 3)),
                PieceSet::new(Piece::King(Color::White), pos(1, 0, 4)),
            ],

            black: [
                PieceSet::new(Piece::Pawn(Color::Black), pos(0b11111111, 6, 0)),
                PieceSet::new(Piece::Rook(Color::Black), pos(0b10000001, 7, 0)),
                PieceSet::new(Piece::Knight(Color::Black), pos(0b01000010, 7, 0)),
                PieceSet::new(Piece::Bishop(Color::Black), pos(0b00100100, 7, 0)),
                PieceSet::new(Piece::Queen(Color::Black), pos(1, 7, 3)),
                PieceSet::new(Piece::King(Color::Black), pos(1, 7, 4)),
            ],
        }
    }

    pub fn mov(&mut self, from: &Pos, to: &Pos) {
        let pset = self.at_mut(from).expect("cannot move square without piece");
        pset.mov(from, to);
    }

    pub fn at(&self, pos: &Pos) -> Option<&PieceSet> {
        self.white
            .iter()
            .chain(self.black.iter())
            .find(|piece| piece.at(pos) == 1)
    }

    pub fn at_mut(&mut self, pos: &Pos) -> Option<&mut PieceSet> {
        self.white
            .iter_mut()
            .chain(self.black.iter_mut())
            .find(|piece| piece.at(pos) == 1)
    }

    pub fn save(&self, fname: &str) {
        let mut w = File::create(fname).unwrap();
        self.white.iter().for_each(|pset| {
            w.write(&pset.nle()).unwrap();
        });
        self.black.iter().for_each(|pset| {
            w.write(&pset.nle()).unwrap();
        });
    }
}
