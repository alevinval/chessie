use crate::{
    board::Board,
    pieces::Color,
    pos::{Dir, Pos},
};

use super::{
    placement::{Placement, PlacementCnd},
    Move,
};

#[derive(Debug)]
pub struct Generator<'board> {
    board: &'board Board,
    color: Color,
    from: Pos,
    moves: Vec<Move>,
}

impl<'board> Generator<'board> {
    pub fn new<P: Into<Pos>>(board: &'board Board, from: P) -> Self {
        let from = from.into();
        Generator {
            board,
            from,
            moves: vec![],
            color: board
                .at(from)
                .expect("should generate moves for a piece")
                .color(),
        }
    }

    pub fn board(&self) -> &Board {
        self.board
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn row(&self) -> u8 {
        self.from.row()
    }

    pub fn col(&self) -> u8 {
        self.from.col()
    }

    pub fn mov(&mut self, m: Move) {
        self.moves.push(m);
    }

    pub fn dir(&mut self, d: Dir, cnd: PlacementCnd) -> Placement {
        let to = self.from.to(d);
        self.pos(to, cnd)
    }

    pub fn pos<P: Into<Pos>>(&mut self, to: P, cnd: PlacementCnd) -> Placement {
        let to = to.into();
        let placement = cnd(self.board, self.from, to);
        if let Some(m) = placement.movement() {
            self.moves.push(m);
        }
        placement
    }

    pub fn moves(self) -> Vec<Move> {
        self.moves
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

        assert!(sut.moves().is_empty());
    }

    #[test]
    fn generator_from_direction_empty_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3));

        assert_eq!(
            Placement::Empty((1, 3).into(), (2, 3).into()),
            sut.dir(Dir::Up(1), empty_placement)
        );

        let expected: Vec<Move> = vec![Move::Basic((1, 3).into(), (2, 3).into())];
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

        let expected = vec![Move::Basic((1, 3).into(), (2, 3).into())];
        assert_eq!(expected, sut.moves());
    }

    #[test]
    fn generator_from_direction_invalid_placement() {
        let board = Board::default();
        let mut sut = Generator::new(&board, (1, 3));

        assert_eq!(Placement::Invalid, sut.dir(Dir::Up(1), invalid_placement));

        assert!(sut.moves().is_empty());
    }
}
