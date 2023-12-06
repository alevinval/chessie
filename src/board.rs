use std::iter::zip;
use std::{fs::File, io::Write};

use crate::{Board, Direction, Piece, PieceSet};
use crate::{Color, Pos};

fn pos(value: u64, row: usize, col: usize) -> u64 {
    (value << row * 8) << col
}

impl Board {
    pub fn new() -> Self {
        Self {
            white: [
                PieceSet::new(Piece::Pawn(Color::White), pos(0b11111111, 1, 0)),
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

    pub fn clear(&mut self) {
        for pboard in self.white.iter_mut().chain(self.black.iter_mut()) {
            pboard.clear();
        }
    }

    pub fn set(&mut self, pos: &Pos, piece: &Piece) {
        let pset = match piece.color() {
            Color::Black => &mut self.black,
            Color::White => &mut self.white,
        };
        pset.iter_mut()
            .find(|ps| ps.piece == *piece)
            .map(|ps| ps.bit_board |= pos.as_bit_board());
    }

    pub fn apply_move(&mut self, from: &Pos, to: &Pos) {
        let pset = self.at_mut(from).expect("cannot move square without piece");
        pset.apply_move(from, to);
    }

    pub fn at(&self, pos: &Pos) -> Option<&PieceSet> {
        self.white
            .iter()
            .chain(self.black.iter())
            .find(|piece| piece.at(pos) > 0)
    }

    pub fn at_mut(&mut self, pos: &Pos) -> Option<&mut PieceSet> {
        self.white
            .iter_mut()
            .chain(self.black.iter_mut())
            .find(|piece| piece.at(pos) > 0)
    }

    pub fn save(&self, fname: &str) {
        let mut w = File::create(fname).unwrap();
        self.white.iter().for_each(|pset| {
            w.write(&pset.to_le_bytes()).unwrap();
        });
        self.black.iter().for_each(|pset| {
            w.write(&pset.to_le_bytes()).unwrap();
        });
    }

    pub fn generate_moves(&self, pos: &Pos) -> Vec<Pos> {
        self.at(pos).map_or(vec![], |set| {
            let mut moves = vec![];
            match &set.piece {
                Piece::Pawn(c) => match c {
                    Color::Black => {
                        if pos.row() > 0 {
                            moves.push(pos.to(Direction::Bottom));
                            if pos.col() < 7 {
                                moves.push(pos.to(Direction::BottomRight));
                            }
                            if pos.col() > 0 {
                                moves.push(pos.to(Direction::BottomLeft));
                            }
                            if pos.row() == 6 {
                                moves.push(pos.to(Direction::Bottom).to(Direction::Bottom));
                            }
                        }
                    }
                    Color::White => {
                        if pos.row() < 7 {
                            moves.push(pos.to(Direction::Top));
                            if pos.row() == 1 {
                                moves.push(pos.to(Direction::Top).to(Direction::Top));
                            }
                            if pos.col() < 7 {
                                moves.push(pos.to(Direction::TopRight));
                            }
                            if pos.col() > 0 {
                                moves.push(pos.to(Direction::TopLeft));
                            }
                        }
                    }
                },
                Piece::Rook(_) => cross(pos, &mut moves),
                Piece::Bishop(_) => diagonals(pos, &mut moves),
                Piece::Queen(_) => {
                    cross(pos, &mut moves);
                    diagonals(pos, &mut moves);
                }
                Piece::Knight(_) | Piece::King(_) => (),
            };

            println!("moves for {pos:?}");
            println!("{moves:?}");
            moves
        })
    }
}

fn cross(pos: &Pos, out: &mut Vec<Pos>) {
    for r in (0..pos.row()).chain(pos.row() + 1..8) {
        out.push(Pos(r, pos.col()));
    }
    for c in (0..pos.col()).chain(pos.col() + 1..8) {
        out.push(Pos(pos.row(), c));
    }
}

fn diagonals(pos: &Pos, out: &mut Vec<Pos>) {
    for (row, col) in zip(pos.row() + 1..8, pos.col() + 1..8) {
        out.push(Pos(row, col));
    }
    for (row, col) in zip(0..pos.row(), pos.col() + 1..8) {
        out.push(Pos(pos.row() - 1 - row, col));
    }
    for (row, col) in zip(pos.row() + 1..8, 0..pos.col()) {
        out.push(Pos(row, pos.col() - col - 1));
    }
    for (row, col) in zip(0..pos.row(), 0..pos.col()) {
        out.push(Pos(pos.row() - row - 1, pos.col() - col - 1));
    }
}
