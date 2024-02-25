use std::iter::zip;

use crate::{board::Board, pos::Direction, BitBoard, Pos};

use super::generator::{Generator, Placement};

pub fn bishop(mut g: Generator) -> BitBoard {
    diagonals(&mut g);
    g.moves()
}

pub fn rook(mut g: Generator) -> BitBoard {
    cross(&mut g);
    g.moves()
}

pub fn queen(mut g: Generator) -> BitBoard {
    diagonals(&mut g);
    cross(&mut g);
    g.moves()
}

pub fn black_pawn(mut g: Generator) -> BitBoard {
    if g.row() > 0 {
        if g.dir(Direction::Bottom(1), is_empty).yes() && g.row() == 6 {
            g.dir(Direction::Bottom(2), is_empty);
        }

        if g.col() < 7 {
            g.dir(Direction::Custom(-1, 1), is_opposite);
        }

        if g.col() > 0 {
            g.dir(Direction::Custom(-1, -1), is_opposite);
        }
    }
    g.moves()
}

pub fn white_pawn(mut g: Generator) -> BitBoard {
    if g.row() < 7 {
        if g.dir(Direction::Top(1), is_empty).yes() && g.row() == 1 {
            g.dir(Direction::Top(2), is_empty);
        }
        if g.col() < 7 {
            g.dir(Direction::Custom(1, 1), is_opposite);
        }
        if g.col() > 0 {
            g.dir(Direction::Custom(1, -1), is_opposite);
        }
    }
    g.moves()
}

pub fn knight(mut g: Generator) -> BitBoard {
    let has_one_right = g.col() < 7;
    let has_two_right = g.col() < 6;
    let has_one_left = g.col() > 0;
    let has_two_left = g.col() > 1;

    if g.row() < 6 {
        if has_one_left {
            g.dir(Direction::Custom(2, -1), is_empty_or_opposite);
        }
        if has_one_right {
            g.dir(Direction::Custom(2, 1), is_empty_or_opposite);
        }
    }

    if g.row() > 2 {
        if has_one_left {
            g.dir(Direction::Custom(-2, -1), is_empty_or_opposite);
        }
        if has_one_right {
            g.dir(Direction::Custom(-2, 1), is_empty_or_opposite);
        }
    }

    if g.row() < 7 {
        if has_two_left {
            g.dir(Direction::Custom(1, -2), is_empty_or_opposite);
        }
        if has_two_right {
            g.dir(Direction::Custom(1, 2), is_empty_or_opposite);
        }
    }

    if g.row() > 0 {
        if has_two_left {
            g.dir(Direction::Custom(-1, -2), is_empty_or_opposite);
        }
        if has_two_right {
            g.dir(Direction::Custom(-1, 2), is_empty_or_opposite);
        }
    }
    g.moves()
}

pub fn king(mut g: Generator) -> BitBoard {
    if g.row() < 7 {
        g.dir(Direction::Top(1), is_empty_or_opposite);

        if g.col() < 7 {
            g.dir(Direction::Custom(1, 1), is_empty_or_opposite);
        }

        if g.col() > 0 {
            g.dir(Direction::Custom(1, -1), is_empty_or_opposite);
        }
    }

    if g.row() > 0 {
        g.dir(Direction::Bottom(1), is_empty_or_opposite);

        if g.col() < 7 {
            g.dir(Direction::Custom(-1, 1), is_empty_or_opposite);
        }

        if g.col() > 0 {
            g.dir(Direction::Custom(-1, -1), is_empty_or_opposite);
        }
    }

    if g.col() < 7 {
        g.dir(Direction::Right(1), is_empty_or_opposite);
    }

    if g.col() > 0 {
        g.dir(Direction::Left(1), is_empty_or_opposite);
    }
    g.moves()
}

fn cross(g: &mut Generator) {
    let (row, col) = (g.row(), g.col());

    for r in (0..row).rev() {
        if g.pos((r, col), is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for r in row + 1..8 {
        if g.pos((r, col), is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for c in (0..col).rev() {
        if g.pos((row, c), is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for c in col + 1..8 {
        if g.pos((row, c), is_empty_or_opposite).should_stop() {
            break;
        }
    }
}

fn diagonals(g: &mut Generator) {
    let (row, col) = (g.row(), g.col());

    for pos in zip(row + 1..8, col + 1..8) {
        if g.pos(pos, is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for pos in zip((0..row).rev(), col + 1..8) {
        if g.pos(pos, is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for pos in zip(row + 1..8, (0..col).rev()) {
        if g.pos(pos, is_empty_or_opposite).should_stop() {
            break;
        }
    }

    for pos in zip((0..row).rev(), (0..col).rev()) {
        if g.pos(pos, is_empty_or_opposite).should_stop() {
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
