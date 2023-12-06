use crate::{BitBoard, Piece, PieceSet, Pos};

impl PieceSet {
    pub fn new(piece: Piece, bit_board: BitBoard) -> Self {
        Self { piece, bit_board }
    }

    pub fn clear(&mut self) {
        self.bit_board = 0;
    }

    pub fn at(&self, pos: &Pos) -> BitBoard {
        self.bit_board & pos.as_bit_board()
    }

    pub fn apply_move(&mut self, from: &Pos, to: &Pos) {
        self.bit_board ^= from.as_bit_board() | to.as_bit_board();
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        u64::to_le_bytes(self.bit_board)
    }
}

#[cfg(test)]
mod test {
    use crate::{pos::ORIGIN, Color};

    use super::*;

    static PIECE: Piece = Piece::Pawn(Color::White);
    static TARGET: &Pos = &Pos(3, 3);

    #[test]
    fn at() {
        let sut = PieceSet::new(PIECE, 0b0);
        assert!(sut.at(ORIGIN) == 0, "{ORIGIN:?} should be empty");

        let sut = PieceSet::new(PIECE, 0b1);
        assert!(sut.at(ORIGIN) > 0, "{ORIGIN:?} should not be empty");

        let sut = PieceSet::new(PIECE, TARGET.as_bit_board());
        assert!(sut.at(TARGET) > 0, "{TARGET:?} should not be empty");
    }

    #[test]
    fn mov() {
        let mut sut = PieceSet::new(PIECE, 0b1);
        assert!(sut.at(ORIGIN) > 0, "should have piece at {ORIGIN:?}");

        sut.apply_move(ORIGIN, TARGET);
        assert!(sut.at(ORIGIN) == 0, "{ORIGIN:?} should be empty");
        assert!(sut.at(TARGET) > 0, "{TARGET:?} should contain a piece");
    }

    #[test]
    fn to_le_bytes() {
        let input = u64::MAX;
        let sut = PieceSet::new(PIECE, input);
        let actual = sut.to_le_bytes();
        assert!(8 == actual.len());
        assert!(actual.iter().all(|n| *n == 255), "should all be max u8")
    }

    #[test]
    fn clear() {
        let mut sut = PieceSet::new(PIECE, 1);
        assert!(sut.at(ORIGIN) > 0, "should have piece at ORIGIN");

        sut.clear();
        assert!(sut.at(ORIGIN) == 0, "should be empty after clear");
    }
}
