use std::fmt::Display;

use crate::defs::{BitBoard, Dir, Sq};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos(Sq);

impl Pos {
    #[must_use]
    pub const fn new(row: u8, col: u8) -> Self {
        Self(row * 8 + col)
    }

    #[must_use]
    pub const fn sq(self) -> Sq {
        self.0
    }

    #[must_use]
    pub const fn row(self) -> u8 {
        self.0 >> 3
    }

    #[must_use]
    pub const fn col(self) -> u8 {
        self.0 & 7
    }

    #[must_use]
    pub const fn bb(self) -> BitBoard {
        1 << self.0
    }

    #[must_use]
    pub const fn is_central(self) -> bool {
        let (row, col) = (self.row(), self.col());
        row >= 3 && col >= 3 && row <= 4 && col <= 4
    }

    #[must_use]
    pub const fn to(self, d: Dir) -> Self {
        Self(d.apply(self.sq()))
    }
}

impl From<Sq> for Pos {
    fn from(value: Sq) -> Self {
        Self(value)
    }
}

impl From<(u8, u8)> for Pos {
    fn from((row, col): (u8, u8)) -> Self {
        Self::new(row, col)
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.row(), self.col()))
    }
}

#[cfg(test)]
mod test {

    use test_case::test_case;

    use super::*;

    #[test_case((0, 0), 0x1)]
    #[test_case((0, 3), 0x8)]
    #[test_case((1, 0), 0x100)]
    #[test_case((1, 3), 0x800)]
    #[test_case((7, 7), 0x8000000000000000)]
    fn bb<P: Into<Pos>>(input: P, expected: BitBoard) {
        assert_eq!(expected, input.into().bb());
    }

    #[test_case((0, 0), 0)]
    #[test_case((0, 3), 3)]
    #[test_case((1, 0), 8)]
    #[test_case((1, 3), 11)]
    #[test_case((7, 7), 63)]
    fn sq<P: Into<Pos>>(input: P, expected: Sq) {
        assert_eq!(expected, input.into().sq());
    }

    #[test_case((0, 0), 0)]
    #[test_case((0, 3), 0)]
    #[test_case((1, 0), 1)]
    #[test_case((1, 3), 1)]
    #[test_case((7, 7), 7)]
    fn row<P: Into<Pos>>(input: P, expected: u8) {
        assert_eq!(expected, input.into().row());
    }

    #[test_case((0, 0), 0)]
    #[test_case((0, 3), 3)]
    #[test_case((1, 0), 0)]
    #[test_case((1, 3), 3)]
    #[test_case((7, 7), 7)]
    fn col<P: Into<Pos>>(input: P, expected: u8) {
        assert_eq!(expected, input.into().col());
    }

    #[test_case((0, 0), false)]
    #[test_case((0, 3), false)]
    #[test_case((1, 0), false)]
    #[test_case((1, 3), false)]
    #[test_case((7, 7), false)]
    #[test_case((4, 4), true)]
    #[test_case((4, 3), true)]
    #[test_case((3, 3), true)]
    #[test_case((3, 4), true)]
    fn is_central<P: Into<Pos>>(input: P, expected: bool) {
        assert_eq!(expected, input.into().is_central());
    }

    #[test_case((2, 2), Dir::Up(1), (3, 2))]
    #[test_case((2, 2), Dir::Down(1), (1, 2))]
    #[test_case((2, 2), Dir::Right(1), (2, 3))]
    #[test_case((2, 2), Dir::Left(1), (2, 1))]
    #[test_case((2, 2), Dir::Custom(-2, -2), (0, 0))]
    #[test_case((2, 2), Dir::Custom(2, 3), (4, 5))]
    fn to<P: Into<Pos>>(input: P, dir: Dir, expected: P) {
        assert_eq!(expected.into(), input.into().to(dir));
    }

    #[test]
    fn display() {
        assert_eq!("(2,3)", Pos::new(2, 3).to_string())
    }
}
