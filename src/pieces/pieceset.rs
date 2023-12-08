use crate::pos::Pos;

use super::{BitBoard, Piece};

pub struct PieceSet {
    pub piece: Piece,
    pub bitboard: BitBoard,
}

impl PieceSet {
    pub fn new(piece: Piece) -> Self {
        Self::new_bitboard(piece, Self::initial_position(&piece))
    }

    pub fn new_bitboard<P: Into<BitBoard>>(piece: Piece, bitboard: P) -> Self {
        Self {
            piece,
            bitboard: bitboard.into(),
        }
    }

    pub fn clear(&mut self) {
        self.bitboard = BitBoard(0);
    }

    pub fn at(&self, pos: Pos) -> BitBoard {
        self.bitboard.and(pos)
    }

    pub fn apply_move(&mut self, from: Pos, to: Pos) {
        let from: BitBoard = from.into();
        self.bitboard.xor_mut(from.or(to));
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
    use crate::{pieces::Color, pos::ORIGIN};

    use super::*;

    static PIECE: Piece = Piece::Pawn(Color::White);
    static TARGET: Pos = Pos(3, 3);

    #[test]
    fn at() {
        let sut = PieceSet::new_bitboard(PIECE, BitBoard(0b0));
        assert!(sut.at(ORIGIN).is_empty(), "{ORIGIN:?} should be empty");

        let sut = PieceSet::new_bitboard(PIECE, BitBoard(0b1));
        assert!(!sut.at(ORIGIN).is_empty(), "{ORIGIN:?} should not be empty");

        let sut = PieceSet::new_bitboard(PIECE, TARGET);
        assert!(!sut.at(TARGET).is_empty(), "{TARGET:?} should not be empty");
    }

    #[test]
    fn mov() {
        let mut sut = PieceSet::new_bitboard(PIECE, BitBoard(0b1));
        assert!(
            !sut.at(ORIGIN).is_empty(),
            "should have piece at {ORIGIN:?}"
        );

        sut.apply_move(ORIGIN, TARGET);
        assert!(sut.at(ORIGIN).is_empty(), "{ORIGIN:?} should be empty");
        assert!(
            !sut.at(TARGET).is_empty(),
            "{TARGET:?} should contain a piece"
        );
    }

    #[test]
    fn clear() {
        let mut sut = PieceSet::new_bitboard(PIECE, BitBoard(1));
        assert!(!sut.at(ORIGIN).is_empty(), "should have piece at ORIGIN");

        sut.clear();
        assert!(sut.at(ORIGIN).is_empty(), "should be empty after clear");
    }
}
