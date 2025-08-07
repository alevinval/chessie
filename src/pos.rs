use crate::defs::{BitBoard, Sq};

#[macro_export]
macro_rules! sq {
    ( $row:expr, $col:expr ) => {
        ($row * 8 + $col) as u8
    };
    ( $tuple:expr ) => {
        sq!($tuple.0, $tuple.1)
    };
}

#[must_use]
pub(crate) const fn row(sq: Sq) -> u8 {
    sq >> 3
}

#[must_use]
pub const fn col(sq: Sq) -> u8 {
    sq & 7
}

#[must_use]
pub const fn bb(sq: Sq) -> BitBoard {
    1 << sq
}

#[must_use]
#[allow(dead_code)]
pub(crate) const fn is_central(sq: Sq) -> bool {
    let (row, col) = (row(sq), col(sq));
    row >= 3 && col >= 3 && row <= 4 && col <= 4
}

#[must_use]
pub(crate) fn str(sq: Sq) -> String {
    format!("{}{}", display_col(col(sq)), display_row(row(sq)))
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
    fn test_bb((r, c): (u8, u8), expected: BitBoard) {
        let pos = sq!(r, c);
        assert_eq!(expected, bb(pos));
    }

    #[test_case((0, 0), 0)]
    #[test_case((0, 3), 0)]
    #[test_case((1, 0), 1)]
    #[test_case((1, 3), 1)]
    #[test_case((7, 7), 7)]
    fn test_row((r, c): (u8, u8), expected: u8) {
        let pos = sq!(r, c);
        assert_eq!(expected, row(pos));
    }

    #[test_case((0, 0), 0)]
    #[test_case((0, 3), 3)]
    #[test_case((1, 0), 0)]
    #[test_case((1, 3), 3)]
    #[test_case((7, 7), 7)]
    fn test_col((r, c): (u8, u8), expected: u8) {
        let pos = sq!(r, c);
        assert_eq!(expected, col(pos));
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
    fn test_is_central((r, c): (u8, u8), expected: bool) {
        let pos = sq!(r, c);
        assert_eq!(expected, is_central(pos));
    }

    #[test]
    fn test_str() {
        assert_eq!("d3", str(sq!(2, 3)))
    }
}
