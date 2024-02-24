use board::Board;
use pieces::{BitBoard, Color};

use crate::pos::Pos;

mod board;
mod pieces;
mod pos;

fn print_board(board: &Board, highlights: &[BitBoard]) {
    for row in (0..8).rev() {
        println!("+---+---+---+---+---+---+---+---+");
        for col in 0..8 {
            let pos = Pos(row, col);
            let mark = highlights.iter().find(|h| h.has_piece(pos)).map(|_| "@");
            let piece = board.at(pos).map(|set| set.piece.as_str()).unwrap_or(" ");
            print!("| {} ", mark.unwrap_or(piece));
        }
        println!("| {}", row + 1);
    }
    println!("+---+---+---+---+---+---+---+---+");
    println!("  a   b   c   d   e   f   g   h  ");
}

pub fn main() {
    let mut board = Board::new();
    let test_pos = Pos(4, 6);
    board.apply_move(Pos(0, 4), test_pos);
    board.save("board.cb");
    // board.clear();

    let positions = [test_pos]; //, Pos(0, 1), Pos(1, 3)];
                                // board.set(positions[1], Piece::Queen(Color::Black));
                                // board.set(positions[2], Piece::Queen(Color::White));

    print_board(&board, &[board.generate_moves(test_pos)]);
    for pos in positions {
        board
            .generate_moves(pos)
            .positions()
            .into_iter()
            .for_each(|gen_pos| {
                let mut new_board = board.clone();
                new_board.apply_move(pos, gen_pos);
                let eval = new_board.evaluate(Color::White);
                if eval > 0.0 {
                    print_board(&new_board, &[new_board.generate_moves(gen_pos)]);
                    println!("candidate={gen_pos:?} eval={eval}");
                }
            })
    }
}
