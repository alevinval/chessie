use crate::pos::Pos;

use super::{Color, Piece};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitBoard {
    piece: Piece,
    value: u64,
}

impl BitBoard {
    pub fn new(piece: Piece) -> Self {
        Self::initial_position(piece)
    }

    pub fn load(piece: Piece, value: u64) -> Self {
        Self { piece, value }
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn color(&self) -> Color {
        self.piece.color()
    }

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    pub fn has_piece<P: Into<Pos>>(&self, pos: P) -> bool {
        rshiftpos(self.value, pos.into()) == 1
    }

    pub fn apply_move<P: Into<u64>>(&mut self, from: P, to: P) {
        self.value ^= from.into() | to.into()
    }

    pub fn set<P: Into<u64>>(&mut self, other: P) {
        self.value |= other.into()
    }

    pub fn unset<P: Into<u64>>(&mut self, other: P) {
        self.value &= !other.into();
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        u64::to_le_bytes(self.value)
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..8).flat_map(move |row| {
            let ro = row * 8;
            (0..8).flat_map(move |col| {
                if self.value >> (ro + col) & 1 == 1 {
                    Some((row, col).into())
                } else {
                    None
                }
            })
        })
    }

    fn initial_position(piece: Piece) -> BitBoard {
        match piece {
            Piece::Pawn(c) => BitBoard::load(piece, 0b11111111 << 8 * c.pawn_row()),
            Piece::Rook(c) => BitBoard::load(piece, 0b10000001 << 8 * c.piece_row()),
            Piece::Knight(c) => BitBoard::load(piece, 0b01000010 << 8 * c.piece_row()),
            Piece::Bishop(c) => BitBoard::load(piece, 0b00100100 << 8 * c.piece_row()),
            Piece::Queen(c) => BitBoard::load(piece, 0b00010000 << 8 * c.piece_row()),
            Piece::King(c) => BitBoard::load(piece, 0b00001000 << 8 * c.piece_row()),
        }
    }
}

fn rshiftpos(value: u64, pos: Pos) -> u64 {
    let other: u64 = pos.into();
    (value & other) >> (pos.row() * 8 + pos.col())
}

impl From<Pos> for u64 {
    fn from(value: Pos) -> Self {
        1 << (value.row() * 8 + value.col())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    static ORIGIN: Pos = Pos::new(0, 0);
    static TARGET: Pos = Pos::new(3, 3);

    #[test]
    fn has_piece() {
        let sut = BitBoard::load(Piece::Pawn(Color::White), 0);
        assert!(!sut.has_piece(ORIGIN), "{ORIGIN:?} should not have piece");

        let sut = BitBoard::load(Piece::Pawn(Color::White), 1);
        assert!(sut.has_piece(ORIGIN), "{ORIGIN:?} should have piece");

        let sut = BitBoard::load(Piece::Pawn(Color::White), 0);
        assert!(!sut.has_piece(TARGET), "{TARGET:?} should not have piece");

        let sut = BitBoard::load(Piece::Pawn(Color::White), TARGET.into());
        assert!(sut.has_piece(TARGET), "{TARGET:?} should have piece");
    }

    #[test]
    fn to_le_bytes() {
        let sut = BitBoard::load(Piece::Pawn(Color::White), u64::MAX);
        let actual = sut.to_le_bytes();
        assert!(8 == actual.len());
        assert!(actual.iter().all(|n| *n == 255), "should all be max u8");
    }
}
