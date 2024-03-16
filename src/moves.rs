mod movement;
mod placement;

use std::iter::zip;

use crate::{
    bitboard::Bits,
    board::{Board, Castling},
    defs::BitBoard,
    magic::{Magic, KING_MAGIC, KNIGHT_MAGIC},
    piece::Piece,
    print_board, Color, Pos,
};

pub use self::movement::Move;
use self::placement::{empty_or_take, is_empty, Placement, StopCondition};

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
        let from = from.into();
        Self { board, from, color, piece, moves: vec![], check_legal }
    }

    pub fn moves(self) -> Vec<Move> {
        self.moves
    }

    pub fn generate(mut self) -> Vec<Move> {
        match self.piece {
            Piece::Pawn => match self.color {
                Color::B => self.black_pawn(),
                Color::W => self.white_pawn(),
            },
            Piece::Rook => self.cross(empty_or_take),
            Piece::Bishop => self.diagonals(empty_or_take),
            Piece::Queen => self.queen(),
            Piece::Knight => self.knight(),
            Piece::King => self.king(),
        };
        self.moves
    }

    pub fn slides_from_magic(&mut self, bb: BitBoard) {
        let from = self.from;
        for to in Bits::pos(bb) {
            if self.piece == Piece::Pawn && to.row() == self.color.flip().piece_row() {
                self.emit_pawn_promos(to);
                continue;
            }
            self.emit_move(Move::Slide { from, to });
        }
    }

    fn emit_move(&mut self, m: Move) {
        if !self.check_legal || self.is_legal(m) {
            self.moves.push(m);
        }
    }

    fn pos<P: Into<Pos>>(&mut self, to: P, stop_at: StopCondition) -> Option<Placement> {
        let placement = stop_at(self.board, self.from, to.into());

        if let Some(placement) = &placement {
            self.emit_move(placement.movement());
        }

        placement
    }

    fn moves_from_magic(&mut self, bb: BitBoard) {
        let takes = bb & self.board.side(self.color.flip());
        let empty = bb & !takes & !self.board.side(self.color);

        self.takes_from_magic(takes);
        self.slides_from_magic(empty);
    }

    fn takes_from_magic(&mut self, bb: BitBoard) {
        let from = self.from;

        for to in Bits::pos(bb) {
            if self.piece == Piece::Pawn && to.row() == self.color.flip().piece_row() {
                self.emit_pawn_promos(to);
                continue;
            }
            self.emit_move(Move::Takes { from, to });
        }
    }

    fn emit_pawn_promos(&mut self, to: Pos) {
        for piece in Piece::PROMO {
            let promo = Move::PawnPromo { from: self.from, to, piece };
            self.emit_move(promo);
        }
    }

    fn left(&mut self, stop_at: StopCondition) {
        for c in (0..self.col()).rev() {
            if !self.pos((self.row(), c), stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }
    }

    fn right(&mut self, stop_at: StopCondition) {
        for c in self.col() + 1..8 {
            if !self.pos((self.row(), c), stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }
    }

    fn cross(&mut self, stop_at: StopCondition) {
        let (row, col) = (self.row(), self.col());

        for r in (0..row).rev() {
            if !self.pos((r, col), stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }

        for r in row + 1..8 {
            if !self.pos((r, col), stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }

        self.left(stop_at);
        self.right(stop_at);
    }

    fn diagonals(&mut self, stop_at: StopCondition) {
        let (row, col) = (self.row(), self.col());

        for pos in zip(row + 1..8, col + 1..8) {
            if !self.pos(pos, stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }

        for pos in zip((0..row).rev(), col + 1..8) {
            if !self.pos(pos, stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }

        for pos in zip(row + 1..8, (0..col).rev()) {
            if !self.pos(pos, stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }

        for pos in zip((0..row).rev(), (0..col).rev()) {
            if !self.pos(pos, stop_at).is_some_and(|p| !p.stop()) {
                break;
            }
        }
    }

    fn black_pawn(&mut self) {
        let pawns = self.board.get_piece(Color::B, Piece::Pawn) & self.from.bb();
        let white_side = self.board.side(Color::W);
        let side_attack = (Bits::southeast(pawns) & Magic::NOT_A_FILE & white_side)
            | (Bits::southwest(pawns) & Magic::NOT_H_FILE & white_side);
        self.takes_from_magic(side_attack);

        let first_push = Bits::south(pawns) & !white_side;
        let second_push = Bits::south(first_push & Magic::RANK_6) & !white_side;
        let pushes = first_push | second_push;
        self.slides_from_magic(pushes);
    }

    fn white_pawn(&mut self) {
        let pawns = self.board.get_piece(Color::W, Piece::Pawn) & self.from.bb();
        let black_side = self.board.side(Color::B);
        let side_attack = (Bits::northeast(pawns) & Magic::NOT_A_FILE & black_side)
            | (Bits::northwest(pawns) & Magic::NOT_H_FILE & black_side);
        self.takes_from_magic(side_attack);

        let first_push = Bits::north(pawns) & !black_side;
        let second_push = Bits::north(first_push & Magic::RANK_3) & !black_side;
        let pushes = first_push | second_push;
        self.slides_from_magic(pushes);
    }

    fn queen(&mut self) {
        self.diagonals(empty_or_take);
        self.cross(empty_or_take);
    }

    fn knight(&mut self) {
        let bb = KNIGHT_MAGIC[self.from.sq()];
        self.moves_from_magic(bb);
    }

    fn king(&mut self) {
        self.king_castle();

        let bb = KING_MAGIC[self.from.sq()];
        self.moves_from_magic(bb);
    }

    fn king_castle(&mut self) {
        let rights = self.board.castling_rights(self.color);

        if Castling::None == rights {
            return;
        }

        if let Castling::Some(left, right) = rights {
            let king = self.board.get_piece(self.color, self.piece);
            let king = Bits::pos(king);
            let pos = king.first().unwrap_or_else(|| unreachable!("should have king position"));
            if right {
                let mut subgen = Generator::from_board(self.board, *pos, false);
                subgen.right(is_empty);
                if subgen.moves().len() == 2 {
                    self.emit_move(Move::RightCastle { mover: self.color });
                }
            }

            if left {
                let mut subgen = Generator::from_board(self.board, *pos, false);
                subgen.left(is_empty);
                if subgen.moves().len() == 3 {
                    self.emit_move(Move::LeftCastle { mover: self.color });
                }
            }
        }
    }

    fn is_legal(&self, movement: Move) -> bool {
        !movement.apply(self.board).in_check(self.color)
    }

    fn row(&self) -> usize {
        self.from.row()
    }

    fn col(&self) -> usize {
        self.from.col()
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

        print_board(&board, &[]);

        board.next_turn();

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
}
