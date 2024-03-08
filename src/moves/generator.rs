use std::iter::zip;

use crate::{
    board::Board,
    pos::{Dir, Pos},
};

use super::{
    placement::{Placement, StopCondition},
    Move,
};

#[derive(Debug)]
pub struct Generator<'board> {
    board: &'board Board,
    from: Pos,
    moves: Vec<Move>,
    check_legal: bool,
}

impl<'board> Generator<'board> {
    pub fn new<P: Into<Pos>>(board: &'board Board, from: P, check_legal: bool) -> Self {
        Generator {
            board,
            from: from.into(),
            moves: vec![],
            check_legal,
        }
    }

    pub fn row(&self) -> u8 {
        self.from.row()
    }

    pub fn col(&self) -> u8 {
        self.from.col()
    }

    pub fn emit_move(&mut self, m: Move) {
        if !self.check_legal || self.is_legal(m) {
            self.moves.push(m);
        }
    }

    pub fn check_dir(&self, d: Dir, stop_at: StopCondition) -> Placement {
        let to = self.from.to(d);
        stop_at(self.board, self.from, to)
    }

    pub fn dir(&mut self, d: Dir, stop_at: StopCondition) -> Placement {
        let to = self.from.to(d);
        self.pos(to, stop_at)
    }

    pub fn pos<P: Into<Pos>>(&mut self, to: P, stop_at: StopCondition) -> Placement {
        let to = to.into();
        let placement = stop_at(self.board, self.from, to);

        if let Some(m) = placement.movement() {
            self.emit_move(m);
        }
        placement
    }

    pub fn moves(self) -> Vec<Move> {
        self.moves
    }

    pub fn pawn_promo(&mut self, d: Dir) {
        let to = self.from.to(d);

        self.board
            .pieces(self.board.mover())
            .iter()
            .filter(|bitboard| !bitboard.piece().is_king())
            .for_each(|bitboard| {
                let promo = Move::PawnPromo {
                    from: self.from,
                    to,
                    piece: bitboard.piece(),
                };
                self.emit_move(promo);
            });
    }

    pub fn left(&mut self, stop_at: StopCondition) {
        for c in (0..self.col()).rev() {
            if self.pos((self.row(), c), stop_at).stop() {
                break;
            }
        }
    }

    pub fn right(&mut self, stop_at: StopCondition) {
        for c in self.col() + 1..8 {
            if self.pos((self.row(), c), stop_at).stop() {
                break;
            }
        }
    }

    pub fn cross(&mut self, stop_at: StopCondition) {
        let (row, col) = (self.row(), self.col());

        for r in (0..row).rev() {
            if self.pos((r, col), stop_at).stop() {
                break;
            }
        }

        for r in row + 1..8 {
            if self.pos((r, col), stop_at).stop() {
                break;
            }
        }

        self.left(stop_at);
        self.right(stop_at);
    }

    pub fn diagonals(&mut self, stop_at: StopCondition) {
        let (row, col) = (self.row(), self.col());

        for pos in zip(row + 1..8, col + 1..8) {
            if self.pos(pos, stop_at).stop() {
                break;
            }
        }

        for pos in zip((0..row).rev(), col + 1..8) {
            if self.pos(pos, stop_at).stop() {
                break;
            }
        }

        for pos in zip(row + 1..8, (0..col).rev()) {
            if self.pos(pos, stop_at).stop() {
                break;
            }
        }

        for pos in zip((0..row).rev(), (0..col).rev()) {
            if self.pos(pos, stop_at).stop() {
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

    static FROM: Pos = Pos::new(1, 1);
    static TO: Pos = Pos::new(2, 2);

    fn empty_placement(_b: &Board, from: Pos, to: Pos) -> Placement {
        Placement::Empty { from, to }
    }

    fn invalid_placement(_b: &Board, _from: Pos, _to: Pos) -> Placement {
        Placement::Invalid
    }

    fn takes_placement(_b: &Board, from: Pos, to: Pos) -> Placement {
        Placement::Takes { from, to }
    }

    #[test]
    fn placement_stop_when_applicable() {
        assert!(Placement::Invalid.stop());
        assert!(Placement::Takes { from: FROM, to: TO }.stop());

        assert!(!Placement::Empty { from: FROM, to: TO }.stop());
    }

    #[test]
    fn placement_is_placed_when_applicable() {
        assert!(Placement::Empty { from: FROM, to: TO }.placed());
        assert!(Placement::Takes { from: FROM, to: TO }.placed());

        assert!(!Placement::Invalid.placed());
    }

    #[test]
    fn generator_row_and_col() {
        let board = Board::default();
        let sut = Generator::new(&board, (1, 3), false);

        assert_eq!(1, sut.row());
        assert_eq!(3, sut.col());
    }

    #[test]
    fn generator_default_bitboard() {
        let board = Board::default();
        let sut = Generator::new(&board, (1, 3), false);

        assert!(sut.moves().is_empty());
    }

    #[test]
    fn generator_from_direction_empty_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3), false);

        assert_eq!(
            Placement::Empty {
                from: (1, 3).into(),
                to: (2, 3).into()
            },
            sut.dir(Dir::Up(1), empty_placement)
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
        let mut sut = Generator::new(&board, (1, 3), false);

        assert_eq!(
            Placement::Takes {
                from: (1, 3).into(),
                to: (2, 3).into()
            },
            sut.dir(Dir::Up(1), takes_placement)
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
        let mut sut = Generator::new(&board, (1, 3), false);

        assert_eq!(Placement::Invalid, sut.dir(Dir::Up(1), invalid_placement));

        assert!(sut.moves().is_empty());
    }
}
