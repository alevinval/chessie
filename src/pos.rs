use crate::defs::{BitBoard, Sq};

pub(crate) struct Pos();

#[macro_export]
macro_rules! sq {
    ( $row:expr, $col:expr ) => {
        ($row * 8 + $col) as u8
    };
    ( $tuple:expr ) => {
        sq!($tuple.0, $tuple.1)
    };
}

impl Pos {
    #[must_use]
    pub(crate) const fn row(sq: Sq) -> u8 {
        sq >> 3
    }

    #[must_use]
    pub(crate) const fn col(sq: Sq) -> u8 {
        sq & 7
    }

    #[must_use]
    pub(crate) const fn bb(sq: Sq) -> BitBoard {
        1 << sq
    }

    #[must_use]
    #[allow(dead_code)]
    pub(crate) const fn is_central(sq: Sq) -> bool {
        let (row, col) = (Self::row(sq), Self::col(sq));
        row >= 3 && col >= 3 && row <= 4 && col <= 4
    }

    #[must_use]
    pub(crate) fn str(sq: Sq) -> String {
        format!("{}{}", display_col(Pos::col(sq)), display_row(Pos::row(sq)))
    }
}

const fn display_col(col: u8) -> char {
    match col {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        8..=u8::MAX => panic!("column out of bounds"),
    }
}

const fn display_row(row: u8) -> u8 {
    row + 1
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
    fn bb((row, col): (u8, u8), expected: BitBoard) {
        let pos = sq!(row, col);
        assert_eq!(expected, Pos::bb(pos));
    }

    #[test_case((0, 0), 0)]
    #[test_case((0, 3), 0)]
    #[test_case((1, 0), 1)]
    #[test_case((1, 3), 1)]
    #[test_case((7, 7), 7)]
    fn row((row, col): (u8, u8), expected: u8) {
        let pos = sq!(row, col);
        assert_eq!(expected, Pos::row(pos));
    }

    #[test_case((0, 0), 0)]
    #[test_case((0, 3), 3)]
    #[test_case((1, 0), 0)]
    #[test_case((1, 3), 3)]
    #[test_case((7, 7), 7)]
    fn col((row, col): (u8, u8), expected: u8) {
        let pos = sq!(row, col);
        assert_eq!(expected, Pos::col(pos));
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
    fn is_central((row, col): (u8, u8), expected: bool) {
        let pos = sq!(row, col);
        assert_eq!(expected, Pos::is_central(pos));
    }

    #[test]
    fn display() {
        assert_eq!("d3", Pos::str(sq!(2, 3)))
    }
}
