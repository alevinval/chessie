use crate::{board::Board, pos::Pos};

use super::Move;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Placement {
    Empty { from: Pos, to: Pos },
    Takes { from: Pos, to: Pos },
}

impl Placement {
    pub fn stop(&self) -> bool {
        matches!(self, Self::Takes { from: _, to: _ })
    }

    pub fn movement(&self) -> Move {
        match *self {
            Placement::Empty { from, to } => Move::Slide { from, to },
            Placement::Takes { from, to } => Move::Takes { from, to },
        }
    }
}

pub type StopCondition = fn(&Board, Pos, Pos) -> Option<Placement>;

pub fn is_empty(board: &Board, from: Pos, to: Pos) -> Option<Placement> {
    board
        .at(to)
        .map_or(Some(Placement::Empty { from, to }), |_| None)
}

pub fn takes(board: &Board, from: Pos, to: Pos) -> Option<Placement> {
    board.at(from).and_then(|(color_from, _)| {
        board.at(to).and_then(|(color_to, _)| {
            if color_from == color_to {
                None
            } else {
                Some(Placement::Takes { from, to })
            }
        })
    })
}

pub fn empty_or_take(board: &Board, from: Pos, to: Pos) -> Option<Placement> {
    match is_empty(board, from, to) {
        Some(placement) => Some(placement),
        _ => takes(board, from, to),
    }
}
