use crate::{
    color::Color,
    defs::{BitBoard, Sq},
    pos, sq,
};

pub(crate) use magic_movements::MagicMovements;

mod magic_movements;

pub struct Masks;
pub struct MagicCastling;

impl Masks {
    /// Squares
    pub const ALL: BitBoard = BitBoard::MAX;

    pub const A1: BitBoard = pos::bb(0);
    pub const B1: BitBoard = pos::bb(1);
    pub const C1: BitBoard = pos::bb(2);
    pub const D1: BitBoard = pos::bb(3);
    pub const E1: BitBoard = pos::bb(4);
    pub const F1: BitBoard = pos::bb(5);
    pub const G1: BitBoard = pos::bb(6);
    pub const H1: BitBoard = pos::bb(7);

    pub const A2: BitBoard = pos::bb(8);
    pub const B2: BitBoard = pos::bb(9);
    pub const C2: BitBoard = pos::bb(10);
    pub const D2: BitBoard = pos::bb(11);
    pub const E2: BitBoard = pos::bb(12);
    pub const F2: BitBoard = pos::bb(13);
    pub const G2: BitBoard = pos::bb(14);
    pub const H2: BitBoard = pos::bb(15);

    pub const A3: BitBoard = pos::bb(16);
    pub const B3: BitBoard = pos::bb(17);
    pub const C3: BitBoard = pos::bb(18);
    pub const D3: BitBoard = pos::bb(19);
    pub const E3: BitBoard = pos::bb(20);
    pub const F3: BitBoard = pos::bb(21);
    pub const G3: BitBoard = pos::bb(22);
    pub const H3: BitBoard = pos::bb(23);

    pub const A4: BitBoard = pos::bb(24);
    pub const B4: BitBoard = pos::bb(25);
    pub const C4: BitBoard = pos::bb(26);
    pub const D4: BitBoard = pos::bb(27);
    pub const E4: BitBoard = pos::bb(28);
    pub const F4: BitBoard = pos::bb(29);
    pub const G4: BitBoard = pos::bb(30);
    pub const H4: BitBoard = pos::bb(31);

    pub const A5: BitBoard = pos::bb(32);
    pub const B5: BitBoard = pos::bb(33);
    pub const C5: BitBoard = pos::bb(34);
    pub const D5: BitBoard = pos::bb(35);
    pub const E5: BitBoard = pos::bb(36);
    pub const F5: BitBoard = pos::bb(37);
    pub const G5: BitBoard = pos::bb(38);
    pub const H5: BitBoard = pos::bb(39);

    pub const A6: BitBoard = pos::bb(40);
    pub const B6: BitBoard = pos::bb(41);
    pub const C6: BitBoard = pos::bb(42);
    pub const D6: BitBoard = pos::bb(43);
    pub const E6: BitBoard = pos::bb(44);
    pub const F6: BitBoard = pos::bb(45);
    pub const G6: BitBoard = pos::bb(46);
    pub const H6: BitBoard = pos::bb(47);

    pub const A7: BitBoard = pos::bb(48);
    pub const B7: BitBoard = pos::bb(49);
    pub const C7: BitBoard = pos::bb(50);
    pub const D7: BitBoard = pos::bb(51);
    pub const E7: BitBoard = pos::bb(52);
    pub const F7: BitBoard = pos::bb(53);
    pub const G7: BitBoard = pos::bb(54);
    pub const H7: BitBoard = pos::bb(55);

    pub const A8: BitBoard = pos::bb(56);
    pub const B8: BitBoard = pos::bb(57);
    pub const C8: BitBoard = pos::bb(58);
    pub const D8: BitBoard = pos::bb(59);
    pub const E8: BitBoard = pos::bb(60);
    pub const F8: BitBoard = pos::bb(61);
    pub const G8: BitBoard = pos::bb(62);
    pub const H8: BitBoard = pos::bb(63);

    /// FILES
    pub const FILE_A: BitBoard =
        Self::A1 | Self::A2 | Self::A3 | Self::A4 | Self::A5 | Self::A6 | Self::A7 | Self::A8;
    pub const FILE_B: BitBoard = 2 * Self::FILE_A;
    pub const FILE_C: BitBoard = 2 * Self::FILE_B;
    pub const FILE_D: BitBoard = 2 * Self::FILE_C;
    pub const FILE_E: BitBoard = 2 * Self::FILE_D;
    pub const FILE_F: BitBoard = 2 * Self::FILE_E;
    pub const FILE_G: BitBoard = 2 * Self::FILE_F;
    pub const FILE_H: BitBoard = 2 * Self::FILE_G;

    pub const NOT_FILE_A: BitBoard = Self::ALL & !Self::FILE_A;
    pub const NOT_FILE_H: BitBoard = Self::ALL & !Self::FILE_H;

    /// RANKS
    pub const RANK_1: BitBoard = 0xff;
    pub const RANK_2: BitBoard = Self::RANK_1 << 8;
    pub const RANK_3: BitBoard = Self::RANK_2 << 8;
    pub const RANK_4: BitBoard = Self::RANK_3 << 8;
    pub const RANK_5: BitBoard = Self::RANK_4 << 8;
    pub const RANK_6: BitBoard = Self::RANK_5 << 8;
    pub const RANK_7: BitBoard = Self::RANK_6 << 8;
    pub const RANK_8: BitBoard = Self::RANK_7 << 8;
}

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

#[cfg(test)]
mod test {
    use crate::util::print_bitboard;

    use super::*;

    #[test]
    fn print_magic_masks() {
        print_bitboard(Masks::FILE_A);
        print_bitboard(Masks::FILE_B);
        print_bitboard(Masks::FILE_C);
        print_bitboard(Masks::FILE_D);
        print_bitboard(Masks::FILE_E);
        print_bitboard(Masks::FILE_F);
        print_bitboard(Masks::FILE_G);

        print_bitboard(Masks::NOT_FILE_A);
        print_bitboard(Masks::NOT_FILE_H);

        print_bitboard(Masks::RANK_1);
        print_bitboard(Masks::RANK_2);
        print_bitboard(Masks::RANK_3);
        print_bitboard(Masks::RANK_4);
        print_bitboard(Masks::RANK_5);
        print_bitboard(Masks::RANK_6);
        print_bitboard(Masks::RANK_7);
        print_bitboard(Masks::RANK_8);
    }

    #[test]
    fn print_magic_castling() {
        print_bitboard(MagicCastling::WHITE_LEFT_CASTLE);
        print_bitboard(MagicCastling::WHITE_RIGHT_CASTLE);
        print_bitboard(MagicCastling::BLACK_LEFT_CASTLE);
        print_bitboard(MagicCastling::BLACK_RIGHT_CASTLE);
    }
}
