#[derive(PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

pub enum Piece {
    Pawn(Color, u64),
    Rook(Color, u64),
    Knight(Color, u64),
    Bishop(Color, u64),
    Queen(Color, u64),
    King(Color, u64),
}

impl Piece {
    pub fn to_str(&self) -> &str {
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

    pub fn is_pawn(&self) -> bool {
        match self {
            Piece::Pawn(_, _) => true,
            Piece::Rook(_, _)
            | Piece::Knight(_, _)
            | Piece::Bishop(_, _)
            | Piece::Queen(_, _)
            | Piece::King(_, _) => false,
        }
    }

    pub fn color(&self) -> &Color {
        match self {
            Piece::Pawn(c, _)
            | Piece::Rook(c, _)
            | Piece::Knight(c, _)
            | Piece::Bishop(c, _)
            | Piece::Queen(c, _)
            | Piece::King(c, _) => c,
        }
    }

    pub fn at(&self, row: usize, col: usize) -> u64 {
        let mask = (1u64 << col) << row * 8;
        ((self.n() & mask) >> row * 8) >> col
    }

    pub fn n(&self) -> u64 {
        *match self {
            Piece::Pawn(_, n)
            | Piece::Rook(_, n)
            | Piece::Knight(_, n)
            | Piece::Bishop(_, n)
            | Piece::Queen(_, n)
            | Piece::King(_, n) => n,
        }
    }

    pub fn nle(&self) -> [u8; 8] {
        u64::to_le_bytes(self.n())
    }
}
