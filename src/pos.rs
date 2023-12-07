use crate::{BitBoard, Direction};

pub static ORIGIN: Pos = Pos(0, 0);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Pos(pub usize, pub usize);

impl Pos {
    #[must_use]
    pub fn to(&self, d: Direction) -> Self {
        let (row, col) = (self.0, self.1);
        let pos = match d {
            Direction::Top => (row + 1, col),
            Direction::TopLeft => (row + 1, col - 1),
            Direction::TopRight => (row + 1, col + 1),
            Direction::Bottom => (row - 1, col),
            Direction::BottomLeft => (row - 1, col - 1),
            Direction::BottomRight => (row - 1, col + 1),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        };

        self.assert_bounds();
        Self(pos.0, pos.1)
    }

    #[must_use]
    pub fn row(&self) -> usize {
        self.assert_bounds();
        self.0
    }

    #[must_use]
    pub fn col(&self) -> usize {
        self.assert_bounds();
        self.1
    }

    #[must_use]
    pub fn as_bit_board(&self) -> BitBoard {
        self.assert_bounds();
        BitBoard((1 << (self.0 * 8)) << self.1)
    }

    fn assert_bounds(&self) {
        debug_assert!(
            self.0 < 8 && self.1 < 8,
            "position outside of bounds ({self:?})"
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to() {
        let sut = Pos(4, 4);

        assert!(Pos(5, 4) == sut.to(Direction::Top), "should have moved top");
        assert!(
            Pos(5, 5) == sut.to(Direction::TopRight),
            "should have moved top-right"
        );
        assert!(
            Pos(5, 3) == sut.to(Direction::TopLeft),
            "should have moved top-left"
        );
        assert!(
            Pos(3, 4) == sut.to(Direction::Bottom),
            "should have moved bottom"
        );
        assert!(
            Pos(3, 5) == sut.to(Direction::BottomRight),
            "should have moved bottom-right"
        );
        assert!(
            Pos(3, 3) == sut.to(Direction::BottomLeft),
            "should have moved bottom-left"
        );
    }

    #[test]
    #[should_panic(expected = "")]
    fn to_outside_bounds() {
        let _ = Pos(0, 0).to(Direction::Bottom);
    }
}
