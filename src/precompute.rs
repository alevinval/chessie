use crate::{
    bits::Bits,
    board::Board,
    defs::{BitBoard, Sq},
    magic::Magic,
    pos::Pos,
};

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn king() -> [BitBoard; 64] {
    let mut board = Board::default();
    board.clear();

    let mut gen = [0; 64];
    for (sq, gen_bb) in gen.iter_mut().enumerate() {
        let from = sq as Sq;
        let bb = Pos::bb(from);
        let mut pattern = Bits::north(bb)
            | Bits::northwest(bb)
            | Bits::northeast(bb)
            | Bits::south(bb)
            | Bits::southwest(bb)
            | Bits::southeast(bb)
            | Bits::west(bb)
            | Bits::east(bb);

        if Pos::col(from) == 0 {
            pattern &= Magic::NOT_H_FILE;
        } else if Pos::col(from) == 7 {
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
#[allow(clippy::cast_possible_truncation)]
pub fn diag_slider() -> [BitBoard; 64] {
    let mut ans: [BitBoard; 64] = [0; 64];
    for (sq, bb) in ans.iter_mut().enumerate().rev() {
        let p = sq as Sq;
        let o = 1 << sq;

        let mut v = 0;
        for (s, _) in (Pos::col(p)..8).enumerate() {
            v |= o << (8 * s + s);
        }

        for (s, _) in (0..=Pos::col(p)).enumerate() {
            v |= o >> (8 * s + s);
        }
        *bb = v;
    }

    ans
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn antidiag_slider() -> [BitBoard; 64] {
    let mut ans: [BitBoard; 64] = [0; 64];
    for (sq, bb) in ans.iter_mut().enumerate().rev() {
        let p = sq as Sq;
        let o = 1 << sq;

        let mut v = 0;
        for (s, _) in (0..=Pos::col(p)).enumerate() {
            v |= o << (8 * s - s);
        }

        for (s, _) in (Pos::col(p)..8).enumerate() {
            v |= o >> (8 * s - s);
        }
        *bb = v;
    }

    ans
}
