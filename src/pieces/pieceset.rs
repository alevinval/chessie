use crate::pos::Pos;

use super::{BitBoard, Color, Piece};

#[derive(Debug, Clone)]
pub struct PieceSet {
    piece: Piece,
    bitboard: BitBoard,
}

impl PieceSet {
    pub fn new(piece: Piece) -> Self {
        Self::new_bitboard(piece, Self::initial_position(&piece))
    }

    fn new_bitboard<P: Into<BitBoard>>(piece: Piece, bitboard: P) -> Self {
        Self {
            piece,
            bitboard: bitboard.into(),
        }
    }

    pub fn has_piece<P: Into<Pos>>(&self, pos: P) -> bool {
        self.bitboard.has_piece(pos)
    }

    pub fn apply_move<P: Into<u64>>(&mut self, from: P, to: P) {
        self.bitboard.apply_move(from, to);
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn color(&self) -> Color {
        self.piece.color()
    }

    pub fn score(&self) -> f32 {
        self.iter_pos()
            .map(|p| self.piece.score() + if p.is_central() { 0.25 } else { 0.0 })
            .sum()
    }

    pub fn unset(&mut self, pos: Pos) {
        self.bitboard.unset(pos);
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = Pos> + '_ {
        self.bitboard.iter_pos()
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        self.bitboard.to_le_bytes()
    }

    fn initial_position(piece: &Piece) -> BitBoard {
        match piece {
            Piece::Pawn(c) => BitBoard::load_row(0b11111111, c.pawn_row()),
            Piece::Rook(c) => BitBoard::load_row(0b10000001, c.piece_row()),
            Piece::Knight(c) => BitBoard::load_row(0b01000010, c.piece_row()),
            Piece::Bishop(c) => BitBoard::load_row(0b00100100, c.piece_row()),
            Piece::Queen(c) => BitBoard::load_row(0b00010000, c.piece_row()),
            Piece::King(c) => BitBoard::load_row(0b00001000, c.piece_row()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::pieces::Color;

    use super::*;

    static PIECE: Piece = Piece::Pawn(Color::White);
    static ORIGIN: Pos = Pos::new(0, 0);
    static TARGET: Pos = Pos::new(3, 3);

    #[test]
    fn at() {
        let sut = PieceSet::new_bitboard(PIECE, 0b0);
        assert!(!sut.has_piece(ORIGIN), "{ORIGIN:?} should be empty");

        let sut = PieceSet::new_bitboard(PIECE, 0b1);
        assert!(sut.has_piece(ORIGIN), "{ORIGIN:?} should not be empty");

        let sut = PieceSet::new_bitboard(PIECE, TARGET);
        assert!(sut.has_piece(TARGET), "{TARGET:?} should not be empty");
    }

    #[test]
    fn mov() {
        let mut sut = PieceSet::new_bitboard(PIECE, 0b1);
        assert!(sut.has_piece(ORIGIN), "should have piece at {ORIGIN:?}");

        sut.apply_move(ORIGIN, TARGET);
        assert!(!sut.has_piece(ORIGIN), "{ORIGIN:?} should be empty");
        assert!(sut.has_piece(TARGET), "{TARGET:?} should contain a piece");
    }
}
