use crate::{
    bits,
    defs::{BitBoard, Sq},
    moves::{self, Generator, Move},
    piece::Piece,
    Color,
};

pub(crate) use self::state::GameState;

mod state;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Board {
    white: [BitBoard; 6],
    black: [BitBoard; 6],
    white_side: BitBoard,
    black_side: BitBoard,
    occupancy: BitBoard,
    state: GameState,
}

impl Board {
    #[must_use]
    pub(crate) const fn state(&self) -> &GameState {
        &self.state
    }

    #[must_use]
    pub(crate) const fn occupancy_side(&self, color: Color) -> BitBoard {
        match color {
            Color::B => self.black_side,
            Color::W => self.white_side,
        }
    }

    #[must_use]
    pub(crate) const fn occupancy(&self) -> BitBoard {
        self.occupancy
    }

    #[must_use]
    pub(crate) fn state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }

    pub(crate) fn add(&mut self, color: Color, piece: Piece, sq: Sq) {
        match color {
            Color::B => bits::set(&mut self.black[piece.idx()], sq),
            Color::W => bits::set(&mut self.white[piece.idx()], sq),
        }
    }

    #[must_use]
    pub(crate) const fn get(&self, color: Color, piece: Piece) -> BitBoard {
        match color {
            Color::B => self.black[piece.idx()],
            Color::W => self.white[piece.idx()],
        }
    }

    pub(crate) fn pieces(&self, color: Color) -> impl Iterator<Item = (Piece, BitBoard)> + '_ {
        match color {
            Color::B => self.black,
            Color::W => self.white,
        }
        .into_iter()
        .enumerate()
        .map(|(i, bb)| (Piece::from_idx(i), bb))
    }

    #[must_use]
    pub(crate) fn at(&self, sq: Sq) -> Option<(Color, Piece, BitBoard)> {
        self.white
            .into_iter()
            .position(|bb| bits::has_piece(bb, sq))
            .map(|i| (Color::W, Piece::from_idx(i), self.white[i]))
            .or_else(|| {
                self.black
                    .into_iter()
                    .position(|bb| bits::has_piece(bb, sq))
                    .map(|i| (Color::B, Piece::from_idx(i), self.black[i]))
            })
    }

    #[must_use]
    pub(crate) fn at_mut(&mut self, sq: Sq) -> Option<(Color, Piece, &mut BitBoard)> {
        self.white
            .into_iter()
            .position(|bb| bits::has_piece(bb, sq))
            .map(|i| (Color::W, Piece::from_idx(i), &mut self.white[i]))
            .or_else(|| {
                self.black
                    .into_iter()
                    .position(|bb| bits::has_piece(bb, sq))
                    .map(|i| (Color::B, Piece::from_idx(i), &mut self.black[i]))
            })
    }

    pub(crate) fn advance(&mut self) {
        self.calculate_occupancies();
        self.state.advance();
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
    pub(crate) fn count_pieces(&self) -> usize {
        self.pieces(Color::W)
            .chain(self.pieces(Color::B))
            .filter(|(p, _)| *p != Piece::Pawn)
            .map(|(_, bb)| bits::count(bb))
            .sum()
    }

    #[must_use]
    pub(crate) fn in_check(&self, color: Color) -> bool {
        if let Some(pos) = bits::first_pos(self.get(color, Piece::King)) {
            let moves = self.pseudo_movements(color.flip());
            moves::is_attacked(&moves, pos)
        } else {
            true
        }
    }

    pub(crate) fn clear(&mut self) {
        self.white.iter_mut().for_each(|bb| *bb = 0);
        self.black.iter_mut().for_each(|bb| *bb = 0);
    }

    pub(crate) fn calculate_occupancies(&mut self) {
        self.white_side = collapse(self.white);
        self.black_side = collapse(self.black);
        self.occupancy = self.white_side | self.black_side;
    }

    fn generate_movements(&self, color: Color, legal_only: bool) -> Vec<Move> {
        self.pieces(color)
            .flat_map(|(_, bb)| bits::pos(bb))
            .flat_map(|p| Generator::from_board(self, p, legal_only).generate())
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn black(&mut self, idx: crate::piece::Idx) -> &mut BitBoard {
        &mut self.black[idx]
    }

    #[cfg(test)]
    pub(crate) fn white(&mut self, idx: crate::piece::Idx) -> &mut BitBoard {
        &mut self.white[idx]
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Self {
            white: init_pieces(Color::W),
            black: init_pieces(Color::B),
            state: GameState::default(),
            white_side: 0,
            black_side: 0,
            occupancy: 0,
        };
        board.calculate_occupancies();
        board
    }
}

const fn init_pieces(color: Color) -> [BitBoard; 6] {
    [
        bits::init(Piece::Pawn, color),
        bits::init(Piece::Knight, color),
        bits::init(Piece::Bishop, color),
        bits::init(Piece::Rook, color),
        bits::init(Piece::Queen, color),
        bits::init(Piece::King, color),
    ]
}

const fn collapse(bbs: [BitBoard; 6]) -> BitBoard {
    bbs[0] | bbs[1] | bbs[2] | bbs[3] | bbs[4] | bbs[5]
}

#[cfg(test)]
mod test {

    use std::mem;

    use crate::sq;

    use super::*;

    #[test]
    fn at_white_king() {
        let sut = Board::default();
        let king = sut.at(sq!(0, 4));
        assert!(king.is_some());

        if let Some((color, piece, _bb)) = king {
            assert_eq!(Color::W, color);
            assert_eq!(Piece::King, piece);
        }
    }

    #[test]
    fn at_black_king() {
        let sut = Board::default();
        let king = sut.at(sq!(7, 4));

        assert!(king.is_some());

        if let Some((color, piece, _bb)) = king {
            assert_eq!(Color::B, color);
            assert_eq!(Piece::King, piece);
        }
    }

    #[test]
    fn mut_at_white() {
        let pos = sq!(0, 0);

        assert_eq!(Board::default().at(pos).unwrap().1, Board::default().at_mut(pos).unwrap().1);
    }

    #[test]
    fn mut_at_black() {
        let pos = sq!(7, 7);

        assert_eq!(Board::default().at(pos).unwrap().1, Board::default().at_mut(pos).unwrap().1);
    }

    #[test]
    fn count_pieces() {
        let sut = Board::default();
        assert_eq!(16, sut.count_pieces());
    }

    #[test]
    fn size() {
        assert_eq!(136, mem::size_of::<Board>());
        assert_eq!(8, mem::size_of::<&Board>());
    }
}
