use crate::{
    board::Board,
    pieces::{BitBoard, Color, Piece},
    pos::Pos,
};

#[derive(Debug, Eq, PartialEq)]
pub enum Move {
    None,
    Basic(Pos, Pos),
    PawnPromo(Pos, Pos, Piece),
    LeftCastle(Color),
    RightCastle(Color),
}

impl Move {
    pub fn apply(&self, board: &mut Board) {
        match *self {
            Move::Basic(from, to) => {
                Self::clear_dst(board, to);
                self.apply_move(board, from, to);
            }
            Move::PawnPromo(from, to, piece) => {
                Self::clear_dst(board, to);
                self.apply_move(board, from, to);
                self.promo(board, to, piece);
            }
            Move::LeftCastle(c) => match c {
                Color::Black => {
                    self.apply_move(board, (7, 4), (7, 2));
                    self.apply_move(board, (7, 0), (7, 3));
                }
                Color::White => {
                    self.apply_move(board, (0, 4), (0, 2));
                    self.apply_move(board, (0, 0), (0, 3));
                }
            },
            Move::RightCastle(c) => match c {
                Color::Black => {
                    self.apply_move(board, (7, 4), (7, 6));
                    self.apply_move(board, (7, 7), (7, 5));
                }
                Color::White => {
                    self.apply_move(board, (0, 4), (0, 6));
                    self.apply_move(board, (0, 7), (0, 5));
                }
            },
            Move::None => unreachable!("should never apply a non-move"),
        }
    }

    fn clear_dst(board: &mut Board, to: Pos) {
        if let Some(dst) = board.at_mut(to) {
            dst.unset(to);
        }
    }

    fn promo<P: Into<Pos>>(&self, board: &mut Board, pos: P, piece: Piece) {
        let pos = pos.into();
        Self::clear_dst(board, pos);

        let pieces = board.pieces_mut(piece.color());
        match piece {
            Piece::Pawn(_) => &mut pieces.pawns,
            Piece::Rook(_, _, _) => &mut pieces.rooks,
            Piece::Knight(_) => &mut pieces.knights,
            Piece::Bishop(_) => &mut pieces.bishops,
            Piece::Queen(_) => &mut pieces.queen,
            Piece::King(_, _) => unreachable!("cannot promote pawn to king"),
        }
        .set(pos)
    }

    fn apply_move<P: Into<Pos>>(&self, board: &mut Board, from: P, to: P) {
        let from = from.into();
        match board.at_mut(from) {
            Some(bb) => {
                bb.slide(from, to.into());
                self.flag_piece_movement(bb);
            }
            None => {
                unreachable!("cannot move square without piece {from:?}");
            }
        }
    }

    fn flag_piece_movement(&self, bb: &mut BitBoard) {
        bb.update_piece(match bb.piece() {
            Piece::Rook(c, left, right) => match self {
                Move::Basic(from, _) => {
                    Piece::Rook(c, left || from.col() == 0, right || from.col() == 7)
                }
                Move::LeftCastle(_) => Piece::Rook(c, true, right),
                Move::RightCastle(_) => Piece::Rook(c, left, true),
                Move::None | Move::PawnPromo(_, _, _) => unreachable!(),
            },
            Piece::King(c, _) => Piece::King(c, true),
            piece => piece,
        });
    }
}
