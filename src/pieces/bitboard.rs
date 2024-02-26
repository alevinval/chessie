use crate::pos::Pos;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct BitBoard {
    value: u64,
}

impl BitBoard {
    #[cfg(test)]
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn load_row(value: u64, row: usize) -> Self {
        Self {
            value: value << (row * 8),
        }
    }

    pub fn has_piece(&self, pos: Pos) -> bool {
        ((self.value >> (pos.row() * 8)) >> pos.col()) & 1 == 1
    }

    pub fn or_mut<P: Into<BitBoard>>(&mut self, other: P) {
        self.value |= other.into().value
    }

    pub fn or<P: Into<BitBoard>>(&self, other: P) -> BitBoard {
        BitBoard {
            value: self.value | other.into().value,
        }
    }

    pub fn and<P: Into<BitBoard>>(&self, other: P) -> BitBoard {
        BitBoard {
            value: self.value & other.into().value,
        }
    }

    pub fn xor_mut<P: Into<BitBoard>>(&mut self, other: P) {
        self.value ^= other.into().value;
    }

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        u64::to_le_bytes(self.value)
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..8).flat_map(move |row| {
            let ro = row * 8;
            (0..8).flat_map(move |col| {
                if ((self.value >> ro) >> col) & 1 == 1 {
                    Some((row, col).into())
                } else {
                    None
                }
            })
        })
    }
}

impl From<Pos> for BitBoard {
    fn from(value: Pos) -> Self {
        BitBoard {
            value: (1 << (value.row() * 8)) << value.col(),
        }
    }
}

impl From<(u8, u8)> for BitBoard {
    fn from(value: (u8, u8)) -> Self {
        let p: Pos = value.into();
        p.into()
    }
}

impl From<u64> for BitBoard {
    fn from(value: u64) -> Self {
        BitBoard { value }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    static ORIGIN: Pos = Pos::new(0, 0);
    static TARGET: Pos = Pos::new(3, 3);

    #[test]
    fn has_piece() {
        let sut = BitBoard::new(0);
        assert!(!sut.has_piece(ORIGIN), "{ORIGIN:?} should not have piece");

        let sut = BitBoard::new(1);
        assert!(sut.has_piece(ORIGIN), "{ORIGIN:?} should have piece");

        let sut = BitBoard::new(0);
        assert!(!sut.has_piece(TARGET), "{TARGET:?} should not have piece");

        let sut: BitBoard = TARGET.into();
        assert!(sut.has_piece(TARGET), "{TARGET:?} should have piece");
    }

    #[test]
    fn to_le_bytes() {
        let sut = BitBoard::new(u64::MAX);
        let actual = sut.to_le_bytes();
        assert!(8 == actual.len());
        assert!(actual.iter().all(|n| *n == 255), "should all be max u8");
    }
}
