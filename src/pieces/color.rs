#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn piece_row(&self) -> usize {
        match self {
            Color::Black => 7,
            Color::White => 0,
        }
    }

    pub fn pawn_row(&self) -> usize {
        match self {
            Color::Black => 6,
            Color::White => 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn piece_row_for_white() {
        assert_eq!(0, Color::White.piece_row());
    }

    #[test]
    fn piece_row_for_black() {
        assert_eq!(7, Color::Black.piece_row());
    }

    #[test]
    fn pawn_row_for_white() {
        assert_eq!(1, Color::White.pawn_row());
    }

    #[test]
    fn pawn_row_for_black() {
        assert_eq!(6, Color::Black.pawn_row());
    }
}
