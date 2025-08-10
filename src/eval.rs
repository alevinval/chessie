use crate::{bits, board::Board, color::Color, defs::BitBoard, piece::Piece};

pub mod legacy;

pub const MATE_SCORE: f64 = 100_000_000.0;

#[derive(Default)]
pub struct Scorer {}

impl Scorer {
    pub fn eval(board: &Board) -> f64 {
        let mover = board.state().mover();
        Self::score(board, mover, false) - Self::score(board, mover.flip(), false)
    }

    #[allow(dead_code)]
    pub(crate) fn debug_eval(board: &Board) -> f64 {
        let mover = board.state().mover();
        Self::score(board, mover, true) - Self::score(board, mover.flip(), true)
    }

    fn score(board: &Board, color: Color, debug: bool) -> f64 {
        if board.get(color, Piece::King) == 0
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

pub(crate) const fn score_piece(piece: Piece) -> f64 {
    match piece {
        Piece::Pawn => 100.0,
        Piece::Rook => 500.0,
        Piece::Knight => 280.0,
        Piece::Bishop => 300.0,
        Piece::Queen => 900.0,
        Piece::King => 20000.0,
    }
}
