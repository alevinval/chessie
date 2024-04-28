use std::fmt;

use crate::{
    bits::Bits,
    board::Board,
    defs::{Castling, Sq},
    piece::Piece,
    pos::Pos,
    Color,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Move {
    Takes { from: Pos, to: Pos, value: f64 },
    Slide { from: Pos, to: Pos },
    PawnPromo { from: Pos, to: Pos, piece: Piece },
    LeftCastle { mover: Color },
    RightCastle { mover: Color },
}

impl Move {
    #[must_use]
    pub(crate) const fn to(self) -> Sq {
        match self {
            Move::Takes { to, .. } | Move::Slide { to, .. } | Move::PawnPromo { to, .. } => to.sq(),
            Move::LeftCastle { mover } => Pos::new(mover.piece_row(), 2).sq(),
            Move::RightCastle { mover } => Pos::new(mover.piece_row(), 6).sq(),
        }
    }

    #[must_use]
    pub(crate) const fn from(self) -> Sq {
        match self {
            Move::Takes { from, .. } | Move::Slide { from, .. } | Move::PawnPromo { from, .. } => {
                from.sq()
            }
            Move::LeftCastle { mover } | Move::RightCastle { mover } => {
                Pos::new(mover.piece_row(), 4).sq()
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

    #[must_use]
    pub(crate) fn apply(self, board: &Board) -> Board {
        let mut next = board.clone();
        self.inner_apply(&mut next);
        next.advance();
        next
    }

    fn inner_apply(self, board: &mut Board) {
        match self {
            Move::Takes { from, to, .. } => {
                Self::clear(board, to);
                self.update_castling(board, from);
                self.slide(board, from, to);
            }
            Move::Slide { from, to } => {
                self.update_castling(board, from);
                self.slide(board, from, to);
            }
            Move::PawnPromo { from, to, piece } => {
                Self::clear(board, from);
                Self::clear(board, to);
                board.add(board.state().mover(), piece, to);
            }
            Move::LeftCastle { mover } => {
                board.state_mut().set_castled();
                match mover {
                    Color::B => {
                        self.slide(board, Pos::new(7, 4), Pos::new(7, 2));
                        self.slide(board, Pos::new(7, 0), Pos::new(7, 3));
                    }
                    Color::W => {
                        self.slide(board, Pos::new(0, 4), Pos::new(0, 2));
                        self.slide(board, Pos::new(0, 0), Pos::new(0, 3));
                    }
                }
            }
            Move::RightCastle { mover } => {
                board.state_mut().set_castled();
                match mover {
                    Color::B => {
                        self.slide(board, Pos::new(7, 4), Pos::new(7, 6));
                        self.slide(board, Pos::new(7, 7), Pos::new(7, 5));
                    }
                    Color::W => {
                        self.slide(board, Pos::new(0, 4), Pos::new(0, 6));
                        self.slide(board, Pos::new(0, 7), Pos::new(0, 5));
                    }
                }
            }
        }
    }

    fn clear(board: &mut Board, pos: Pos) {
        if let Some((_, _, bb)) = board.at_mut(pos) {
            Bits::unset(bb, pos);
        }
    }

    fn slide(self, board: &mut Board, from: Pos, to: Pos) {
        let (_, _, bb) = board.at_mut(from).unwrap_or_else(|| {
            unreachable!("must have a piece in order to move {:?} {:?}", self, from)
        });
        Bits::slide(bb, from, to);
    }

    fn update_castling(self, board: &mut Board, from: Pos) {
        let color = board.state().mover();
        if let Castling::Some { left, right } = board.state().castling(color) {
            if !left && !right {
                board.state_mut().set_castled();
                return;
            }

            let (_, piece, _) = board.at(from).unwrap_or_else(|| {
                unreachable!("must have a piece in order to move {:?} {:?}", self, from)
            });

            if let Piece::King = piece {
                board.state_mut().set_castled();
                return;
            }

            if let Piece::Rook = piece {
                board
                    .state_mut()
                    .update_castling(left || from.col() == 0, right || from.col() == 7);
            }
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Takes { from, to, .. } => f.write_fmt(format_args!("{from} × {to}")),
            Move::Slide { from, to } => f.write_fmt(format_args!("{from} → {to}")),
            Move::PawnPromo { to, piece, .. } => f.write_fmt(format_args!("{to}/{piece}")),
            Move::LeftCastle { .. } => f.write_fmt(format_args!("O-O-O")),
            Move::RightCastle { .. } => f.write_fmt(format_args!("O-O")),
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
        assert_eq!(TO.sq(), Move::Slide { from: FROM, to: TO }.to());
        assert_eq!(TO.sq(), Move::PawnPromo { from: FROM, to: TO, piece: Piece::Pawn }.to());
        assert_eq!(Pos::new(0, 2).sq(), Move::LeftCastle { mover: Color::W }.to());
        assert_eq!(Pos::new(7, 2).sq(), Move::LeftCastle { mover: Color::B }.to());
        assert_eq!(Pos::new(0, 6).sq(), Move::RightCastle { mover: Color::W }.to());
        assert_eq!(Pos::new(7, 6).sq(), Move::RightCastle { mover: Color::B }.to());
    }

    #[test]
    fn from() {
        assert_eq!(FROM.sq(), Move::Slide { from: FROM, to: TO }.from());
        assert_eq!(FROM.sq(), Move::PawnPromo { from: FROM, to: TO, piece: Piece::Pawn }.from());
        assert_eq!(Pos::new(0, 4).sq(), Move::LeftCastle { mover: Color::W }.from());
        assert_eq!(Pos::new(7, 4).sq(), Move::LeftCastle { mover: Color::B }.from());
        assert_eq!(Pos::new(0, 4).sq(), Move::RightCastle { mover: Color::W }.from());
        assert_eq!(Pos::new(7, 4).sq(), Move::RightCastle { mover: Color::B }.from());
    }

    #[test]
    fn size() {
        assert_eq!(16, mem::size_of::<Move>());
        assert_eq!(8, mem::size_of::<&Move>());
    }
}
