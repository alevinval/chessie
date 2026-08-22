use std::fmt;

use crate::{
    Color,
    board::Board,
    defs::{CastlingUpdate, Sq},
    piece::Piece,
    pos,
    squares::*,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Takes {
        from: Sq,
        to: Sq,
        piece: Piece,
        value: i32,
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
    pub(crate) fn priority(self) -> i32 {
        match self {
            Move::Slide { .. } => 1,
            Move::LeftCastle { .. } => 10,
            Move::RightCastle { .. } => 10,
            Move::Takes { value, .. } => 100 + value,
            Move::PawnPromo { .. } => 900,
        }
    }

    pub(crate) fn apply(self, board: &mut Board) {
        let mover = board.state().mover();
        let opponent = mover.flip();

        match self {
            Move::Takes { from, to, castling_update, target_castling_update, .. } => {
                board.clear(to);
                board.slide(from, to);
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
                board.slide(from, to);
            }
            Move::PawnPromo { from, to, promo_piece: piece, .. } => {
                board.clear(from);
                board.clear(to);
                board.add(mover, piece, to);
            }
            Move::LeftCastle { mover, castling_update } => {
                board.disable_castling(mover, castling_update);
                let (king_from, king_to, rook_from, rook_to) = match mover {
                    Color::B => (E8, C8, A8, D8),
                    Color::W => (E1, C1, A1, D1),
                };
                board.slide(king_from, king_to);
                board.slide(rook_from, rook_to);
            }
            Move::RightCastle { mover, castling_update } => {
                board.disable_castling(mover, castling_update);
                let (king_from, king_to, rook_from, rook_to) = match mover {
                    Color::B => (E8, G8, H8, F8),
                    Color::W => (E1, G1, H1, F1),
                };
                board.slide(king_from, king_to);
                board.slide(rook_from, rook_to);
            }
        }
    }

    pub(crate) fn unapply(&self, board: &mut Board) {
        let opponent = board.state().mover();
        let mover = opponent.flip();
        match *self {
            Move::Takes { from, to, piece, castling_update, target_castling_update, .. } => {
                board.slide(to, from);
                board.add(opponent, piece, to);
                if let Some(update) = castling_update {
                    board.enable_castling(mover, update);
                }
                if let Some(update) = target_castling_update {
                    board.enable_castling(opponent, update);
                }
            }
            Move::Slide { from, to, castling_update, .. } => {
                board.slide(to, from);
                if let Some(update) = castling_update {
                    board.enable_castling(mover, update);
                }
            }
            Move::PawnPromo { from, to, taken_piece, .. } => {
                board.clear(to);
                board.add(mover, Piece::Pawn, from);
                if let Some(piece) = taken_piece {
                    board.add(opponent, piece, to);
                }
            }
            Move::LeftCastle { mover, castling_update } => {
                board.enable_castling(mover, castling_update);
                let (king_from, king_to, rook_from, rook_to) = match mover {
                    Color::B => (C8, E8, D8, A8),
                    Color::W => (C1, E1, D1, A1),
                };
                board.slide(king_from, king_to);
                board.slide(rook_from, rook_to);
            }
            Move::RightCastle { mover, castling_update } => {
                board.enable_castling(mover, castling_update);
                let (king_from, king_to, rook_from, rook_to) = match mover {
                    Color::B => (G8, E8, F8, H8),
                    Color::W => (G1, E1, F1, H1),
                };
                board.slide(king_from, king_to);
                board.slide(rook_from, rook_to);
            }
        }
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
        assert_eq!(12, mem::size_of::<Move>());
        assert_eq!(8, mem::size_of::<&Move>());
    }
}
