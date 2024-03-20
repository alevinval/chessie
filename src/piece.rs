use super::Color;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub type Idx = usize;

impl Piece {
    pub const P: Idx = 0;
    pub const N: Idx = 1;
    pub const B: Idx = 2;
    pub const R: Idx = 3;
    pub const Q: Idx = 4;
    pub const K: Idx = 5;

    #[must_use]
    pub const fn as_str(self, c: Color) -> &'static str {
        match self {
            Self::Bishop => match c {
                Color::B => "♝",
                Color::W => "♗",
            },
            Self::King => match c {
                Color::B => "♚",
                Color::W => "♔",
            },
            Self::Knight => match c {
                Color::B => "♞",
                Color::W => "♘",
            },
            Self::Pawn => match c {
                Color::B => "♟",
                Color::W => "♙",
            },
            Self::Queen => match c {
                Color::B => "♛",
                Color::W => "♕",
            },
            Self::Rook => match c {
                Color::B => "♜",
                Color::W => "♖",
            },
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case(Piece::Pawn, Color::W, "♙")]
    #[test_case(Piece::Pawn, Color::B, "♟")]
    #[test_case(Piece::Bishop, Color::W, "♗")]
    #[test_case(Piece::Bishop, Color::B, "♝")]
    #[test_case(Piece::Knight, Color::W, "♘")]
    #[test_case(Piece::Knight, Color::B, "♞")]
    #[test_case(Piece::Rook, Color::W, "♖")]
    #[test_case(Piece::Rook, Color::B, "♜")]
    #[test_case(Piece::Queen, Color::W, "♕")]
    #[test_case(Piece::Queen, Color::B, "♛")]
    #[test_case(Piece::King, Color::W, "♔")]
    #[test_case(Piece::King, Color::B, "♚")]
    fn as_str(piece: Piece, color: Color, expected: &str) {
        assert_eq!(expected, piece.as_str(color));
    }
}
