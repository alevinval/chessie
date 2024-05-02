use crate::{
    bits,
    board::{Board, GameState},
    defs::{BitBoard, CastlingUpdate, Sq},
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
    state: &'board GameState,
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
        Self { board, state: board.state(), from, color, piece, moves: vec![], only_legal }
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
        let side_attack = (bits::southeast(pawns) & Magic::NOT_A_FILE & white_side)
            | (bits::southwest(pawns) & Magic::NOT_H_FILE & white_side);

        let first_push = bits::south(pawns) & !self.board.occupancy();
        let second_push = bits::south(first_push & Magic::RANK_6) & !self.board.occupancy();
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
        let side_attack = (bits::northeast(pawns) & Magic::NOT_A_FILE & black_side)
            | (bits::northwest(pawns) & Magic::NOT_H_FILE & black_side);

        let first_push = bits::north(pawns) & !self.board.occupancy();
        let second_push = bits::north(first_push & Magic::RANK_3) & !self.board.occupancy();
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
        if let Some(to) = bits::first_pos(bb) {
            let taken_piece = self.board.at(to).map(|(_, piece, _)| piece);
            for promo_piece in Piece::PROMO {
                self.push_move(Move::PawnPromo { from: self.from, to, promo_piece, taken_piece });
            }
        }
    }

    fn emit_castling(&mut self) {
        let (left, right) = self.state.castling(self.color);
        if left || right {
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

    fn get_castling_update(&self, color: Color, piece: Piece, pos: Sq) -> Option<CastlingUpdate> {
        let mut update = None;
        let is_king = matches!(piece, Piece::King);
        let is_rook = matches!(piece, Piece::Rook);
        if is_king || is_rook {
            let (left, right) = self.state.castling(color);

            if is_king {
                if left && right {
                    update = Some(CastlingUpdate::Both);
                } else if left {
                    update = Some(CastlingUpdate::Left);
                } else if right {
                    update = Some(CastlingUpdate::Right);
                }
            } else if is_rook {
                if pos == MagicCastling::right_rook(color) && right {
                    update = Some(CastlingUpdate::Right);
                } else if pos == MagicCastling::left_rook(color) && left {
                    update = Some(CastlingUpdate::Left);
                }
            }
        }
        update
    }

    fn emit_slides(&mut self, bb: BitBoard) {
        for to in bits::pos(bb) {
            self.push_move(Move::Slide {
                from: self.from,
                to,
                castling_update: self.get_castling_update(self.color, self.piece, self.from),
            });
        }
    }

    fn emit_takes(&mut self, bb: BitBoard) {
        for to in bits::pos(bb) {
            let (_, moved_piece, _) = self.board.at(self.from).unwrap();
            let (_, taken_piece, _) = self.board.at(to).unwrap();
            let value = score_piece(taken_piece) - score_piece(moved_piece);
            self.push_move(Move::Takes {
                from: self.from,
                to,
                piece: taken_piece,
                value,
                castling_update: self.get_castling_update(self.color, self.piece, self.from),
                target_castling_update: self.get_castling_update(
                    self.color.flip(),
                    taken_piece,
                    to,
                ),
            });
        }
    }

    fn push_move(&mut self, m: Move) {
        if !self.only_legal || self.is_legal(m) {
            self.moves.push(m);
        }
    }

    fn is_legal(&self, movement: Move) -> bool {
        let next = self.board.apply_clone(movement);
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

    use super::*;
    use crate::{defs::Sq, fen, sq, util::print_hboard};
    use test_case::test_case;

    fn gen_squares(board: &Board, sq: Sq) -> Vec<Sq> {
        let m = Generator::from_board(board, sq, true).generate();
        moves::attacked_positions(&m).collect()
    }

    #[test_case(sq!(1, 1), vec![18, 17, 25])]
    #[test_case(sq!(1, 2), vec![])]
    #[test_case(sq!(4, 5), vec![45])]
    #[test_case(sq!(5, 7), vec![54])]
    fn white_pawn_gen(sq: Sq, expected: Vec<Sq>) {
        let board =
            fen::decode("rnbqkbnr/pppppppp/7P/5P2/5p2/2p5/PPPPPPPP/RNBQKBNR w KQkq - 0 2").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq);
        print_hboard(&board, &actual);
        assert_eq!(expected, actual);
    }

    #[test_case(sq!(6, 7), vec![])]
    #[test_case(sq!(6, 6), vec![47, 38, 46])]
    #[test_case(sq!(3, 5), vec![21])]
    #[test_case(sq!(2, 7), vec![14])]
    fn black_pawn_gen(sq: Sq, expected: Vec<Sq>) {
        let board =
            fen::decode("rnbqkbnr/pppppppp/7P/5P2/5p2/7p/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq);
        print_hboard(&board, &actual);
        assert_eq!(expected, actual);
    }

    #[test_case("8/8/8/6q1/5K2/8/4P3/8 w - - 0 2", sq!(3, 5), vec![38, 21, 28])]
    #[test_case("8/8/8/6q1/8/8/3KP3/8 w - - 0 3", sq!(1, 3), vec![3, 4, 10, 18, 19])]
    #[test_case("8/8/8/6q1/8/8/4P3/K7 w - - 0 4", sq!(0, 0), vec![1, 8, 9])]
    fn king_gen(input: &str, at: Sq, expected: Vec<Sq>) {
        let board = fen::decode(input).unwrap();
        print_board(&board);

        let actual = gen_squares(&board, at);
        print_hboard(&board, &actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn slide_gen() {
        let board = fen::decode("8/6k1/8/6r1/8/8/6K1/8 b KQkq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq!(4, 6));
        print_hboard(&board, &actual);
        assert_eq!(vec![14, 22, 30, 32, 33, 34, 35, 36, 37, 39, 46], actual);
    }

    #[test]
    fn slide_three() {
        let board = fen::decode("1k4nr/7Q/8/8/8/6K1/8/8 b KQkq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq!(7, 7));
        print_hboard(&board, &actual);
        assert_eq!(vec![55], actual);
    }

    #[test]
    fn slide_gen_two() {
        let board = Board::default();
        print_board(&board);

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
