use std::sync::LazyLock;

use crate::bits;
use crate::board::Board;
use crate::{color::Color, defs::Sq, piece::Piece};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

static TABLE: LazyLock<ZobristTable> = LazyLock::new(seed_table);

fn seed_table() -> ZobristTable {
    let mut rng = StdRng::seed_from_u64(0);
    let mut piece_sq_keys = [0u64; ZobristTable::COUNT];
    for color in 0..ZobristTable::COLORS {
        for piece in 0..ZobristTable::PIECE_TYPES {
            for sq in 0..ZobristTable::SQUARES {
                piece_sq_keys[ZobristTable::piece_idx_usize(color, piece, sq)] = rng.next_u64();
            }
        }
    }
    let side_to_move = rng.next_u64();
    let mut castling_keys = [0u64; ZobristTable::CASTLING];
    for key in castling_keys.iter_mut() {
        *key = rng.next_u64();
    }
    ZobristTable { piece_sq_keys, side_to_move, castling_keys }
}

/// The precomputed random keys used to build position hashes.
///
/// Generated once from a fixed seed (see `seed_table`), so the values are
/// deterministic across runs.
struct ZobristTable {
    piece_sq_keys: [u64; Self::COUNT],
    castling_keys: [u64; Self::CASTLING],
    side_to_move: u64,
}

impl ZobristTable {
    const COLORS: usize = 2;
    const PIECE_TYPES: usize = 6;
    const SQUARES: usize = 64;
    const COUNT: usize = Self::COLORS * Self::PIECE_TYPES * Self::SQUARES;
    const CASTLING: usize = Self::COLORS * 2;

    const fn piece_idx(color: Color, piece: Piece, sq: Sq) -> usize {
        Self::piece_idx_usize(Self::color_idx(color), piece.idx(), sq as usize)
    }

    const fn piece_idx_usize(color: usize, piece: usize, sq: usize) -> usize {
        (color * Self::PIECE_TYPES + piece) * Self::SQUARES + sq
    }

    const fn color_idx(color: Color) -> usize {
        match color {
            Color::W => 0,
            Color::B => 1,
        }
    }

    const fn castling(&self, color: Color, kingside: bool) -> u64 {
        self.castling_keys[Self::color_idx(color) * 2 + kingside as usize]
    }

    const fn piece(&self, color: Color, piece: Piece, sq: Sq) -> u64 {
        self.piece_sq_keys[Self::piece_idx(color, piece, sq)]
    }

    const fn side_to_move(&self) -> u64 {
        self.side_to_move
    }
}

/// The incremental Zobrist hash of a position.
///
/// Every mutation that changes the position (a move, or a side-to-move flip) must apply
/// the matching fold here. Because XOR is its own inverse, both applying or unapplying
/// a move use the same key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) struct ZobristHash(u64);

impl ZobristHash {
    #[must_use]
    pub(crate) const fn get(self) -> u64 {
        self.0
    }

    pub(crate) fn flip_piece(&mut self, color: Color, piece: Piece, sq: Sq) {
        self.0 ^= TABLE.piece(color, piece, sq);
    }

    pub(crate) fn flip_side(&mut self) {
        self.0 ^= TABLE.side_to_move();
    }

    pub(crate) fn flip_castling(&mut self, color: Color, kingside: bool) {
        self.0 ^= TABLE.castling(color, kingside);
    }
    #[must_use]
    pub(crate) fn from_board(board: &Board) -> Self {
        let mut hash = 0u64;
        for color in [Color::W, Color::B] {
            board.pieces(color).for_each(|(piece, bb)| {
                for sq in bits::pos(bb) {
                    hash ^= TABLE.piece(color, piece, sq);
                }
            });
        }
        let state = board.state();
        if state.mover() == Color::B {
            hash ^= TABLE.side_to_move();
        }
        for (color, (left, right)) in [
            (Color::W, state.castling_rights(Color::W)),
            (Color::B, state.castling_rights(Color::B)),
        ] {
            if left {
                hash ^= TABLE.castling(color, false);
            }
            if right {
                hash ^= TABLE.castling(color, true);
            }
        }
        Self(hash)
    }
}

#[cfg(test)]
mod test {

    use crate::board::Board;

    use super::*;

    #[test]
    fn starting_position_hash() {
        assert_eq!(632210712590947314, ZobristHash::from_board(&Board::default()).get());
    }

    #[test]
    fn side_to_move_changes_hash() {
        let mut board = Board::default();
        let white_to_move = ZobristHash::from_board(&board).get();

        board.set_mover(Color::B);
        let black_to_move = ZobristHash::from_board(&board).get();

        assert_ne!(white_to_move, black_to_move);
    }
}
