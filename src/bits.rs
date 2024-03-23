use crate::{
    defs::{BitBoard, Sq},
    piece::Piece,
    pos::Pos,
};

use super::Color;

pub struct Bits();

impl Bits {
    #[must_use]
    pub const fn init(piece: Piece, color: Color) -> BitBoard {
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
    pub const fn count(mut bb: BitBoard) -> usize {
        let mut count = 0;
        while bb != 0 {
            count += 1;
            bb &= bb - 1;
        }
        count
    }

    #[must_use]
    pub fn has_piece(bb: BitBoard, pos: Pos) -> bool {
        bb & pos.bb() != 0
    }

    pub fn slide(bb: &mut BitBoard, from: Pos, to: Pos) {
        *bb ^= from.bb() | to.bb();
    }

    pub fn set(bb: &mut BitBoard, pos: Pos) {
        *bb |= pos.bb();
    }

    pub fn unset(bb: &mut BitBoard, pos: Pos) {
        *bb &= !pos.bb();
    }

    #[must_use]
    pub fn pos(mut bb: BitBoard) -> Vec<Pos> {
        let mut acc: Vec<Pos> = vec![];
        let mut square: Sq = 0;
        while bb > 0 {
            let mut shift = bb.trailing_zeros() as u8;
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
    pub fn first_pos(bb: BitBoard) -> Option<Pos> {
        if bb == 0 {
            return None;
        }
        Some((bb.trailing_zeros() as Sq).into())
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
    fn pos() {
        let sut = 0x800c00000a007000;
        let expected: Vec<_> =
            vec![12, 13, 14, 25, 27, 50, 51, 63].into_iter().map(Pos::from).collect();
        assert_eq!(expected, Bits::pos(sut));
    }

    #[test]
    fn first_pos() {
        let sut = 0x800c00000a007000;
        assert_eq!(Some(12.into()), Bits::first_pos(sut));
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
}
