mod generator;
mod movement;

use crate::{defs::Sq, pos::Pos};

pub(crate) use self::{generator::Generator, movement::Move};

pub(crate) fn is_attacked(moves: &[Move], target: Sq) -> bool {
    targeted_squares(moves).any(|sq| sq == target)
}

pub(crate) fn attacked_positions(moves: &[Move]) -> impl Iterator<Item = Pos> + '_ {
    moves.iter().map(|m| m.to())
}

fn targeted_squares(moves: &[Move]) -> impl Iterator<Item = Sq> + '_ {
    attacked_positions(moves).map(|p| p.sq())
}
