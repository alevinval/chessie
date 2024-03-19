pub type Sq = u8;
pub type BitBoard = u64;

pub enum Dir {
    Up(u8),
    Down(u8),
    Right(u8),
    Left(u8),
    Custom(i8, i8),
}

impl Dir {
    #[must_use]
    pub const fn apply(self, sq: Sq) -> Sq {
        match self {
            Dir::Up(n) => sq + 8 * n,
            Dir::Down(n) => sq - 8 * n,
            Dir::Left(n) => sq - n,
            Dir::Right(n) => sq + n,
            Dir::Custom(nr, nc) => (sq as i8 + (8 * nr + nc)) as u8,
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;
    use crate::Pos;

    #[test_case((2, 2), Dir::Up(1), (3, 2))]
    #[test_case((2, 2), Dir::Down(1), (1, 2))]
    #[test_case((2, 2), Dir::Right(1), (2, 3))]
    #[test_case((2, 2), Dir::Left(1), (2, 1))]
    #[test_case((2, 2), Dir::Custom(-2, -2), (0, 0))]
    #[test_case((2, 2), Dir::Custom(2, 3), (4, 5))]
    fn apply<P: Into<Pos>>(input: P, dir: Dir, expected: P) {
        assert_eq!(expected.into().sq(), dir.apply(input.into().sq()));
    }
}
