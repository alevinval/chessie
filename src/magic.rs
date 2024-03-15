use crate::defs::BitBoard;

pub struct Magic();

#[allow(clippy::unreadable_literal)]
impl Magic {
    pub const NOT_A_FILE: u64 = 0xfefefefefefefefe;
    pub const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

    pub const RANK_3: u64 = 0xff0000;
    pub const RANK_6: u64 = 0xff0000000000;
}

#[allow(clippy::unreadable_literal)]
pub const KNIGHT_MAGIC: [BitBoard; 64] = [
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

#[cfg(test)]
mod test {
    use super::*;

    use crate::print_bitboard;

    #[test]
    fn print_magics() {
        print_bitboard(Magic::NOT_A_FILE);
        print_bitboard(Magic::NOT_H_FILE);
        print_bitboard(Magic::RANK_3);
        print_bitboard(Magic::RANK_6);
    }
}
