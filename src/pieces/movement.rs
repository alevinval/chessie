use std::iter::zip;

use crate::{board::Board, pos::Direction, BitBoard, Pos};

pub fn bishop(mut g: Generator) -> BitBoard {
    diagonals(&mut g);
    g.moves
}

pub fn rook(mut g: Generator) -> BitBoard {
    cross(&mut g);
    g.moves
}

pub fn queen(mut g: Generator) -> BitBoard {
    diagonals(&mut g);
    cross(&mut g);
    g.moves
}

pub fn black_pawn(mut g: Generator) -> BitBoard {
    if g.row() > 0 {
        if g.gen(g.to(Direction::Bottom(1)), is_empty).yes() && g.row() == 6 {
            g.gen(g.to(Direction::Bottom(2)), is_empty);
        }

        if g.col() < 7 {
            g.gen(g.to(Direction::Custom(-1, 1)), is_opposite);
        }

        if g.col() > 0 {
            g.gen(g.to(Direction::Custom(-1, -1)), is_opposite);
        }
    }
    g.moves
}

pub fn white_pawn(mut g: Generator) -> BitBoard {
    if g.row() < 7 {
        if g.gen(g.to(Direction::Top(1)), is_empty).yes() && g.row() == 1 {
            g.gen(g.to(Direction::Top(2)), is_empty);
        }
        if g.col() < 7 {
            g.gen(g.to(Direction::Custom(1, 1)), is_opposite);
        }
        if g.col() > 0 {
            g.gen(g.to(Direction::Custom(1, -1)), is_opposite);
        }
    }
    g.moves
}

pub fn knight(mut g: Generator) -> BitBoard {
    let has_one_right = g.col() < 7;
    let has_two_right = g.col() < 6;
    let has_one_left = g.col() > 0;
    let has_two_left = g.col() > 1;

    if g.row() < 6 {
        if has_one_left {
            g.gen(g.to(Direction::Custom(2, -1)), is_empty_or_opposite);
        }
        if has_one_right {
            g.gen(g.to(Direction::Custom(2, 1)), is_empty_or_opposite);
        }
    }

    if g.row() > 2 {
        if has_one_left {
            g.gen(g.to(Direction::Custom(-2, -1)), is_empty_or_opposite);
        }
        if has_one_right {
            g.gen(g.to(Direction::Custom(-2, 1)), is_empty_or_opposite);
        }
    }

    if g.row() < 7 {
        if has_two_left {
            g.gen(g.to(Direction::Custom(1, -2)), is_empty_or_opposite);
        }
        if has_two_right {
            g.gen(g.to(Direction::Custom(1, 2)), is_empty_or_opposite);
        }
    }

    if g.row() > 0 {
        if has_two_left {
            g.gen(g.to(Direction::Custom(-1, -2)), is_empty_or_opposite);
        }
        if has_two_right {
            g.gen(g.to(Direction::Custom(-1, 2)), is_empty_or_opposite);
        }
    }
    g.moves
}

pub fn king(mut g: Generator) -> BitBoard {
    if g.row() < 7 {
        g.gen(g.to(Direction::Top(1)), is_empty_or_opposite);

        if g.col() < 7 {
            g.gen(g.to(Direction::Custom(1, 1)), is_empty_or_opposite);
        }

        if g.col() > 0 {
            g.gen(g.to(Direction::Custom(1, -1)), is_empty_or_opposite);
        }
    }

    if g.row() > 0 {
        g.gen(g.to(Direction::Bottom(1)), is_empty_or_opposite);

        if g.col() < 7 {
            g.gen(g.to(Direction::Custom(-1, 1)), is_empty_or_opposite);
        }

        if g.col() > 0 {
            g.gen(g.to(Direction::Custom(-1, -1)), is_empty_or_opposite);
        }
    }

    if g.col() < 7 {
        g.gen(g.to(Direction::Right(1)), is_empty_or_opposite);
    }

    if g.col() > 0 {
        g.gen(g.to(Direction::Left(1)), is_empty_or_opposite);
    }
    g.moves
}

fn cross(g: &mut Generator) {
    let row = g.row();
    let col = g.col();

    for pos in (0..row).rev().map(|r| Pos(r, col)) {
        if g.gen(pos, is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for pos in (row + 1..8).map(|r| Pos(r, col)) {
        if g.gen(pos, is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for pos in (0..col).rev().map(|c| Pos(row, c)) {
        if g.gen(pos, is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for pos in (col + 1..8).map(|c| Pos(row, c)) {
        if g.gen(pos, is_empty_or_opposite).should_stop() {
            break;
        }
    }
}

fn diagonals(g: &mut Generator) {
    let row = g.row();
    let col = g.col();

    for pos in zip(row + 1..8, col + 1..8).map(|(r, c)| Pos(r, c)) {
        if g.gen(pos, is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for pos in zip((0..row).rev(), col + 1..8).map(|(r, c)| Pos(r, c)) {
        if g.gen(pos, is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for pos in zip(row + 1..8, (0..col).rev()).map(|(r, c)| Pos(r, c)) {
        if g.gen(pos, is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for pos in zip((0..row).rev(), (0..col).rev()).map(|(r, c)| Pos(r, c)) {
        if g.gen(pos, is_empty_or_opposite).should_stop() {
            break;
        }
    }
}

fn is_empty(board: &Board, _from: Pos, to: Pos) -> Placement {
    board
        .at(to)
        .map(|_| Placement::No)
        .unwrap_or(Placement::EmptyCell)
}

fn is_opposite(board: &Board, from: Pos, to: Pos) -> Placement {
    board
        .at(from)
        .map(|ps_from| {
            board
                .at(to)
                .map(|ps_to| {
                    if ps_from.piece.color() != ps_to.piece.color() {
                        Placement::Takes
                    } else {
                        Placement::No
                    }
                })
                .unwrap_or(Placement::No)
        })
        .unwrap_or(Placement::No)
}

fn is_empty_or_opposite(board: &Board, from: Pos, to: Pos) -> Placement {
    match is_empty(board, from, to) {
        Placement::EmptyCell => Placement::EmptyCell,
        _ => is_opposite(board, from, to),
    }
}

#[derive(Debug)]
pub struct Generator<'b> {
    board: &'b Board,
    from: Pos,
    moves: BitBoard,
}

impl<'b> Generator<'b> {
    pub fn new(board: &'b Board, from: Pos) -> Self {
        Generator {
            board,
            from,
            moves: BitBoard::default(),
        }
    }

    pub fn row(&self) -> u8 {
        self.from.row()
    }

    pub fn col(&self) -> u8 {
        self.from.col()
    }

    pub fn to(&self, direction: Direction) -> Pos {
        self.from.to(direction)
    }

    pub fn gen(&mut self, to: Pos, condition: fn(&Board, Pos, Pos) -> Placement) -> Placement {
        match condition(self.board, self.from, to) {
            Placement::No => Placement::No,
            placement => {
                self.moves.or_mut(to);
                placement
            }
        }
    }
}

pub enum Placement {
    No,
    EmptyCell,
    Takes,
}

impl Placement {
    fn should_stop(&self) -> bool {
        matches!(self, Self::No | Self::Takes)
    }

    fn no(&self) -> bool {
        matches!(self, Self::No)
    }

    fn yes(&self) -> bool {
        !self.no()
    }
}
