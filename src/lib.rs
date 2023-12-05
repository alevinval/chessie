use board::Board;
use piece::{Color, Piece};

mod board;
mod piece;

fn get_moves(board: &Board, row: usize, col: usize) -> Vec<(usize, usize)> {
    let moved_piece = board.at(row, col);
    moved_piece
        .map_or(vec![], |piece| match piece {
            Piece::Pawn(c, _) => match c {
                Color::Black => vec![(row - 1, col), (row - 1, col - 1), (row - 1, col + 1)],
                Color::White => vec![(row + 1, col), (row + 1, col - 1), (row + 1, col + 1)],
            },
            Piece::Rook(c, _)
            | Piece::Knight(c, _)
            | Piece::Bishop(c, _)
            | Piece::Queen(c, _)
            | Piece::King(c, _) => vec![],
        })
        .into_iter()
        .filter(|(row, col)| {
            board.at(*row, *col).map_or(
                if moved_piece.unwrap().is_pawn() {
                    false
                } else {
                    true
                },
                |p| p.color() != moved_piece.unwrap().color(),
            )
        })
        .collect()
}

fn print_board(board: &Board, highlights: Vec<(usize, usize)>) {
    for row in (0..8).rev() {
        print!("+---+---+---+---+---+---+---+---+\n");
        for col in 0..8 {
            print!(
                "| {} ",
                highlights
                    .iter()
                    .find(|pos| pos.0 == row && pos.1 == col)
                    .map_or(board.at(row, col).map_or(" ", |p| p.to_str()), |_| "@"),
            );
        }
        print!("| {}\n", row + 1);
    }
    print!("+---+---+---+---+---+---+---+---+\n");
    print!("  a   b   c   d   e   f   g   h  \n");
}

pub fn main() {
    let board = Board::new();
    board.save("board.cb");
    print_board(&board, get_moves(&board, 5, 3));
}
