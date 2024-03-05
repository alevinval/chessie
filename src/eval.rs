use rand::Rng;

use crate::{
    board::Board,
    pieces::{BitBoard, Piece},
    Color,
};

#[derive(Default)]
pub struct Scorer {}

impl Scorer {
    pub fn eval(board: &Board, maxer: Color) -> f32 {
        Self::inner_eval(board, maxer, false)
    }

    pub fn debug_eval(board: &Board, maxer: Color) -> f32 {
        Self::inner_eval(board, maxer, true)
    }

    fn inner_eval(board: &Board, maxer: Color, debug: bool) -> f32 {
        if board.pieces_for(maxer).king.is_empty() {
            return f32::NEG_INFINITY;
        } else if board.pieces_for(maxer.opposite()).king.is_empty() {
            return f32::INFINITY;
        }

        let offset = rand::thread_rng().gen_range(-0.00001..0.00001);
        let white = Scorer::score(board, Color::W, debug);
        let black = Scorer::score(board, Color::B, debug);
        let score = match maxer {
            Color::B => black - white,
            Color::W => white - black,
        };
        score + offset
    }

    fn score(board: &Board, color: Color, debug: bool) -> f32 {
        let material_score: f32 = board
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

    fn score_piece(piece: Piece) -> f32 {
        match piece {
            Piece::Pawn(_) => 1.0,
            Piece::Rook(_, _, _) => 5.0,
            Piece::Knight(_) => 2.8,
            Piece::Bishop(_) => 3.0,
            Piece::Queen(_) => 9.0,
            Piece::King(_, _) => 0.0,
        }
    }

    fn score_bitboard(bitboard: &BitBoard) -> f32 {
        bitboard.iter_pos().count() as f32 * Self::score_piece(bitboard.piece())
    }
}
