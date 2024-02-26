use std::iter::zip;

pub mod generator;

use crate::{
    board::Board,
    pieces::{Color, Piece},
    pos::Dir,
    Pos,
};

use self::generator::{Generator, Movements, Placement};

pub struct MoveGen<'board> {
    gen: Generator<'board>,
}

impl<'board> MoveGen<'board> {
    pub fn new<P: Into<Pos>>(board: &'board Board, from: P) -> Self {
        Self {
            gen: Generator::new(board, from),
        }
    }

    pub fn gen(mut self, piece: &Piece) -> Movements {
        match piece {
            Piece::Pawn(color) => match color {
                Color::Black => self.black_pawn(),
                Color::White => self.white_pawn(),
            },
            Piece::Rook(_) => self.rook(),
            Piece::Bishop(_) => self.bishop(),
            Piece::Queen(_) => self.queen(),
            Piece::Knight(_) => self.knight(),
            Piece::King(_) => self.king(),
        };
        self.gen.moves()
    }

    fn bishop(&mut self) {
        diagonals(&mut self.gen);
    }

    fn rook(&mut self) {
        cross(&mut self.gen);
    }

    fn queen(&mut self) {
        diagonals(&mut self.gen);
        cross(&mut self.gen);
    }

    fn black_pawn(&mut self) {
        black_pawn(&mut self.gen);
    }

    fn white_pawn(&mut self) {
        white_pawn(&mut self.gen);
    }

    fn knight(&mut self) {
        knight(&mut self.gen);
    }

    fn king(&mut self) {
        king(&mut self.gen);
    }
}

fn black_pawn(g: &mut Generator) {
    if g.row() > 0 {
        if g.dir(Dir::Down(1), is_empty).placed() && g.row() == 6 {
            g.dir(Dir::Down(2), is_empty);
        }

        if g.col() < 7 {
            g.dir(Dir::Custom(-1, 1), takes);
        }

        if g.col() > 0 {
            g.dir(Dir::Custom(-1, -1), takes);
        }
    }
}

fn white_pawn(g: &mut Generator) {
    if g.row() < 7 {
        if g.dir(Dir::Up(1), is_empty).placed() && g.row() == 1 {
            g.dir(Dir::Up(2), is_empty);
        }
        if g.col() < 7 {
            g.dir(Dir::Custom(1, 1), takes);
        }
        if g.col() > 0 {
            g.dir(Dir::Custom(1, -1), takes);
        }
    }
}

fn knight(g: &mut Generator) {
    let has_one_right = g.col() < 7;
    let has_two_right = g.col() < 6;
    let has_one_left = g.col() > 0;
    let has_two_left = g.col() > 1;

    if g.row() < 6 {
        if has_one_left {
            g.dir(Dir::Custom(2, -1), empty_or_take);
        }
        if has_one_right {
            g.dir(Dir::Custom(2, 1), empty_or_take);
        }
    }

    if g.row() > 2 {
        if has_one_left {
            g.dir(Dir::Custom(-2, -1), empty_or_take);
        }
        if has_one_right {
            g.dir(Dir::Custom(-2, 1), empty_or_take);
        }
    }

    if g.row() < 7 {
        if has_two_left {
            g.dir(Dir::Custom(1, -2), empty_or_take);
        }
        if has_two_right {
            g.dir(Dir::Custom(1, 2), empty_or_take);
        }
    }

    if g.row() > 0 {
        if has_two_left {
            g.dir(Dir::Custom(-1, -2), empty_or_take);
        }
        if has_two_right {
            g.dir(Dir::Custom(-1, 2), empty_or_take);
        }
    }
}

fn king(g: &mut Generator) {
    if g.row() < 7 {
        g.dir(Dir::Up(1), empty_or_take);

        if g.col() < 7 {
            g.dir(Dir::Custom(1, 1), empty_or_take);
        }

        if g.col() > 0 {
            g.dir(Dir::Custom(1, -1), empty_or_take);
        }
    }

    if g.row() > 0 {
        g.dir(Dir::Down(1), empty_or_take);

        if g.col() < 7 {
            g.dir(Dir::Custom(-1, 1), empty_or_take);
        }

        if g.col() > 0 {
            g.dir(Dir::Custom(-1, -1), empty_or_take);
        }
    }

    if g.col() < 7 {
        g.dir(Dir::Left(1), empty_or_take);
    }

    if g.col() > 0 {
        g.dir(Dir::Right(1), empty_or_take);
    }
}

fn cross(g: &mut Generator) {
    let (row, col) = (g.row(), g.col());

    for r in (0..row).rev() {
        if g.pos((r, col), empty_or_take).stop() {
            break;
        }
    }

    for r in row + 1..8 {
        if g.pos((r, col), empty_or_take).stop() {
            break;
        }
    }

    for c in (0..col).rev() {
        if g.pos((row, c), empty_or_take).stop() {
            break;
        }
    }

    for c in col + 1..8 {
        if g.pos((row, c), empty_or_take).stop() {
            break;
        }
    }
}

fn diagonals(g: &mut Generator) {
    let (row, col) = (g.row(), g.col());

    for pos in zip(row + 1..8, col + 1..8) {
        if g.pos(pos, empty_or_take).stop() {
            break;
        }
    }

    for pos in zip((0..row).rev(), col + 1..8) {
        if g.pos(pos, empty_or_take).stop() {
            break;
        }
    }

    for pos in zip(row + 1..8, (0..col).rev()) {
        if g.pos(pos, empty_or_take).stop() {
            break;
        }
    }

    for pos in zip((0..row).rev(), (0..col).rev()) {
        if g.pos(pos, empty_or_take).stop() {
            break;
        }
    }
}

fn is_empty(board: &Board, from: Pos, to: Pos) -> Placement {
    board
        .at(to)
        .map(|_| Placement::Invalid)
        .unwrap_or(Placement::Empty(from, to))
}

fn takes(board: &Board, from: Pos, to: Pos) -> Placement {
    board
        .at(from)
        .map(|ps_from| {
            board
                .at(to)
                .map(|ps_to| {
                    if ps_from.color() != ps_to.color() {
                        Placement::Takes(from, to)
                    } else {
                        Placement::Invalid
                    }
                })
                .unwrap_or(Placement::Invalid)
        })
        .unwrap_or(Placement::Invalid)
}

fn empty_or_take(board: &Board, from: Pos, to: Pos) -> Placement {
    match is_empty(board, from, to) {
        Placement::Empty(from, to) => Placement::Empty(from, to),
        _ => takes(board, from, to),
    }
}
