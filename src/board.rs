use std::{fs::File, io::Write};

use crate::movement::generator::Movements;
use crate::movement::MoveGen;
use crate::pieces::{Color, Piece, PieceSet, Pieces};
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

    pub fn apply_move<P: Into<Pos>>(&mut self, from: P, to: P) {
        let from = from.into();
        let to = to.into();

        if self.at(from).is_none() {
            return;
        }

        if let Some(dst) = self.at_mut(to) {
            dst.unset(to)
        }

        self.at_mut(from)
            .expect("cannot move square without piece")
            .apply_move(from, to);
    }

    pub fn pieces(&self, color: Color) -> &Pieces {
        match color {
            Color::Black => &self.black,
            Color::White => &self.white,
        }
    }

    pub fn at(&self, pos: Pos) -> Option<&PieceSet> {
        self.white
            .iter()
            .chain(self.black.iter())
            .find(|piece_set| !piece_set.at(pos).is_empty())
    }

    pub fn at_mut(&mut self, pos: Pos) -> Option<&mut PieceSet> {
        self.white
            .iter_mut()
            .chain(self.black.iter_mut())
            .find(|piece| !piece.at(pos).is_empty())
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

    pub fn generate_moves(&self, pos: Pos) -> Movements {
        self.at(pos).map_or(Movements::default(), |piece_set| {
            MoveGen::new(self, pos).gen(&piece_set.piece())
        })
    }

    pub fn eval(&self, color: Color, print: bool) -> f32 {
        let mut white: f32 = self.white.iter().map(|ps| ps.score()).sum();
        let mut black: f32 = self.black.iter().map(|ps| ps.score()).sum();

        if print {
            println!("material score");
            println!(" white: {white}");
            println!(" black: {black}");
        }

        let white_space_score: f32 = self
            .white
            .get(Piece::Pawn(Color::White))
            .positions()
            .map(|p| (p.row() as f32 - 1.0) * if p.is_central() { 1.2 } else { 1.0 })
            .sum();

        let black_space_score: f32 = self
            .black
            .get(Piece::Pawn(Color::Black))
            .positions()
            .map(|p| (p.row() as f32 - 6.0) * if p.is_central() { -1.2 } else { -1.0 })
            .sum();

        white += white_space_score / 100.0;
        black += black_space_score / 100.0;

        if print {
            println!("space score");
            println!(" white: {white_space_score}");
            println!(" black: {black_space_score}");
        }

        // self.white.get(Piece::King(Color::White)).movements(self, )

        match color {
            Color::Black => black - white,
            Color::White => white - black,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
