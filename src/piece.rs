use crate::position::Pos;

#[derive(PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

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
    pub table: u64,
}

impl Piece {
    pub fn to_str(&self) -> &str {
        match self {
            Piece::Pawn(c) => match c {
                Color::Black => "♟",
                Color::White => "♙",
            },
            Piece::Rook(c) => match c {
                Color::Black => "♜",
                Color::White => "♖",
            },
            Piece::Knight(c) => match c {
                Color::Black => "♞",
                Color::White => "♘",
            },
            Piece::Bishop(c) => match c {
                Color::Black => "♝",
                Color::White => "♗",
            },
            Piece::Queen(c) => match c {
                Color::Black => "♛",
                Color::White => "♕",
            },
            Piece::King(c) => match c {
                Color::Black => "♚",
                Color::White => "♔",
            },
        }
    }

    pub fn is_pawn(&self) -> bool {
        match self {
            Piece::Pawn(_) => true,
            _ => false,
        }
    }

    pub fn color(&self) -> &Color {
        match self {
            Piece::Pawn(c)
            | Piece::Rook(c)
            | Piece::Knight(c)
            | Piece::Bishop(c)
            | Piece::Queen(c)
            | Piece::King(c) => c,
        }
    }
}

impl PieceSet {
    pub fn new(piece: Piece, table: u64) -> Self {
        Self { piece, table }
    }

    pub fn at(&self, pos: &Pos) -> u64 {
        let mask = self.mask_pos(pos);
        ((self.table & mask) >> pos.row() * 8) >> pos.col()
    }

    pub fn mov(&mut self, from: &Pos, to: &Pos) {
        self.table ^= self.mask_pos(from) | self.mask_pos(to);
    }

    pub fn nle(&self) -> [u8; 8] {
        u64::to_le_bytes(self.table)
    }

    pub fn color(&self) -> &Color {
        self.piece.color()
    }

    fn mask_pos(&self, pos: &Pos) -> u64 {
        (1u64 << pos.col()) << pos.row() * 8
    }

    pub fn generate_moves(&self, pos: &Pos) -> Vec<Pos> {
        let mut moves = vec![];
        match &self.piece {
            Piece::Pawn(c) => match c {
                Color::Black => {
                    moves.push(pos.d());
                    moves.push(pos.dr());
                    moves.push(pos.dl());
                }
                Color::White => {
                    moves.push(pos.u());
                    moves.push(pos.ur());
                    moves.push(pos.ul());
                }
            },
            Piece::Rook(_) => {
                for r in (0..pos.row()).chain(pos.row() + 1..8) {
                    moves.push(Pos::new(r, pos.col()));
                }
                for c in (0..pos.col()).chain(pos.col() + 1..8) {
                    moves.push(Pos::new(pos.row(), c));
                }
            }
            Piece::Knight(_) | Piece::Bishop(_) | Piece::Queen(_) | Piece::King(_) => (),
        };
        moves
    }
}
