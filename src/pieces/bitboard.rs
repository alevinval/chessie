use crate::pos::Pos;

#[derive(Debug, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn load_row(value: u64, row: usize) -> Self {
        Self(value << (row * 8))
    }

    pub fn has_piece(&self, pos: Pos) -> bool {
        ((self.0 >> (pos.row() * 8)) >> pos.col()) & 1 == 1
    }

    pub fn or_mut<P: Into<BitBoard>>(&mut self, other: P) {
        self.0 |= other.into().0
    }

    pub fn or<P: Into<BitBoard>>(&self, other: P) -> BitBoard {
        BitBoard(self.0 | other.into().0)
    }

    pub fn and<P: Into<BitBoard>>(&self, other: P) -> BitBoard {
        BitBoard(self.0 & other.into().0)
    }

    pub fn xor_mut<P: Into<BitBoard>>(&mut self, other: P) {
        self.0 ^= other.into().0;
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        u64::to_le_bytes(self.0)
    }
}

impl From<Pos> for BitBoard {
    fn from(value: Pos) -> Self {
        BitBoard((1 << (value.row() * 8)) << value.col())
    }
}

impl From<u64> for BitBoard {
    fn from(value: u64) -> Self {
        BitBoard(value)
    }
}

#[cfg(test)]
mod test {

    use crate::pos::ORIGIN;

    use super::*;

    static TARGET: Pos = Pos(3, 3);

    #[test]
    fn has_piece() {
        let sut = BitBoard(0);
        println!("SUT: {sut:?}");
        assert!(!sut.has_piece(ORIGIN), "{ORIGIN:?} should not have piece");

        let sut = BitBoard(1);
        assert!(sut.has_piece(ORIGIN), "{ORIGIN:?} should have piece");

        let sut = BitBoard(0);
        assert!(!sut.has_piece(TARGET), "{TARGET:?} should not have piece");

        let sut: BitBoard = TARGET.into();
        assert!(sut.has_piece(TARGET), "{TARGET:?} should have piece");
    }

    #[test]
    fn to_le_bytes() {
        let input = u64::MAX;
        let sut = BitBoard(input);
        let actual = sut.to_le_bytes();
        assert!(8 == actual.len());
        assert!(actual.iter().all(|n| *n == 255), "should all be max u8");
    }
}
