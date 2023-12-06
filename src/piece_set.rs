use std::iter::zip;

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
        println!("moves for {pos:?}");
        let mut moves = vec![];
        match &self.piece {
            Piece::Pawn(c) => match c {
                Color::Black => {
                    moves.push(pos.d());
                    moves.push(pos.dr());
                    if pos.col() < 8 && pos.col() > 0 {
                        moves.push(pos.dl());
                    }
                    if pos.row() == 6 {
                        moves.push(pos.d().d());
                    }
                }
                Color::White => {
                    moves.push(pos.u());
                    if pos.col() < 8 && pos.col() > 0 {
                        moves.push(pos.ur());
                    }
                    moves.push(pos.ul());
                    if pos.row() == 1 {
                        moves.push(pos.u().u());
                    }
                }
            },
            Piece::Rook(_) => cross(pos, &mut moves),
            Piece::Bishop(_) => diagonals(pos, &mut moves),
            Piece::Queen(_) => {
                cross(pos, &mut moves);
                diagonals(pos, &mut moves);
            }
            Piece::Knight(_) | Piece::King(_) => (),
        };
        moves
    }
}

fn cross(pos: &Pos, out: &mut Vec<Pos>) {
    for r in (0..pos.row()).chain(pos.row() + 1..8) {
        out.push(Pos::new(r, pos.col()));
    }
    for c in (0..pos.col()).chain(pos.col() + 1..8) {
        out.push(Pos::new(pos.row(), c));
    }
}

fn diagonals(pos: &Pos, out: &mut Vec<Pos>) {
    for (row, col) in zip(pos.row() + 1..8, pos.col() + 1..8) {
        out.push(Pos::new(row, col));
    }
    for (row, col) in zip(0..pos.row(), pos.col() + 1..8) {
        out.push(Pos::new(pos.row() - 1 - row, col));
    }
    for (row, col) in zip(pos.row() + 1..8, 0..pos.col()) {
        out.push(Pos::new(row, pos.col() - col - 1));
    }
    for (row, col) in zip(0..pos.row(), 0..pos.col()) {
        out.push(Pos::new(pos.row() - row - 1, pos.col() - col - 1));
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
