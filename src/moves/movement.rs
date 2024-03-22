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
    #[must_use]
    pub const fn to(self) -> Pos {
        match self {
            Move::Takes { from: _, to }
            | Move::Slide { from: _, to }
            | Move::PawnPromo { from: _, to, piece: _ } => to,
            Move::LeftCastle { mover } => Pos::new(mover.piece_row(), 2),
            Move::RightCastle { mover } => Pos::new(mover.piece_row(), 6),
        }
    }

    #[must_use]
    pub const fn from(self) -> Pos {
        match self {
            Move::Takes { from, to: _ }
            | Move::Slide { from, to: _ }
            | Move::PawnPromo { from, to: _, piece: _ } => from,
            Move::LeftCastle { mover } | Move::RightCastle { mover } => {
                Pos::new(mover.piece_row(), 4)
            }
        }
    }

    #[must_use]
    pub const fn priority(self) -> f64 {
        match self {
            Move::Takes { from: _, to: _ } => 5.0,
            Move::Slide { from: _, to: _ } => 1.0,
            Move::PawnPromo { from: _, to: _, piece: _ } => 10.0,
            Move::LeftCastle { mover: _ } => 6.0,
            Move::RightCastle { mover: _ } => 7.0,
        }
    }

    #[must_use]
    pub fn apply(self, board: &Board) -> Board {
        let mut next = board.clone();
        self.inner_apply(&mut next);
        next.next_turn();
        next
    }

    fn inner_apply(self, board: &mut Board) {
        match self {
            Move::Takes { from, to } => {
                Self::clear_dst(board, to);
                self.update_castling(board, from);
                self.apply_move(board, from, to);
            }
            Move::Slide { from, to } => {
                self.update_castling(board, from);
                self.apply_move(board, from, to);
            }
            Move::PawnPromo { from, to, piece } => {
                Self::clear_dst(board, to);
                self.apply_move(board, from, to);
                Self::promo(board, to, piece);
            }
            Move::LeftCastle { mover } => {
                Self::disable_castling(board);
                match mover {
                    Color::B => {
                        self.apply_move(board, (7, 4), (7, 2));
                        self.apply_move(board, (7, 0), (7, 3));
                    }
                    Color::W => {
                        self.apply_move(board, (0, 4), (0, 2));
                        self.apply_move(board, (0, 0), (0, 3));
                    }
                }
            }
            Move::RightCastle { mover } => {
                Self::disable_castling(board);
                match mover {
                    Color::B => {
                        self.apply_move(board, (7, 4), (7, 6));
                        self.apply_move(board, (7, 7), (7, 5));
                    }
                    Color::W => {
                        self.apply_move(board, (0, 4), (0, 6));
                        self.apply_move(board, (0, 7), (0, 5));
                    }
                }
            }
        }
    }

    fn clear_dst(board: &mut Board, to: Pos) {
        if let Some((_, _, bb)) = board.at_mut(to) {
            bb.unset(to);
        }
    }

    fn promo<P: Into<Pos>>(board: &mut Board, pos: P, piece: Piece) {
        let pos = pos.into();
        Self::clear_dst(board, pos);
        board.add_piece(pos, piece);
    }

    fn apply_move<P: Into<Pos>>(self, board: &mut Board, from: P, to: P) {
        let from = from.into();
        let (_, _, bb) = board.at_mut(from).unwrap_or_else(|| {
            unreachable!("must have a piece in order to move {:?} {:?}", self, from)
        });
        bb.slide(from, to.into());
    }

    fn update_castling(self, board: &mut Board, from: Pos) {
        let color = board.mover();
        if let Castling::Some(left, right) = board.castling(color) {
            if !left && !right {
                Self::disable_castling(board);
                return;
            }

            let (_, piece, _) = board.at(from).unwrap_or_else(|| {
                unreachable!("must have a piece in order to move {:?} {:?}", self, from)
            });
            if let Piece::King = piece {
                board.set_castling(color, Castling::None);
                return;
            }

            if let Piece::Rook = piece {
                board.set_castling(
                    color,
                    Castling::Some(left || from.col() == 0, right || from.col() == 7),
                );
            }
        }
    }

    fn disable_castling(board: &mut Board) {
        board.set_castling(board.mover(), Castling::None);
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
        assert_eq!(TO, Move::PawnPromo { from: FROM, to: TO, piece: Piece::Pawn }.to());
        assert_eq!(Pos::new(0, 2), Move::LeftCastle { mover: Color::W }.to());
        assert_eq!(Pos::new(7, 2), Move::LeftCastle { mover: Color::B }.to());
        assert_eq!(Pos::new(0, 6), Move::RightCastle { mover: Color::W }.to());
        assert_eq!(Pos::new(7, 6), Move::RightCastle { mover: Color::B }.to());
    }

    #[test]
    fn from() {
        assert_eq!(FROM, Move::Slide { from: FROM, to: TO }.from());
        assert_eq!(FROM, Move::PawnPromo { from: FROM, to: TO, piece: Piece::Pawn }.from());
        assert_eq!(Pos::new(0, 4), Move::LeftCastle { mover: Color::W }.from());
        assert_eq!(Pos::new(7, 4), Move::LeftCastle { mover: Color::B }.from());
        assert_eq!(Pos::new(0, 4), Move::RightCastle { mover: Color::W }.from());
        assert_eq!(Pos::new(7, 4), Move::RightCastle { mover: Color::B }.from());
    }

    #[test]
    fn size() {
        assert_eq!(3, mem::size_of::<Move>());
        assert_eq!(8, mem::size_of::<&Move>());
    }
}
