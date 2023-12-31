use std::{fs::File, io::Write};

#[cfg(test)]
use crate::pieces::Piece;
use crate::pieces::{BitBoard, Color, PieceSet, Pieces};
use crate::pos::Pos;

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

    #[cfg(test)]
    pub fn clear(&mut self) {
        self.white.clear();
        self.black.clear();
    }

    #[cfg(test)]
    pub fn set(&mut self, pos: Pos, piece: Piece) {
        let pieces = match piece.color() {
            Color::Black => &mut self.black,
            Color::White => &mut self.white,
        };
        if let Some(ps) = pieces.0.iter_mut().find(|ps| ps.piece == piece) {
            ps.bitboard.or_mut(pos);
        }
    }

    pub fn apply_move(&mut self, from: Pos, to: Pos) {
        let pset = self.at_mut(from).expect("cannot move square without piece");
        pset.apply_move(from, to);
    }

    pub fn at(&self, pos: Pos) -> Option<&PieceSet> {
        self.white
            .0
            .iter()
            .chain(self.black.0.iter())
            .find(|piece_set| !piece_set.at(pos).is_empty())
    }

    pub fn at_mut(&mut self, pos: Pos) -> Option<&mut PieceSet> {
        self.white
            .0
            .iter_mut()
            .chain(self.black.0.iter_mut())
            .find(|piece| !piece.at(pos).is_empty())
    }

    pub fn save(&self, fname: &str) {
        let mut w = File::create(fname).unwrap();
        self.white.0.iter().for_each(|pset| {
            w.write_all(&pset.bitboard.to_le_bytes()).unwrap();
        });
        self.black.0.iter().for_each(|pset| {
            w.write_all(&pset.bitboard.to_le_bytes()).unwrap();
        });
    }

    pub fn generate_moves(&self, pos: Pos) -> BitBoard {
        self.at(pos)
            .map_or(BitBoard(0), |piece_set| piece_set.piece.movements(pos))
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

    static COLOR: Color = Color::White;

    #[test]
    fn generates_all_positions() {
        let mut sut = Board::new();
        sut.clear();
        sut.white = Pieces::new(COLOR);

        let positions: Vec<Pos> = (0..8)
            .flat_map(|row| (0..8).map(move |col| Pos(row, col)))
            .collect();

        for pos in positions {
            for i in 0..sut.white.0.len() {
                sut.white.0[i].bitboard = pos.into();
                let gen = vec![sut.generate_moves(pos)];
                print_board(&sut, &gen);
                assert!(
                    gen[0] != 0.into()
                        || (sut.white.0[i].piece.is_pawn() && (pos.row() == 7 || pos.row() == 0))
                );
                sut.clear();
            }
        }
    }
}
