#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Up(u8),
    Down(u8),
    Right(u8),
    Left(u8),
    Custom(i8, i8),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Pos(u8, u8);

impl Pos {
    pub const fn new(row: u8, col: u8) -> Self {
        Self(row, col)
    }

    pub fn to(self, d: Dir) -> Self {
        let (row, col) = (self.0, self.1);
        let pos = match d {
            Dir::Up(n) => (row + n, col),
            Dir::Down(n) => (row - n, col),
            Dir::Left(n) => (row, col - n),
            Dir::Right(n) => (row, col + n),
            Dir::Custom(nr, nc) => (((row as i8) + nr) as u8, ((col as i8) + nc) as u8),
        };

        self.assert_bounds();
        Self(pos.0, pos.1)
    }

    pub fn row(self) -> u8 {
        self.0
    }

    pub fn col(self) -> u8 {
        self.1
    }

    pub fn is_central(self) -> bool {
        self.0 >= 3 && self.1 >= 3 && self.0 <= 4 && self.1 <= 4
    }

    fn assert_bounds(self) -> Self {
        debug_assert!(
            self.0 < 8 && self.1 < 8,
            "position outside of bounds ({self:?})"
        );
        self
    }
}

impl From<(u8, u8)> for Pos {
    fn from(value: (u8, u8)) -> Self {
        Pos(value.0, value.1).assert_bounds()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to() {
        let sut = Pos(4, 4);

        assert!(Pos(5, 4) == sut.to(Dir::Up(1)), "should have moved top");
        assert!(
            Pos(3, 4) == sut.to(Dir::Down(1)),
            "should have moved bottom"
        );
        assert!(Pos(4, 5) == sut.to(Dir::Right(1)), "should have moved left");
        assert!(Pos(4, 3) == sut.to(Dir::Left(1)), "should have moved right");
    }

    #[test]
    #[should_panic(expected = "")]
    fn to_outside_bounds() {
        let _ = Pos(0, 0).to(Dir::Down(1));
    }
}
