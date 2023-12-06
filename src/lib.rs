mod board;
mod piece;
mod piece_set;
mod pos;

pub type BitBoard = u64;

#[derive(PartialEq, Eq, Debug)]
pub struct Pos(usize, usize);

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

fn get_moves(board: &Board, pos: &Pos) -> Vec<Pos> {
    let pset = board.at(pos).expect("cannot move where there is no piece");
    pset.generate_moves(pos)
}

fn print_board(board: &Board, highlights: Vec<Pos>) {
    println!("{highlights:?}");
    for row in (0..8).rev() {
        print!("+---+---+---+---+---+---+---+---+\n");
        for col in 0..8 {
            let p = &Pos::new(row, col);
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
    board.mov(&Pos::new(0, 0), &Pos::new(4, 5));
    board.save("board.cb");
    print_board(&board, get_moves(&board, &Pos::new(4, 5)));
}
