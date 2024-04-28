use crate::{bits, board::Board, color::Color, defs::BitBoard, piece::Piece};

use super::{score_piece, MATE_SCORE};

#[derive(Default)]
pub(crate) struct LegacyScorer {}

impl LegacyScorer {
    pub(crate) fn eval(board: &Board) -> f64 {
        let mover = board.state().mover();
        Self::score(board, mover, false) - Self::score(board, mover.flip(), false)
    }

    #[allow(dead_code)]
    pub(crate) fn debug_eval(board: &Board) -> f64 {
        let mover = board.state().mover();
        Self::score(board, mover, true) - Self::score(board, mover.flip(), true)
    }

    fn score(board: &Board, color: Color, debug: bool) -> f64 {
        if board.get(board.state().mover(), Piece::King) == 0
            || (board.in_check(color) && board.movements(color).is_empty())
        {
            return -MATE_SCORE;
        }

        let material_score: f64 =
            board.pieces(color).map(|(piece, bb)| Self::score_bitboard(piece, bb)).sum();

        if debug {
            println!("{color:?}");
            println!("  material: {material_score}");
        }

        material_score
    }

    #[allow(clippy::cast_precision_loss)]
    fn score_bitboard(piece: Piece, bb: BitBoard) -> f64 {
        bits::count(bb) as f64 * score_piece(piece)
    }
}
