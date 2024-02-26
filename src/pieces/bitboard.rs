use crate::pos::Pos;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct BitBoard {
    value: u64,
}

impl BitBoard {
    pub fn load_row(value: u64, row: usize) -> Self {
        Self {
            value: value << (row * 8),
        }
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
}

fn rshiftpos(value: u64, pos: Pos) -> u64 {
    let other: u64 = pos.into();
    (value & other) >> (pos.row() * 8 + pos.col())
}

impl From<u64> for BitBoard {
    fn from(value: u64) -> Self {
        BitBoard { value }
    }
}

impl From<Pos> for BitBoard {
    fn from(value: Pos) -> Self {
        let value: u64 = value.into();
        BitBoard { value }
    }
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
        let sut: BitBoard = 0.into();
        assert!(!sut.has_piece(ORIGIN), "{ORIGIN:?} should not have piece");

        let sut: BitBoard = 1.into();
        assert!(sut.has_piece(ORIGIN), "{ORIGIN:?} should have piece");

        let sut: BitBoard = 0.into();
        assert!(!sut.has_piece(TARGET), "{TARGET:?} should not have piece");

        let sut: BitBoard = TARGET.into();
        assert!(sut.has_piece(TARGET), "{TARGET:?} should have piece");
    }

    #[test]
    fn to_le_bytes() {
        let sut: BitBoard = u64::MAX.into();
        let actual = sut.to_le_bytes();
        assert!(8 == actual.len());
        assert!(actual.iter().all(|n| *n == 255), "should all be max u8");
    }
}
