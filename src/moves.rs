pub mod generator;
mod movement;
mod placement;

use crate::bitboard::Bits;
use crate::board::Castling;
use crate::defs::Dir;
use crate::magic::{Magic, KNIGHT_MAGIC};
use crate::Pos;
use crate::{board::Board, piece::Piece, print_board, Color};

pub use self::movement::Move;

use self::{
    generator::Generator,
    placement::{empty_or_take, is_empty},
};

pub struct MoveGen<'board> {
    board: &'board Board,
    color: Color,
    piece: Piece,
    from: Pos,
}

impl<'board> MoveGen<'board> {
    pub fn new<P: Into<Pos>>(board: &'board Board, from: P) -> Self {
        let from = from.into();
        let (color, piece, _) = board.at(from).unwrap_or_else(|| {
            print_board(board, &[]);
            unreachable!("cannot generate moves for empty position {from:?}")
        });

        Self { board, color, piece, from }
    }

    pub fn generate(self, check_legal: bool) -> Vec<Move> {
        let mut gen = Generator::new(self.board, self.from, self.color, self.piece, check_legal);
        match self.piece {
            Piece::Pawn => match self.color {
                Color::B => black_pawn(self.board, self.from, &mut gen),
                Color::W => white_pawn(self.board, self.from, &mut gen),
            },
            Piece::Rook => gen.cross(empty_or_take),
            Piece::Bishop => gen.diagonals(empty_or_take),
            Piece::Queen => {
                gen.diagonals(empty_or_take);
                gen.cross(empty_or_take);
            }
            Piece::Knight => {
                let bb = KNIGHT_MAGIC[self.from.sq()];
                gen.moves_from_magic(bb);
            }
            Piece::King => {
                king(&mut gen);
                self.king_castle(&mut gen);
            }
        };
        gen.moves()
    }

    fn king_castle(&self, gen: &mut Generator) {
        let rights = self.board.castling_rights(self.color);

        if Castling::None == rights {
            return;
        }

        if let Castling::Some(left, right) = rights {
            let king = self.board.get_piece(self.board.mover(), Piece::King);
            let king = Bits::pos(king);
            let pos = king.first().expect("should be there");
            if right {
                let mut subgen = Generator::new(self.board, *pos, self.color, self.piece, false);
                subgen.right(is_empty);
                if subgen.moves().len() == 2 {
                    gen.emit_move(Move::RightCastle { mover: self.board.mover() });
                }
            }

            if left {
                let mut subgen = Generator::new(self.board, *pos, self.color, self.piece, false);
                subgen.left(is_empty);
                if subgen.moves().len() == 3 {
                    gen.emit_move(Move::LeftCastle { mover: self.board.mover() });
                }
            }
        }
    }
}

fn black_pawn(board: &Board, from: Pos, g: &mut Generator) {
    let pawns = board.get_piece(Color::B, Piece::Pawn) & from.bb();
    let white_side = board.side(Color::W);
    let side_attack = (Bits::southeast(pawns) & Magic::NOT_A_FILE & white_side)
        | (Bits::southwest(pawns) & Magic::NOT_H_FILE & white_side);
    g.takes_from_magic(side_attack);

    let first_push = Bits::south(pawns) & !white_side;
    let second_push = Bits::south(first_push & Magic::RANK_6) & !white_side;
    let pushes = first_push | second_push;
    g.slides_from_magic(pushes);
}

fn white_pawn(board: &Board, from: Pos, g: &mut Generator) {
    let pawns = board.get_piece(Color::W, Piece::Pawn) & from.bb();
    let black_side = board.side(Color::B);
    let side_attack = (Bits::northeast(pawns) & Magic::NOT_A_FILE & black_side)
        | (Bits::northwest(pawns) & Magic::NOT_H_FILE & black_side);
    g.takes_from_magic(side_attack);

    let first_push = Bits::north(pawns) & !black_side;
    let second_push = Bits::north(first_push & Magic::RANK_3) & !black_side;
    let pushes = first_push | second_push;
    g.slides_from_magic(pushes);
}

#[deprecated]
pub fn knight(g: &mut Generator) {
    let has_one_right = g.col() < 7;
    let has_two_right = g.col() < 6;
    let has_one_left = g.col() > 0;
    let has_two_left = g.col() > 1;

    if g.row() < 6 {
        if has_one_left {
            g.dir(Dir::Custom(2, -1), empty_or_take);
        }
        if has_one_right {
            g.dir(Dir::Custom(2, 1), empty_or_take);
        }
    }

    if g.row() > 2 {
        if has_one_left {
            g.dir(Dir::Custom(-2, -1), empty_or_take);
        }
        if has_one_right {
            g.dir(Dir::Custom(-2, 1), empty_or_take);
        }
    }

    if g.row() < 7 {
        if has_two_left {
            g.dir(Dir::Custom(1, -2), empty_or_take);
        }
        if has_two_right {
            g.dir(Dir::Custom(1, 2), empty_or_take);
        }
    }

    if g.row() > 0 {
        if has_two_left {
            g.dir(Dir::Custom(-1, -2), empty_or_take);
        }
        if has_two_right {
            g.dir(Dir::Custom(-1, 2), empty_or_take);
        }
    }
}

fn king(g: &mut Generator) {
    if g.row() < 7 {
        g.dir(Dir::Up, empty_or_take);

        if g.col() < 7 {
            g.dir(Dir::Custom(1, 1), empty_or_take);
        }

        if g.col() > 0 {
            g.dir(Dir::Custom(1, -1), empty_or_take);
        }
    }

    if g.row() > 0 {
        g.dir(Dir::Down, empty_or_take);

        if g.col() < 7 {
            g.dir(Dir::Custom(-1, 1), empty_or_take);
        }

        if g.col() > 0 {
            g.dir(Dir::Custom(-1, -1), empty_or_take);
        }
    }

    if g.col() < 7 {
        g.dir(Dir::Right, empty_or_take);
    }

    if g.col() > 0 {
        g.dir(Dir::Left, empty_or_take);
    }
}

#[cfg(test)]
mod test {

    use crate::{defs::Sq, Pos};

    use super::*;

    fn gen_squares<P: Into<Pos>>(board: &Board, pos: P) -> Vec<Pos> {
        let m = MoveGen::new(board, pos).generate(true);
        m.iter().map(|m| m.to()).collect()
    }

    fn assert_moves(expected: Vec<Sq>, actual: Vec<Pos>) {
        let actual: Vec<Sq> = actual.into_iter().map(|p| p.sq()).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn white_pawn_gen() {
        let mut board = Board::default();

        Bits::set(&mut board.black[0], Pos::new(3, 5));
        Bits::set(&mut board.black[0], Pos::new(2, 2));
        Bits::set(&mut board.white[0], Pos::new(4, 5));
        Bits::set(&mut board.white[0], Pos::new(5, 7));

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

        Bits::set(&mut board.black[0], Pos::new(3, 5));
        Bits::set(&mut board.black[0], Pos::new(2, 7));
        Bits::set(&mut board.white[0], Pos::new(4, 5));
        Bits::set(&mut board.white[0], Pos::new(5, 7));

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
}
