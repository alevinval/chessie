use crate::defs::{BitBoard, Sq};

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

    use crate::squares::*;

    use super::*;

    #[test_case(A1, 0x1)]
    #[test_case(D1, 0x8)]
    #[test_case(A2, 0x100)]
    #[test_case(D2, 0x800)]
    #[test_case(H8, 0x8000000000000000)]
    fn test_bb(sq: Sq, expected: BitBoard) {
        assert_eq!(expected, bb(sq));
    }

    #[test_case(A1, 0)]
    #[test_case(D1, 0)]
    #[test_case(A2, 1)]
    #[test_case(D2, 1)]
    #[test_case(H8, 7)]
    fn test_row(sq: Sq, expected: u8) {
        assert_eq!(expected, row(sq));
    }

    #[test_case(A1, 0)]
    #[test_case(D1, 3)]
    #[test_case(A2, 0)]
    #[test_case(D2, 3)]
    #[test_case(H8, 7)]
    fn test_col(sq: Sq, expected: u8) {
        assert_eq!(expected, col(sq));
    }

    #[test_case(A1, false)]
    #[test_case(D1, false)]
    #[test_case(A2, false)]
    #[test_case(D2, false)]
    #[test_case(H8, false)]
    #[test_case(D4, true)]
    #[test_case(E4, true)]
    #[test_case(D5, true)]
    #[test_case(E5, true)]
    fn test_is_central(sq: Sq, expected: bool) {
        assert_eq!(expected, is_central(sq));
    }

    #[test]
    fn test_str() {
        assert_eq!("d3", str(D3))
    }
}
