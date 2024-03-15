use crate::defs::BitBoard;
use crate::{
    bitboard::Bits,
    board::Board,
    moves::{generator::Generator, knight},
    piece::Piece,
    Color,
};

pub fn precompute_knights() -> [BitBoard; 64] {
    let mut board = Board::default();
    board.clear();

    let mut gen = [0; 64];
    for (sq, bb) in gen.iter_mut().enumerate() {
        let mut g = Generator::new(&board, sq, Color::W, Piece::Knight, false);
        knight(&mut g);
        let moves = g.moves();

        moves
            .iter()
            .map(|m| m.to())
            .for_each(|dst| Bits::set(bb, dst));
    }

    gen
}

#[cfg(test)]
mod test {

    use crate::{moves::MoveGen, print_bitboard, print_board, Pos};

    use super::*;

    #[test]
    fn test_magic_knight() {
        let pregen = precompute_knights();
        println!("const KNIGHT_MAGIC: [BitBoard; 64] = [");
        for g in pregen {
            println!("  0x{:x},", g);
        }
        println!("]");

        print_bitboard(pregen[Pos::from((0, 6)).sq()]);

        println!("pos={:?}", Bits::pos(pregen[Pos::from((0, 6)).sq()]));

        let mut board = Board::default();
        Bits::set(&mut board.black[0], Pos::new(2, 5));

        let m = MoveGen::new(&board, (0, 6)).generate(true);

        let t: Vec<_> = m.iter().map(|m| m.to()).collect();
        print_board(&board, &t);

        // assert!(false);
    }

    #[test]
    fn test_pawns() {}
}
