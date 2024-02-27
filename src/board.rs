use std::{fs::File, io::Write};

use crate::movement::{Move, MoveGen};
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
        self.white
            .iter()
            .chain(self.black.iter())
            .find(|piece_set| piece_set.has_piece(pos))
    }

    pub fn at_mut(&mut self, pos: Pos) -> Option<&mut BitBoard> {
        self.white
            .iter_mut()
            .chain(self.black.iter_mut())
            .find(|piece| piece.has_piece(pos))
    }

    pub fn save(&self, fname: &str) {
        let mut w = File::create(fname).unwrap();
        self.white.iter().for_each(|pset| {
            w.write_all(&pset.to_le_bytes()).unwrap();
        });
        self.black.iter().for_each(|pset| {
            w.write_all(&pset.to_le_bytes()).unwrap();
        });
    }

    pub fn generate_moves(&self, pos: Pos) -> Vec<Move> {
        self.at(pos).map_or(vec![], |piece_set| {
            MoveGen::new(self, pos).gen(&piece_set.piece())
        })
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
