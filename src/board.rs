use crate::pieces::{BitBoard, Color, Pieces};
use crate::pos::Pos;

#[derive(Debug, Clone)]
pub struct Board {
    white: Pieces,
    black: Pieces,
}

impl Board {
    pub fn pieces(&self, color: Color) -> &Pieces {
        match color {
            Color::Black => &self.black,
            Color::White => &self.white,
        }
    }

    pub fn at<P: Into<Pos>>(&self, pos: P) -> Option<&BitBoard> {
        let pos = pos.into();
        self.white.at(pos).or_else(|| self.black.at(pos))
    }

    pub fn at_mut<P: Into<Pos>>(&mut self, pos: P) -> Option<&mut BitBoard> {
        let pos = pos.into();
        self.white.at_mut(pos).or_else(|| self.black.at_mut(pos))
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white: Pieces::new(Color::White),
            black: Pieces::new(Color::Black),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::pieces::Piece;

    use super::*;

    #[test]
    fn pieces() {
        let sut = Board::default();
        assert_eq!(&sut.white, sut.pieces(Color::White));
        assert_eq!(&sut.black, sut.pieces(Color::Black));
    }

    #[test]
    fn at_white_king() {
        let sut = Board::default();
        let king = sut.at((0, 4));

        assert!(king.is_some());

        if let Some(king) = king {
            assert_eq!(Color::White, king.color());
            assert_eq!(Piece::King(Color::White, false), king.piece());
        }
    }

    #[test]
    fn at_black_king() {
        let sut = Board::default();
        let king = sut.at((7, 4));

        assert!(king.is_some());

        if let Some(king) = king {
            assert_eq!(Color::Black, king.color());
            assert_eq!(Piece::King(Color::Black, false), king.piece());
        }
    }

    #[test]
    fn mut_at_white() {
        let pos = (0, 0);

        assert_eq!(
            Board::default().at(pos).unwrap(),
            Board::default().at_mut(pos).unwrap()
        );
    }

    #[test]
    fn mut_at_black() {
        let pos = (7, 7);

        assert_eq!(
            Board::default().at(pos).unwrap(),
            Board::default().at_mut(pos).unwrap()
        );
    }
}
