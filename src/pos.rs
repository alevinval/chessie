use std::fmt::Display;

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
    #[cfg(test)]
    pub const fn new(row: u8, col: u8) -> Self {
        Self(row, col)
    }

    #[must_use]
    pub fn to(self, d: Dir) -> Self {
        let (row, col) = (self.0, self.1);
        match d {
            Dir::Up(n) => (row + n, col),
            Dir::Down(n) => (row - n, col),
            Dir::Left(n) => (row, col - n),
            Dir::Right(n) => (row, col + n),
            Dir::Custom(nr, nc) => (((row as i8) + nr) as u8, ((col as i8) + nc) as u8),
        }
        .into()
    }

    #[must_use]
    pub fn row(self) -> u8 {
        self.0
    }

    #[must_use]
    pub fn col(self) -> u8 {
        self.1
    }

    #[must_use]
    pub fn is_central(self) -> bool {
        self.0 >= 3 && self.1 >= 3 && self.0 <= 4 && self.1 <= 4
    }

    fn assert_bounds(self) -> Self {
        debug_assert!(
            self.0 < 8 && self.1 < 8,
            "position {self} outside of bounds"
        );
        self
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.0, self.1))
    }
}

impl From<(u8, u8)> for Pos {
    fn from(value: (u8, u8)) -> Self {
        Self(value.0, value.1).assert_bounds()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to() {
        let sut = Pos(4, 4);

        assert_eq!(Pos(5, 4), sut.to(Dir::Up(1)));
        assert_eq!(Pos(3, 4), sut.to(Dir::Down(1)));
        assert_eq!(Pos(4, 5), sut.to(Dir::Right(1)));
        assert_eq!(Pos(4, 3), sut.to(Dir::Left(1)));
        assert_eq!(Pos(7, 0), sut.to(Dir::Custom(3, -4)));
    }

    #[test]
    fn is_central() {
        let mut sut: Pos = (4, 4).into();
        assert!(sut.is_central());
        sut = (3, 4).into();
        assert!(sut.is_central());
        sut = (3, 3).into();
        assert!(sut.is_central());
        sut = (4, 3).into();
        assert!(sut.is_central());

        sut = (5, 3).into();
        assert!(!sut.is_central());
        sut = (2, 3).into();
        assert!(!sut.is_central());
        sut = (4, 5).into();
        assert!(!sut.is_central());
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn to_outside_bounds_lower() {
        let _ = Pos(0, 0).to(Dir::Down(1));
    }

    #[test]
    #[should_panic(expected = "position (9,7) outside of bounds")]
    fn to_outside_bounds_upper() {
        let _ = Pos(7, 7).to(Dir::Up(2));
    }

    #[test]
    #[should_panic(expected = "position (8,8) outside of bounds")]
    fn into_outside_bounds() {
        let _: Pos = (8, 8).into();
    }
}
