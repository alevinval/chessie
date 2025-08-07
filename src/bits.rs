use super::Color;
use crate::{
    defs::{BitBoard, Sq},
    piece::Piece,
    pos,
};

#[must_use]
pub(crate) const fn init(piece: Piece, color: Color) -> BitBoard {
    let bb = match piece {
        Piece::Pawn => 0b1111_1111,
        Piece::Rook => 0b1000_0001,
        Piece::Knight => 0b0100_0010,
        Piece::Bishop => 0b0010_0100,
        Piece::Queen => 0b0000_1000,
        Piece::King => 0b0001_0000,
    };

    bb << (8 * if matches!(piece, Piece::Pawn) { color.pawn_row() } else { color.piece_row() })
}

#[must_use]
pub(crate) const fn count(mut bb: BitBoard) -> usize {
    let mut count = 0;
    while bb != 0 {
        count += 1;
        bb &= bb - 1;
    }
    count
}

#[must_use]
pub(crate) fn has_piece(bb: BitBoard, sq: Sq) -> bool {
    bb & pos::bb(sq) != 0
}

pub(crate) fn slide(bb: &mut BitBoard, from: Sq, to: Sq) {
    *bb ^= pos::bb(from) | pos::bb(to);
}

pub(crate) fn set(bb: &mut BitBoard, sq: Sq) {
    *bb |= pos::bb(sq);
}

pub(crate) fn unset(bb: &mut BitBoard, sq: Sq) {
    *bb &= !pos::bb(sq);
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn pos(mut bb: BitBoard) -> Vec<Sq> {
    let mut acc: Vec<Sq> = vec![];
    let mut square: Sq = 0;
    while bb > 0 {
        let mut shift = bb.trailing_zeros() as u8;
        if shift == 0 {
            acc.push(square);
            shift = 1;
        }
        bb >>= shift;
        square += shift;
    }
    acc
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn first_pos(bb: BitBoard) -> Option<Sq> {
    if bb == 0 {
        return None;
    }
    Some(bb.trailing_zeros() as Sq)
}

#[must_use]
pub const fn north(bb: BitBoard) -> BitBoard {
    bb << 8
}

#[must_use]
pub const fn northeast(bb: BitBoard) -> BitBoard {
    bb << 9
}

#[must_use]
pub const fn northwest(bb: BitBoard) -> BitBoard {
    bb << 7
}

#[must_use]
pub const fn south(bb: BitBoard) -> BitBoard {
    bb >> 8
}

#[must_use]
pub const fn southeast(bb: BitBoard) -> BitBoard {
    bb >> 7
}

#[must_use]
pub const fn southwest(bb: BitBoard) -> BitBoard {
    bb >> 9
}

#[must_use]
pub const fn west(bb: BitBoard) -> BitBoard {
    bb >> 1
}

#[must_use]
pub const fn east(bb: BitBoard) -> BitBoard {
    bb << 1
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{bits, pos, sq, util::print_bitboard};

    static ORIGIN: Sq = sq!(0, 0);
    static TARGET: Sq = sq!(3, 3);

    #[test]
    fn count() {
        let sut = 0x800c00000a007000;
        print_bitboard(sut);
        assert_eq!(8, bits::count(sut));
    }

    #[test]
    fn pos() {
        let sut = 0x800c00000a007000;
        let expected: Vec<Sq> = vec![12, 13, 14, 25, 27, 50, 51, 63];
        assert_eq!(expected, bits::pos(sut));
    }

    #[test]
    fn first_pos() {
        let sut = 0x800c00000a007000;
        assert_eq!(Some(12), bits::first_pos(sut));
    }

    #[test]
    fn has_piece() {
        assert!(!bits::has_piece(0, ORIGIN));
        assert!(bits::has_piece(pos::bb(ORIGIN), ORIGIN));

        assert!(!bits::has_piece(0, TARGET));
        assert!(bits::has_piece(pos::bb(TARGET), TARGET));
    }

    #[test]
    fn slide() {
        let from = sq!(1, 1);
        let to = sq!(2, 2);

        let mut sut = bits::init(Piece::Pawn, Color::W);
        assert!(!bits::has_piece(sut, to));

        bits::slide(&mut sut, from, to);
        assert!(bits::has_piece(sut, to));

        assert!(!bits::has_piece(sut, from));
    }

    #[test]
    fn unset() {
        let pos = sq!(1, 1);
        let mut sut = bits::init(Piece::Pawn, Color::W);
        assert!(bits::has_piece(sut, pos));

        bits::unset(&mut sut, pos);
        assert!(!bits::has_piece(sut, pos));
    }

    #[test]
    fn north() {
        let input: BitBoard = 0x1;
        print_bitboard(input);

        let actual = bits::north(input);
        print_bitboard(actual);

        assert_eq!(0x100, actual);
    }

    #[test]
    fn northwest() {
        let input: BitBoard = 0xe00;
        print_bitboard(input);

        let actual = bits::northwest(input);
        print_bitboard(actual);

        assert_eq!(0x70000, actual);
    }

    #[test]
    fn northeast() {
        let input: BitBoard = 0x1;
        print_bitboard(input);

        let actual = bits::northeast(input);
        print_bitboard(actual);

        assert_eq!(0x200, actual);
    }

    #[test]
    fn south() {
        let input: BitBoard = 0x40000;
        print_bitboard(input);

        let actual = bits::south(input);
        print_bitboard(actual);

        assert_eq!(0x400, actual);
    }

    #[test]
    fn southwest() {
        let input: BitBoard = 0x40000;
        print_bitboard(input);

        let actual = bits::southwest(input);
        print_bitboard(actual);

        assert_eq!(0x200, actual);
    }

    #[test]
    fn southeast() {
        let input: BitBoard = 0x40000;
        print_bitboard(input);

        let actual = bits::southeast(input);
        print_bitboard(actual);

        assert_eq!(0x800, actual);
    }

    #[test]
    fn east() {
        let input: BitBoard = 0x40000;
        print_bitboard(input);

        let actual = bits::east(input);
        print_bitboard(actual);

        assert_eq!(0x80000, actual);
    }

    #[test]
    fn west() {
        let input: BitBoard = 0x40000;
        print_bitboard(input);

        let actual = bits::west(input);
        print_bitboard(actual);

        assert_eq!(0x20000, actual);
    }
}
