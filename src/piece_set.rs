use crate::{BitBoard, Color, Piece, PieceSet, Pos};

impl PieceSet {
    pub fn new(piece: Piece, bit_board: BitBoard) -> Self {
        Self { piece, bit_board }
    }

    pub fn clear(&mut self) {
        self.bit_board = 0;
    }

    pub fn at(&self, pos: &Pos) -> BitBoard {
        self.bit_board & self.mask_pos(pos)
    }

    pub fn mov(&mut self, from: &Pos, to: &Pos) {
        self.bit_board ^= self.mask_pos(from) | self.mask_pos(to);
    }

    pub fn nle(&self) -> [u8; 8] {
        u64::to_le_bytes(self.bit_board)
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

#[cfg(test)]
mod test {
    use crate::pos::ORIGIN;

    use super::*;

    static PIECE: Piece = Piece::Pawn(Color::White);
    static TARGET: &Pos = &Pos(3, 3);

    #[test]
    fn test_pieceset_at() {
        let sut = PieceSet::new(PIECE, 0b0);
        assert!(sut.at(ORIGIN) == 0, "{ORIGIN:?} should be empty");

        let sut = PieceSet::new(PIECE, 0b1);
        assert!(sut.at(ORIGIN) > 0, "{ORIGIN:?} should not be empty");

        let sut = PieceSet::new(PIECE, TARGET.as_bit_board());
        assert!(sut.at(TARGET) > 0, "{TARGET:?} should not be empty");
    }

    #[test]
    fn test_pieceset_mov() {
        let mut sut = PieceSet::new(PIECE, 0b1);
        assert!(sut.at(ORIGIN) > 0, "should have piece at {ORIGIN:?}");

        sut.mov(ORIGIN, TARGET);
        assert!(sut.at(ORIGIN) == 0, "{ORIGIN:?} should be empty");
        assert!(sut.at(TARGET) > 0, "{TARGET:?} should contain a piece");
    }

    #[test]
    fn test_pieceset_nle() {
        let input = u64::MAX;
        let sut = PieceSet::new(PIECE, input);
        let actual = sut.nle();
        assert!(8 == actual.len());
        assert!(actual.iter().all(|n| *n == 255), "should all be max u8")
    }

    #[test]
    fn test_pieceset_color() {
        let sut = PieceSet::new(PIECE, 0);
        assert!(sut.color() == PIECE.color(), "color should match");
    }

    #[test]
    fn test_pieceset_clear() {
        let mut sut = PieceSet::new(PIECE, 1);
        assert!(sut.at(ORIGIN) > 0, "should have piece at ORIGIN");

        sut.clear();
        assert!(sut.at(ORIGIN) == 0, "should be empty after clear");
    }
}
