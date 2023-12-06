use crate::{BitBoard, Pos};

pub static ORIGIN: &Pos = &Pos(0, 0);

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self(row, col)
    }

    pub fn row(&self) -> usize {
        self.0
    }

    pub fn col(&self) -> usize {
        self.1
    }

    pub fn u(&self) -> Pos {
        Self(self.0 + 1, self.1)
    }

    pub fn d(&self) -> Pos {
        Self(self.0 - 1, self.1)
    }

    pub fn ur(&self) -> Pos {
        Self(self.0 + 1, self.1 + 1)
    }

    pub fn ul(&self) -> Pos {
        Self(self.0 + 1, self.1 - 1)
    }

    pub fn dr(&self) -> Pos {
        Self(self.0 - 1, self.1 + 1)
    }

    pub fn dl(&self) -> Pos {
        Self(self.0 - 1, self.1 - 1)
    }

    pub fn as_bit_board(&self) -> BitBoard {
        (1 << self.0 * 8) << self.1
    }
}
