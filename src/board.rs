use std::fs::File;
use std::io::Write;

use crate::moves::{Move, MoveGen};
use crate::pieces::{BitBoard, Color, Pieces};
use crate::pos::Pos;

#[derive(Debug, Clone)]
pub struct Board {
    white: Pieces,
    black: Pieces,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white: Pieces::new(Color::White),
            black: Pieces::new(Color::Black),
        }
    }

    pub fn pieces(&self, color: Color) -> &Pieces {
        match color {
            Color::Black => &self.black,
            Color::White => &self.white,
        }
    }

    pub fn at(&self, pos: Pos) -> Option<&BitBoard> {
        self.white.at(pos).or(self.black.at(pos))
    }

    pub fn at_mut(&mut self, pos: Pos) -> Option<&mut BitBoard> {
        self.white.at_mut(pos).or(self.black.at_mut(pos))
    }

    pub fn save(&self, fname: &str) {
        let mut w = File::create(fname).unwrap();
        self.white.iter().for_each(|bb| {
            w.write_all(&bb.to_le_bytes()).unwrap();
        });
        self.black.iter().for_each(|bb| {
            w.write_all(&bb.to_le_bytes()).unwrap();
        });
    }

    pub fn generate_moves(&self, mover: Color, pos: Pos) -> Vec<Move> {
        self.pieces(mover)
            .at(pos)
            .map_or(vec![], |bitboard| MoveGen::new(self, bitboard, pos).gen())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
