use std::fmt;

use crate::{
    Color, bits,
    board::Board,
    defs::{CastlingUpdate, Sq},
    piece::Piece,
    pos,
    squares::*,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Move {
    Takes {
        from: Sq,
        to: Sq,
        piece: Piece,
        value: f64,
        castling_update: Option<CastlingUpdate>,
        target_castling_update: Option<CastlingUpdate>,
    },
    Slide {
        from: Sq,
        to: Sq,
        castling_update: Option<CastlingUpdate>,
    },
    PawnPromo {
        from: Sq,
        to: Sq,
        promo_piece: Piece,
        taken_piece: Option<Piece>,
    },
    LeftCastle {
        mover: Color,
        castling_update: CastlingUpdate,
    },
    RightCastle {
        mover: Color,
        castling_update: CastlingUpdate,
    },
}

impl Move {
    #[must_use]
    pub(crate) const fn to(self) -> Sq {
        match self {
            Move::Slide { to, .. } | Move::Takes { to, .. } | Move::PawnPromo { to, .. } => to,
            Move::LeftCastle { mover, .. } => mover.piece_row() * 8 + 2,
            Move::RightCastle { mover, .. } => mover.piece_row() * 8 + 6,
        }
    }

    #[must_use]
    pub(crate) const fn from(self) -> Sq {
        match self {
            Move::Slide { from, .. } | Move::Takes { from, .. } | Move::PawnPromo { from, .. } => {
                from
            }
            Move::LeftCastle { mover, .. } | Move::RightCastle { mover, .. } => {
                mover.piece_row() * 8 + 4
            }
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
        let mover = board.state().mover();
        let opponent = mover.flip();

        match self {
            Move::Takes { from, to, castling_update, target_castling_update, .. } => {
                board.clear(to);
                self.slide(board, from, to);
                if let Some(update) = castling_update {
                    board.disable_castling(mover, update);
                }
                if let Some(update) = target_castling_update {
                    board.disable_castling(opponent, update);
                }
            }
            Move::Slide { from, to, castling_update, .. } => {
                if let Some(update) = castling_update {
                    board.disable_castling(mover, update);
                }
                self.slide(board, from, to);
            }
            Move::PawnPromo { from, to, promo_piece: piece, .. } => {
                board.clear(from);
                board.clear(to);
                board.add(mover, piece, to);
            }
            Move::LeftCastle { mover, castling_update } => {
                board.disable_castling(mover, castling_update);
                match mover {
                    Color::B => {
                        self.slide(board, E8, C8);
                        self.slide(board, A8, D8);
                    }
                    Color::W => {
                        self.slide(board, E1, C1);
                        self.slide(board, A1, D1);
                    }
                }
            }
            Move::RightCastle { mover, castling_update } => {
                board.disable_castling(mover, castling_update);
                match mover {
                    Color::B => {
                        self.slide(board, E8, G8);
                        self.slide(board, H8, F8);
                    }
                    Color::W => {
                        self.slide(board, E1, G1);
                        self.slide(board, H1, F1);
                    }
                }
            }
        }
    }

    pub(crate) fn unapply(&self, board: &mut Board) {
        let opponent = board.state().mover();
        let mover = opponent.flip();
        match *self {
            Move::Takes { from, to, piece, castling_update, target_castling_update, .. } => {
                self.slide(board, to, from);
                board.add(opponent, piece, to);
                if let Some(update) = castling_update {
                    board.enable_castling(mover, update);
                }
                if let Some(update) = target_castling_update {
                    board.enable_castling(opponent, update);
                }
            }
            Move::Slide { from, to, castling_update, .. } => {
                self.slide(board, to, from);
                if let Some(update) = castling_update {
                    board.enable_castling(mover, update);
                }
            }
            Move::PawnPromo { from, to, taken_piece, .. } => {
                let (_, _, bb) = board.at_mut(to).unwrap_or_else(|| {
                    unreachable!("unapply pawn promotion without piece on destination: {self}")
                });
                bits::unset(bb, to);
                board.add(mover, Piece::Pawn, from);
                if let Some(piece) = taken_piece {
                    board.add(opponent, piece, to);
                }
            }
            Move::LeftCastle { mover, castling_update } => {
                board.enable_castling(mover, castling_update);
                match mover {
                    Color::B => {
                        self.slide(board, C8, E8);
                        self.slide(board, D8, A8);
                    }
                    Color::W => {
                        self.slide(board, C1, E1);
                        self.slide(board, D1, A1);
                    }
                }
            }
            Move::RightCastle { mover, castling_update } => {
                board.enable_castling(mover, castling_update);
                match mover {
                    Color::B => {
                        self.slide(board, G8, E8);
                        self.slide(board, F8, H8);
                    }
                    Color::W => {
                        self.slide(board, G1, E1);
                        self.slide(board, F1, H1);
                    }
                }
            }
        }
    }

    fn slide(&self, board: &mut Board, from: Sq, to: Sq) {
        let (_, _, bb) = board
            .at_mut(from)
            .unwrap_or_else(|| unreachable!("must have a piece in order to move {:?}", self));
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

    const FROM: Sq = B2;
    const TO: Sq = D4;

    #[test]
    fn to() {
        assert_eq!(TO, Move::Slide { from: FROM, to: TO, castling_update: None }.to());
        assert_eq!(
            TO,
            Move::PawnPromo { from: FROM, to: TO, promo_piece: Piece::Pawn, taken_piece: None }
                .to()
        );
        assert_eq!(
            C1,
            Move::LeftCastle { mover: Color::W, castling_update: CastlingUpdate::Left }.to()
        );
        assert_eq!(
            C8,
            Move::LeftCastle { mover: Color::B, castling_update: CastlingUpdate::Both }.to()
        );
        assert_eq!(
            G1,
            Move::RightCastle { mover: Color::W, castling_update: CastlingUpdate::Right }.to()
        );
        assert_eq!(
            G8,
            Move::RightCastle { mover: Color::B, castling_update: CastlingUpdate::Both }.to()
        );
    }
    #[test]
    fn from() {
        assert_eq!(FROM, Move::Slide { from: FROM, to: TO, castling_update: None }.from());
        assert_eq!(
            FROM,
            Move::PawnPromo { from: FROM, to: TO, promo_piece: Piece::Pawn, taken_piece: None }
                .from()
        );
        assert_eq!(
            E1,
            Move::LeftCastle { mover: Color::W, castling_update: CastlingUpdate::Left }.from()
        );
        assert_eq!(
            E8,
            Move::LeftCastle { mover: Color::B, castling_update: CastlingUpdate::Both }.from()
        );
        assert_eq!(
            E1,
            Move::RightCastle { mover: Color::W, castling_update: CastlingUpdate::Right }.from()
        );
        assert_eq!(
            E8,
            Move::RightCastle { mover: Color::B, castling_update: CastlingUpdate::Both }.from()
        );
    }

    #[test]
    fn size() {
        assert_eq!(16, mem::size_of::<Move>());
        assert_eq!(8, mem::size_of::<&Move>());
    }
}
