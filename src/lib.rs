pub use pos::Pos;

mod board;
mod piece;
mod piece_set;
mod pos;

pub type BitBoard = u64;

pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

pub struct PieceSet {
    pub piece: Piece,
    pub bit_board: BitBoard,
}

pub struct Board {
    white: [PieceSet; 6],
    black: [PieceSet; 6],
}

fn print_board(board: &Board, highlights: Vec<Pos>) {
    for row in (0..8).rev() {
        print!("+---+---+---+---+---+---+---+---+\n");
        for col in 0..8 {
            let p = &Pos(row, col);
            print!(
                "| {} ",
                highlights
                    .iter()
                    .find(|hpos| hpos.row() == p.row() && hpos.col() == p.col())
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
    board.apply_move(&Pos(1, 0), &Pos(7, 1));
    // board.save("board.cb");
    // board.clear();

    let pos = [&Pos(1, 1), &Pos(7, 1), &Pos(4, 5)];
    // board.set(pos[1], &Piece::Queen(Color::Black));
    // board.set(pos[2], &Piece::Queen(Color::White));

    let moves = pos.iter().flat_map(|p| board.generate_moves(p)).collect();
    print_board(&board, moves);
}
