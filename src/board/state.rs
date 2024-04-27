use crate::{color::Color, defs::Castling};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GameState {
    mover: Color,
    white_castling: Castling,
    black_castling: Castling,
    fullmove: usize,
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
    pub(crate) const fn castling(&self, color: Color) -> Castling {
        match color {
            Color::B => self.black_castling,
            Color::W => self.white_castling,
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

    pub(crate) fn set_castled(&mut self) {
        *self.castling_for(self.mover) = Castling::None;
    }

    pub(crate) fn update_castling(&mut self, left: bool, right: bool) {
        *self.castling_for(self.mover) = Castling::Some { left, right };
    }

    pub(crate) fn castling_for(&mut self, color: Color) -> &mut Castling {
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
            fullmove: 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn castling() {
        let sut = GameState::default();
        assert_eq!(Castling::Some { left: true, right: true }, sut.castling(Color::W));
        assert_eq!(Castling::Some { left: true, right: true }, sut.castling(Color::B));
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
        assert_eq!(Castling::Some { left: false, right: true }, sut.castling(Color::W));
    }
}
