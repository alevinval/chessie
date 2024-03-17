use crate::{board::Board, pos::Pos};

use super::Move;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Placement {
    Empty { from: Pos, to: Pos },
}

impl Placement {
    pub fn movement(&self) -> Move {
        match *self {
            Placement::Empty { from, to } => Move::Slide { from, to },
        }
    }
}

pub type StopCondition = fn(&Board, Pos, Pos) -> Option<Placement>;

pub fn is_empty(board: &Board, from: Pos, to: Pos) -> Option<Placement> {
    board.at(to).map_or(Some(Placement::Empty { from, to }), |_| None)
}
