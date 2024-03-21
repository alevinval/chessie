use either::Either;

use crate::{defs, piece::Piece, pos::Pos};

use super::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BitBoard {
    bb: defs::BitBoard,
}

impl BitBoard {
    #[must_use]
    pub const fn new(bb: defs::BitBoard) -> Self {
        Self { bb }
    }

    #[must_use]
    pub const fn init(piece: Piece, color: Color) -> Self {
        let mut bb = match piece {
            Piece::Pawn => 0b1111_1111,
            Piece::Rook => 0b1000_0001,
            Piece::Knight => 0b0100_0010,
            Piece::Bishop => 0b0010_0100,
            Piece::Queen => 0b0000_1000,
            Piece::King => 0b0001_0000,
        };
        bb <<= 8 * if matches!(piece, Piece::Pawn) { color.pawn_row() } else { color.piece_row() };

        Self { bb }
    }

    #[must_use]
    pub const fn count(self) -> usize {
        let mut bb = self.bb;
        let mut count = 0;
        while bb != 0 {
            count += 1;
            bb &= bb - 1;
        }
        count
    }

    #[must_use]
    pub const fn value(self) -> defs::BitBoard {
        self.bb
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.bb == 0
    }

    #[must_use]
    pub fn has_piece<P: Into<defs::BitBoard>>(self, pos: P) -> bool {
        self.bb & pos.into() != 0
    }

    pub fn slide<P: Into<defs::BitBoard>>(&mut self, from: P, to: P) {
        self.bb ^= from.into() | to.into();
    }

    pub fn set<P: Into<defs::BitBoard>>(&mut self, pos: P) {
        self.bb |= pos.into();
    }

    pub fn unset<P: Into<defs::BitBoard>>(&mut self, pos: P) {
        self.bb &= !pos.into();
    }

    pub fn pos(self, color: Color) -> impl Iterator<Item = Pos> {
        let rows = match color {
            Color::B => Either::Left((0..8).rev()),
            Color::W => Either::Right(0..8),
        };
        rows.flat_map(move |row| {
            let ro = row * 8;
            (0..8).filter_map(move |col| {
                if self.bb & (1 << (ro + col)) > 0 {
                    Some((row, col).into())
                } else {
                    None
                }
            })
        })
        .take(self.count())
    }
}

impl From<Pos> for u64 {
    fn from(pos: Pos) -> Self {
        pos.bb()
    }
}

#[cfg(test)]
mod test {

    use crate::print_bitboard;

    use super::*;

    static ORIGIN: Pos = Pos::new(0, 0);
    static TARGET: Pos = Pos::new(3, 3);

    #[test]
    fn count() {
        let sut = BitBoard::new(0x800c00000a007000);
        print_bitboard(sut);
        assert_eq!(8, sut.count());
    }

    #[test]
    fn pos() {
        let sut = BitBoard::new(0x800c00000a007000);
        let positions: Vec<_> = sut.pos(Color::W).collect();
        let expected: Vec<_> =
            vec![12, 13, 14, 25, 27, 50, 51, 63].into_iter().map(|sq| Pos::from(sq)).collect();

        assert_eq!(expected, positions);
    }

    #[test]
    fn has_piece() {
        let sut = BitBoard::new(0);
        assert!(!sut.has_piece(ORIGIN), "{ORIGIN:?} should not have piece");

        let sut = BitBoard::new(1);
        assert!(sut.has_piece(ORIGIN), "{ORIGIN:?} should have piece");

        let sut = BitBoard::new(0);
        assert!(!sut.has_piece(TARGET), "{TARGET:?} should not have piece");

        let sut = BitBoard::new(TARGET.into());
        assert!(sut.has_piece(TARGET), "{TARGET:?} should have piece");
    }

    #[test]
    fn slide() {
        let from = Pos::new(1, 1);
        let to = Pos::new(2, 2);

        let mut sut = BitBoard::init(Piece::Pawn, Color::W);
        assert!(!sut.has_piece(to));

        sut.slide(from, to);
        assert!(sut.has_piece(to));

        assert!(!sut.has_piece(from));
    }

    #[test]
    fn unset() {
        let pos = Pos::new(1, 1);
        let mut sut = BitBoard::init(Piece::Pawn, Color::W);
        assert!(sut.has_piece(pos));

        sut.unset(pos);
        assert!(!sut.has_piece(pos));
    }
}
