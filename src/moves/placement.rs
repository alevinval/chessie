use crate::{board::Board, pos::Pos};

use super::Move;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Placement {
    Invalid,
    Empty { from: Pos, to: Pos },
    Takes { from: Pos, to: Pos },
}

impl Placement {
    pub fn stop(&self) -> bool {
        matches!(self, Self::Invalid) | matches!(self, Self::Takes { from: _, to: _ })
    }

    pub fn placed(&self) -> bool {
        matches!(
            self,
            Self::Takes { from: _, to: _ } | Self::Empty { from: _, to: _ }
        )
    }

    pub fn movement(&self) -> Option<Move> {
        match *self {
            Placement::Invalid => None,
            Placement::Empty { from, to } => Some(Move::Slide { from, to }),
            Placement::Takes { from, to } => Some(Move::Takes { from, to }),
        }
    }
}

pub type StopCondition = fn(&Board, Pos, Pos) -> Placement;

pub fn is_empty(board: &Board, from: Pos, to: Pos) -> Placement {
    board
        .at(to)
        .map_or(Placement::Empty { from, to }, |_| Placement::Invalid)
}

pub fn takes(board: &Board, from: Pos, to: Pos) -> Placement {
    board.at(from).map_or(Placement::Invalid, |ps_from| {
        board.at(to).map_or(Placement::Invalid, |ps_to| {
            if ps_from.color() == ps_to.color() {
                Placement::Invalid
            } else {
                Placement::Takes { from, to }
            }
        })
    })
}

pub fn empty_or_take(board: &Board, from: Pos, to: Pos) -> Placement {
    match is_empty(board, from, to) {
        Placement::Empty { from, to } => Placement::Empty { from, to },
        _ => takes(board, from, to),
    }
}
