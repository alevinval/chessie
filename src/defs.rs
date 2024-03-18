pub type Sq = usize;
pub type BitBoard = u64;

pub enum Dir {
    Up,
    Down,
    Right,
    Left,
    Custom(i8, i8),
}

impl Dir {
    #[must_use]
    pub const fn apply(self, sq: Sq) -> Sq {
        match self {
            Dir::Up => sq + 8,
            Dir::Down => sq - 8,
            Dir::Left => sq - 1,
            Dir::Right => sq + 1,
            Dir::Custom(nr, nc) => (sq as i8 + (8 * nr + nc)) as usize,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Pos;
    use test_case::test_case;

    use super::*;

    #[test_case((2, 2), Dir::Up, (3, 2))]
    #[test_case((2, 2), Dir::Down, (1, 2))]
    #[test_case((2, 2), Dir::Right, (2, 3))]
    #[test_case((2, 2), Dir::Left, (2, 1))]
    #[test_case((2, 2), Dir::Custom(-2, -2), (0, 0))]
    #[test_case((2, 2), Dir::Custom(2, 3), (4, 5))]
    fn apply<P: Into<Pos>>(input: P, dir: Dir, expected: P) {
        assert_eq!(expected.into().sq(), dir.apply(input.into().sq()));
    }
}
