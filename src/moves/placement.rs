use super::Move;
use crate::{board::Board, eval::Scorer, pos::Pos};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Placement {
    Invalid,
    Empty { from: Pos, to: Pos },
    Takes { from: Pos, to: Pos },
}

impl Placement {
    pub(crate) fn stop(&self) -> bool {
        matches!(self, Self::Invalid) | matches!(self, Self::Takes { from: _, to: _ })
    }

    pub(crate) fn placed(&self) -> bool {
        matches!(self, Self::Takes { from: _, to: _ } | Self::Empty { from: _, to: _ })
    }

    pub(crate) fn movement(&self, board: &Board) -> Option<Move> {
        match *self {
            Placement::Invalid => None,
            Placement::Empty { from, to } => Some(Move::Slide { from, to }),
            Placement::Takes { from, to } => {
                let (_, pf, _) = board.at(from).expect("must be there");
                let (_, pt, _) = board.at(to).expect("must be there");
                Some(Move::Takes {
                    from,
                    to,
                    value: Scorer::score_piece(pt) - Scorer::score_piece(pf),
                })
            }
        }
    }
}

pub(crate) type StopCondition = fn(&Board, Pos, Pos) -> Placement;

pub(crate) fn is_empty(board: &Board, from: Pos, to: Pos) -> Placement {
    board.at(to).map_or(Placement::Empty { from, to }, |_| Placement::Invalid)
}

pub(crate) fn takes(board: &Board, from: Pos, to: Pos) -> Placement {
    board.at(from).map_or(Placement::Invalid, |(color_from, _, _)| {
        board.at(to).map_or(Placement::Invalid, |(color_to, _, _)| {
            if color_from == color_to {
                Placement::Invalid
            } else {
                Placement::Takes { from, to }
            }
        })
    })
}

pub(crate) fn empty_or_take(board: &Board, from: Pos, to: Pos) -> Placement {
    match is_empty(board, from, to) {
        Placement::Empty { from, to } => Placement::Empty { from, to },
        _ => takes(board, from, to),
    }
}
