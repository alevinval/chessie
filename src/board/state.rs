use crate::{color::Color, defs::Castling};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GameState {
    mover: Color,
    white_castling: Castling,
    black_castling: Castling,
    n: usize,
}

impl GameState {
    #[must_use]
    pub(crate) const fn n(&self) -> usize {
        self.n
    }

    #[must_use]
    pub(crate) const fn mover(&self) -> Color {
        self.mover
    }

    #[must_use]
    pub(crate) const fn castling(&self, color: Color) -> Castling {
        match color {
            Color::B => self.black_castling,
            Color::W => self.white_castling,
        }
    }

    pub(super) fn advance(&mut self) {
        self.mover = self.mover.flip();
        self.n += 1;
    }

    pub(crate) fn set_castled(&mut self) {
        *self.castling_for(self.mover) = Castling::None;
    }

    pub(crate) fn update_castling(&mut self, left: bool, right: bool) {
        *self.castling_for(self.mover) = Castling::Some(left, right);
    }

    fn castling_for(&mut self, color: Color) -> &mut Castling {
        match color {
            Color::B => &mut self.black_castling,
            Color::W => &mut self.white_castling,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            mover: Color::W,
            white_castling: Castling::default(),
            black_castling: Castling::default(),
            n: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn n() {
        let mut sut = GameState::default();
        assert_eq!(0, sut.n());

        sut.advance();
        assert_eq!(1, sut.n());
    }

    #[test]
    fn mover() {
        let mut sut = GameState::default();
        assert_eq!(Color::W, sut.mover());

        sut.advance();
        assert_eq!(Color::B, sut.mover());
    }

    #[test]
    fn castling() {
        let sut = GameState::default();
        assert_eq!(Castling::Some(true, true), sut.castling(Color::W));
        assert_eq!(Castling::Some(true, true), sut.castling(Color::B));
    }

    #[test]
    fn set_castled_white() {
        let mut sut = GameState::default();
        sut.set_castled();
        assert_eq!(Castling::None, sut.castling(Color::W));
    }

    #[test]
    fn set_castled_black() {
        let mut sut = GameState::default();
        sut.advance();
        sut.set_castled();
        assert_eq!(Castling::None, sut.castling(Color::B));
    }

    #[test]
    fn set_castling() {
        let mut sut = GameState::default();
        sut.update_castling(false, true);
        assert_eq!(Castling::Some(false, true), sut.castling(Color::W));
    }
}
