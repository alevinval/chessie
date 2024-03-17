use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Color {
    B,
    W,
}

impl Color {
    #[must_use]
    pub const fn piece_row(self) -> usize {
        match self {
            Color::B => 7,
            Color::W => 0,
        }
    }

    #[must_use]
    pub const fn pawn_row(self) -> usize {
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

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Color::B => "Black",
            Color::W => "White",
        })
    }
}

#[cfg(test)]
mod test {
    use std::mem;

    use super::*;

    #[test]
    fn piece_row_for_white() {
        assert_eq!(0, Color::W.piece_row());
    }

    #[test]
    fn piece_row_for_black() {
        assert_eq!(7, Color::B.piece_row());
    }

    #[test]
    fn pawn_row_for_white() {
        assert_eq!(1, Color::W.pawn_row());
    }

    #[test]
    fn pawn_row_for_black() {
        assert_eq!(6, Color::B.pawn_row());
    }

    #[test]
    fn flip() {
        assert_eq!(Color::B, Color::W.flip());
        assert_eq!(Color::W, Color::B.flip());
    }

    #[test]
    fn size() {
        assert_eq!(1, mem::size_of::<Color>());
        assert_eq!(8, mem::size_of::<&Color>());
    }
}
