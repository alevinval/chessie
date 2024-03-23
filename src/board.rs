use crate::{
    bits::Bits,
    defs::BitBoard,
    moves::{Move, MoveGen},
    piece::Piece,
    pos::Pos,
    Color,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Castling {
    None,
    Some(bool, bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Board {
    mover: Color,
    white: [BitBoard; 6],
    black: [BitBoard; 6],
    white_castling: Castling,
    black_castling: Castling,
    n: usize,
}

impl Board {
    #[must_use]
    pub(crate) const fn mover(&self) -> Color {
        self.mover
    }

    #[must_use]
    pub(crate) const fn n(&self) -> usize {
        self.n
    }

    #[must_use]
    pub(crate) const fn castling(&self, color: Color) -> Castling {
        match color {
            Color::B => self.black_castling,
            Color::W => self.white_castling,
        }
    }

    pub(crate) fn add_piece(&mut self, pos: Pos, piece: Piece) {
        match self.mover {
            Color::B => Bits::set(&mut self.black[piece.idx()], pos),
            Color::W => Bits::set(&mut self.white[piece.idx()], pos),
        }
    }

    pub(crate) fn set_castling(&mut self, color: Color, rights: Castling) {
        match color {
            Color::B => self.black_castling = rights,
            Color::W => self.white_castling = rights,
        }
    }

    #[must_use]
    pub(crate) fn get_piece(&self, color: Color, piece: Piece) -> BitBoard {
        match color {
            Color::B => self.black[piece.idx()],
            Color::W => self.white[piece.idx()],
        }
    }

    pub(crate) fn pieces_iter(&self, color: Color) -> impl Iterator<Item = (Piece, BitBoard)> + '_ {
        match color {
            Color::B => self.black,
            Color::W => self.white,
        }
        .into_iter()
        .enumerate()
        .map(|(i, bb)| (Piece::from_idx(i), bb))
    }

    pub(crate) fn at<P: Into<Pos>>(&self, pos: P) -> Option<(Color, Piece, BitBoard)> {
        let pos = pos.into();
        self.white
            .into_iter()
            .position(|bb| Bits::has_piece(bb, pos))
            .map(|i| (Color::W, Piece::from_idx(i), self.white[i]))
            .or_else(|| {
                self.black
                    .into_iter()
                    .position(|bb| Bits::has_piece(bb, pos))
                    .map(|i| (Color::B, Piece::from_idx(i), self.black[i]))
            })
    }

    pub(crate) fn at_mut<P: Into<Pos>>(&mut self, pos: P) -> Option<(Color, Piece, &mut BitBoard)> {
        let pos = pos.into();
        self.white
            .into_iter()
            .position(|bb| Bits::has_piece(bb, pos))
            .map(|i| (Color::W, Piece::from_idx(i), &mut self.white[i]))
            .or_else(|| {
                self.black
                    .into_iter()
                    .position(|bb| Bits::has_piece(bb, pos))
                    .map(|i| (Color::B, Piece::from_idx(i), &mut self.black[i]))
            })
    }

    pub(crate) fn next_turn(&mut self) {
        self.mover = self.mover.flip();
        self.n += 1;
    }

    #[must_use]
    pub(crate) fn movements(&self, color: Color) -> Vec<Move> {
        self.generate_movements(color, true)
    }

    #[must_use]
    pub(crate) fn pseudo_movements(&self, color: Color) -> Vec<Move> {
        self.generate_movements(color, false)
    }

    #[must_use]
    pub(crate) fn piece_count(&self) -> usize {
        let w: usize = self
            .pieces_iter(Color::W)
            .filter(|(p, _)| *p != Piece::Pawn)
            .map(|(_, bb)| Bits::count(bb))
            .sum();

        let b: usize = self
            .pieces_iter(Color::B)
            .filter(|(p, _)| *p != Piece::Pawn)
            .map(|(_, bb)| Bits::count(bb))
            .sum();

        w + b
    }

    pub(crate) fn in_check(&self, color: Color) -> bool {
        if let Some(pos) = Bits::first_pos(self.get_piece(color, Piece::King)) {
            self.pseudo_movements(color.flip()).iter().any(|m| m.to() == pos)
        } else {
            true
        }
    }

    pub(crate) fn clear(&mut self) {
        self.white.iter_mut().for_each(|bb| *bb = 0);
        self.black.iter_mut().for_each(|bb| *bb = 0);
        self.white_castling = Castling::None;
        self.black_castling = Castling::None;
    }

    fn generate_movements(&self, color: Color, legal_only: bool) -> Vec<Move> {
        self.pieces_iter(color)
            .flat_map(|(_, bb)| Bits::pos(bb))
            .flat_map(|p| MoveGen::new(self, p).generate(legal_only))
            .collect()
    }

    fn gen_pieces(color: Color) -> [BitBoard; 6] {
        [
            Bits::init(Piece::Pawn, color),
            Bits::init(Piece::Knight, color),
            Bits::init(Piece::Bishop, color),
            Bits::init(Piece::Rook, color),
            Bits::init(Piece::Queen, color),
            Bits::init(Piece::King, color),
        ]
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            mover: Color::W,
            white: Board::gen_pieces(Color::W),
            black: Board::gen_pieces(Color::B),
            white_castling: Castling::Some(true, true),
            black_castling: Castling::Some(true, true),
            n: 0,
        }
    }
}

#[cfg(test)]
mod test {

    use std::mem;

    use super::*;

    #[test]
    fn mover() {
        let sut = Board::default();
        assert_eq!(Color::W, sut.mover());
    }

    #[test]
    fn at_white_king() {
        let sut = Board::default();
        let king = sut.at((0, 4));
        assert!(king.is_some());

        if let Some((color, piece, _bb)) = king {
            assert_eq!(Color::W, color);
            assert_eq!(Piece::King, piece);
        }
    }

    #[test]
    fn at_black_king() {
        let sut = Board::default();
        let king = sut.at((7, 4));

        assert!(king.is_some());

        if let Some((color, piece, _bb)) = king {
            assert_eq!(Color::B, color);
            assert_eq!(Piece::King, piece);
        }
    }

    #[test]
    fn mut_at_white() {
        let pos = (0, 0);

        assert_eq!(Board::default().at(pos).unwrap().2, *Board::default().at_mut(pos).unwrap().2);
    }

    #[test]
    fn mut_at_black() {
        let pos = (7, 7);

        assert_eq!(Board::default().at(pos).unwrap().2, *Board::default().at_mut(pos).unwrap().2);
    }

    #[test]
    fn piece_count() {
        let sut = Board::default();
        assert_eq!(16, sut.piece_count());
    }

    #[test]
    fn size() {
        assert_eq!(112, mem::size_of::<Board>());
        assert_eq!(8, mem::size_of::<&Board>());
    }
}
