use std::iter::zip;

use crate::{pos::Direction, BitBoard, Pos};

pub fn bishop(pos: Pos) -> BitBoard {
    let mut out = BitBoard::default();
    diagonals(pos, &mut out);
    out
}

pub fn rook(pos: Pos) -> BitBoard {
    let mut out = BitBoard::default();
    cross(pos, &mut out);
    out
}

pub fn queen(pos: Pos) -> BitBoard {
    let mut out = BitBoard::default();
    diagonals(pos, &mut out);
    cross(pos, &mut out);
    out
}

pub fn black_pawn(pos: Pos) -> BitBoard {
    let mut out = BitBoard::default();
    if pos.row() > 0 {
        out.or_mut(pos.to(Direction::Bottom));
        if pos.row() == 6 {
            out.or_mut(pos.to(Direction::Bottom).to(Direction::Bottom));
        }
        if pos.col() < 7 {
            out.or_mut(pos.to(Direction::BottomRight));
        }
        if pos.col() > 0 {
            out.or_mut(pos.to(Direction::BottomLeft));
        }
    }
    out
}

pub fn white_pawn(pos: Pos) -> BitBoard {
    let mut out = BitBoard::default();
    if pos.row() < 7 {
        out.or_mut(pos.to(Direction::Top));
        if pos.row() == 1 {
            out.or_mut(pos.to(Direction::Top).to(Direction::Top));
        }
        if pos.col() < 7 {
            out.or_mut(pos.to(Direction::TopRight));
        }
        if pos.col() > 0 {
            out.or_mut(pos.to(Direction::TopLeft));
        }
    }
    out
}

pub fn knight(pos: Pos) -> BitBoard {
    let mut out = BitBoard::default();
    let has_one_right = pos.col() < 7;
    let has_two_right = pos.col() < 6;
    let has_one_left = pos.col() > 0;
    let has_two_left = pos.col() > 1;

    if pos.row() < 6 {
        if has_one_left {
            out.or_mut(pos.to(Direction::Top).to(Direction::TopLeft));
        }
        if has_one_right {
            out.or_mut(pos.to(Direction::Top).to(Direction::TopRight));
        }
    }

    if pos.row() > 2 {
        if has_one_left {
            out.or_mut(pos.to(Direction::Bottom).to(Direction::BottomLeft));
        }
        if has_one_right {
            out.or_mut(pos.to(Direction::Bottom).to(Direction::BottomRight));
        }
    }

    if pos.row() < 7 {
        if has_two_left {
            out.or_mut(pos.to(Direction::Left).to(Direction::TopLeft));
        }
        if has_two_right {
            out.or_mut(pos.to(Direction::Right).to(Direction::TopRight));
        }
    }

    if pos.row() > 0 {
        if has_two_left {
            out.or_mut(pos.to(Direction::Left).to(Direction::BottomLeft));
        }
        if has_two_right {
            out.or_mut(pos.to(Direction::Right).to(Direction::BottomRight));
        }
    }
    out
}

pub fn king(pos: Pos) -> BitBoard {
    let mut out = BitBoard::default();
    if pos.row() < 7 {
        out.or_mut(pos.to(Direction::Top));

        if pos.col() < 7 {
            out.or_mut(pos.to(Direction::TopRight));
        }

        if pos.col() > 0 {
            out.or_mut(pos.to(Direction::TopLeft));
        }
    }

    if pos.row() > 0 {
        out.or_mut(pos.to(Direction::Bottom));

        if pos.col() < 7 {
            out.or_mut(pos.to(Direction::BottomRight));
        }

        if pos.col() > 0 {
            out.or_mut(pos.to(Direction::BottomLeft));
        }
    }

    if pos.col() < 7 {
        out.or_mut(pos.to(Direction::Right));
    }

    if pos.col() > 0 {
        out.or_mut(pos.to(Direction::Left));
    }
    out
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
