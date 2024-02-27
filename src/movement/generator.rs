use crate::{
    board::Board,
    pos::{Dir, Pos},
};

use super::placement::{Placement, PlacementCnd};

#[derive(Debug)]
pub struct Generator<'board> {
    board: &'board Board,
    from: Pos,
    takes: Vec<(Pos, Pos)>,
    empty: Vec<(Pos, Pos)>,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Moves {
    pub takes: Vec<(Pos, Pos)>,
    pub empty: Vec<(Pos, Pos)>,
}

impl<'board> Generator<'board> {
    pub fn new<P: Into<Pos>>(board: &'board Board, from: P) -> Self {
        Generator {
            board,
            from: from.into(),
            takes: vec![],
            empty: vec![],
        }
    }

    pub fn row(&self) -> u8 {
        self.from.row()
    }

    pub fn col(&self) -> u8 {
        self.from.col()
    }

    pub fn dir(&mut self, d: Dir, cnd: PlacementCnd) -> Placement {
        let to = self.from.to(d);
        self.pos(to, cnd)
    }

    pub fn pos<P: Into<Pos>>(&mut self, to: P, cnd: PlacementCnd) -> Placement {
        let to = to.into();
        let placement = cnd(self.board, self.from, to);
        match placement {
            Placement::Empty(from, to) => self.empty.push((from, to)),
            Placement::Takes(from, to) => self.takes.push((from, to)),
            Placement::Invalid => {}
        }

        placement
    }

    pub fn moves(self) -> Moves {
        Moves {
            takes: self.takes,
            empty: self.empty,
        }
    }
}

impl Moves {
    pub fn into_iter(self) -> impl Iterator<Item = (Pos, Pos)> {
        self.takes.into_iter().chain(self.empty)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static FROM: Pos = Pos::new(1, 1);
    static TO: Pos = Pos::new(2, 2);

    fn empty_placement(_b: &Board, from: Pos, to: Pos) -> Placement {
        Placement::Empty(from, to)
    }

    fn invalid_placement(_b: &Board, _from: Pos, _to: Pos) -> Placement {
        Placement::Invalid
    }

    fn takes_placement(_b: &Board, from: Pos, to: Pos) -> Placement {
        Placement::Takes(from, to)
    }

    #[test]
    fn placement_stop_when_applicable() {
        assert!(Placement::Invalid.stop());
        assert!(Placement::Takes(FROM, TO).stop());

        assert!(!Placement::Empty(FROM, TO).stop());
    }

    #[test]
    fn placement_is_placed_when_applicable() {
        assert!(Placement::Empty(FROM, TO).placed());
        assert!(Placement::Takes(FROM, TO).placed());

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

        assert_eq!(Moves::default(), sut.moves());
    }

    #[test]
    fn generator_from_direction_empty_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3));

        assert_eq!(
            Placement::Empty((1, 3).into(), (2, 3).into()),
            sut.dir(Dir::Up(1), empty_placement)
        );

        let expected = Moves {
            empty: vec![((1, 3).into(), (2, 3).into())],
            takes: vec![],
        };
        assert_eq!(expected, sut.moves());
    }

    #[test]
    fn generator_from_direction_takes_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3));

        assert_eq!(
            Placement::Takes((1, 3).into(), (2, 3).into()),
            sut.dir(Dir::Up(1), takes_placement)
        );

        let expected = Moves {
            takes: vec![((1, 3).into(), (2, 3).into())],
            empty: vec![],
        };
        assert_eq!(expected, sut.moves());
    }

    #[test]
    fn generator_from_direction_invalid_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3));

        assert_eq!(Placement::Invalid, sut.dir(Dir::Up(1), invalid_placement));

        assert_eq!(Moves::default(), sut.moves());
    }
}
