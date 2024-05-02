use std::fmt;

use crate::{
    bits,
    board::Board,
    defs::{CastlingUpdate, Sq},
    piece::Piece,
    pos, sq, Color,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Move {
    Takes {
        from: Sq,
        to: Sq,
        piece: Piece,
        value: f64,
        castling_update: CastlingUpdate,
        target_castling_update: CastlingUpdate,
    },
    Slide {
        from: Sq,
        to: Sq,
        castling_update: CastlingUpdate,
    },
    PawnPromo {
        from: Sq,
        to: Sq,
        promo_piece: Piece,
        taken_piece: Option<Piece>,
    },
    LeftCastle {
        from: Sq,
        to: Sq,
        castling_update: CastlingUpdate,
    },
    RightCastle {
        from: Sq,
        to: Sq,
        castling_update: CastlingUpdate,
    },
}

impl Move {
    #[must_use]
    pub(crate) const fn to(self) -> Sq {
        match self {
            Move::Slide { to, .. }
            | Move::Takes { to, .. }
            | Move::PawnPromo { to, .. }
            | Move::LeftCastle { to, .. }
            | Move::RightCastle { to, .. } => to,
        }
    }

    #[must_use]
    pub(crate) const fn from(self) -> Sq {
        match self {
            Move::Slide { from, .. }
            | Move::Takes { from, .. }
            | Move::PawnPromo { from, .. }
            | Move::LeftCastle { from, .. }
            | Move::RightCastle { from, .. } => from,
        }
    }

    #[must_use]
    pub(crate) fn priority(self) -> f64 {
        match self {
            Move::Slide { .. } => 1.0,
            Move::LeftCastle { .. } => 10.0,
            Move::RightCastle { .. } => 13.0,
            Move::Takes { value, .. } => 100.0 + value,
            Move::PawnPromo { .. } => 900.0,
        }
    }

    pub(crate) fn apply(self, board: &mut Board) {
        match self {
            Move::Takes { from, to, castling_update, target_castling_update, .. } => {
                let mover = board.state().mover();
                let opponent = mover.flip();
                Self::clear(board, to);
                board.state_mut().set_castling(mover, castling_update, false);
                board.state_mut().set_castling(opponent, target_castling_update, false);
                self.slide(board, from, to);
            }
            Move::Slide { from, to, castling_update, .. } => {
                let mover = board.state().mover();
                board.state_mut().set_castling(mover, castling_update, false);
                self.slide(board, from, to);
            }
            Move::PawnPromo { from, to, promo_piece: piece, .. } => {
                Self::clear(board, from);
                Self::clear(board, to);
                board.add(board.state().mover(), piece, to);
            }
            Move::LeftCastle { castling_update, .. } => {
                let mover = board.state().mover();
                board.state_mut().set_castling(mover, castling_update, false);
                match mover {
                    Color::B => {
                        self.slide(board, sq!(7, 4), sq!(7, 2));
                        self.slide(board, sq!(7, 0), sq!(7, 3));
                    }
                    Color::W => {
                        self.slide(board, sq!(0, 4), sq!(0, 2));
                        self.slide(board, sq!(0, 0), sq!(0, 3));
                    }
                }
            }
            Move::RightCastle { castling_update, .. } => {
                let mover = board.state().mover();
                board.state_mut().set_castling(mover, castling_update, false);
                match mover {
                    Color::B => {
                        self.slide(board, sq!(7, 4), sq!(7, 6));
                        self.slide(board, sq!(7, 7), sq!(7, 5));
                    }
                    Color::W => {
                        self.slide(board, sq!(0, 4), sq!(0, 6));
                        self.slide(board, sq!(0, 7), sq!(0, 5));
                    }
                }
            }
        }
    }

    fn clear(board: &mut Board, sq: Sq) {
        if let Some((_, _, bb)) = board.at_mut(sq) {
            bits::unset(bb, sq);
        }
    }

    fn slide(self, board: &mut Board, from: Sq, to: Sq) {
        let (_, _, bb) = board.at_mut(from).unwrap_or_else(|| {
            unreachable!("must have a piece in order to move {:?} {:?}", self, from)
        });
        bits::slide(bb, from, to);
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Takes { from, to, .. } | Move::Slide { from, to, .. } => {
                let from = pos::str(*from);
                let to = pos::str(*to);
                f.write_fmt(format_args!("{from} × {to}"))
            }
            Move::PawnPromo { to, promo_piece, .. } => {
                f.write_fmt(format_args!("{to}/{promo_piece}"))
            }
            Move::LeftCastle { .. } => f.write_fmt(format_args!("O-O-O")),
            Move::RightCastle { .. } => f.write_fmt(format_args!("O-O")),
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem;

    use super::*;

    const FROM: Sq = sq!(1, 1);
    const TO: Sq = sq!(3, 3);

    #[test]
    fn to() {
        assert_eq!(
            TO,
            Move::Slide { from: FROM, to: TO, castling_update: CastlingUpdate::None }.to()
        );
        assert_eq!(
            TO,
            Move::PawnPromo { from: FROM, to: TO, promo_piece: Piece::Pawn, taken_piece: None }
                .to()
        );
        assert_eq!(
            sq!(0, 2),
            Move::LeftCastle { from: FROM, to: TO, castling_update: CastlingUpdate::Left }.to()
        );
        assert_eq!(
            sq!(7, 2),
            Move::LeftCastle { from: FROM, to: TO, castling_update: CastlingUpdate::Both }.to()
        );
        assert_eq!(
            sq!(0, 6),
            Move::RightCastle { from: FROM, to: TO, castling_update: CastlingUpdate::Right }.to()
        );
        assert_eq!(
            sq!(7, 6),
            Move::RightCastle { from: FROM, to: TO, castling_update: CastlingUpdate::Both }.to()
        );
    }
    #[test]
    fn from() {
        assert_eq!(
            FROM,
            Move::Slide { from: FROM, to: TO, castling_update: CastlingUpdate::None }.from()
        );
        assert_eq!(
            FROM,
            Move::PawnPromo { from: FROM, to: TO, promo_piece: Piece::Pawn, taken_piece: None }
                .from()
        );
        assert_eq!(
            sq!(0, 4),
            Move::LeftCastle { from: FROM, to: TO, castling_update: CastlingUpdate::Left }.from()
        );
        assert_eq!(
            sq!(7, 4),
            Move::LeftCastle { from: FROM, to: TO, castling_update: CastlingUpdate::Both }.from()
        );
        assert_eq!(
            sq!(0, 4),
            Move::RightCastle { from: FROM, to: TO, castling_update: CastlingUpdate::Right }.from()
        );
        assert_eq!(
            sq!(7, 4),
            Move::RightCastle { from: FROM, to: TO, castling_update: CastlingUpdate::Both }.from()
        );
    }

    #[test]
    fn size() {
        assert_eq!(16, mem::size_of::<Move>());
        assert_eq!(8, mem::size_of::<&Move>());
    }
}
