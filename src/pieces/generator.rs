use crate::{
    board::Board,
    pos::{Direction, Pos},
};

use super::BitBoard;

#[derive(Debug)]
pub struct Generator<'b> {
    board: &'b Board,
    from: Pos,
    moves: BitBoard,
}

pub enum Placement {
    No,
    EmptyCell,
    Takes,
}

impl Placement {
    pub fn should_stop(&self) -> bool {
        matches!(self, Self::No | Self::Takes)
    }

    pub fn no(&self) -> bool {
        matches!(self, Self::No)
    }

    pub fn yes(&self) -> bool {
        !self.no()
    }
}

impl<'b> Generator<'b> {
    pub fn new(board: &'b Board, from: Pos) -> Self {
        Generator {
            board,
            from,
            moves: BitBoard::default(),
        }
    }

    pub fn row(&self) -> u8 {
        self.from.row()
    }

    pub fn col(&self) -> u8 {
        self.from.col()
    }

    pub fn dir(&mut self, d: Direction, condition: fn(&Board, Pos, Pos) -> Placement) -> Placement {
        let to = self.from.to(d);
        self.pos(to, condition)
    }

    pub fn pos<P: Into<Pos>>(
        &mut self,
        to: P,
        condition: fn(&Board, Pos, Pos) -> Placement,
    ) -> Placement {
        let to = to.into();
        match condition(self.board, self.from, to) {
            Placement::No => Placement::No,
            placement => {
                self.moves.or_mut(to);
                placement
            }
        }
    }

    pub fn moves(self) -> BitBoard {
        self.moves
    }
}
