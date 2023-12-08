use board::Board;
use pieces::BitBoard;

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

    let positions = [test_pos];
    // board.set(positions[1], Piece::Queen(Color::Black));
    // board.set(positions[2], Piece::Queen(Color::White));

    let moves: Vec<BitBoard> = positions.iter().map(|p| board.generate_moves(*p)).collect();
    print_board(&board, &moves);
}
