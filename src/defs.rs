pub(crate) type Sq = u8;

pub(crate) type BitBoard = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Castling {
    None,
    Some(bool, bool),
}

pub(crate) enum Dir {
    Up(u8),
    Down(u8),
    Right(u8),
    Left(u8),
    Custom(i8, i8),
}

impl Dir {
    #[must_use]
    pub(crate) const fn apply(self, sq: Sq) -> Sq {
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

    #[test_case(10, Dir::Up(1), 18)]
    #[test_case(10, Dir::Down(1), 2)]
    #[test_case(10, Dir::Right(1), 11)]
    #[test_case(10, Dir::Left(1), 9)]
    #[test_case(20, Dir::Custom(-2, -2), 2)]
    #[test_case(10, Dir::Custom(2, 3), 29)]
    fn apply(input: Sq, dir: Dir, expected: Sq) {
        assert_eq!(expected, dir.apply(input));
    }
}
