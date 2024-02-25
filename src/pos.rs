#[cfg(test)]
pub static ORIGIN: Pos = Pos(0, 0);

pub enum Direction {
    Top(u8),
    Bottom(u8),
    Left(u8),
    Right(u8),
    Custom(i8, i8),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Pos(pub u8, pub u8);

impl Pos {
    pub fn to(&self, d: Direction) -> Self {
        let (row, col) = (self.0, self.1);
        let pos = match d {
            Direction::Top(n) => (row + n, col),
            Direction::Bottom(n) => (row - n, col),
            Direction::Left(n) => (row, col - n),
            Direction::Right(n) => (row, col + n),
            Direction::Custom(nr, nc) => (((row as i8) + nr) as u8, ((col as i8) + nc) as u8),
        };

        self.assert_bounds();
        Self(pos.0, pos.1)
    }

    pub fn row(&self) -> u8 {
        self.assert_bounds();
        self.0
    }

    pub fn col(&self) -> u8 {
        self.assert_bounds();
        self.1
    }

    pub fn is_central(&self) -> bool {
        self.0 > 3 && self.1 > 3 && self.0 < 5 && self.1 < 5
    }

    fn assert_bounds(&self) {
        debug_assert!(
            self.0 < 8 && self.1 < 8,
            "position outside of bounds ({self:?})"
        );
    }
}

impl From<(u8, u8)> for Pos {
    fn from(value: (u8, u8)) -> Self {
        Pos(value.0, value.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to() {
        let sut = Pos(4, 4);

        assert!(
            Pos(5, 4) == sut.to(Direction::Top(1)),
            "should have moved top"
        );
        assert!(
            Pos(3, 4) == sut.to(Direction::Bottom(1)),
            "should have moved bottom"
        );
        assert!(
            Pos(4, 3) == sut.to(Direction::Left(1)),
            "should have moved left"
        );
        assert!(
            Pos(4, 5) == sut.to(Direction::Right(1)),
            "should have moved right"
        );
    }

    #[test]
    #[should_panic(expected = "")]
    fn to_outside_bounds() {
        let _ = Pos(0, 0).to(Direction::Bottom(1));
    }
}
