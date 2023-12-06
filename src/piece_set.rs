use crate::{Color, Piece, PieceSet, Pos};

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
