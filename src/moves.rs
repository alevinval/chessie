mod generator;
mod movement;
mod placement;

use crate::{
    board::Board,
    pieces::{BitBoard, Color, Piece},
    pos::Dir,
    Pos,
};

pub use self::movement::Move;

use self::{
    generator::Generator,
    placement::{empty_or_take, is_empty, takes},
};

pub struct MoveGen<'board> {
    board: &'board Board,
    bitboard: &'board BitBoard,
    from: Pos,
}

impl<'board> MoveGen<'board> {
    pub fn new<P: Into<Pos>>(board: &'board Board, bitboard: &'board BitBoard, from: P) -> Self {
        Self {
            board,
            bitboard,
            from: from.into(),
        }
    }

    pub fn gen(self) -> Vec<Move> {
        let mut gen = Generator::new(self.board, self.from);
        match self.bitboard.piece() {
            Piece::Pawn(color) => match color {
                Color::Black => black_pawn(&mut gen),
                Color::White => white_pawn(&mut gen),
            },
            Piece::Rook(_, _, _) => gen.cross(empty_or_take),
            Piece::Bishop(_) => gen.diagonals(empty_or_take),
            Piece::Queen(_) => {
                gen.diagonals(empty_or_take);
                gen.cross(empty_or_take);
            }
            Piece::Knight(_) => knight(&mut gen),
            Piece::King(_, _) => {
                king(&mut gen);
                self.king_castle(&mut gen);
            }
        };
        gen.moves()
    }

    fn king_castle(&self, gen: &mut Generator) {
        let color = self.bitboard.color();
        let pieces = self.board.pieces(color);
        if let Piece::King(_, has_moved) = pieces.king.piece() {
            if has_moved {
                return;
            }
        }

        if let Piece::Rook(_, lrm, rrm) = pieces.rooks.piece() {
            let pos = pieces.king.iter_pos().next().expect("should be there");
            if !rrm {
                let mut subgen = Generator::new(self.board, pos);
                subgen.right(is_empty);
                if subgen.moves().len() == 2 {
                    gen.mov(Move::RightCastle(color));
                }
            }
            if !lrm {
                let mut subgen = Generator::new(self.board, pos);
                subgen.left(is_empty);
                if subgen.moves().len() == 3 {
                    gen.mov(Move::LeftCastle(color));
                }
            }
        }
    }
}

fn black_pawn(g: &mut Generator) {
    if g.row() > 0 {
        if g.dir(Dir::Down(1), is_empty).placed() && g.row() == 6 {
            g.dir(Dir::Down(2), is_empty);
        }

        if g.col() < 7 {
            g.dir(Dir::Custom(-1, 1), takes);
        }

        if g.col() > 0 {
            g.dir(Dir::Custom(-1, -1), takes);
        }
    }
}

fn white_pawn(g: &mut Generator) {
    if g.row() < 7 {
        if g.dir(Dir::Up(1), is_empty).placed() && g.row() == 1 {
            g.dir(Dir::Up(2), is_empty);
        }
        if g.col() < 7 {
            g.dir(Dir::Custom(1, 1), takes);
        }
        if g.col() > 0 {
            g.dir(Dir::Custom(1, -1), takes);
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
