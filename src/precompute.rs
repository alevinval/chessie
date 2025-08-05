use crate::{
    bits,
    defs::{BitBoard, Sq},
    magic::MagicMask,
    pos,
};

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn king() -> [BitBoard; 64] {
    let mut gen = [0; 64];
    for (sq, gen_bb) in gen.iter_mut().enumerate() {
        let from = sq as Sq;
        let bb = pos::bb(from);
        let mut pattern = bits::north(bb)
            | bits::northwest(bb)
            | bits::northeast(bb)
            | bits::south(bb)
            | bits::southwest(bb)
            | bits::southeast(bb)
            | bits::west(bb)
            | bits::east(bb);

        if pos::col(from) == 0 {
            pattern &= MagicMask::NOT_H_FILE;
        } else if pos::col(from) == 7 {
            pattern &= MagicMask::NOT_A_FILE;
        }
        *gen_bb = pattern;
    }
    gen
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn knight() -> [BitBoard; 64] {
    let mut gen = [0; 64];
    for (sq, gen_bb) in gen.iter_mut().enumerate() {
        let from = sq as Sq;
        let bb = pos::bb(from);

        let mut pattern = bits::northwest(bits::north(bb))
            | bits::northwest(bits::west(bb))
            | bits::southwest(bits::west(bb))
            | bits::southwest(bits::south(bb))
            | bits::northeast(bits::north(bb))
            | bits::northeast(bits::east(bb))
            | bits::southeast(bits::east(bb))
            | bits::southeast(bits::south(bb));

        if pos::col(from) < 2 {
            pattern &= MagicMask::NOT_H_FILE & bits::west(MagicMask::NOT_H_FILE);
        } else if pos::col(from) > 5 {
            pattern &= MagicMask::NOT_A_FILE & bits::east(MagicMask::NOT_A_FILE);
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
        for (s, _) in (pos::col(p)..8).enumerate() {
            v |= o << (8 * s + s);
        }

        for (s, _) in (0..=pos::col(p)).enumerate() {
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
        for (s, _) in (0..=pos::col(p)).enumerate() {
            v |= o << (8 * s - s);
        }

        for (s, _) in (pos::col(p)..8).enumerate() {
            v |= o >> (8 * s - s);
        }
        *bb = v;
    }

    ans
}
