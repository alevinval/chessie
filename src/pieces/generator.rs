use crate::{
    board::Board,
    pos::{Direction, Pos},
};

use super::BitBoard;

#[derive(Debug, Eq, PartialEq)]
pub enum Placement {
    Invalid,
    Empty,
    Takes,
}

impl Placement {
    pub fn stop(&self) -> bool {
        matches!(self, Self::Invalid | Self::Takes)
    }

    pub fn placed(&self) -> bool {
        matches!(self, Self::Takes | Self::Empty)
    }
}

type PlacementCnd = fn(&Board, Pos, Pos) -> Placement;

#[derive(Debug)]
pub struct Generator<'board> {
    board: &'board Board,
    from: Pos,
    moves: BitBoard,
}

impl<'board> Generator<'board> {
    pub fn new<P: Into<Pos>>(board: &'board Board, from: P) -> Self {
        Generator {
            board,
            from: from.into(),
            moves: BitBoard::default(),
        }
    }

    pub fn row(&self) -> u8 {
        self.from.row()
    }

    pub fn col(&self) -> u8 {
        self.from.col()
    }

    pub fn dir(&mut self, d: Direction, cnd: PlacementCnd) -> Placement {
        let to = self.from.to(d);
        self.pos(to, cnd)
    }

    pub fn pos<P: Into<Pos>>(&mut self, to: P, cnd: PlacementCnd) -> Placement {
        let to = to.into();
        match cnd(self.board, self.from, to) {
            Placement::Invalid => Placement::Invalid,
            placement => {
                self.moves.or_mut(to);
                placement
            }
        }
    }

    pub fn moves(self) -> BitBoard {
        self.moves
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn empty_placement(_b: &Board, _from: Pos, _to: Pos) -> Placement {
        Placement::Empty
    }

    fn invalid_placement(_b: &Board, _from: Pos, _to: Pos) -> Placement {
        Placement::Invalid
    }

    fn takes_placement(_b: &Board, _from: Pos, _to: Pos) -> Placement {
        Placement::Takes
    }

    #[test]
    fn placement_stop_when_applicable() {
        assert!(Placement::Invalid.stop());
        assert!(Placement::Takes.stop());

        assert!(!Placement::Empty.stop());
    }

    #[test]
    fn placement_is_placed_when_applicable() {
        assert!(Placement::Empty.placed());
        assert!(Placement::Takes.placed());

        assert!(!Placement::Invalid.placed());
    }

    #[test]
    fn generator_row_and_col() {
        let board = Board::default();
        let sut = Generator::new(&board, (1, 3));

        assert_eq!(1, sut.row());
        assert_eq!(3, sut.col());
    }

    #[test]
    fn generator_default_bitboard() {
        let board = Board::default();
        let sut = Generator::new(&board, (1, 3));

        assert_eq!(BitBoard::default(), sut.moves());
    }

    #[test]
    fn generator_from_direction_empty_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3));

        assert_eq!(
            Placement::Empty,
            sut.dir(Direction::Top(1), empty_placement)
        );

        let expected: BitBoard = (2, 3).into();
        assert_eq!(expected, sut.moves());
    }

    #[test]
    fn generator_from_direction_takes_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3));

        assert_eq!(
            Placement::Takes,
            sut.dir(Direction::Top(1), takes_placement)
        );

        let expected: BitBoard = (2, 3).into();
        assert_eq!(expected, sut.moves());
    }

    #[test]
    fn generator_from_direction_invalid_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3));

        assert_eq!(
            Placement::Invalid,
            sut.dir(Direction::Top(1), invalid_placement)
        );

        assert_eq!(BitBoard::default(), sut.moves());
    }
}
