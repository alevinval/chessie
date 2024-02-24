use std::{fs::File, io::Write};

use crate::pieces::{BitBoard, Color, PieceSet, Pieces};
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

    pub fn apply_move(&mut self, from: Pos, to: Pos) {
        if self.at(from).is_none() {
            return;
        }

        if let Some(dst) = self.at_mut(to) {
            dst.bitboard.xor_mut(to);
        }

        self.at_mut(from)
            .expect("cannot move square without piece")
            .apply_move(from, to);
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
            w.write_all(&pset.bitboard.to_le_bytes()).unwrap();
        });
        self.black.iter().for_each(|pset| {
            w.write_all(&pset.bitboard.to_le_bytes()).unwrap();
        });
    }

    pub fn generate_moves(&self, pos: Pos) -> BitBoard {
        self.at(pos).map_or(BitBoard::default(), |piece_set| {
            piece_set.piece.movements(self, pos)
        })
    }

    pub fn evaluate(&self, color: Color) -> f32 {
        let white: f32 = self
            .white
            .iter()
            .map(|ps| ps.bitboard.positions().len() as f32 * ps.piece.score())
            .sum();
        let black: f32 = self
            .black
            .iter()
            .map(|ps| ps.bitboard.positions().len() as f32 * ps.piece.score())
            .sum();

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

#[cfg(test)]
mod test {

    use crate::print_board;

    use super::*;

    // #[test]
    // fn generates_all_positions() {
    //     let mut sut = Board::new();
    //     sut.clear();
    //     sut.white = Pieces::new(Color::White);

    //     let positions: Vec<Pos> = (0..8)
    //         .flat_map(|row| (0..8).map(move |col| Pos(row, col)))
    //         .collect();

    //     for pos in positions {
    //         for piece_set in sut.white.iter() {
    //             let gen = vec![sut.generate_moves(pos)];
    //             print_board(&sut, &gen);
    //             println!("pos={pos:?} gen={gen:?}");
    //             assert!(
    //                 gen[0] != 0.into()
    //                     || (piece_set.piece.is_pawn() && (pos.row() == 7 || pos.row() == 0))
    //             );
    //         }
    //     }
    // }
}
