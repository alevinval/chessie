mod generator;
mod movement;

use crate::defs::Sq;

pub(crate) use self::{generator::Generator, movement::Move};

pub(crate) fn is_attacked(moves: &[Move], target: Sq) -> bool {
    attacked_positions(moves).any(|sq| sq == target)
}

pub(crate) fn attacked_positions(moves: &[Move]) -> impl Iterator<Item = Sq> + '_ {
    moves.iter().map(|m| m.to())
}
