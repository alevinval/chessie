use std::{fs::File, io::Write};

enum Color {
    Black,
    White,
}

enum Piece {
    Pawn(Color, u64),
    Rook(Color, u64),
    Knight(Color, u64),
    Bishop(Color, u64),
    Queen(Color, u64),
    King(Color, u64),
}

impl Piece {
    fn to_str(&self) -> &str {
        match self {
            Piece::Pawn(c, _) => match c {
                Color::Black => "♟",
                Color::White => "♙",
            },
            Piece::Rook(c, _) => match c {
                Color::Black => "♜",
                Color::White => "♖",
            },
            Piece::Knight(c, _) => match c {
                Color::Black => "♞",
                Color::White => "♘",
            },
            Piece::Bishop(c, _) => match c {
                Color::Black => "♝",
                Color::White => "♗",
            },
            Piece::Queen(c, _) => match c {
                Color::Black => "♛",
                Color::White => "♕",
            },
            Piece::King(c, _) => match c {
                Color::Black => "♚",
                Color::White => "♔",
            },
        }
    }

    fn at(&self, row: usize, col: usize) -> u64 {
        let mask = (1u64 << col) << row * 8;
        ((self.n() & mask) >> row * 8) >> col
    }

    fn n(&self) -> u64 {
        *match self {
            Piece::Pawn(_, n)
            | Piece::Rook(_, n)
            | Piece::Knight(_, n)
            | Piece::Bishop(_, n)
            | Piece::Queen(_, n)
            | Piece::King(_, n) => n,
        }
    }

    fn nle(&self) -> [u8; 8] {
        u64::to_le_bytes(self.n())
    }
}

struct Board {
    white: [Piece; 6],
    black: [Piece; 6],
}

impl Board {
    fn new() -> Self {
        Self {
            white: [
                Piece::Pawn(Color::White, pos(0b11111111, 1, 0)),
                Piece::Rook(Color::White, pos(0b10000001, 0, 0)),
                Piece::Knight(Color::White, pos(0b01000010, 0, 0)),
                Piece::Bishop(Color::White, pos(0b00100100, 0, 0)),
                Piece::Queen(Color::White, pos(1, 0, 3)),
                Piece::King(Color::White, pos(1, 0, 4)),
            ],

            black: [
                Piece::Pawn(Color::Black, pos(0b11111111, 6, 0)),
                Piece::Rook(Color::White, pos(0b10000001, 7, 0)),
                Piece::Knight(Color::White, pos(0b01000010, 7, 0)),
                Piece::Bishop(Color::White, pos(0b00100100, 7, 0)),
                Piece::Queen(Color::White, pos(1, 7, 3)),
                Piece::King(Color::White, pos(1, 7, 4)),
            ],
        }
    }

    fn save(&self, fname: &str) {
        let mut w = File::create(fname).unwrap();
        self.white.iter().for_each(|p| {
            w.write(&p.nle()).unwrap();
        });
        self.black.iter().for_each(|p| {
            w.write(&p.nle()).unwrap();
        });
    }

    fn at(&self, row: usize, col: usize) -> &str {
        for piece in self.white.iter() {
            if piece.at(row, col) == 1 {
                return piece.to_str();
            }
        }

        for piece in self.black.iter() {
            if piece.at(row, col) == 1 {
                return piece.to_str();
            }
        }
        return " ";
    }
}

fn pos(value: u64, row: usize, col: usize) -> u64 {
    (value << row * 8) << col
}

fn print_board(board: &Board) {
    for row in (0..8).rev() {
        print!("+---+---+---+---+---+---+---+---+\n");
        for col in 0..8 {
            print!("| {} ", board.at(row, col));
        }
        print!("| {}\n", row + 1);
    }
    print!("+---+---+---+---+---+---+---+---+\n");
    print!("  a   b   c   d   e   f   g   h  \n");
}

fn main() {
    let board = Board::new();
    board.save("board.cb");
    print_board(&board);
}
