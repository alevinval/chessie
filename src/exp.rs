use crate::defs::BitBoard;
use crate::magic::Magic;

use crate::Pos;
use crate::{bitboard::Bits, board::Board, moves::generator::Generator, piece::Piece, Color};

pub fn precompute_king() -> [BitBoard; 64] {
    let mut board = Board::default();
    board.clear();

    let mut gen = [0; 64];
    for (sq, gen_bb) in gen.iter_mut().enumerate() {
        let from: Pos = sq.into();
        let mut g = Generator::new(&board, from, Color::W, Piece::King, false);

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

        g.slides_from_magic(pattern);

        let moves = g.moves();
        moves.iter().map(|m| m.to()).for_each(|dst| Bits::set(gen_bb, dst));
    }

    gen
}

#[cfg(test)]
mod test {

    use crate::print_bitboard;

    use super::*;

    #[test]
    fn test_precompute_king() {
        let pregen = precompute_king();

        for g in pregen {
            print_bitboard(g);
        }

        println!("const KING_MAGIC: [BitBoard; 64] = [");
        for g in pregen {
            println!("  0x{:x},", g);
        }
        println!("]");
    }
}
