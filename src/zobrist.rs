use std::collections::HashMap;

use crate::bits;
use crate::board::Board;
use crate::{color::Color, piece::Piece};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

pub type ZobristTable = HashMap<(Color, usize, u64), u64>;

pub fn get_table() -> ZobristTable {
    let mut rng = StdRng::seed_from_u64(0);
    let mut table = ZobristTable::new();
    for color in [Color::W, Color::B] {
        for piece in [Piece::P, Piece::N, Piece::B, Piece::R, Piece::Q, Piece::K] {
            for sq in 0..64 {
                table.insert((color, piece, sq), rng.next_u64());
            }
        }
    }
    table
}

pub fn hash_board(board: &Board, table: &ZobristTable) -> u64 {
    let mut hash = 0;
    board.pieces(Color::W).for_each(|(piece, bb)| {
        for sq in bits::pos(bb) {
            hash ^= table.get(&(Color::W, piece.idx(), sq as u64)).expect("");
        }
    });
    board.pieces(Color::B).for_each(|(piece, bb)| {
        for sq in bits::pos(bb) {
            hash ^= table.get(&(Color::B, piece.idx(), sq as u64)).expect("")
        }
    });
    hash
}

#[cfg(test)]
mod test {

    use crate::board::Board;

    use super::*;

    #[test]
    fn zobrist_table() {
        let table = get_table();

        println!("Zobrist table size: {:?}", table);
        assert_eq!(11513799526627155764, hash_board(&Board::default(), &table));
    }
}
