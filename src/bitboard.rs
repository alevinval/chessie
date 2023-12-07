use crate::Pos;

#[derive(Debug)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn load(value: u64, row: usize, col: usize) -> Self {
        Self((value << (row * 8)) << col)
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
        value.as_bit_board()
    }
}

#[cfg(test)]
mod test {

    use crate::pos::ORIGIN;

    use super::*;

    static TARGET: Pos = Pos(3, 3);

    #[test]
    fn has_piece() {
        let sut = BitBoard::empty();
        println!("SUT: {sut:?}");
        assert!(!sut.has_piece(ORIGIN), "{ORIGIN:?} should not have piece");

        let sut = BitBoard(1);
        assert!(sut.has_piece(ORIGIN), "{ORIGIN:?} should have piece");

        let sut = BitBoard::empty();
        assert!(!sut.has_piece(TARGET), "{TARGET:?} should not have piece");

        let sut = TARGET.as_bit_board();
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
