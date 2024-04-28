use crate::{
    bits::Bits,
    board::Board,
    defs::{BitBoard, Castling, Sq},
    eval::score_piece,
    magic::{Magic, MagicCastling},
    moves,
    piece::Piece,
    pos,
    util::print_board,
    Color,
};

use super::Move;

#[derive(Debug)]
pub(crate) struct Generator<'board> {
    board: &'board Board,
    color: Color,
    piece: Piece,
    from: Sq,
    moves: Vec<Move>,
    only_legal: bool,
}

impl<'board> Generator<'board> {
    pub(crate) fn from_board(board: &'board Board, from: Sq, only_legal: bool) -> Self {
        let (color, piece, _) = board.at(from).unwrap_or_else(|| {
            print_board(board);
            unreachable!("cannot generate moves for empty position {from:?}")
        });

        Self::new(board, from, color, piece, only_legal)
    }

    pub(crate) fn new(
        board: &'board Board,
        from: Sq,
        color: Color,
        piece: Piece,
        only_legal: bool,
    ) -> Self {
        Self { board, from, color, piece, moves: vec![], only_legal }
    }

    #[must_use]
    pub(crate) fn generate(mut self) -> Vec<Move> {
        match self.piece {
            Piece::Pawn => match self.color {
                Color::B => self.emit_black_pawn(),
                Color::W => self.emit_white_pawn(),
            },
            Piece::Rook => self.emit(self.cross()),
            Piece::Bishop => self.emit(self.diag()),
            Piece::Queen => self.emit(self.cross() | self.diag()),
            Piece::Knight => {
                self.emit(Magic::KNIGHT_MOVES[self.from as usize]);
            }
            Piece::King => {
                self.emit_castling();
                self.emit(Magic::KING_MOVES[self.from as usize]);
            }
        };
        self.moves
    }

    fn cross(&self) -> BitBoard {
        let col = self.hyper_quint(Magic::COL_SLIDER[pos::col(self.from) as usize]);
        let row = self.hyper_quint(Magic::ROW_SLIDER[pos::row(self.from) as usize]);
        col | row
    }

    fn diag(&self) -> BitBoard {
        let diag = self.hyper_quint(Magic::DIAG_SLIDER[self.from as usize]);
        let antidiag = self.hyper_quint(Magic::ANTIDIAG_SLIDER[self.from as usize]);
        diag | antidiag
    }

    fn emit_black_pawn(&mut self) {
        let pawns = self.board.get(Color::B, Piece::Pawn) & pos::bb(self.from);
        let white_side = self.board.occupancy_side(Color::W);
        let side_attack = (Bits::southeast(pawns) & Magic::NOT_A_FILE & white_side)
            | (Bits::southwest(pawns) & Magic::NOT_H_FILE & white_side);

        let first_push = Bits::south(pawns) & !self.board.occupancy();
        let second_push = Bits::south(first_push & Magic::RANK_6) & !self.board.occupancy();
        let pushes = first_push | second_push;

        if pos::row(self.from) == self.color.flip().pawn_row() {
            self.emit_pawn_promos(side_attack);
            self.emit_pawn_promos(pushes);
        } else {
            self.emit_takes(side_attack);
            self.emit_slides(pushes);
        }
    }

    fn emit_white_pawn(&mut self) {
        let pawns = self.board.get(Color::W, Piece::Pawn) & pos::bb(self.from);
        let black_side = self.board.occupancy_side(Color::B);
        let side_attack = (Bits::northeast(pawns) & Magic::NOT_A_FILE & black_side)
            | (Bits::northwest(pawns) & Magic::NOT_H_FILE & black_side);

        let first_push = Bits::north(pawns) & !self.board.occupancy();
        let second_push = Bits::north(first_push & Magic::RANK_3) & !self.board.occupancy();
        let pushes = first_push | second_push;

        if pos::row(self.from) == self.color.flip().pawn_row() {
            self.emit_pawn_promos(side_attack);
            self.emit_pawn_promos(pushes);
        } else {
            self.emit_takes(side_attack);
            self.emit_slides(pushes);
        }
    }

    fn emit_pawn_promos(&mut self, bb: BitBoard) {
        if let Some(to) = Bits::first_pos(bb) {
            for piece in Piece::PROMO {
                self.push_move(Move::PawnPromo { from: self.from, to, piece });
            }
        }
    }

    fn emit_castling(&mut self) {
        if let Castling::Some { left, right } = self.board.state().castling(self.color) {
            debug_assert!(
                right | left,
                "should never emit castling if there are no castling rights"
            );

            let occ = self.board.occupancy();
            let side = self.board.occupancy_side(self.color);

            if right {
                let right_msk = MagicCastling::right(self.color);
                let right_sq = match self.color {
                    Color::B => Magic::H8,
                    Color::W => Magic::H1,
                };
                if (right_msk & occ == right_sq) && (right_msk & side == right_sq) {
                    self.push_move(Move::RightCastle { mover: self.color });
                }
            }

            if left {
                let left_msk = MagicCastling::left(self.color);
                let left_sq = match self.color {
                    Color::B => Magic::A8,
                    Color::W => Magic::A1,
                };
                if (left_msk & occ == left_sq) && (left_msk & side) == left_sq {
                    self.push_move(Move::LeftCastle { mover: self.color });
                }
            }
        }
    }

    fn emit(&mut self, bb: BitBoard) {
        self.emit_takes(bb & self.board.occupancy_side(self.color.flip()));
        self.emit_slides(bb & !self.board.occupancy());
    }

    fn emit_slides(&mut self, bb: BitBoard) {
        for to in Bits::pos(bb) {
            self.push_move(Move::Slide { from: self.from, to });
        }
    }

    fn emit_takes(&mut self, bb: BitBoard) {
        for to in Bits::pos(bb) {
            let (_, pf, _) = self.board.at(self.from).unwrap();
            let (_, pt, _) = self.board.at(to).unwrap();
            let value = score_piece(pt) - score_piece(pf);
            self.push_move(Move::Takes { from: self.from, to, value });
        }
    }

    fn push_move(&mut self, m: Move) {
        if !self.only_legal || self.is_legal(m) {
            self.moves.push(m);
        }
    }

    fn is_legal(&self, movement: Move) -> bool {
        let next = movement.apply(self.board);
        if matches!(movement, Move::LeftCastle { .. })
            && moves::is_attacked(
                &next.pseudo_movements(self.color.flip()),
                MagicCastling::left_xray(self.color),
            )
        {
            return false;
        }

        if matches!(movement, Move::RightCastle { .. })
            && moves::is_attacked(
                &next.pseudo_movements(self.color.flip()),
                MagicCastling::right_xray(self.color),
            )
        {
            return false;
        }

        !next.in_check(self.color)
    }

    fn hyper_quint(&self, mask: BitBoard) -> BitBoard {
        let o = self.board.occupancy() & mask;
        let r = pos::bb(self.from);
        let line = (o.wrapping_sub(r.wrapping_mul(2)))
            ^ (o.reverse_bits().wrapping_sub(r.reverse_bits().wrapping_mul(2))).reverse_bits();
        line & mask
    }
}
#[cfg(test)]
mod test {

    use crate::{defs::Sq, fen, sq, util::print_hboard};

    use super::*;

    fn gen_squares(board: &Board, sq: Sq) -> Vec<Sq> {
        let m = Generator::from_board(board, sq, true).generate();
        moves::attacked_positions(&m).collect()
    }

    #[test]
    fn white_pawn_gen() {
        let mut board = Board::default();

        Bits::set(board.black(Piece::P), sq!(3, 5));
        Bits::set(board.black(Piece::P), sq!(2, 2));
        Bits::set(board.white(Piece::P), sq!(4, 5));
        Bits::set(board.white(Piece::P), sq!(5, 7));
        board.advance();
        board.advance();

        print_hboard(&board, &[]);

        let actual = gen_squares(&board, sq!(1, 1));
        print_hboard(&board, &actual);
        assert_eq!(vec![18, 17, 25], actual);

        let actual = gen_squares(&board, sq!(1, 2));
        print_hboard(&board, &actual);
        assert_eq!(vec![] as Vec<Sq>, actual);

        let actual = gen_squares(&board, sq!(4, 5));
        print_hboard(&board, &actual);
        assert_eq!(vec![45], actual);

        let actual = gen_squares(&board, sq!(5, 7));
        print_hboard(&board, &actual);
        assert_eq!(vec![54], actual);
    }

    #[test]
    fn black_pawn_gen() {
        let mut board = Board::default();

        Bits::set(board.black(Piece::P), sq!(3, 5));
        Bits::set(board.black(Piece::P), sq!(2, 7));
        Bits::set(board.white(Piece::P), sq!(4, 5));
        Bits::set(board.white(Piece::P), sq!(5, 7));

        print_hboard(&board, &[]);

        board.advance();

        let actual = gen_squares(&board, sq!(6, 7));
        print_hboard(&board, &actual);
        assert_eq!(vec![] as Vec<Sq>, actual);

        let actual = gen_squares(&board, sq!(6, 6));
        print_hboard(&board, &actual);
        assert_eq!(vec![47, 38, 46], actual);

        let actual = gen_squares(&board, sq!(3, 5));
        print_hboard(&board, &actual);
        assert_eq!(vec![21], actual);

        let actual = gen_squares(&board, sq!(2, 7));
        print_hboard(&board, &actual);
        assert_eq!(vec![14], actual);
    }

    #[test]
    fn king_gen() {
        let mut board = Board::default();
        board.state_mut().set_castled();
        board.clear();
        Bits::set(board.black(Piece::Q), sq!(4, 6));
        Bits::set(board.white(Piece::P), sq!(1, 4));
        Bits::set(board.white(Piece::K), sq!(3, 5));
        board.advance();
        board.advance();

        print_hboard(&board, &[]);

        let actual = gen_squares(&board, sq!(3, 5));
        print_hboard(&board, &actual);
        assert_eq!(vec![38, 21, 28], actual);

        Bits::slide(board.white(Piece::K), sq!(3, 5), sq!(1, 3));
        board.advance();
        board.advance();
        let actual = gen_squares(&board, sq!(1, 3));
        print_hboard(&board, &actual);
        assert_eq!(vec![3, 4, 10, 18, 19], actual);

        Bits::slide(board.white(Piece::K), sq!(1, 3), sq!(0, 0));
        board.advance();
        board.advance();
        let actual = gen_squares(&board, sq!(0, 0));
        print_hboard(&board, &actual);
        assert_eq!(vec![1, 8, 9], actual);
    }

    #[test]
    fn slide_gen() {
        let mut board = Board::default();
        board.clear();
        Bits::set(board.white(Piece::K), sq!(1, 6));
        Bits::set(board.black(Piece::K), sq!(6, 6));
        Bits::set(board.black(Piece::R), sq!(4, 6));
        board.advance();
        print_hboard(&board, &[]);

        let actual = gen_squares(&board, sq!(4, 6));
        print_hboard(&board, &actual);
        assert_eq!(vec![14, 22, 30, 32, 33, 34, 35, 36, 37, 39, 46], actual);
    }

    #[test]
    fn slide_three() {
        let mut board = Board::default();
        board.clear();
        Bits::set(board.white(Piece::K), sq!(2, 6));
        Bits::set(board.white(Piece::Q), sq!(6, 7));

        Bits::set(board.black(Piece::K), sq!(7, 1));
        Bits::set(board.black(Piece::N), sq!(7, 6));
        Bits::set(board.black(Piece::R), sq!(7, 7));
        board.advance();
        print_hboard(&board, &[]);

        let actual = gen_squares(&board, sq!(7, 7));
        print_hboard(&board, &actual);
        assert_eq!(vec![55], actual);
    }

    #[test]
    fn slide_gen_two() {
        let board = Board::default();
        print_hboard(&board, &[]);

        let actual = gen_squares(&board, sq!(0, 0));
        print_hboard(&board, &actual);
        assert_eq!(vec![] as Vec<Sq>, actual);
    }

    #[test]
    fn emit_castling() {
        let board = fen::decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq!(0, 4));
        assert_eq!(vec![6, 2, 3, 5], actual);

        let board = fen::decode("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq!(7, 4));
        assert_eq!(vec![62, 58, 59, 61], actual);
    }

    #[test]
    fn emit_castling_xray_white() {
        let board = fen::decode("4k3/8/8/8/q7/7q/3PPP2/R3K2R b KQ - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq!(0, 4));
        assert_eq!(vec![] as Vec<Sq>, actual);

        let board = fen::decode("4k3/8/8/8/8/8/3PPP2/R3K2R b KQ - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq!(0, 4));
        assert_eq!(vec![6, 2, 3, 5], actual);
    }

    #[test]
    fn emit_castling_xray_black() {
        let board = fen::decode("r3k2r/3ppp2/1Q5Q/8/8/8/3PPP2/4K3 b kq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq!(7, 4));
        assert_eq!(vec![] as Vec<Sq>, actual);

        let board = fen::decode("r3k2r/3ppp2/8/8/8/8/3PPP2/4K3 b kq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq!(7, 4));
        assert_eq!(vec![62, 58, 59, 61], actual);
    }
}
