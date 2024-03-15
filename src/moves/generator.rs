use std::iter::zip;

use crate::defs::{BitBoard, Dir};
use crate::{bitboard::Bits, board::Board, piece::Piece, pos::Pos, Color};

use super::{
    placement::{Placement, StopCondition},
    Move,
};

#[derive(Debug)]
pub struct Generator<'board> {
    board: &'board Board,
    from: Pos,
    color: Color,
    piece: Piece,
    moves: Vec<Move>,
    check_legal: bool,
}

impl<'board> Generator<'board> {
    pub fn new<P: Into<Pos>>(
        board: &'board Board,
        from: P,
        color: Color,
        piece: Piece,
        check_legal: bool,
    ) -> Self {
        Generator {
            board,
            from: from.into(),
            color,
            piece,
            moves: vec![],
            check_legal,
        }
    }

    pub fn row(&self) -> usize {
        self.from.row()
    }

    pub fn col(&self) -> usize {
        self.from.col()
    }

    pub fn emit_move(&mut self, m: Move) {
        if !self.check_legal || self.is_legal(m) {
            self.moves.push(m);
        }
    }

    pub fn dir(&mut self, d: Dir, stop_at: StopCondition) -> Option<Placement> {
        self.pos(self.from.to(d), stop_at)
    }

    pub fn pos<P: Into<Pos>>(&mut self, to: P, stop_at: StopCondition) -> Option<Placement> {
        let placement = stop_at(self.board, self.from, to.into());

        if let Some(placement) = &placement {
            self.emit_move(placement.movement());
        }

        placement
    }

    pub fn moves(self) -> Vec<Move> {
        self.moves
    }

    pub fn moves_from_magic(&mut self, mut bb: BitBoard) {
        bb ^= bb & self.board.side(self.board.mover());
        let takes = bb & self.board.side(self.board.mover().opposite());
        let empty = bb & !takes;

        self.takes_from_magic(takes);
        self.slides_from_magic(empty);
    }

    pub fn takes_from_magic(&mut self, bb: BitBoard) {
        let from = self.from;

        for to in Bits::pos(bb) {
            if self.piece == Piece::Pawn && to.row() == self.color.opposite().piece_row() {
                self.emit_pawn_promos(to);
                continue;
            }
            self.emit_move(Move::Takes { from, to });
        }
    }

    pub fn slides_from_magic(&mut self, bb: BitBoard) {
        let from = self.from;
        for to in Bits::pos(bb) {
            if self.piece == Piece::Pawn && to.row() == self.color.opposite().piece_row() {
                self.emit_pawn_promos(to);
                continue;
            }
            self.emit_move(Move::Slide { from, to });
        }
    }

    pub fn emit_pawn_promos(&mut self, to: Pos) {
        for piece in Piece::PROMO {
            let promo = Move::PawnPromo {
                from: self.from,
                to,
                piece,
            };
            self.emit_move(promo);
        }
    }

    pub fn left(&mut self, stop_at: StopCondition) {
        for c in (0..self.col()).rev() {
            if !self
                .pos((self.row(), c), stop_at)
                .is_some_and(|p| !p.stop())
            {
                break;
            }
        }
    }

    pub fn right(&mut self, stop_at: StopCondition) {
        for c in self.col() + 1..8 {
            if !self
                .pos((self.row(), c), stop_at)
                .is_some_and(|p| !p.stop())
            {
                break;
            }
        }
    }

    pub fn cross(&mut self, stop_at: StopCondition) {
        let (row, col) = (self.row(), self.col());

        for r in (0..row).rev() {
            if !self.pos((r, col), stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }

        for r in row + 1..8 {
            if !self.pos((r, col), stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }

        self.left(stop_at);
        self.right(stop_at);
    }

    pub fn diagonals(&mut self, stop_at: StopCondition) {
        let (row, col) = (self.row(), self.col());

        for pos in zip(row + 1..8, col + 1..8) {
            if !self.pos(pos, stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }

        for pos in zip((0..row).rev(), col + 1..8) {
            if !self.pos(pos, stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }

        for pos in zip(row + 1..8, (0..col).rev()) {
            if !self.pos(pos, stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }

        for pos in zip((0..row).rev(), (0..col).rev()) {
            if !self.pos(pos, stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }
    }

    fn is_legal(&self, movement: Move) -> bool {
        let next = movement.apply(self.board);
        !next.in_check(self.board.mover())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn empty_placement(_b: &Board, from: Pos, to: Pos) -> Option<Placement> {
        Some(Placement::Empty { from, to })
    }

    fn invalid_placement(_b: &Board, _from: Pos, _to: Pos) -> Option<Placement> {
        None
    }

    fn takes_placement(_b: &Board, from: Pos, to: Pos) -> Option<Placement> {
        Some(Placement::Takes { from, to })
    }

    #[test]
    fn generator_row_and_col() {
        let board = Board::default();
        let sut = Generator::new(&board, (1, 3), Color::W, Piece::Pawn, false);

        assert_eq!(1, sut.row());
        assert_eq!(3, sut.col());
    }

    #[test]
    fn generator_default_bitboard() {
        let board = Board::default();
        let sut = Generator::new(&board, (1, 3), Color::W, Piece::Pawn, false);

        assert!(sut.moves().is_empty());
    }

    #[test]
    fn generator_from_direction_empty_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3), Color::W, Piece::Pawn, false);

        assert_eq!(
            Some(Placement::Empty {
                from: (1, 3).into(),
                to: (2, 3).into()
            }),
            sut.dir(Dir::Up, empty_placement)
        );

        let expected: Vec<Move> = vec![Move::Slide {
            from: (1, 3).into(),
            to: (2, 3).into(),
        }];
        assert_eq!(expected, sut.moves());
    }

    #[test]
    fn generator_from_direction_takes_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3), Color::W, Piece::Pawn, false);

        assert_eq!(
            Some(Placement::Takes {
                from: (1, 3).into(),
                to: (2, 3).into(),
            }),
            sut.dir(Dir::Up, takes_placement)
        );

        let expected = vec![Move::Takes {
            from: (1, 3).into(),
            to: (2, 3).into(),
        }];
        assert_eq!(expected, sut.moves());
    }

    #[test]
    fn generator_from_direction_invalid_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3), Color::W, Piece::Pawn, false);

        assert_eq!(None, sut.dir(Dir::Up, invalid_placement));

        assert!(sut.moves().is_empty());
    }
}
