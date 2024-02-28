use crate::pos::Pos;

use super::{Color, Piece};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitBoard {
    piece: Piece,
    value: u64,
}

impl BitBoard {
    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn color(&self) -> Color {
        self.piece.color()
    }

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    pub fn has_piece<P: Into<Pos>>(&self, pos: P) -> bool {
        rshiftpos(self.value, pos.into()) == 1
    }

    pub fn set<P: Into<u64>>(&mut self, pos: P) {
        self.value |= pos.into();
    }

    pub fn unset<P: Into<u64>>(&mut self, pos: P) {
        self.value &= !pos.into();
    }

    pub fn update_piece(&mut self, piece: Piece) {
        self.piece = piece;
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        u64::to_le_bytes(self.value)
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..8).flat_map(move |row| {
            let ro = row * 8;
            (0..8).filter_map(move |col| {
                if self.value >> (ro + col) & 1 == 1 {
                    Some((row, col).into())
                } else {
                    None
                }
            })
        })
    }
}

fn rshiftpos(value: u64, pos: Pos) -> u64 {
    let other: u64 = pos.into();
    (value & other) >> (pos.row() * 8 + pos.col())
}

impl From<Pos> for u64 {
    fn from(pos: Pos) -> Self {
        1 << (pos.row() * 8 + pos.col())
    }
}

impl From<Piece> for BitBoard {
    fn from(piece: Piece) -> Self {
        let color = piece.color();
        let mut value = match piece {
            Piece::Pawn(_) => 0b11111111,
            Piece::Rook(_, _, _) => 0b10000001,
            Piece::Knight(_) => 0b01000010,
            Piece::Bishop(_) => 0b00100100,
            Piece::Queen(_) => 0b00001000,
            Piece::King(_, _) => 0b00010000,
        };
        value <<= 8 * if piece.is_pawn() {
            color.pawn_row()
        } else {
            color.piece_row()
        };
        Self { piece, value }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    static ORIGIN: Pos = Pos::new(0, 0);
    static TARGET: Pos = Pos::new(3, 3);

    #[test]
    fn has_piece() {
        let sut = BitBoard {
            piece: Piece::Pawn(Color::White),
            value: 0,
        };
        assert!(!sut.has_piece(ORIGIN), "{ORIGIN:?} should not have piece");

        let sut = BitBoard {
            piece: Piece::Pawn(Color::White),
            value: 1,
        };
        assert!(sut.has_piece(ORIGIN), "{ORIGIN:?} should have piece");

        let sut = BitBoard {
            piece: Piece::Pawn(Color::White),
            value: 0,
        };
        assert!(!sut.has_piece(TARGET), "{TARGET:?} should not have piece");

        let sut = BitBoard {
            piece: Piece::Pawn(Color::White),
            value: TARGET.into(),
        };
        assert!(sut.has_piece(TARGET), "{TARGET:?} should have piece");
    }

    #[test]
    fn set_and_unset_piece() {
        let pos = Pos::from((3, 3));
        let mut sut: BitBoard = Piece::Pawn(Color::White).into();
        assert!(!sut.has_piece(pos));

        sut.set(pos);
        assert!(sut.has_piece(pos));

        sut.unset(pos);
        assert!(!sut.has_piece(pos));
    }

    #[test]
    fn to_le_bytes() {
        let sut = BitBoard {
            piece: Piece::Pawn(Color::White),
            value: u64::MAX,
        };
        let actual = sut.to_le_bytes();
        assert!(8 == actual.len());
        assert!(actual.iter().all(|n| *n == 255), "should all be max u8");
    }
}
