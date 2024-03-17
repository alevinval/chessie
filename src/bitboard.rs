use crate::{
    defs::{BitBoard, Sq},
    piece::Piece,
    Pos,
};

use super::Color;

pub struct Bits();

impl Bits {
    #[must_use]
    pub const fn init(piece: Piece, color: Color) -> BitBoard {
        let mut value = match piece {
            Piece::Pawn => 0b1111_1111,
            Piece::Rook => 0b1000_0001,
            Piece::Knight => 0b0100_0010,
            Piece::Bishop => 0b0010_0100,
            Piece::Queen => 0b0000_1000,
            Piece::King => 0b000_10000,
        };
        value <<=
            8 * if matches!(piece, Piece::Pawn) { color.pawn_row() } else { color.piece_row() };

        value
    }

    #[must_use]
    pub const fn count(mut bb: BitBoard) -> usize {
        let mut count = 0;
        while bb != 0 {
            count += 1;
            bb &= bb - 1;
        }
        count
    }

    #[must_use]
    pub fn has_piece<P: Into<BitBoard>>(bb: BitBoard, pos: P) -> bool {
        bb & pos.into() != 0
    }

    pub fn slide<P: Into<BitBoard>>(bb: &mut BitBoard, from: P, to: P) {
        *bb ^= from.into() | to.into();
    }

    pub fn set<P: Into<BitBoard>>(bb: &mut BitBoard, pos: P) {
        *bb |= pos.into();
    }

    pub fn unset<P: Into<BitBoard>>(bb: &mut BitBoard, pos: P) {
        *bb &= !pos.into();
    }

    #[must_use]
    pub fn pos(mut bb: BitBoard) -> Vec<Pos> {
        let mut acc: Vec<Pos> = vec![];
        let mut square: Sq = 0;
        while bb > 0 {
            let mut shift = bb.trailing_zeros() as usize;
            if shift == 0 {
                acc.push(square.into());
                shift = 1;
            }
            bb >>= shift;
            square += shift;
        }
        acc
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
}

#[cfg(test)]
mod test {

    use crate::{print_bitboard, Pos};

    use super::*;

    static ORIGIN: Pos = Pos::new(0, 0);
    static TARGET: Pos = Pos::new(3, 3);

    #[test]
    fn count() {
        let sut = 0x800c00000a007000;
        print_bitboard(sut);
        assert_eq!(8, Bits::count(sut));
    }

    #[test]
    fn has_piece() {
        assert!(!Bits::has_piece(0, ORIGIN));
        assert!(Bits::has_piece(ORIGIN.bb(), ORIGIN));

        assert!(!Bits::has_piece(0, TARGET));
        assert!(Bits::has_piece(TARGET.bb(), TARGET));
    }

    #[test]
    fn slide() {
        let from = Pos::new(1, 1);
        let to = Pos::new(2, 2);

        let mut sut = Bits::init(Piece::Pawn, Color::W);
        assert!(!Bits::has_piece(sut, to));

        Bits::slide(&mut sut, from, to);
        assert!(Bits::has_piece(sut, to));

        assert!(!Bits::has_piece(sut, from));
    }

    #[test]
    fn unset() {
        let pos = Pos::new(1, 1);
        let mut sut = Bits::init(Piece::Pawn, Color::W);
        assert!(Bits::has_piece(sut, pos));

        Bits::unset(&mut sut, pos);
        assert!(!Bits::has_piece(sut, pos));
    }

    #[test]
    fn north() {
        let input: BitBoard = 0x1;
        print_bitboard(input);

        let actual = Bits::north(input);
        print_bitboard(actual);

        assert_eq!(0x100, actual);
    }

    #[test]
    fn northwest() {
        let input: BitBoard = 0xe00;
        print_bitboard(input);

        let actual = Bits::northwest(input);
        print_bitboard(actual);

        assert_eq!(0x70000, actual);
    }

    #[test]
    fn northeast() {
        let input: BitBoard = 0x1;
        print_bitboard(input);

        let actual = Bits::northeast(input);
        print_bitboard(actual);

        assert_eq!(0x200, actual);
    }

    #[test]
    fn south() {
        let input: BitBoard = 0x40000;
        print_bitboard(input);

        let actual = Bits::south(input);
        print_bitboard(actual);

        assert_eq!(0x400, actual);
    }

    #[test]
    fn southwest() {
        let input: BitBoard = 0x40000;
        print_bitboard(input);

        let actual = Bits::southwest(input);
        print_bitboard(actual);

        assert_eq!(0x200, actual);
    }

    #[test]
    fn southeast() {
        let input: BitBoard = 0x40000;
        print_bitboard(input);

        let actual = Bits::southeast(input);
        print_bitboard(actual);

        assert_eq!(0x800, actual);
    }

    #[test]
    fn east() {
        let input: BitBoard = 0x40000;
        print_bitboard(input);

        let actual = Bits::east(input);
        print_bitboard(actual);

        assert_eq!(0x80000, actual);
    }

    #[test]
    fn west() {
        let input: BitBoard = 0x40000;
        print_bitboard(input);

        let actual = Bits::west(input);
        print_bitboard(actual);

        assert_eq!(0x20000, actual);
    }
}
