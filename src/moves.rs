mod generator;
mod movement;
mod placement;

use crate::bitboard::Bits;
use crate::board::Castling;
use crate::{board::Board, piece::Piece, pos::Dir, print_board, Color, Pos};

pub use self::movement::Move;

use self::{
    generator::Generator,
    placement::{empty_or_take, is_empty, takes},
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

        Self {
            board,
            color,
            piece,
            from,
        }
    }

    pub fn generate(self, check_legal: bool) -> Vec<Move> {
        let mut gen = Generator::new(self.board, self.from, check_legal);
        match self.piece {
            Piece::Pawn => match self.color {
                Color::B => black_pawn(&mut gen),
                Color::W => white_pawn(&mut gen),
            },
            Piece::Rook => gen.cross(empty_or_take),
            Piece::Bishop => gen.diagonals(empty_or_take),
            Piece::Queen => {
                gen.diagonals(empty_or_take);
                gen.cross(empty_or_take);
            }
            Piece::Knight => knight(&mut gen),
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
            let pos = Bits::iter_pos(king).next().expect("should be there");
            if right {
                let mut subgen = Generator::new(self.board, pos, false);
                subgen.right(is_empty);
                if subgen.moves().len() == 2 {
                    gen.emit_move(Move::RightCastle {
                        mover: self.board.mover(),
                    });
                }
            }

            if left {
                let mut subgen = Generator::new(self.board, pos, false);
                subgen.left(is_empty);
                if subgen.moves().len() == 3 {
                    gen.emit_move(Move::LeftCastle {
                        mover: self.board.mover(),
                    });
                }
            }
        }
    }
}

fn black_pawn(g: &mut Generator) {
    if g.row() > 1 {
        if g.dir(Dir::Down(1), is_empty).is_some() && g.row() == 6 {
            g.dir(Dir::Down(2), is_empty);
        }

        if g.col() < 7 {
            g.dir(Dir::Custom(-1, 1), takes);
        }

        if g.col() > 0 {
            g.dir(Dir::Custom(-1, -1), takes);
        }
    } else {
        if g.check_dir(Dir::Down(1), is_empty).is_some() {
            g.pawn_promo(Dir::Down(1));
        }
        if g.col() < 7 && g.check_dir(Dir::Custom(-1, 1), takes).is_some() {
            g.pawn_promo(Dir::Custom(-1, 1));
        }
        if g.col() > 0 && g.check_dir(Dir::Custom(-1, -1), takes).is_some() {
            g.pawn_promo(Dir::Custom(-1, -1));
        }
    }
}

fn white_pawn(g: &mut Generator) {
    if g.row() < 6 {
        if g.dir(Dir::Up(1), is_empty).is_some() && g.row() == 1 {
            g.dir(Dir::Up(2), is_empty);
        }
        if g.col() < 7 {
            g.dir(Dir::Custom(1, 1), takes);
        }
        if g.col() > 0 {
            g.dir(Dir::Custom(1, -1), takes);
        }
    } else {
        if g.check_dir(Dir::Up(1), is_empty).is_some() {
            g.pawn_promo(Dir::Up(1));
        }
        if g.col() < 7 && g.check_dir(Dir::Custom(1, 1), takes).is_some() {
            g.pawn_promo(Dir::Custom(1, 1));
        }
        if g.col() > 0 && g.check_dir(Dir::Custom(1, -1), takes).is_some() {
            g.pawn_promo(Dir::Custom(1, -1));
        }
    }
}

fn knight(g: &mut Generator) {
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
        g.dir(Dir::Up(1), empty_or_take);

        if g.col() < 7 {
            g.dir(Dir::Custom(1, 1), empty_or_take);
        }

        if g.col() > 0 {
            g.dir(Dir::Custom(1, -1), empty_or_take);
        }
    }

    if g.row() > 0 {
        g.dir(Dir::Down(1), empty_or_take);

        if g.col() < 7 {
            g.dir(Dir::Custom(-1, 1), empty_or_take);
        }

        if g.col() > 0 {
            g.dir(Dir::Custom(-1, -1), empty_or_take);
        }
    }

    if g.col() < 7 {
        g.dir(Dir::Right(1), empty_or_take);
    }

    if g.col() > 0 {
        g.dir(Dir::Left(1), empty_or_take);
    }
}
