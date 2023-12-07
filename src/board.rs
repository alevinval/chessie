use std::iter::zip;
use std::{fs::File, io::Write};

use crate::bitboard::BitBoard;
use crate::{Board, Direction, Piece, PieceSet};
use crate::{Color, Pos};

impl Board {
    pub fn new() -> Self {
        let white = Color::White;
        let black = Color::Black;
        Self {
            white: [
                PieceSet::new(Piece::Pawn(white), BitBoard::load(0b11111111, 1, 0)),
                PieceSet::new(Piece::Rook(white), BitBoard::load(0b10000001, 0, 0)),
                PieceSet::new(Piece::Knight(white), BitBoard::load(0b01000010, 0, 0)),
                PieceSet::new(Piece::Bishop(white), BitBoard::load(0b00100100, 0, 0)),
                PieceSet::new(Piece::Queen(white), BitBoard::load(1, 0, 3)),
                PieceSet::new(Piece::King(white), BitBoard::load(1, 0, 4)),
            ],
            black: [
                PieceSet::new(Piece::Pawn(black), BitBoard::load(0b11111111, 6, 0)),
                PieceSet::new(Piece::Rook(black), BitBoard::load(0b10000001, 7, 0)),
                PieceSet::new(Piece::Knight(black), BitBoard::load(0b01000010, 7, 0)),
                PieceSet::new(Piece::Bishop(black), BitBoard::load(0b00100100, 7, 0)),
                PieceSet::new(Piece::Queen(black), BitBoard::load(1, 7, 3)),
                PieceSet::new(Piece::King(black), BitBoard::load(1, 7, 4)),
            ],
        }
    }

    pub fn clear(&mut self) {
        for pboard in self.white.iter_mut().chain(self.black.iter_mut()) {
            pboard.clear();
        }
    }

    pub fn set(&mut self, pos: Pos, piece: Piece) {
        let pset = match piece.color() {
            Color::Black => &mut self.black,
            Color::White => &mut self.white,
        };
        if let Some(ps) = pset.iter_mut().find(|ps| ps.piece == piece) {
            ps.bit_board.or_mut(pos);
        }
    }

    pub fn apply_move(&mut self, from: Pos, to: Pos) {
        let pset = self.at_mut(from).expect("cannot move square without piece");
        pset.apply_move(from, to);
    }

    #[must_use]
    pub fn at(&self, pos: Pos) -> Option<&PieceSet> {
        self.white
            .iter()
            .chain(self.black.iter())
            .find(|piece| !piece.at(pos).is_empty())
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
            w.write_all(&pset.bit_board.to_le_bytes()).unwrap();
        });
        self.black.iter().for_each(|pset| {
            w.write_all(&pset.bit_board.to_le_bytes()).unwrap();
        });
    }

    #[must_use]
    pub fn generate_moves(&self, pos: Pos) -> BitBoard {
        self.at(pos).map_or(BitBoard(0), |set| {
            let mut moves = BitBoard::empty();
            match &set.piece {
                Piece::Pawn(c) => match c {
                    Color::Black => {
                        if pos.row() > 0 {
                            moves.or_mut(pos.to(Direction::Bottom));
                            if pos.row() == 6 {
                                moves.or_mut(pos.to(Direction::Bottom).to(Direction::Bottom));
                            }
                            if pos.col() < 7 {
                                moves.or_mut(pos.to(Direction::BottomRight));
                            }
                            if pos.col() > 0 {
                                moves.or_mut(pos.to(Direction::BottomLeft));
                            }
                        }
                    }
                    Color::White => {
                        if pos.row() < 7 {
                            moves.or_mut(pos.to(Direction::Top));
                            if pos.row() == 1 {
                                moves.or_mut(pos.to(Direction::Top).to(Direction::Top));
                            }
                            if pos.col() < 7 {
                                moves.or_mut(pos.to(Direction::TopRight));
                            }
                            if pos.col() > 0 {
                                moves.or_mut(pos.to(Direction::TopLeft));
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
            moves
        })
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

fn cross(pos: Pos, out: &mut BitBoard) {
    (0..pos.row())
        .chain(pos.row() + 1..8)
        .map(|r| Pos(r, pos.col()))
        .for_each(|p| out.or_mut(p));

    (0..pos.col())
        .chain(pos.col() + 1..8)
        .map(|c| Pos(pos.row(), c))
        .for_each(|p| out.or_mut(p));
}

fn diagonals(from: Pos, out: &mut BitBoard) {
    zip(from.row() + 1..8, from.col() + 1..8)
        .map(|(r, c)| Pos(r, c))
        .for_each(|p| out.or_mut(p));

    zip(0..from.row(), from.col() + 1..8)
        .map(|(r, c)| Pos(from.row() - 1 - r, c))
        .for_each(|p| out.or_mut(p));

    zip(from.row() + 1..8, 0..from.col())
        .map(|(r, c)| Pos(r, from.col() - c - 1))
        .for_each(|p| out.or_mut(p));

    zip(0..from.row(), 0..from.col())
        .map(|(r, c)| Pos(from.row() - r - 1, from.col() - c - 1))
        .for_each(|p| out.or_mut(p));
}
