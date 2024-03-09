use crate::{piece::Piece, pos::Pos};

use super::Color;

pub type BitBoard = u64;

pub struct Bits();

impl Bits {
    pub const fn init(piece: Piece, color: Color) -> BitBoard {
        let mut value = match piece {
            Piece::Pawn => 0b1111_1111,
            Piece::Rook => 0b1000_0001,
            Piece::Knight => 0b0100_0010,
            Piece::Bishop => 0b0010_0100,
            Piece::Queen => 0b0000_1000,
            Piece::King => 0b000_10000,
        };
        value <<= 8 * if matches!(piece, Piece::Pawn) {
            color.pawn_row()
        } else {
            color.piece_row()
        };

        value
    }

    pub fn count(mut bb: BitBoard) -> usize {
        let mut count = 0;
        while bb != 0 {
            count += 1;
            bb &= bb - 1;
        }
        count
    }

    pub fn has_piece<P: Into<u64>>(bb: BitBoard, pos: P) -> bool {
        bb & pos.into() != 0
    }

    pub fn slide<P: Into<u64>>(bb: &mut BitBoard, from: P, to: P) {
        *bb ^= from.into() | to.into();
    }

    pub fn set<P: Into<u64>>(bb: &mut BitBoard, pos: P) {
        *bb |= pos.into();
    }

    pub fn unset<P: Into<u64>>(bb: &mut BitBoard, pos: P) {
        *bb &= !pos.into();
    }

    pub fn pos(mut bb: BitBoard) -> Vec<Pos> {
        let mut pos: Vec<_> = vec![];
        let mut curr_square = 0;
        while bb > 0 {
            let trailing = bb.trailing_zeros() as u8;
            if trailing == 0 {
                pos.push(Pos::from_sq(curr_square));
                curr_square += 1;
                bb >>= 1;
                continue;
            }
            bb >>= trailing;
            curr_square += trailing;
        }
        pos
    }
}

impl From<Pos> for u64 {
    fn from(pos: Pos) -> Self {
        1 << (pos.row() * 8 + pos.col())
    }
}

#[cfg(test)]
mod test {

    use std::mem;

    use super::*;

    static ORIGIN: Pos = Pos::new(0, 0);
    static TARGET: Pos = Pos::new(3, 3);

    #[test]
    fn has_piece() {
        assert!(
            !Bits::has_piece(0, ORIGIN),
            "{ORIGIN:?} should not have piece"
        );
        assert_eq!(0, Bits::count(0));

        assert!(Bits::has_piece(1, ORIGIN), "{ORIGIN:?} should have piece");
        assert_eq!(1, Bits::count(1));

        assert!(
            !Bits::has_piece(0, TARGET),
            "{TARGET:?} should not have piece"
        );

        assert!(
            Bits::has_piece(TARGET.into(), TARGET),
            "{TARGET:?} should have piece"
        );
    }

    #[test]
    fn mov() {
        let from: Pos = (1, 1).into();
        let to: Pos = (2, 2).into();

        let mut sut = Bits::init(Piece::Pawn, Color::W);
        assert!(!Bits::has_piece(sut, to));

        Bits::slide(&mut sut, from, to);
        assert!(Bits::has_piece(sut, to));

        assert!(!Bits::has_piece(sut, from));
    }

    #[test]
    fn unset() {
        let pos: Pos = (1, 1).into();
        let mut sut = Bits::init(Piece::Pawn, Color::W);
        assert!(Bits::has_piece(sut, pos));

        Bits::unset(&mut sut, pos);
        assert!(!Bits::has_piece(sut, pos));
    }

    #[test]
    fn size() {
        assert_eq!(8, mem::size_of::<BitBoard>());
        assert_eq!(8, mem::size_of::<&BitBoard>());
    }
}
