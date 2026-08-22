use crate::{
    color::Color,
    defs::{CastlingRights, CastlingUpdate},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GameState {
    mover: Color,
    fullmove: usize,
    white_castling: CastlingRights,
    black_castling: CastlingRights,
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
    pub(crate) const fn castling_rights(&self, color: Color) -> CastlingRights {
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

    pub(super) fn backwards(&mut self) {
        if self.mover == Color::W {
            self.fullmove -= 1;
        }
        self.mover = self.mover.flip();
    }

    pub(crate) fn set_mover(&mut self, mover: Color) -> bool {
        if self.mover != mover {
            self.mover = mover;
            true
        } else {
            false
        }
    }

    pub(crate) fn set_fullmove(&mut self, fullmove: usize) {
        self.fullmove = fullmove;
    }

    /// Sets the given side(s) of `color` to `value` and returns which sides
    /// actually changed, as a `(left, right)` tuple.
    pub(crate) fn set_castling(
        &mut self,
        color: Color,
        update: CastlingUpdate,
        value: bool,
    ) -> (bool, bool) {
        let (old_left, old_right) = self.castling_rights(color);
        let (left, right) = match update {
            CastlingUpdate::Left => (value, old_right),
            CastlingUpdate::Right => (old_left, value),
            CastlingUpdate::Both => (value, value),
        };

        match color {
            Color::B => self.black_castling = (left, right),
            Color::W => self.white_castling = (left, right),
        };

        (old_left != left, old_right != right)
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            mover: Color::W,
            fullmove: 1,
            white_castling: (true, true),
            black_castling: (true, true),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn advance_and_backwards() {
        let mut sut = GameState::default();
        let modified = sut.clone();

        sut.advance();
        assert_ne!(modified, sut);

        sut.advance();
        assert_ne!(modified, sut);

        sut.backwards();
        assert_ne!(modified, sut);

        sut.backwards();
        assert_eq!(modified, sut);
    }

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
        assert_eq!((true, true), sut.castling_rights(color));

        sut.set_castling(color, update, value);
        assert_eq!(expected, sut.castling_rights(color));
    }

    #[test]
    fn set_mover_reports_changed() {
        let mut sut = GameState::default();
        assert!(!sut.set_mover(Color::W));
        assert!(sut.set_mover(Color::B));
    }

    #[test]
    fn set_castling_reports_changed_sides() {
        let mut sut = GameState::default();
        assert_eq!((false, false), sut.set_castling(Color::W, CastlingUpdate::Both, true));
        assert_eq!((true, false), sut.set_castling(Color::W, CastlingUpdate::Left, false));
        assert_eq!((true, true), sut.set_castling(Color::B, CastlingUpdate::Both, false));
    }
}
