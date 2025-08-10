use crate::{
    Color, bits,
    defs::{BitBoard, CastlingUpdate, Sq},
    moves::{self, Generator, Move},
    piece::Piece,
    zobrist::ZobristHash,
};

pub(crate) use self::state::GameState;

mod state;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    white: [BitBoard; 6],
    black: [BitBoard; 6],
    white_side: BitBoard,
    black_side: BitBoard,
    occupancy: BitBoard,
    state: GameState,
    hash: ZobristHash,
}

impl Board {
    pub(crate) fn empty() -> Self {
        let mut board = Self::default();
        board.white.iter_mut().for_each(|bb| *bb = 0);
        board.black.iter_mut().for_each(|bb| *bb = 0);
        board.calculate_occupancies();
        board.recompute_hash();
        board
    }

    #[must_use]
    pub(crate) const fn state(&self) -> &GameState {
        &self.state
    }

    #[must_use]
    pub(crate) const fn hash(&self) -> ZobristHash {
        self.hash
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

    pub(crate) fn add(&mut self, color: Color, piece: Piece, sq: Sq) {
        match color {
            Color::B => bits::set(&mut self.black[piece.idx()], sq),
            Color::W => bits::set(&mut self.white[piece.idx()], sq),
        }
        self.hash.flip_piece(color, piece, sq);
    }

    pub(crate) fn slide(&mut self, from: Sq, to: Sq) {
        let (color, piece, bb) =
            self.at_mut(from).unwrap_or_else(|| unreachable!("slide from an empty square"));
        bits::slide(bb, from, to);
        self.hash.flip_piece(color, piece, from);
        self.hash.flip_piece(color, piece, to);
    }

    pub(crate) fn clear(&mut self, sq: Sq) {
        if let Some((color, piece, bb)) = self.at_mut(sq) {
            bits::unset(bb, sq);
            self.hash.flip_piece(color, piece, sq);
        }
    }

    pub(crate) fn disable_castling(&mut self, color: Color, update: CastlingUpdate) {
        self.set_castling(color, update, false);
    }

    pub(crate) fn enable_castling(&mut self, color: Color, update: CastlingUpdate) {
        self.set_castling(color, update, true);
    }

    fn set_castling(&mut self, color: Color, update: CastlingUpdate, value: bool) {
        let (left_changed, right_changed) = self.state.set_castling(color, update, value);
        if left_changed {
            self.hash.flip_castling(color, false);
        }
        if right_changed {
            self.hash.flip_castling(color, true);
        }
    }

    pub(crate) fn set_mover(&mut self, mover: Color) {
        if self.state.set_mover(mover) {
            self.hash.flip_side();
        }
    }

    pub(crate) fn set_fullmove(&mut self, fullmove: usize) {
        self.state.set_fullmove(fullmove);
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
        self.hash.flip_side();
    }

    pub(crate) fn backwards(&mut self) {
        self.calculate_occupancies();
        self.state.backwards();
        self.hash.flip_side();
    }

    pub(crate) fn recompute_hash(&mut self) {
        self.hash = ZobristHash::from_board(self);
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

    pub(crate) fn calculate_occupancies(&mut self) {
        self.white_side = collapse(self.white);
        self.black_side = collapse(self.black);
        self.occupancy = self.white_side | self.black_side;
    }

    pub(crate) fn apply_mut(&mut self, movement: Move) {
        movement.apply(self);
        self.advance();
    }

    pub(crate) fn unapply_mut(&mut self, movement: Move) {
        movement.unapply(self);
        self.backwards();
    }

    fn generate_movements(&self, color: Color, legal_only: bool) -> Vec<Move> {
        self.pieces(color)
            .flat_map(|(_, bb)| bits::pos(bb))
            .flat_map(|p| Generator::from_board(self, p, legal_only).generate())
            .collect()
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
            hash: ZobristHash::default(),
        };
        board.calculate_occupancies();
        board.recompute_hash();
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

    use crate::squares::*;

    use super::*;

    #[test]
    fn at_white_king() {
        let sut = Board::default();
        let king = sut.at(E1);
        assert!(king.is_some());

        if let Some((color, piece, _bb)) = king {
            assert_eq!(Color::W, color);
            assert_eq!(Piece::King, piece);
        }
    }

    #[test]
    fn at_black_king() {
        let sut = Board::default();
        let king = sut.at(E8);

        assert!(king.is_some());

        if let Some((color, piece, _bb)) = king {
            assert_eq!(Color::B, color);
            assert_eq!(Piece::King, piece);
        }
    }

    #[test]
    fn mut_at_white() {
        let pos = A1;

        assert_eq!(Board::default().at(pos).unwrap().1, Board::default().at_mut(pos).unwrap().1);
    }

    #[test]
    fn mut_at_black() {
        let pos = H8;

        assert_eq!(Board::default().at(pos).unwrap().1, Board::default().at_mut(pos).unwrap().1);
    }

    #[test]
    fn count_pieces() {
        let sut = Board::default();
        assert_eq!(16, sut.count_pieces());
    }

    #[test]
    fn size() {
        assert_eq!(144, mem::size_of::<Board>());
        assert_eq!(8, mem::size_of::<&Board>());
    }

    #[test]
    fn empty_board_is_valid() {
        let mut board = Board::empty();
        assert_eq!(0, board.occupancy());
        let before = board.hash();
        board.recompute_hash();
        assert_eq!(before, board.hash());
        assert_eq!(before, ZobristHash::from_board(&board));
    }

    #[test]
    fn castling_change_changes_hash() {
        let mut board = Board::default();
        let start = board.hash();

        board.disable_castling(Color::W, CastlingUpdate::Right);
        assert_ne!(start, board.hash());
        assert_eq!(board.hash(), ZobristHash::from_board(&board));

        board.enable_castling(Color::W, CastlingUpdate::Right);
        assert_eq!(start, board.hash());
        assert_eq!(board.hash(), ZobristHash::from_board(&board));
    }

    #[test]
    fn capture_unmake_hash_check() {
        let mut board = crate::fen::decode("6k1/8/3p4/4P3/8/8/8/7K w - - 0 1").unwrap();
        for m in board.movements(board.state().mover()) {
            if m.from() == crate::squares::E5 && m.to() == crate::squares::D6 {
                board.apply_mut(m);
                assert_eq!(board.hash(), ZobristHash::from_board(&board), "after apply");
                board.unapply_mut(m);
                assert_eq!(board.hash(), ZobristHash::from_board(&board), "after unapply");
                return;
            }
        }
        panic!("no capture found");
    }

    #[test]
    fn hash_survives_random_moves() {
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};

        const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        const WALK: usize = 50;

        for seed in 0..10u64 {
            let mut board = crate::fen::decode(START_FEN).unwrap();
            let start_hash = board.hash();
            let mut rng = StdRng::seed_from_u64(seed);
            let mut applied = Vec::new();

            for ply in 0..WALK {
                let moves = board.movements(board.state().mover());
                if moves.is_empty() {
                    break;
                }
                let movement = moves[rng.gen_range(0..moves.len())];
                board.apply_mut(movement);
                assert_eq!(
                    board.hash(),
                    ZobristHash::from_board(&board),
                    "seed {seed} after applying ply {ply}"
                );
                applied.push(movement);
            }

            for (ply, movement) in applied.iter().rev().enumerate() {
                board.unapply_mut(*movement);
                assert_eq!(
                    board.hash(),
                    ZobristHash::from_board(&board),
                    "seed {seed} after unmaking ply {ply}"
                );
            }
            assert_eq!(board.hash(), start_hash, "seed {seed} back at the start position");
        }
    }
}
