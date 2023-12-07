use crate::{BitBoard, Piece, PieceSet, Pos};

impl PieceSet {
    #[must_use]
    pub fn new<P: Into<BitBoard>>(piece: Piece, bit_board: P) -> Self {
        Self {
            piece,
            bit_board: bit_board.into(),
        }
    }

    pub fn clear(&mut self) {
        self.bit_board = BitBoard::empty();
    }

    #[must_use]
    pub fn at(&self, pos: Pos) -> BitBoard {
        self.bit_board.and(pos)
    }

    pub fn apply_move(&mut self, from: Pos, to: Pos) {
        self.bit_board.xor_mut(from.as_bit_board().or(to));
    }
}

#[cfg(test)]
mod test {
    use crate::{pos::ORIGIN, Color};

    use super::*;

    static PIECE: Piece = Piece::Pawn(Color::White);
    static TARGET: Pos = Pos(3, 3);

    #[test]
    fn at() {
        let sut = PieceSet::new(PIECE, BitBoard(0b0));
        assert!(sut.at(ORIGIN).is_empty(), "{ORIGIN:?} should be empty");

        let sut = PieceSet::new(PIECE, BitBoard(0b1));
        assert!(!sut.at(ORIGIN).is_empty(), "{ORIGIN:?} should not be empty");

        let sut = PieceSet::new(PIECE, TARGET);
        assert!(!sut.at(TARGET).is_empty(), "{TARGET:?} should not be empty");
    }

    #[test]
    fn mov() {
        let mut sut = PieceSet::new(PIECE, BitBoard(0b1));
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
        let mut sut = PieceSet::new(PIECE, BitBoard(1));
        assert!(!sut.at(ORIGIN).is_empty(), "should have piece at ORIGIN");

        sut.clear();
        assert!(sut.at(ORIGIN).is_empty(), "should be empty after clear");
    }
}
