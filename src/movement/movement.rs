use crate::{
    board::Board,
    pieces::{BitBoard, Piece},
    pos::Pos,
};

#[derive(Debug, Eq, PartialEq)]
pub enum Move {
    None,
    Basic(Pos, Pos),
}

impl Move {
    pub fn apply(&self, board: &mut Board) {
        match *self {
            Move::Basic(from, to) => {
                self.clear_dst(board, to);
                self.apply_move(board, from, to);
            }
            Move::None => unreachable!("should never apply a non-move"),
        }
    }

    fn clear_dst(&self, board: &mut Board, to: Pos) {
        if let Some(dst) = board.at_mut(to) {
            dst.unset(to);
        }
    }

    fn apply_move(&self, board: &mut Board, from: Pos, to: Pos) {
        match board.at_mut(from) {
            Some(bb) => {
                bb.unset(from);
                bb.set(to);
                self.flag_piece_movement(bb);
            }
            None => {
                unreachable!("cannot move square without piece {from:?} to {to:?}");
            }
        }
    }

    fn flag_piece_movement(&self, bb: &mut BitBoard) {
        bb.update_piece(match bb.piece() {
            Piece::Rook(c, lrm, rrm) => match self {
                Move::Basic(from, _) => {
                    Piece::Rook(c, lrm || from.col() == 0, rrm || from.col() == 7)
                }
                _ => unreachable!(),
            },
            Piece::King(c, _) => Piece::King(c, true),
            piece => piece,
        });
    }
}
