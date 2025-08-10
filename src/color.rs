use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Color {
    B,
    W,
}

impl Color {
    #[must_use]
    pub(crate) const fn piece_row(self) -> u8 {
        match self {
            Color::B => 7,
            Color::W => 0,
        }
    }

    #[must_use]
    pub(crate) const fn pawn_row(self) -> u8 {
        match self {
            Color::B => 6,
            Color::W => 1,
        }
    }

    #[must_use]
    pub(crate) const fn flip(self) -> Self {
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
    use test_case::test_case;

    use super::*;

    #[test_case(Color::W, 0)]
    #[test_case(Color::B, 7)]
    fn piece_row(color: Color, expected: u8) {
        assert_eq!(expected, color.piece_row());
    }

    #[test_case(Color::W, 1)]
    #[test_case(Color::B, 6)]
    fn pawn_row(color: Color, expected: u8) {
        assert_eq!(expected, color.pawn_row());
    }

    #[test]
    fn flip() {
        assert_eq!(Color::B, Color::W.flip());
        assert_eq!(Color::W, Color::B.flip());
    }
}
