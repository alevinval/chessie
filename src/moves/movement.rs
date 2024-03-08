use crate::{bitboard::BitBoard, board::Board, piece::Piece, pos::Pos, Color};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Move {
    Takes { from: Pos, to: Pos },
    Slide { from: Pos, to: Pos },
    PawnPromo { from: Pos, to: Pos, piece: Piece },
    LeftCastle { mover: Color },
    RightCastle { mover: Color },
}

impl Move {
    pub fn apply(self, board: &Board) -> Board {
        let mut next = board.clone();
        self.inner_apply(&mut next);
        next.next_turn();
        next
    }

    pub fn to(self) -> Pos {
        match self {
            Move::Takes { from: _, to }
            | Move::Slide { from: _, to }
            | Move::PawnPromo {
                from: _,
                to,
                piece: _,
            } => to,
            Move::LeftCastle { mover } => (mover.piece_row(), 2).into(),
            Move::RightCastle { mover } => (mover.piece_row(), 6).into(),
        }
    }

    pub fn from(self) -> Pos {
        match self {
            Move::Takes { from, to: _ }
            | Move::Slide { from, to: _ }
            | Move::PawnPromo {
                from,
                to: _,
                piece: _,
            } => from,
            Move::LeftCastle { mover } | Move::RightCastle { mover } => {
                (mover.piece_row(), 4).into()
            }
        }
    }

    pub fn priority(self) -> f64 {
        match self {
            _ => 0.0,
        }
    }

    fn inner_apply(self, board: &mut Board) {
        match self {
            Move::Takes { from, to } => {
                Self::clear_dst(board, to);
                self.apply_move(board, from, to);
            }
            Move::Slide { from, to } => {
                self.apply_move(board, from, to);
            }
            Move::PawnPromo { from, to, piece } => {
                Self::clear_dst(board, to);
                self.apply_move(board, from, to);
                Self::promo(board, to, piece);
            }
            Move::LeftCastle { mover } => match mover {
                Color::B => {
                    self.apply_move(board, (7, 4), (7, 2));
                    self.apply_move(board, (7, 0), (7, 3));
                }
                Color::W => {
                    self.apply_move(board, (0, 4), (0, 2));
                    self.apply_move(board, (0, 0), (0, 3));
                }
            },
            Move::RightCastle { mover } => match mover {
                Color::B => {
                    self.apply_move(board, (7, 4), (7, 6));
                    self.apply_move(board, (7, 7), (7, 5));
                }
                Color::W => {
                    self.apply_move(board, (0, 4), (0, 6));
                    self.apply_move(board, (0, 7), (0, 5));
                }
            },
        }
    }

    fn clear_dst(board: &mut Board, to: Pos) {
        if let Some(dst) = board.at_mut(to) {
            dst.unset(to);
        }
    }

    fn promo<P: Into<Pos>>(board: &mut Board, pos: P, piece: Piece) {
        let pos = pos.into();
        Self::clear_dst(board, pos);

        let pieces = board.pieces_mut();
        match piece {
            Piece::Pawn(_) => pieces[Board::P],
            Piece::Rook(_, _, _) => pieces[Board::R],
            Piece::Knight(_) => pieces[Board::N],
            Piece::Bishop(_) => pieces[Board::B],
            Piece::Queen(_) => pieces[Board::Q],
            Piece::King(_, _) => unreachable!("cannot promote pawn to king"),
        }
        .set(pos);
    }

    fn apply_move<P: Into<Pos>>(self, board: &mut Board, from: P, to: P) {
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

    fn flag_piece_movement(self, bb: &mut BitBoard) {
        bb.update_piece(match bb.piece() {
            Piece::Rook(c, left, right) => match self {
                Move::Slide { from, to: _ } | Move::Takes { from, to: _ } => {
                    Piece::Rook(c, left || from.col() == 0, right || from.col() == 7)
                }
                Move::LeftCastle { mover } => Piece::Rook(mover, true, right),
                Move::RightCastle { mover } => Piece::Rook(mover, left, true),
                Move::PawnPromo {
                    from: _,
                    to: _,
                    piece: _,
                } => unreachable!(),
            },
            Piece::King(c, _) => Piece::King(c, true),
            piece => piece,
        });
    }
}

#[cfg(test)]
mod test {
    use std::mem;

    use super::*;

    const FROM: Pos = Pos::new(1, 1);
    const TO: Pos = Pos::new(3, 3);

    #[test]
    fn to() {
        assert_eq!(TO, Move::Slide { from: FROM, to: TO }.to());
        assert_eq!(
            TO,
            Move::PawnPromo {
                from: FROM,
                to: TO,
                piece: Piece::Pawn(Color::W),
            }
            .to()
        );
        assert_eq!(Pos::new(0, 2), Move::LeftCastle { mover: Color::W }.to());
        assert_eq!(Pos::new(7, 2), Move::LeftCastle { mover: Color::B }.to());
        assert_eq!(Pos::new(0, 6), Move::RightCastle { mover: Color::W }.to());
        assert_eq!(Pos::new(7, 6), Move::RightCastle { mover: Color::B }.to());
    }

    #[test]
    fn from() {
        assert_eq!(FROM, Move::Slide { from: FROM, to: TO }.from());
        assert_eq!(
            FROM,
            Move::PawnPromo {
                from: FROM,
                to: TO,
                piece: Piece::Pawn(Color::W),
            }
            .from()
        );
        assert_eq!(Pos::new(0, 4), Move::LeftCastle { mover: Color::W }.from());
        assert_eq!(Pos::new(7, 4), Move::LeftCastle { mover: Color::B }.from());
        assert_eq!(Pos::new(0, 4), Move::RightCastle { mover: Color::W }.from());
        assert_eq!(Pos::new(7, 4), Move::RightCastle { mover: Color::B }.from());
    }

    #[test]
    fn size() {
        assert_eq!(7, mem::size_of::<Move>());
        assert_eq!(8, mem::size_of::<&Move>());
    }
}
