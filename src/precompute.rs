use crate::{bits::Bits, board::Board, defs::BitBoard, magic::Magic, pos::Pos};

#[must_use]
pub fn king() -> [BitBoard; 64] {
    let mut board = Board::default();
    board.clear();

    let mut gen = [0; 64];
    for (sq, gen_bb) in gen.iter_mut().enumerate() {
        let from = Pos::from(sq as u8);

        let bb = from.bb();
        let mut pattern = Bits::north(bb)
            | Bits::northwest(bb)
            | Bits::northeast(bb)
            | Bits::south(bb)
            | Bits::southwest(bb)
            | Bits::southeast(bb)
            | Bits::west(bb)
            | Bits::east(bb);

        if from.col() == 0 {
            pattern &= Magic::NOT_H_FILE;
        } else if from.col() == 7 {
            pattern &= Magic::NOT_A_FILE;
        }

        *gen_bb = pattern;
    }

    gen
}

#[must_use]
pub fn row_slider() -> [BitBoard; 8] {
    let mut ans: [BitBoard; 8] = [0; 8];
    for (sq, bb) in ans.iter_mut().enumerate() {
        let mut v = 1 << (sq * 8);
        for _ in 0..7 {
            v |= v << 1;
        }
        *bb = v;
    }

    ans
}

#[must_use]
pub fn col_slider() -> [BitBoard; 8] {
    let mut ans: [BitBoard; 8] = [0; 8];
    for (sq, bb) in ans.iter_mut().enumerate() {
        let mut v = 1 << sq;
        for _ in 0..7 {
            v |= v << 8;
        }
        *bb = v;
    }

    ans
}

#[must_use]
pub fn diag_slider() -> [BitBoard; 64] {
    let mut ans: [BitBoard; 64] = [0; 64];
    for (sq, bb) in ans.iter_mut().enumerate().rev() {
        let p = Pos::from(sq as u8);
        let o = 1 << sq;

        let mut v = 0;
        for (s, _) in (p.col()..8).enumerate() {
            v |= o << (8 * s + s);
        }

        for (s, _) in (0..=p.col()).enumerate() {
            v |= o >> (8 * s + s);
        }
        *bb = v;
    }

    ans
}

#[must_use]
pub fn antidiag_slider() -> [BitBoard; 64] {
    let mut ans: [BitBoard; 64] = [0; 64];
    for (sq, bb) in ans.iter_mut().enumerate().rev() {
        let p = Pos::from(sq as u8);
        let o = 1 << sq;

        let mut v = 0;
        for (s, _) in (0..=p.col()).enumerate() {
            v |= o << (8 * s - s);
        }

        for (s, _) in (p.col()..8).enumerate() {
            v |= o >> (8 * s - s);
        }
        *bb = v;
    }

    ans
}
