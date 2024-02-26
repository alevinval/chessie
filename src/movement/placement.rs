use crate::{board::Board, pos::Pos};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Placement {
    Invalid,
    Empty(Pos, Pos),
    Takes(Pos, Pos),
}

impl Placement {
    pub fn stop(&self) -> bool {
        matches!(self, Self::Invalid | Self::Takes(_, _))
    }

    pub fn placed(&self) -> bool {
        matches!(self, Self::Takes(_, _) | Self::Empty(_, _))
    }
}

pub type PlacementCnd = fn(&Board, Pos, Pos) -> Placement;

pub fn is_empty(board: &Board, from: Pos, to: Pos) -> Placement {
    board
        .at(to)
        .map(|_| Placement::Invalid)
        .unwrap_or(Placement::Empty(from, to))
}

pub fn takes(board: &Board, from: Pos, to: Pos) -> Placement {
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

pub fn empty_or_take(board: &Board, from: Pos, to: Pos) -> Placement {
    match is_empty(board, from, to) {
        Placement::Empty(from, to) => Placement::Empty(from, to),
        _ => takes(board, from, to),
    }
}
