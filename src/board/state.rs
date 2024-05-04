use crate::{
    color::Color,
    defs::{CastlingTuple, CastlingUpdate},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GameState {
    mover: Color,
    fullmove: usize,

    // Castling rights
    white_left: bool,
    white_right: bool,
    black_left: bool,
    black_right: bool,
}

impl GameState {
    #[must_use]
    pub(crate) const fn fullmove(&self) -> usize {
        self.fullmove
    }

    #[must_use]
    pub(crate) const fn mover(&self) -> Color {
        self.mover
    }

    #[must_use]
    pub(crate) const fn castling(&self, color: Color) -> CastlingTuple {
        match color {
            Color::B => (self.black_left, self.black_right),
            Color::W => (self.white_left, self.white_right),
        }
    }

    pub(super) fn advance(&mut self) {
        if self.mover == Color::B {
            self.fullmove += 1;
        }
        self.mover = self.mover.flip();
    }

    pub(crate) fn set_mover(&mut self, mover: Color) {
        self.mover = mover;
    }

    pub(crate) fn set_fullmove(&mut self, fullmove: usize) {
        self.fullmove = fullmove;
    }

    pub(crate) fn set_castling(&mut self, color: Color, update: CastlingUpdate, value: bool) {
        let (left, right) = match color {
            Color::B => (&mut self.black_left, &mut self.black_right),
            Color::W => (&mut self.white_left, &mut self.white_right),
        };

        match update {
            CastlingUpdate::Left => {
                *left = value;
            }
            CastlingUpdate::Right => {
                *right = value;
            }
            CastlingUpdate::Both => {
                *left = value;
                *right = value;
            }
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            mover: Color::W,
            fullmove: 1,
            white_left: true,
            white_right: true,
            black_left: true,
            black_right: true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn n() {
        let mut sut = GameState::default();
        assert_eq!(1, sut.fullmove());

        sut.advance();
        assert_eq!(1, sut.fullmove());

        sut.advance();
        assert_eq!(2, sut.fullmove());
    }

    #[test]
    fn mover() {
        let mut sut = GameState::default();
        assert_eq!(Color::W, sut.mover());

        sut.advance();
        assert_eq!(Color::B, sut.mover());
    }

    #[test_case(Color::W, CastlingUpdate::Left, false, (false, true))]
    #[test_case(Color::W, CastlingUpdate::Right, false, (true, false))]
    #[test_case(Color::W, CastlingUpdate::Both, false, (false, false))]
    #[test_case(Color::B, CastlingUpdate::Left, false, (false, true))]
    #[test_case(Color::B, CastlingUpdate::Right, false, (true, false))]
    #[test_case(Color::B, CastlingUpdate::Both, false, (false, false))]
    fn castling(color: Color, update: CastlingUpdate, value: bool, expected: (bool, bool)) {
        let mut sut = GameState::default();
        assert_eq!((true, true), sut.castling(color));

        sut.set_castling(color, update, value);
        assert_eq!(expected, sut.castling(color));
    }
}
