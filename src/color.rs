#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Color {
    B,
    W,
}

impl Color {
    #[must_use]
    pub const fn piece_row(self) -> u8 {
        match self {
            Color::B => 7,
            Color::W => 0,
        }
    }

    #[must_use]
    pub const fn pawn_row(self) -> u8 {
        match self {
            Color::B => 6,
            Color::W => 1,
        }
    }

    #[must_use]
    pub const fn flip(self) -> Self {
        match self {
            Color::B => Color::W,
            Color::W => Color::B,
        }
    }
}

#[cfg(test)]
pub const W: Color = Color::W;

#[cfg(test)]
pub const B: Color = Color::B;

#[cfg(test)]
mod test {
    use std::mem;

    use super::*;

    #[test]
    fn piece_row_for_white() {
        assert_eq!(0, W.piece_row());
    }

    #[test]
    fn piece_row_for_black() {
        assert_eq!(7, B.piece_row());
    }

    #[test]
    fn pawn_row_for_white() {
        assert_eq!(1, W.pawn_row());
    }

    #[test]
    fn pawn_row_for_black() {
        assert_eq!(6, B.pawn_row());
    }

    #[test]
    fn opposite() {
        assert_eq!(B, W.flip());
        assert_eq!(W, B.flip());
    }

    #[test]
    fn size() {
        assert_eq!(1, mem::size_of::<Color>());
        assert_eq!(8, mem::size_of::<&Color>());
    }
}
