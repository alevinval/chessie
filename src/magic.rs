use crate::{color::Color, defs::BitBoard, pos::Pos};

pub struct Magic();
pub struct MagicCastling();

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

    pub(crate) const fn left_xray(color: Color) -> BitBoard {
        match color {
            Color::B => Pos::new(7, 3).bb(),
            Color::W => Pos::new(0, 3).bb(),
        }
    }

    pub(crate) const fn right_xray(color: Color) -> BitBoard {
        match color {
            Color::B => Pos::new(7, 5).bb(),
            Color::W => Pos::new(0, 5).bb(),
        }
    }
}

#[allow(dead_code)]
#[allow(clippy::unreadable_literal)]
impl Magic {
    pub(crate) const NOT_A_FILE: BitBoard = 0xfefefefefefefefe;
    pub(crate) const NOT_H_FILE: BitBoard = 0x7f7f7f7f7f7f7f7f;

    pub(crate) const RANK_3: BitBoard = 0xff0000;
    pub(crate) const RANK_6: BitBoard = 0xff0000000000;

    pub(crate) const A1: BitBoard = 0x1;
    pub(crate) const H1: BitBoard = 0x80;
    pub(crate) const A8: BitBoard = 0x100000000000000;
    pub(crate) const H8: BitBoard = 0x8000000000000000;

    pub(crate) const KNIGHT_MOVES: [BitBoard; 64] = [
        0x20400,
        0x50800,
        0xa1100,
        0x142200,
        0x284400,
        0x508800,
        0xa01000,
        0x402000,
        0x2040004,
        0x5080008,
        0xa110011,
        0x14220022,
        0x28440044,
        0x50880088,
        0xa0100010,
        0x40200020,
        0x204000400,
        0x508000800,
        0xa11001100,
        0x1422002200,
        0x2844004400,
        0x5088008800,
        0xa010001000,
        0x4020002000,
        0x20400040200,
        0x50800080500,
        0xa1100110a00,
        0x142200221400,
        0x284400442800,
        0x508800885000,
        0xa0100010a000,
        0x402000204000,
        0x2040004020000,
        0x5080008050000,
        0xa1100110a0000,
        0x14220022140000,
        0x28440044280000,
        0x50880088500000,
        0xa0100010a00000,
        0x40200020400000,
        0x204000402000000,
        0x508000805000000,
        0xa1100110a000000,
        0x1422002214000000,
        0x2844004428000000,
        0x5088008850000000,
        0xa0100010a0000000,
        0x4020002040000000,
        0x400040200000000,
        0x800080500000000,
        0x1100110a00000000,
        0x2200221400000000,
        0x4400442800000000,
        0x8800885000000000,
        0x100010a000000000,
        0x2000204000000000,
        0x4020000000000,
        0x8050000000000,
        0x110a0000000000,
        0x22140000000000,
        0x44280000000000,
        0x88500000000000,
        0x10a00000000000,
        0x20400000000000,
    ];

    pub(crate) const KING_MOVES: [BitBoard; 64] = [
        0x302,
        0x705,
        0xe0a,
        0x1c14,
        0x3828,
        0x7050,
        0xe0a0,
        0xc040,
        0x30203,
        0x70507,
        0xe0a0e,
        0x1c141c,
        0x382838,
        0x705070,
        0xe0a0e0,
        0xc040c0,
        0x3020300,
        0x7050700,
        0xe0a0e00,
        0x1c141c00,
        0x38283800,
        0x70507000,
        0xe0a0e000,
        0xc040c000,
        0x302030000,
        0x705070000,
        0xe0a0e0000,
        0x1c141c0000,
        0x3828380000,
        0x7050700000,
        0xe0a0e00000,
        0xc040c00000,
        0x30203000000,
        0x70507000000,
        0xe0a0e000000,
        0x1c141c000000,
        0x382838000000,
        0x705070000000,
        0xe0a0e0000000,
        0xc040c0000000,
        0x3020300000000,
        0x7050700000000,
        0xe0a0e00000000,
        0x1c141c00000000,
        0x38283800000000,
        0x70507000000000,
        0xe0a0e000000000,
        0xc040c000000000,
        0x302030000000000,
        0x705070000000000,
        0xe0a0e0000000000,
        0x1c141c0000000000,
        0x3828380000000000,
        0x7050700000000000,
        0xe0a0e00000000000,
        0xc040c00000000000,
        0x203000000000000,
        0x507000000000000,
        0xa0e000000000000,
        0x141c000000000000,
        0x2838000000000000,
        0x5070000000000000,
        0xa0e0000000000000,
        0x40c0000000000000,
    ];

    pub(crate) const ROW_SLIDER: [BitBoard; 8] = [
        0xff,
        0xff00,
        0xff0000,
        0xff000000,
        0xff00000000,
        0xff0000000000,
        0xff000000000000,
        0xff00000000000000,
    ];

    pub(crate) const COL_SLIDER: [BitBoard; 8] = [
        0x101010101010101,
        0x202020202020202,
        0x404040404040404,
        0x808080808080808,
        0x1010101010101010,
        0x2020202020202020,
        0x4040404040404040,
        0x8080808080808080,
    ];

    pub(crate) const DIAG_SLIDER: [BitBoard; 64] = [
        0x8040201008040201,
        0x80402010080402,
        0x804020100804,
        0x8040201008,
        0x80402010,
        0x804020,
        0x8040,
        0x80,
        0x4020100804020100,
        0x8040201008040201,
        0x80402010080402,
        0x804020100804,
        0x8040201008,
        0x80402010,
        0x804020,
        0x8040,
        0x2010080402010000,
        0x4020100804020100,
        0x8040201008040201,
        0x80402010080402,
        0x804020100804,
        0x8040201008,
        0x80402010,
        0x804020,
        0x1008040201000000,
        0x2010080402010000,
        0x4020100804020100,
        0x8040201008040201,
        0x80402010080402,
        0x804020100804,
        0x8040201008,
        0x80402010,
        0x804020100000000,
        0x1008040201000000,
        0x2010080402010000,
        0x4020100804020100,
        0x8040201008040201,
        0x80402010080402,
        0x804020100804,
        0x8040201008,
        0x402010000000000,
        0x804020100000000,
        0x1008040201000000,
        0x2010080402010000,
        0x4020100804020100,
        0x8040201008040201,
        0x80402010080402,
        0x804020100804,
        0x201000000000000,
        0x402010000000000,
        0x804020100000000,
        0x1008040201000000,
        0x2010080402010000,
        0x4020100804020100,
        0x8040201008040201,
        0x80402010080402,
        0x100000000000000,
        0x201000000000000,
        0x402010000000000,
        0x804020100000000,
        0x1008040201000000,
        0x2010080402010000,
        0x4020100804020100,
        0x8040201008040201,
    ];

    pub(crate) const ANTIDIAG_SLIDER: [BitBoard; 64] = [
        0x1,
        0x102,
        0x10204,
        0x1020408,
        0x102040810,
        0x10204081020,
        0x1020408102040,
        0x102040810204080,
        0x102,
        0x10204,
        0x1020408,
        0x102040810,
        0x10204081020,
        0x1020408102040,
        0x102040810204080,
        0x204081020408000,
        0x10204,
        0x1020408,
        0x102040810,
        0x10204081020,
        0x1020408102040,
        0x102040810204080,
        0x204081020408000,
        0x408102040800000,
        0x1020408,
        0x102040810,
        0x10204081020,
        0x1020408102040,
        0x102040810204080,
        0x204081020408000,
        0x408102040800000,
        0x810204080000000,
        0x102040810,
        0x10204081020,
        0x1020408102040,
        0x102040810204080,
        0x204081020408000,
        0x408102040800000,
        0x810204080000000,
        0x1020408000000000,
        0x10204081020,
        0x1020408102040,
        0x102040810204080,
        0x204081020408000,
        0x408102040800000,
        0x810204080000000,
        0x1020408000000000,
        0x2040800000000000,
        0x1020408102040,
        0x102040810204080,
        0x204081020408000,
        0x408102040800000,
        0x810204080000000,
        0x1020408000000000,
        0x2040800000000000,
        0x4080000000000000,
        0x102040810204080,
        0x204081020408000,
        0x408102040800000,
        0x810204080000000,
        0x1020408000000000,
        0x2040800000000000,
        0x4080000000000000,
        0x8000000000000000,
    ];
}

#[cfg(test)]
mod test {
    use crate::util::print_bitboard;

    use super::*;

    #[test]
    fn print_magics() {
        print_bitboard(Magic::NOT_A_FILE);
        print_bitboard(Magic::NOT_H_FILE);

        print_bitboard(Magic::RANK_3);
        print_bitboard(Magic::RANK_6);

        print_bitboard(Magic::A1);
        print_bitboard(Magic::A8);
        print_bitboard(Magic::H1);
        print_bitboard(Magic::H8);
    }

    #[test]
    fn print_castle_magic() {
        print_bitboard(MagicCastling::WHITE_LEFT_CASTLE);
        print_bitboard(MagicCastling::WHITE_RIGHT_CASTLE);
        print_bitboard(MagicCastling::BLACK_LEFT_CASTLE);
        print_bitboard(MagicCastling::BLACK_RIGHT_CASTLE);
    }
}
