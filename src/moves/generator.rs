use crate::{
    Color, bits,
    board::{Board, GameState},
    defs::{BitBoard, CastlingTuple, CastlingUpdate, Sq},
    eval::score_piece,
    magic::{MagicMovements, Masks},
    moves,
    piece::Piece,
    pos,
    squares::*,
    util::print_board,
};

use super::Move;

#[derive(Debug)]
pub(crate) struct Generator<'board> {
    board: &'board Board,
    state: &'board GameState,
    color: Color,
    piece: Piece,
    from: Sq,
    castling: CastlingTuple,
    castling_update: Option<CastlingUpdate>,
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
        let state = board.state();
        let castling = state.castling(color);
        let castling_update = if only_legal {
            if matches!(piece, Piece::King) {
                calc_castling_king(castling)
            } else if matches!(piece, Piece::Rook) {
                calc_castling_rook(color, from, castling)
            } else {
                None
            }
        } else {
            None
        };

        Self {
            board,
            state: board.state(),
            from,
            color,
            piece,
            castling,
            castling_update,
            moves: vec![],
            only_legal,
        }
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
                self.emit(MagicMovements::KNIGHT_MOVES[self.from as usize]);
            }
            Piece::King => {
                self.emit_castling();
                self.emit(MagicMovements::KING_MOVES[self.from as usize]);
            }
        };
        self.moves
    }

    const fn cross(&self) -> BitBoard {
        let col = self.hyper_quint(MagicMovements::COL_SLIDER[pos::col(self.from) as usize]);
        let row = self.hyper_quint(MagicMovements::ROW_SLIDER[pos::row(self.from) as usize]);
        col | row
    }

    const fn diag(&self) -> BitBoard {
        let diag = self.hyper_quint(MagicMovements::DIAG_SLIDER[self.from as usize]);
        let antidiag = self.hyper_quint(MagicMovements::ANTIDIAG_SLIDER[self.from as usize]);
        diag | antidiag
    }

    fn emit_black_pawn(&mut self) {
        let pawns = self.board.get(Color::B, Piece::Pawn) & pos::bb(self.from);
        let white_side = self.board.occupancy_side(Color::W);
        let side_attack = (bits::southeast(pawns) & Masks::NOT_FILE_A & white_side)
            | (bits::southwest(pawns) & Masks::NOT_FILE_H & white_side);

        let first_push = bits::south(pawns) & !self.board.occupancy();
        let second_push = bits::south(first_push & Masks::RANK_6) & !self.board.occupancy();
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
        let side_attack = (bits::northeast(pawns) & Masks::NOT_FILE_A & black_side)
            | (bits::northwest(pawns) & Masks::NOT_FILE_H & black_side);

        let first_push = bits::north(pawns) & !self.board.occupancy();
        let second_push = bits::north(first_push & Masks::RANK_3) & !self.board.occupancy();
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
        let (left, right) = self.castling;
        if left || right {
            let occ = self.board.occupancy();
            let side = self.board.occupancy_side(self.color);

            let castling_update = if left && right {
                CastlingUpdate::Both
            } else if left {
                CastlingUpdate::Left
            } else {
                CastlingUpdate::Right
            };

            if right {
                let right_msk = Masks::castle_right(self.color);
                let right_sq = match self.color {
                    Color::B => Masks::H8,
                    Color::W => Masks::H1,
                };
                if (right_msk & occ == right_sq) && (right_msk & side == right_sq) {
                    self.push_move(Move::RightCastle { mover: self.color, castling_update });
                }
            }

            if left {
                let left_msk = Masks::castle_left(self.color);
                let left_sq = match self.color {
                    Color::B => Masks::A8,
                    Color::W => Masks::A1,
                };
                if (left_msk & occ == left_sq) && (left_msk & side) == left_sq {
                    self.push_move(Move::LeftCastle { mover: self.color, castling_update });
                }
            }
        }
    }

    fn emit(&mut self, bb: BitBoard) {
        self.emit_takes(bb & self.board.occupancy_side(self.color.flip()));
        self.emit_slides(bb & !self.board.occupancy());
    }

    fn emit_slides(&mut self, bb: BitBoard) {
        for to in bits::pos(bb) {
            self.push_move(Move::Slide {
                from: self.from,
                to,
                castling_update: self.castling_update,
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
                castling_update: self.castling_update,
                target_castling_update: if matches!(taken_piece, Piece::Rook) {
                    self.calc_castling_opponent(to)
                } else {
                    None
                },
            });
        }
    }

    fn push_move(&mut self, m: Move) {
        if !self.only_legal || self.is_legal(m) {
            self.moves.push(m);
        }
    }

    fn is_legal(&mut self, movement: Move) -> bool {
        let mut next = self.board.clone();
        next.apply_mut(movement);
        if matches!(movement, Move::LeftCastle { .. } | Move::RightCastle { .. })
            && moves::is_attacked(
                &next.pseudo_movements(self.color.flip()),
                if matches!(movement, Move::LeftCastle { .. }) {
                    left_xray(self.color)
                } else {
                    right_xray(self.color)
                },
            )
        {
            false
        } else {
            !next.in_check(self.color)
        }
    }

    const fn hyper_quint(&self, mask: BitBoard) -> BitBoard {
        let o = self.board.occupancy() & mask;
        let r = pos::bb(self.from);
        let line = (o.wrapping_sub(r.wrapping_mul(2)))
            ^ (o.reverse_bits().wrapping_sub(r.reverse_bits().wrapping_mul(2))).reverse_bits();
        line & mask
    }

    const fn calc_castling_opponent(&self, pos: Sq) -> Option<CastlingUpdate> {
        let color = self.color.flip();
        let (left, right) = self.state.castling(color);
        if pos == right_rook(color) && right {
            Some(CastlingUpdate::Right)
        } else if pos == left_rook(color) && left {
            Some(CastlingUpdate::Left)
        } else {
            None
        }
    }
}

const fn calc_castling_king((left, right): CastlingTuple) -> Option<CastlingUpdate> {
    if left && right {
        Some(CastlingUpdate::Both)
    } else if right {
        Some(CastlingUpdate::Right)
    } else if left {
        Some(CastlingUpdate::Left)
    } else {
        None
    }
}

const fn calc_castling_rook(
    color: Color,
    pos: Sq,
    (left, right): CastlingTuple,
) -> Option<CastlingUpdate> {
    if pos == right_rook(color) && right {
        Some(CastlingUpdate::Right)
    } else if pos == left_rook(color) && left {
        Some(CastlingUpdate::Left)
    } else {
        None
    }
}

const fn left_rook(color: Color) -> Sq {
    match color {
        Color::B => A8,
        Color::W => A1,
    }
}

const fn right_rook(color: Color) -> Sq {
    match color {
        Color::B => H8,
        Color::W => H1,
    }
}

const fn left_xray(color: Color) -> Sq {
    match color {
        Color::B => D8,
        Color::W => D1,
    }
}

const fn right_xray(color: Color) -> Sq {
    match color {
        Color::B => F8,
        Color::W => F1,
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{defs::Sq, fen, util::print_hboard};
    use test_case::test_case;

    fn gen_squares(board: &Board, sq: Sq) -> Vec<Sq> {
        let m = Generator::from_board(board, sq, true).generate();
        moves::attacked_positions(&m).collect()
    }

    #[test_case(B2, vec![C3, B3, B4])]
    #[test_case(C2, vec![])]
    #[test_case(F5, vec![F6])]
    #[test_case(H6, vec![G7])]
    fn white_pawn_gen(sq: Sq, expected: Vec<Sq>) {
        let board =
            fen::decode("rnbqkbnr/pppppppp/7P/5P2/5p2/2p5/PPPPPPPP/RNBQKBNR w KQkq - 0 2").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq);
        print_hboard(&board, &actual);
        assert_eq!(expected, actual);
    }

    #[test_case(H7, vec![])]
    #[test_case(G7, vec![H6, G5, G6])]
    #[test_case(F4, vec![F3])]
    #[test_case(H3, vec![G2])]
    fn black_pawn_gen(sq: Sq, expected: Vec<Sq>) {
        let board =
            fen::decode("rnbqkbnr/pppppppp/7P/5P2/5p2/7p/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, sq);
        print_hboard(&board, &actual);
        assert_eq!(expected, actual);
    }

    #[test_case("8/8/8/6q1/5K2/8/4P3/8 w - - 0 2", F4, vec![G5, F3, E4])]
    #[test_case("8/8/8/6q1/8/8/3KP3/8 w - - 0 3", D2, vec![D1, E1, C2, C3, D3])]
    #[test_case("8/8/8/6q1/8/8/4P3/K7 w - - 0 4", A1, vec![B1, A2, B2])]
    fn king_gen(input: &str, at: Sq, expected: Vec<Sq>) {
        let board = fen::decode(input).unwrap();
        print_board(&board);

        let actual = gen_squares(&board, at);
        print_hboard(&board, &actual);
        assert_eq!(expected, actual);
    }

    #[test_case("8/6k1/8/6r1/8/8/6K1/8 b KQkq - 0 1", G5, vec![G2, G3, G4, A5, B5, C5, D5, E5, F5, H5, G6])]
    #[test_case("1k4nr/7Q/8/8/8/6K1/8/8 b KQkq - 0 1", H8, vec![H7])]
    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", A1, vec![])]
    fn slide_rook(input: &str, at: Sq, expected: Vec<Sq>) {
        let board = fen::decode(input).unwrap();
        print_board(&board);

        let actual = gen_squares(&board, at);
        print_hboard(&board, &actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn emit_castling() {
        let board = fen::decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, E1);
        assert_eq!(vec![G1, C1, D1, F1], actual);

        let board = fen::decode("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, E8);
        assert_eq!(vec![G8, C8, D8, F8], actual);
    }

    #[test]
    fn emit_castling_xray_white() {
        let board = fen::decode("4k3/8/8/8/q7/7q/3PPP2/R3K2R b KQ - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, E1);
        assert_eq!(vec![] as Vec<Sq>, actual);

        let board = fen::decode("4k3/8/8/8/8/8/3PPP2/R3K2R b KQ - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, E1);
        assert_eq!(vec![G1, C1, D1, F1], actual);
    }

    #[test]
    fn emit_castling_xray_black() {
        let board = fen::decode("r3k2r/3ppp2/1Q5Q/8/8/8/3PPP2/4K3 b kq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, E8);
        assert_eq!(vec![] as Vec<Sq>, actual);

        let board = fen::decode("r3k2r/3ppp2/8/8/8/8/3PPP2/4K3 b kq - 0 1").unwrap();
        print_board(&board);

        let actual = gen_squares(&board, E8);
        assert_eq!(vec![G8, C8, D8, F8], actual);
    }
}
