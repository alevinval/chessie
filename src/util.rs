use crate::{
    bits::Bits,
    board::Board,
    defs::BitBoard,
    eval::{legacy::LegacyScorer, Scorer},
    fen,
    pos::Pos,
};

#[allow(dead_code)]
pub(crate) fn print_bitboard(bb: BitBoard) {
    println!("[bitboard=0x{bb:x}]");
    for row in (0..8).rev() {
        println!("+---+---+---+---+---+---+---+---+");
        for col in 0..8 {
            let pos = Pos::new(row, col);
            let piece = if Bits::has_piece(bb, pos) { "@" } else { " " };
            print!("| {piece} ");
        }
        println!("| {row}");
    }
    println!("+---+---+---+---+---+---+---+---+");
    println!("  0   1   2   3   4   5   6   7  ");
}

pub(crate) fn print_board(board: &Board, highlights: &[Pos]) {
    let state = board.state();
    let eval = Scorer::eval(board, state.mover());
    let legacy_eval = LegacyScorer::eval(board, state.mover());
    println!(
        "[move={} mover={} highlights={highlights:?} eval={eval:.1} legacy_eval={legacy_eval:.1}]",
        state.n(),
        state.mover()
    );
    println!("[{}]", fen::encode(board));
    for row in (0..8).rev() {
        println!("+---+---+---+---+---+---+---+---+");
        for col in 0..8 {
            let pos: Pos = (row, col).into();
            let mark = highlights.iter().find(|p| **p == pos).map(|_| "•");
            let piece = board.at(pos).map_or(" ", |(color, piece, _)| piece.as_str(color));
            print!("| {} ", mark.unwrap_or(piece));
        }
        println!("| {row}");
    }
    println!("+---+---+---+---+---+---+---+---+");
    println!("  0   1   2   3   4   5   6   7  ");
}
