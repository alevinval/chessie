use board::Board;
use position::Pos;

mod board;
mod piece;
mod position;

fn get_moves(board: &Board, pos: &Pos) -> Vec<Pos> {
    let pset = board.at(pos).expect("cannot move where there is no piece");
    let moves = pset.generate_moves(pos);
    moves
        .into_iter()
        .filter(|pos| {
            board
                .at(pos)
                .map_or(if pset.piece.is_pawn() { false } else { true }, |p| {
                    p.color() != pset.color()
                })
        })
        .collect()
}

fn print_board(board: &Board, highlights: Vec<Pos>) {
    for row in (0..8).rev() {
        print!("+---+---+---+---+---+---+---+---+\n");
        for col in 0..8 {
            let p = Pos::new(row, col);
            print!(
                "| {} ",
                highlights
                    .iter()
                    .find(|pos| pos.row() == row && pos.col() == col)
                    .map_or(board.at(&p).map_or(" ", |p| p.piece.to_str()), |_| "@"),
            );
        }
        print!("| {}\n", row + 1);
    }
    print!("+---+---+---+---+---+---+---+---+\n");
    print!("  a   b   c   d   e   f   g   h  \n");
}

pub fn main() {
    let mut board = Board::new();
    board.mov(&Pos::new(0, 0), &Pos::new(4, 5));
    board.save("board.cb");
    print_board(&board, get_moves(&board, &Pos::new(4, 5)));
}
