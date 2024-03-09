use crate::{piece::Piece, pos::Pos};

use super::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BitBoard {
    value: u64,
}

impl BitBoard {
    pub const fn empty() -> Self {
        Self { value: 0 }
    }

    pub const fn new(piece: Piece, color: Color) -> Self {
        let mut value = match piece {
            Piece::Pawn => 0b1111_1111,
            Piece::Rook => 0b1000_0001,
            Piece::Knight => 0b0100_0010,
            Piece::Bishop => 0b0010_0100,
            Piece::Queen => 0b0000_1000,
            Piece::King => 0b000_10000,
        };
        value <<= 8 * if matches!(piece, Piece::Pawn) {
            color.pawn_row()
        } else {
            color.piece_row()
        };

        Self { value }
    }

    pub fn count(self) -> usize {
        let mut count = 0;
        let mut value = self.value;
        while value != 0 {
            count += 1;
            value &= value - 1;
        }
        count
    }

    pub fn is_empty(self) -> bool {
        self.value == 0
    }

    pub fn has_piece<P: Into<u64>>(self, pos: P) -> bool {
        self.value & pos.into() != 0
    }

    pub fn slide<P: Into<u64>>(&mut self, from: P, to: P) {
        self.value ^= from.into() | to.into();
    }

    pub fn set<P: Into<u64>>(&mut self, pos: P) {
        self.value |= pos.into();
    }

    pub fn unset<P: Into<u64>>(&mut self, pos: P) {
        self.value &= !pos.into();
    }

    pub fn get_le_bytes(self) -> [u8; 8] {
        u64::to_le_bytes(self.value)
    }

    pub fn iter_pos(self) -> impl Iterator<Item = Pos> {
        (0..8)
            .flat_map(move |row| {
                let ro = row * 8;
                (0..8).filter_map(move |col| {
                    if self.value & (1 << (ro + col)) > 0 {
                        Some((row, col).into())
                    } else {
                        None
                    }
                })
            })
            .take(self.count())
    }
}

impl From<Pos> for u64 {
    fn from(pos: Pos) -> Self {
        1 << (pos.row() * 8 + pos.col())
    }
}

#[cfg(test)]
mod test {

    use std::mem;

    use super::*;

    static ORIGIN: Pos = Pos::new(0, 0);
    static TARGET: Pos = Pos::new(3, 3);

    #[test]
    fn has_piece() {
        let sut = BitBoard { value: 0 };
        assert!(!sut.has_piece(ORIGIN), "{ORIGIN:?} should not have piece");
        assert_eq!(0, sut.count());

        let sut = BitBoard { value: 1 };
        assert!(sut.has_piece(ORIGIN), "{ORIGIN:?} should have piece");
        assert_eq!(1, sut.count());

        let sut = BitBoard { value: 0 };
        assert!(!sut.has_piece(TARGET), "{TARGET:?} should not have piece");

        let sut = BitBoard {
            value: TARGET.into(),
        };
        assert!(sut.has_piece(TARGET), "{TARGET:?} should have piece");
    }

    #[test]
    fn mov() {
        let from: Pos = (1, 1).into();
        let to: Pos = (2, 2).into();

        let mut sut = BitBoard::new(Piece::Pawn, Color::W);
        assert!(!sut.has_piece(to));

        sut.slide(from, to);
        assert!(sut.has_piece(to));

        assert!(!sut.has_piece(from));
    }

    #[test]
    fn unset() {
        let pos: Pos = (1, 1).into();
        let mut sut = BitBoard::new(Piece::Pawn, Color::W);
        assert!(sut.has_piece(pos));

        sut.unset(pos);
        assert!(!sut.has_piece(pos));
    }

    #[test]
    fn to_le_bytes() {
        let sut = BitBoard { value: u64::MAX };
        let actual = sut.get_le_bytes();
        assert!(8 == actual.len());
        assert!(actual.iter().all(|n| *n == 255), "should all be max u8");
    }

    #[test]
    fn size() {
        assert_eq!(8, mem::size_of::<BitBoard>());
        assert_eq!(8, mem::size_of::<&BitBoard>());
    }
}
