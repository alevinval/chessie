mod movement;

use crate::{
    bitboard::Bits,
    board::{Board, Castling},
    defs::BitBoard,
    magic::Magic,
    piece::Piece,
    print_board, Color, Pos,
};

pub use self::movement::Move;

#[derive(Debug)]
pub struct Generator<'board> {
    board: &'board Board,
    from: Pos,
    color: Color,
    piece: Piece,
    moves: Vec<Move>,
    check_legal: bool,
}

impl<'board> Generator<'board> {
    pub fn from_board<P: Into<Pos>>(board: &'board Board, from: P, check_legal: bool) -> Self {
        let from = from.into();
        let (color, piece, _) = board.at(from).unwrap_or_else(|| {
            print_board(board, &[]);
            unreachable!("cannot generate moves for empty position {from:?}")
        });

        Self::new(board, from, color, piece, check_legal)
    }

    pub fn new<P: Into<Pos>>(
        board: &'board Board,
        from: P,
        color: Color,
        piece: Piece,
        check_legal: bool,
    ) -> Self {
        Self { board, from: from.into(), color, piece, moves: vec![], check_legal }
    }

    #[must_use]
    pub fn generate(mut self) -> Vec<Move> {
        match self.piece {
            Piece::Pawn => match self.color {
                Color::B => self.emit_black_pawn(),
                Color::W => self.emit_white_pawn(),
            },
            Piece::Rook => self.emit_cross(),
            Piece::Bishop => self.emit_diag(),
            Piece::Queen => {
                self.emit_cross();
                self.emit_diag();
            }
            Piece::Knight => {
                self.emit(Magic::KNIGHT_MOVES[self.from.sq()]);
            }
            Piece::King => {
                self.emit_castling();
                self.emit(Magic::KING_MOVES[self.from.sq()]);
            }
        };
        self.moves
    }

    #[must_use]
    pub fn moves(self) -> Vec<Move> {
        self.moves
    }

    pub fn emit(&mut self, bb: BitBoard) {
        self.emit_takes(bb & self.board.side(self.color.flip()));
        self.emit_slides(bb & !self.board.occupancy());
    }

    fn emit_slides(&mut self, bb: BitBoard) {
        for to in Bits::pos(bb) {
            if self.piece == Piece::Pawn && to.row() == self.color.flip().piece_row() {
                self.emit_pawn_promos(to);
                continue;
            }
            self.push_move(Move::Slide { from: self.from, to });
        }
    }

    fn emit_takes(&mut self, bb: BitBoard) {
        for to in Bits::pos(bb) {
            if self.piece == Piece::Pawn && to.row() == self.color.flip().piece_row() {
                self.emit_pawn_promos(to);
                continue;
            }
            self.push_move(Move::Takes { from: self.from, to });
        }
    }

    fn emit_cross(&mut self) {
        let col = self.hyper_quint(Magic::COL_SLIDER[self.from.col()]);
        let row = self.hyper_quint(Magic::ROW_SLIDER[self.from.row()]);
        self.emit(col | row);
    }

    fn emit_diag(&mut self) {
        let diag = self.hyper_quint(Magic::DIAG_SLIDER[self.from.sq()]);
        let antidiag = self.hyper_quint(Magic::ANTI_DIAG_SLIDER[self.from.sq()]);
        self.emit(diag | antidiag);
    }

    fn push_move(&mut self, m: Move) {
        if !self.check_legal || self.is_legal(m) {
            self.moves.push(m);
        }
    }

    fn emit_black_pawn(&mut self) {
        let pawns = self.board.get_piece(Color::B, Piece::Pawn) & self.from.bb();
        let white_side = self.board.side(Color::W);
        let side_attack = (Bits::southeast(pawns) & Magic::NOT_A_FILE & white_side)
            | (Bits::southwest(pawns) & Magic::NOT_H_FILE & white_side);
        self.emit_takes(side_attack);

        let first_push = Bits::south(pawns) & !self.board.occupancy();
        let second_push = Bits::south(first_push & Magic::RANK_6) & !self.board.occupancy();
        let pushes = first_push | second_push;
        self.emit_slides(pushes);
    }

    fn emit_white_pawn(&mut self) {
        let pawns = self.board.get_piece(Color::W, Piece::Pawn) & self.from.bb();
        let black_side = self.board.side(Color::B);
        let side_attack = (Bits::northeast(pawns) & Magic::NOT_A_FILE & black_side)
            | (Bits::northwest(pawns) & Magic::NOT_H_FILE & black_side);
        self.emit_takes(side_attack);

        let first_push = Bits::north(pawns) & !self.board.occupancy();
        let second_push = Bits::north(first_push & Magic::RANK_3) & !self.board.occupancy();
        let pushes = first_push | second_push;
        self.emit_slides(pushes);
    }

    fn emit_pawn_promos(&mut self, to: Pos) {
        for piece in Piece::PROMO {
            self.push_move(Move::PawnPromo { from: self.from, to, piece });
        }
    }

    fn emit_castling(&mut self) {
        if let Castling::Some(left, right) = self.board.castling_rights(self.color) {
            let occ = self.board.occupancy();
            let side = self.board.side(self.color);

            if right {
                let right_msk = match self.color {
                    Color::B => Magic::BLACK_RIGHT_CASTLE,
                    Color::W => Magic::WHITE_RIGHT_CASTLE,
                };
                let right_sq = match self.color {
                    Color::B => Magic::H8,
                    Color::W => Magic::H1,
                };
                if (right_msk & occ == right_sq) && (right_msk & side == right_sq) {
                    self.push_move(Move::RightCastle { mover: self.color });
                }
            }

            if left {
                let left_msk = match self.color {
                    Color::B => Magic::BLACK_LEFT_CASTLE,
                    Color::W => Magic::WHITE_LEFT_CASTLE,
                };
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

    fn is_legal(&self, movement: Move) -> bool {
        !movement.apply(self.board).in_check(self.color)
    }

    fn hyper_quint(&self, mask: BitBoard) -> BitBoard {
        let o = self.board.occupancy() & mask;
        let r = self.from.bb();
        let line = (o.wrapping_sub(r.wrapping_mul(2)))
            ^ (o.reverse_bits().wrapping_sub(r.reverse_bits().wrapping_mul(2))).reverse_bits();
        line & mask
    }
}

#[cfg(test)]
mod test {

    use crate::defs::Sq;

    use super::*;

    fn gen_squares<P: Into<Pos>>(board: &Board, pos: P) -> Vec<Pos> {
        let m = Generator::from_board(board, pos, true).generate();
        m.iter().map(|m| m.to()).collect()
    }

    fn assert_moves(expected: Vec<Sq>, actual: Vec<Pos>) {
        let actual: Vec<Sq> = actual.into_iter().map(|p| p.sq()).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn white_pawn_gen() {
        let mut board = Board::default();

        Bits::set(&mut board.black[Piece::P], Pos::new(3, 5));
        Bits::set(&mut board.black[Piece::P], Pos::new(2, 2));
        Bits::set(&mut board.white[Piece::P], Pos::new(4, 5));
        Bits::set(&mut board.white[Piece::P], Pos::new(5, 7));
        board.next_turn();
        board.next_turn();

        print_board(&board, &[]);

        let actual = gen_squares(&board, (1, 1));
        print_board(&board, &actual);
        assert_moves(vec![18, 17, 25], actual);

        let actual = gen_squares(&board, (1, 2));
        print_board(&board, &actual);
        assert_moves(vec![] as Vec<Sq>, actual);

        let actual = gen_squares(&board, (4, 5));
        print_board(&board, &actual);
        assert_moves(vec![45], actual);

        let actual = gen_squares(&board, (5, 7));
        print_board(&board, &actual);
        assert_moves(vec![54], actual);
    }

    #[test]
    fn black_pawn_gen() {
        let mut board = Board::default();

        Bits::set(&mut board.black[Piece::P], Pos::new(3, 5));
        Bits::set(&mut board.black[Piece::P], Pos::new(2, 7));
        Bits::set(&mut board.white[Piece::P], Pos::new(4, 5));
        Bits::set(&mut board.white[Piece::P], Pos::new(5, 7));

        print_board(&board, &[]);

        board.next_turn();

        let actual = gen_squares(&board, (6, 7));
        print_board(&board, &actual);
        assert_moves(vec![] as Vec<Sq>, actual);

        let actual = gen_squares(&board, (6, 6));
        print_board(&board, &actual);
        assert_moves(vec![47, 38, 46], actual);

        let actual = gen_squares(&board, (3, 5));
        print_board(&board, &actual);
        assert_moves(vec![21], actual);

        let actual = gen_squares(&board, (2, 7));
        print_board(&board, &actual);
        assert_moves(vec![14], actual);
    }

    #[test]
    fn king_gen() {
        let mut board = Board::default();
        board.clear();
        Bits::set(&mut board.black[Piece::Q], Pos::new(4, 6));
        Bits::set(&mut board.white[Piece::P], Pos::new(1, 4));
        Bits::set(&mut board.white[Piece::K], Pos::new(3, 5));
        board.next_turn();
        board.next_turn();

        print_board(&board, &[]);

        let actual = gen_squares(&board, Pos::new(3, 5));
        print_board(&board, &actual);
        assert_moves(vec![38, 21, 28], actual);

        Bits::slide(&mut board.white[Piece::K], Pos::new(3, 5), Pos::new(1, 3));
        board.next_turn();
        board.next_turn();
        let actual = gen_squares(&board, Pos::new(1, 3));
        print_board(&board, &actual);
        assert_moves(vec![3, 4, 10, 18, 19], actual);

        Bits::slide(&mut board.white[Piece::K], Pos::new(1, 3), Pos::new(0, 0));
        board.next_turn();
        board.next_turn();
        let actual = gen_squares(&board, Pos::new(0, 0));
        print_board(&board, &actual);
        assert_moves(vec![1, 8, 9], actual);
    }

    #[test]
    fn slide_gen() {
        let mut board = Board::default();
        board.clear();
        Bits::set(&mut board.white[Piece::K], Pos::new(1, 6));
        Bits::set(&mut board.black[Piece::K], Pos::new(6, 6));
        Bits::set(&mut board.black[Piece::R], Pos::new(4, 6));
        board.next_turn();
        print_board(&board, &[]);

        let actual = gen_squares(&board, Pos::new(4, 6));
        print_board(&board, &actual);
        assert_moves(vec![14, 22, 30, 32, 33, 34, 35, 36, 37, 39, 46], actual);
    }

    #[test]
    fn slide_three() {
        let mut board = Board::default();
        board.clear();
        Bits::set(&mut board.white[Piece::K], Pos::new(2, 6));
        Bits::set(&mut board.white[Piece::Q], Pos::new(6, 7));

        Bits::set(&mut board.black[Piece::K], Pos::new(7, 1));
        Bits::set(&mut board.black[Piece::N], Pos::new(7, 6));
        Bits::set(&mut board.black[Piece::R], Pos::new(7, 7));
        board.next_turn();
        print_board(&board, &[]);

        let actual = gen_squares(&board, Pos::new(7, 7));
        print_board(&board, &actual);
        assert_moves(vec![55], actual);
    }

    #[test]
    fn slide_gen_two() {
        let board = Board::default();
        print_board(&board, &[]);

        let actual = gen_squares(&board, Pos::new(0, 0));
        print_board(&board, &actual);
        assert_moves(vec![], actual);
    }

    #[test]
    fn emit_castling() {
        let mut board = Board::default();
        Bits::unset(&mut board.white[Piece::N], Pos::new(0, 6));
        Bits::unset(&mut board.white[Piece::N], Pos::new(0, 1));
        Bits::unset(&mut board.white[Piece::B], Pos::new(0, 5));
        Bits::unset(&mut board.white[Piece::B], Pos::new(0, 2));
        Bits::unset(&mut board.white[Piece::Q], Pos::new(0, 3));
        board.next_turn();
        print_board(&board, &[]);

        let actual = gen_squares(&board, Pos::new(0, 4));
        assert_moves(vec![6, 2, 3, 5], actual);

        let mut board = Board::default();
        Bits::unset(&mut board.black[Piece::N], Pos::new(7, 6));
        Bits::unset(&mut board.black[Piece::N], Pos::new(7, 1));
        Bits::unset(&mut board.black[Piece::B], Pos::new(7, 5));
        Bits::unset(&mut board.black[Piece::B], Pos::new(7, 2));
        Bits::unset(&mut board.black[Piece::Q], Pos::new(7, 3));
        board.next_turn();
        print_board(&board, &[]);

        let actual = gen_squares(&board, Pos::new(7, 4));
        assert_moves(vec![62, 58, 59, 61], actual);
    }
}
