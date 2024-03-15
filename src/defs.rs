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
    pub fn apply(self, sq: Sq) -> Sq {
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

    use super::*;

    #[test]
    fn dir_apply() {
        let sut = Pos::new(4, 4).sq();

        assert_eq!(Pos::new(5, 4).sq(), Dir::Up.apply(sut));
        assert_eq!(Pos::new(3, 4).sq(), Dir::Down.apply(sut));
        assert_eq!(Pos::new(4, 5).sq(), Dir::Right.apply(sut));
        assert_eq!(Pos::new(4, 3).sq(), Dir::Left.apply(sut));
        assert_eq!(Pos::new(7, 0).sq(), Dir::Custom(3, -4).apply(sut));
        assert_eq!(Pos::new(3, 5).sq(), Dir::Custom(-1, 1).apply(sut));
    }
}
