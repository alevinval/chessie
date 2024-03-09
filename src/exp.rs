use crate::{
    bitboard::{BitBoard, Bits},
    board::Board,
    moves::{generator::Generator, knight},
    Pos,
};

pub fn precompute_knights() -> [BitBoard; 64] {
    let mut gen = [0; 64];

    let mut board = Board::default();
    board.clear();

    for sq in 0..64 {
        let p = Pos::from_sq(sq);
        let mut g = Generator::new(&board, p, false);
        knight(&mut g);
        let moves = g.moves();

        let mut bb: BitBoard = 0;
        moves
            .iter()
            .map(|m| m.to())
            .for_each(|dst| Bits::set(&mut bb, dst));

        gen[sq as usize] = bb;
    }

    gen
}

#[cfg(test)]
mod test {

    use crate::{moves::MoveGen, print_bitboard, print_board};

    use super::*;

    #[test]
    fn test_magic_knight() {
        let pregen = precompute_knights();
        println!("const KNIGHT_MAGIC: [BitBoard; 64] = [");
        for g in pregen {
            println!("  {:?},", g);
        }
        println!("]");

        print_bitboard(pregen[Pos::new(0, 6).sq()]);

        println!("pos={:?}", Bits::pos(pregen[Pos::new(0, 6).sq()]));

        let mut board = Board::default();
        Bits::set(&mut board.black[0], Pos::new(2, 5));

        let m = MoveGen::new(&board, Pos::new(0, 6)).generate(true);

        let t: Vec<_> = m.iter().map(|m| m.to()).collect();
        print_board(&board, &t);

        // assert!(false);
    }

    #[test]
    fn test_pawns() {}
}
