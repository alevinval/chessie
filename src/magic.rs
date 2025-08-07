use crate::{
    color::Color,
    defs::{BitBoard, Sq},
    sq,
};

pub(crate) use magic_movements::MagicMovements;

mod magic_movements;

pub struct MagicMask;
pub struct MagicCastling;

impl MagicCastling {
    const WHITE_LEFT_CASTLE: BitBoard = 0xf;
    const WHITE_RIGHT_CASTLE: BitBoard = 0xe0;
    const BLACK_LEFT_CASTLE: BitBoard = 0xf00000000000000;
    const BLACK_RIGHT_CASTLE: BitBoard = 0xe000000000000000;

    pub(crate) const fn left(color: Color) -> BitBoard {
        match color {
            Color::B => Self::BLACK_LEFT_CASTLE,
            Color::W => Self::WHITE_LEFT_CASTLE,
        }
    }

    pub(crate) const fn right(color: Color) -> BitBoard {
        match color {
            Color::B => Self::BLACK_RIGHT_CASTLE,
            Color::W => Self::WHITE_RIGHT_CASTLE,
        }
    }

    pub(crate) const fn left_xray(color: Color) -> Sq {
        match color {
            Color::B => sq!(7, 3),
            Color::W => sq!(0, 3),
        }
    }

    pub(crate) const fn right_xray(color: Color) -> Sq {
        match color {
            Color::B => sq!(7, 5),
            Color::W => sq!(0, 5),
        }
    }

    pub(crate) const fn left_rook(color: Color) -> Sq {
        match color {
            Color::B => sq!(7, 0),
            Color::W => sq!(0, 0),
        }
    }

    pub(crate) const fn right_rook(color: Color) -> Sq {
        match color {
            Color::B => sq!(7, 7),
            Color::W => sq!(0, 7),
        }
    }
}

#[allow(dead_code)]
#[allow(clippy::unreadable_literal)]
impl MagicMask {
    pub(crate) const ALL: BitBoard = BitBoard::MAX;

    pub const NOT_A_FILE: BitBoard = Self::ALL & !Self::FILE_A;
    pub const NOT_H_FILE: BitBoard = Self::ALL & !Self::FILE_H;

    pub(crate) const FILE_A: BitBoard = 0x101010101010101;
    pub(crate) const FILE_B: BitBoard = 2 * Self::FILE_A;
    pub(crate) const FILE_C: BitBoard = 2 * Self::FILE_B;
    pub(crate) const FILE_D: BitBoard = 2 * Self::FILE_C;
    pub(crate) const FILE_E: BitBoard = 2 * Self::FILE_D;
    pub(crate) const FILE_F: BitBoard = 2 * Self::FILE_E;
    pub(crate) const FILE_G: BitBoard = 2 * Self::FILE_F;
    pub(crate) const FILE_H: BitBoard = 2 * Self::FILE_G;

    pub(crate) const RANK_1: BitBoard = 0xff;
    pub(crate) const RANK_2: BitBoard = Self::RANK_1 << 8;
    pub(crate) const RANK_3: BitBoard = Self::RANK_2 << 8;
    pub(crate) const RANK_4: BitBoard = Self::RANK_3 << 8;
    pub(crate) const RANK_5: BitBoard = Self::RANK_4 << 8;
    pub(crate) const RANK_6: BitBoard = Self::RANK_5 << 8;
    pub(crate) const RANK_7: BitBoard = Self::RANK_6 << 8;
    pub(crate) const RANK_8: BitBoard = Self::RANK_7 << 8;

    pub(crate) const A1: BitBoard = 0x1;
    pub(crate) const H1: BitBoard = 0x80;
    pub(crate) const A8: BitBoard = 0x100000000000000;
    pub(crate) const H8: BitBoard = 0x8000000000000000;
}

#[cfg(test)]
mod test {
    use crate::util::print_bitboard;

    use super::*;

    #[test]
    fn print_magic_masks() {
        print_bitboard(MagicMask::NOT_A_FILE);
        print_bitboard(MagicMask::NOT_H_FILE);

        print_bitboard(MagicMask::FILE_A);
        print_bitboard(MagicMask::FILE_B);
        print_bitboard(MagicMask::FILE_C);
        print_bitboard(MagicMask::FILE_D);
        print_bitboard(MagicMask::FILE_E);
        print_bitboard(MagicMask::FILE_F);
        print_bitboard(MagicMask::FILE_G);

        print_bitboard(MagicMask::RANK_1);
        print_bitboard(MagicMask::RANK_2);
        print_bitboard(MagicMask::RANK_3);
        print_bitboard(MagicMask::RANK_4);
        print_bitboard(MagicMask::RANK_5);
        print_bitboard(MagicMask::RANK_6);
        print_bitboard(MagicMask::RANK_7);
        print_bitboard(MagicMask::RANK_8);

        print_bitboard(MagicMask::A1);
        print_bitboard(MagicMask::A8);
        print_bitboard(MagicMask::H1);
        print_bitboard(MagicMask::H8);
    }

    #[test]
    fn print_magic_castling() {
        print_bitboard(MagicCastling::WHITE_LEFT_CASTLE);
        print_bitboard(MagicCastling::WHITE_RIGHT_CASTLE);
        print_bitboard(MagicCastling::BLACK_LEFT_CASTLE);
        print_bitboard(MagicCastling::BLACK_RIGHT_CASTLE);
    }
}
