use crate::{board::Board, pos::Pos};

#[derive(Debug, Eq, PartialEq)]
pub enum Move {
    None,
    Basic(Pos, Pos),
}

impl Move {
    pub fn apply(&self, board: &mut Board) {
        match *self {
            Move::Basic(from, to) => self.apply_basic(board, from, to),
            Move::None => debug_assert!(true, "should never apply a non-move"),
        }
    }

    fn apply_basic(&self, board: &mut Board, from: Pos, to: Pos) {
        debug_assert!(
            board.at(from).is_some(),
            "trying to apply a move but there is no piece on the starting position"
        );

        if let Some(dst) = board.at_mut(to) {
            dst.unset(to);
        }

        board
            .at_mut(from)
            .expect("cannot move square without piece")
            .apply_move(from, to);
    }
}
