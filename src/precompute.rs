use crate::{
    bits,
    defs::{BitBoard, Sq},
    magic::MagicMask,
    pos,
};

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn king() -> [BitBoard; 64] {
    let mut sq_to_moves = [0; 64];
    for (sq, moves) in sq_to_moves.iter_mut().enumerate() {
        let from = sq as Sq;
        let seed = pos::bb(from);
        let mut pattern = bits::north(seed)
            | bits::northwest(seed)
            | bits::northeast(seed)
            | bits::south(seed)
            | bits::southwest(seed)
            | bits::southeast(seed)
            | bits::west(seed)
            | bits::east(seed);

        if pos::col(from) == 0 {
            pattern &= MagicMask::NOT_H_FILE;
        } else if pos::col(from) == 7 {
            pattern &= MagicMask::NOT_A_FILE;
        }
        *moves = pattern;
    }
    sq_to_moves
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn knight() -> [BitBoard; 64] {
    let mut sq_to_moves = [0; 64];
    for (sq, moves) in sq_to_moves.iter_mut().enumerate() {
        let from = sq as Sq;
        let seed = pos::bb(from);

        let mut pattern = bits::northwest(bits::north(seed))
            | bits::northwest(bits::west(seed))
            | bits::southwest(bits::west(seed))
            | bits::southwest(bits::south(seed))
            | bits::northeast(bits::north(seed))
            | bits::northeast(bits::east(seed))
            | bits::southeast(bits::east(seed))
            | bits::southeast(bits::south(seed));

        if pos::col(from) < 2 {
            pattern &= MagicMask::NOT_H_FILE & bits::west(MagicMask::NOT_H_FILE);
        } else if pos::col(from) > 5 {
            pattern &= MagicMask::NOT_A_FILE & bits::east(MagicMask::NOT_A_FILE);
        }

        *moves = pattern;
    }
    sq_to_moves
}

#[must_use]
pub fn row_slider() -> [BitBoard; 8] {
    let mut sq_to_moves: [BitBoard; 8] = [0; 8];
    for (sq, moves) in sq_to_moves.iter_mut().enumerate() {
        let mut v = 1 << (sq * 8);
        for _ in 0..7 {
            v |= v << 1;
        }
        *moves = v;
    }

    sq_to_moves
}

#[must_use]
pub fn col_slider() -> [BitBoard; 8] {
    let mut sq_to_moves: [BitBoard; 8] = [0; 8];
    for (sq, moves) in sq_to_moves.iter_mut().enumerate() {
        let mut v = 1 << sq;
        for _ in 0..7 {
            v |= v << 8;
        }
        *moves = v;
    }

    sq_to_moves
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn diag_slider() -> [BitBoard; 64] {
    let mut sq_to_moves: [BitBoard; 64] = [0; 64];
    for (sq, moves) in sq_to_moves.iter_mut().enumerate().rev() {
        let p = sq as Sq;
        let o = 1 << sq;

        let mut v = 0;
        for (s, _) in (pos::col(p)..8).enumerate() {
            v |= o << (8 * s + s);
        }

        for (s, _) in (0..=pos::col(p)).enumerate() {
            v |= o >> (8 * s + s);
        }
        *moves = v;
    }

    sq_to_moves
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn antidiag_slider() -> [BitBoard; 64] {
    let mut sq_to_moves: [BitBoard; 64] = [0; 64];
    for (sq, moves) in sq_to_moves.iter_mut().enumerate().rev() {
        let p = sq as Sq;
        let o = 1 << sq;

        let mut v = 0;
        for (s, _) in (0..=pos::col(p)).enumerate() {
            v |= o << (8 * s - s);
        }

        for (s, _) in (pos::col(p)..8).enumerate() {
            v |= o >> (8 * s - s);
        }
        *moves = v;
    }

    sq_to_moves
}
