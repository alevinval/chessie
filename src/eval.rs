use rand::Rng;

use crate::{
    board::Board,
    pieces::{BitBoard, Piece},
    Color,
};

#[derive(Default)]
pub struct Scorer {}

impl Scorer {
    pub fn eval(board: &Board, maxer: Color, jitter: bool) -> f64 {
        Self::inner_eval(board, maxer, false, jitter)
    }

    pub fn debug_eval(board: &Board, maxer: Color) -> f64 {
        Self::inner_eval(board, maxer, true, false)
    }

    fn inner_eval(board: &Board, maxer: Color, debug: bool, jitter: bool) -> f64 {
        if board.pieces_for(maxer).king.is_empty() {
            return f64::NEG_INFINITY;
        } else if board.pieces_for(maxer.opposite()).king.is_empty() {
            return f64::INFINITY;
        }

        let offset = if jitter {
            rand::thread_rng().gen_range(-0.00001..0.00001)
        } else {
            0.0
        };
        let white = Scorer::score(board, Color::W, debug);
        let black = Scorer::score(board, Color::B, debug);
        let score = match maxer {
            Color::B => black - white,
            Color::W => white - black,
        };
        score / 100.0 + offset
    }

    fn score(board: &Board, color: Color, debug: bool) -> f64 {
        let material_score: f64 = board
            .pieces_for(color)
            .iter()
            .map(Self::score_bitboard)
            .sum();

        if debug {
            println!("{color:?}");
            println!("  material: {material_score}");
        }

        material_score
    }

    fn score_piece(piece: Piece) -> f64 {
        match piece {
            Piece::Pawn(_) => 100.0,
            Piece::Rook(_, _, _) => 500.0,
            Piece::Knight(_) => 280.0,
            Piece::Bishop(_) => 300.0,
            Piece::Queen(_) => 900.0,
            Piece::King(_, _) => 0.0,
        }
    }

    fn score_bitboard(bitboard: &BitBoard) -> f64 {
        bitboard.iter_pos().count() as f64 * Self::score_piece(bitboard.piece())
    }
}
