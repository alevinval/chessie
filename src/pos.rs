use std::fmt::Display;

use crate::defs::{BitBoard, Dir, Sq};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos(Sq);

impl Pos {
    #[must_use]
    pub const fn new(row: usize, col: usize) -> Self {
        Self(row * 8 + col)
    }

    #[must_use]
    pub const fn sq(self) -> Sq {
        self.0
    }

    #[must_use]
    pub const fn row(self) -> usize {
        self.0 >> 3
    }

    #[must_use]
    pub const fn col(self) -> usize {
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

impl From<(usize, usize)> for Pos {
    fn from((row, col): (usize, usize)) -> Self {
        Self::new(row, col)
    }
}

impl From<Pos> for BitBoard {
    fn from(val: Pos) -> Self {
        val.bb()
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.row(), self.col()))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn bb() {
        assert_eq!(0x1, Pos::new(0, 0).bb());
        assert_eq!(0x8, Pos::new(0, 3).bb());
        assert_eq!(0x100, Pos::new(1, 0).bb());
        assert_eq!(0x800, Pos::new(1, 3).bb());
    }

    #[test]
    fn sq() {
        assert_eq!(0, Pos::new(0, 0).sq());
        assert_eq!(3, Pos::new(0, 3).sq());
        assert_eq!(8, Pos::new(1, 0).sq());
        assert_eq!(11, Pos::new(1, 3).sq());
    }

    #[test]
    fn row() {
        assert_eq!(0, Pos::new(0, 0).row());
        assert_eq!(0, Pos::new(0, 3).row());
        assert_eq!(1, Pos::new(1, 0).row());
        assert_eq!(1, Pos::new(1, 5).row());
    }

    #[test]
    fn col() {
        assert_eq!(0, Pos::new(5, 0).col());
        assert_eq!(3, Pos::new(6, 3).col());
        assert_eq!(7, Pos::new(3, 7).col());
        assert_eq!(5, Pos::new(1, 5).col());
    }

    #[test]
    fn is_central() {
        let mut sut = Pos::new(4, 4);
        assert!(sut.is_central());
        sut = Pos::new(3, 4);
        assert!(sut.is_central());
        sut = Pos::new(3, 3);
        assert!(sut.is_central());
        sut = Pos::new(4, 3);
        assert!(sut.is_central());

        sut = Pos::new(5, 3);
        assert!(!sut.is_central());
        sut = Pos::new(2, 3);
        assert!(!sut.is_central());
        sut = Pos::new(4, 5);
        assert!(!sut.is_central());
    }

    #[test]
    fn display() {
        assert_eq!("(2,3)", Pos::new(2, 3).to_string())
    }
}
