use crate::{
    board::{Board, Castling},
    piece::Piece,
    pos::Pos,
    Color,
};

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
        0.0
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
        if let Some((_, dst)) = board.at_mut(to) {
            dst.unset(to);
        }
    }

    fn promo<P: Into<Pos>>(board: &mut Board, pos: P, piece: Piece) {
        let pos = pos.into();
        Self::clear_dst(board, pos);
        board.apply_promo(pos, piece);
    }

    fn apply_move<P: Into<Pos>>(self, board: &mut Board, from: P, to: P) {
        let from = from.into();
        let rights = board.castling_rights(board.mover());

        let (color, bb) = board.at_mut(from).expect("must have piece to move");
        bb.slide(from, to.into());

        if let Castling::Some(left, right) = rights {
            if bb.piece() == Piece::King {
                board.set_rights(color, Castling::None);
                return;
            }

            if bb.piece() == Piece::Rook {
                match self {
                    Move::Slide { from, to: _ } | Move::Takes { from, to: _ } => {
                        board.set_rights(
                            color,
                            Castling::Some(left || from.col() == 0, right || from.col() == 7),
                        );
                    }
                    Move::LeftCastle { mover: _ } | Move::RightCastle { mover: _ } => {
                        board.set_rights(color, Castling::None);
                    }
                    Move::PawnPromo {
                        from: _,
                        to: _,
                        piece: _,
                    } => unreachable!(),
                }
            }
        }
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
                piece: Piece::Pawn,
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
                piece: Piece::Pawn,
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
        assert_eq!(5, mem::size_of::<Move>());
        assert_eq!(8, mem::size_of::<&Move>());
    }
}
