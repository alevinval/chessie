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
            | Move::PawnPromo { from: _, to, piece: _ } => to,
            Move::LeftCastle { mover } => (mover.piece_row(), 2).into(),
            Move::RightCastle { mover } => (mover.piece_row(), 6).into(),
        }
    }

    pub fn from(self) -> Pos {
        match self {
            Move::Takes { from, to: _ }
            | Move::Slide { from, to: _ }
            | Move::PawnPromo { from, to: _, piece: _ } => from,
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
        if let Some(dst) = board.at_mut(to) {
            dst.unset(to);
        }
    }

    fn promo<P: Into<Pos>>(board: &mut Board, pos: P, piece: Piece) {
        let pos = pos.into();
        Self::clear_dst(board, pos);

        let pieces = board.pieces_mut();
        match piece {
            Piece::Pawn(_) => &mut pieces[Piece::P],
            Piece::Rook(_) => &mut pieces[Piece::R],
            Piece::Knight(_) => &mut pieces[Piece::N],
            Piece::Bishop(_) => &mut pieces[Piece::B],
            Piece::Queen(_) => &mut pieces[Piece::Q],
            Piece::King(_) => unreachable!("cannot promote pawn to king"),
        }
        .set(pos);
    }

    fn apply_move<P: Into<Pos>>(self, board: &mut Board, from: P, to: P) {
        let from = from.into();
        let bb = board.at_mut(from).unwrap_or_else(|| {
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

            let bb = board.at(from).unwrap_or_else(|| {
                unreachable!("must have a piece in order to move {:?} {:?}", self, from)
            });
            let piece = bb.piece();
            if let Piece::King(_) = piece {
                board.set_castling(color, Castling::None);
                return;
            }

            if let Piece::Rook(_) = bb.piece() {
                board.set_castling(
                    bb.color(),
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
        assert_eq!(TO, Move::PawnPromo { from: FROM, to: TO, piece: Piece::Pawn(Color::W) }.to());
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
            Move::PawnPromo { from: FROM, to: TO, piece: Piece::Pawn(Color::W) }.from()
        );
        assert_eq!(Pos::new(0, 4), Move::LeftCastle { mover: Color::W }.from());
        assert_eq!(Pos::new(7, 4), Move::LeftCastle { mover: Color::B }.from());
        assert_eq!(Pos::new(0, 4), Move::RightCastle { mover: Color::W }.from());
        assert_eq!(Pos::new(7, 4), Move::RightCastle { mover: Color::B }.from());
    }

    #[test]
    fn size() {
        assert_eq!(4, mem::size_of::<Move>());
        assert_eq!(8, mem::size_of::<&Move>());
    }
}
