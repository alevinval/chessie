use either::Either;

use crate::{piece::Piece, pos::Pos};

use super::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BitBoard {
    value: u64,
    cnt: u8,
}

impl BitBoard {
    pub const fn new(piece: Piece, color: Color) -> Self {
        let mut value = match piece {
            Piece::Pawn => 0b1111_1111,
            Piece::Rook => 0b1000_0001,
            Piece::Knight => 0b0100_0010,
            Piece::Bishop => 0b0010_0100,
            Piece::Queen => 0b0000_1000,
            Piece::King => 0b0001_0000,
        };
        value <<=
            8 * if matches!(piece, Piece::Pawn) { color.pawn_row() } else { color.piece_row() };

        let cnt = match piece {
            Piece::Pawn => 8,
            Piece::Rook | Piece::Knight | Piece::Bishop => 2,
            Piece::Queen | Piece::King => 1,
        };

        Self { value, cnt }
    }

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    pub fn has_piece<P: Into<u64>>(&self, pos: P) -> bool {
        self.value & pos.into() != 0
    }

    pub fn slide<P: Into<u64>>(&mut self, from: P, to: P) {
        self.value ^= from.into() | to.into();
    }

    pub fn set<P: Into<u64>>(&mut self, pos: P) {
        self.value |= pos.into();
        self.cnt += 1;
    }

    pub fn unset<P: Into<u64>>(&mut self, pos: P) {
        self.value &= !pos.into();
        self.cnt -= 1;
    }

    pub fn get_le_bytes(&self) -> [u8; 8] {
        u64::to_le_bytes(self.value)
    }

    pub fn iter_pos(self, color: Color) -> impl Iterator<Item = Pos> {
        let rows = match color {
            Color::B => Either::Left((0..8).rev()),
            Color::W => Either::Right(0..8),
        };
        rows.flat_map(move |row| {
            let ro = row * 8;
            (0..8).filter_map(move |col| {
                if self.value & (1 << (ro + col)) > 0 {
                    Some((row, col).into())
                } else {
                    None
                }
            })
        })
        .take(self.cnt as usize)
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
        let sut = BitBoard { value: 0, cnt: 0 };
        assert!(!sut.has_piece(ORIGIN), "{ORIGIN:?} should not have piece");

        let sut = BitBoard { value: 1, cnt: 0 };
        assert!(sut.has_piece(ORIGIN), "{ORIGIN:?} should have piece");

        let sut = BitBoard { value: 0, cnt: 0 };
        assert!(!sut.has_piece(TARGET), "{TARGET:?} should not have piece");

        let sut = BitBoard { value: TARGET.into(), cnt: 0 };
        assert!(sut.has_piece(TARGET), "{TARGET:?} should have piece");
    }

    #[test]
    fn slide() {
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
        let sut = BitBoard { value: u64::MAX, cnt: 0 };
        let actual = sut.get_le_bytes();
        assert!(8 == actual.len());
        assert!(actual.iter().all(|n| *n == 255), "should all be max u8");
    }

    #[test]
    fn size() {
        assert_eq!(16, mem::size_of::<BitBoard>());
        assert_eq!(8, mem::size_of::<&BitBoard>());
    }
}