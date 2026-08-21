use crate::{bits, board::Board, color::Color, defs::BitBoard, piece::Piece};

pub mod legacy;

pub const MATE_SCORE: i32 = 100_000_000;

#[derive(Default)]
pub struct Scorer {}

impl Scorer {
    pub fn eval(board: &Board) -> i32 {
        let mover = board.state().mover();
        Self::score(board, mover, false) - Self::score(board, mover.flip(), false)
    }

    #[allow(dead_code)]
    pub(crate) fn debug_eval(board: &Board) -> i32 {
        let mover = board.state().mover();
        Self::score(board, mover, true) - Self::score(board, mover.flip(), true)
    }

    fn score(board: &Board, color: Color, debug: bool) -> i32 {
        let material_score: i32 =
            board.pieces(color).map(|(piece, bb)| Self::score_bitboard(piece, bb)).sum();

        if debug {
            println!("{color:?}");
            println!("  material: {material_score}");
        }

        material_score
    }

    fn score_bitboard(piece: Piece, bb: BitBoard) -> i32 {
        score_material::score(piece, bb) + score_centrality::score(piece, bb)
    }
}

pub(crate) mod score_material {
    use super::*;

    pub(super) fn score(piece: Piece, bb: BitBoard) -> i32 {
        bits::count(bb) as i32 * piece_value(piece)
    }

    pub const fn piece_value(piece: Piece) -> i32 {
        match piece {
            Piece::Pawn => 100,
            Piece::Rook => 500,
            Piece::Knight => 280,
            Piece::Bishop => 300,
            Piece::Queen => 900,
            Piece::King => 0,
        }
    }
}

mod score_centrality {
    use super::*;

    #[rustfmt::skip]
const CENTRALITY_BIAS: [i32; 64] = [
     0,  1,  2,  3,  3,  2,  1,  0,
     1,  2,  3,  4,  4,  3,  2,  1,
     2,  3,  4,  5,  5,  4,  3,  2,
     3,  4,  5,  6,  6,  5,  4,  3,
     3,  4,  5,  6,  6,  5,  4,  3,
     2,  3,  4,  5,  5,  4,  3,  2,
     1,  2,  3,  4,  4,  3,  2,  1,
     0,  1,  2,  3,  3,  2,  1,  0,
];

    pub(super) fn score(piece: Piece, bb: BitBoard) -> i32 {
        if !matches!(piece, Piece::Knight | Piece::Bishop) {
            return 0;
        }
        bits::pos(bb).into_iter().map(|sq| CENTRALITY_BIAS[sq as usize]).sum()
    }
}
