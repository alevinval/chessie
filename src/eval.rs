use rand::Rng;

use crate::defs::BitBoard;
use crate::{bitboard::Bits, board::Board, piece::Piece, Color};

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
        if board.get_piece(maxer, Piece::King) == 0 {
            return f64::NEG_INFINITY;
        } else if board.get_piece(maxer.opposite(), Piece::King) == 0 {
            return f64::INFINITY;
        }

        let offset = if jitter { rand::thread_rng().gen_range(-0.00001..0.00001) } else { 0.0 };
        let white = Scorer::score(board, Color::W, debug);
        let black = Scorer::score(board, Color::B, debug);
        let score = match maxer {
            Color::B => black - white,
            Color::W => white - black,
        };
        score + offset
    }

    fn score(board: &Board, color: Color, debug: bool) -> f64 {
        let material_score: f64 =
            board.pieces_iter(color).map(|(p, bb)| Self::score_bitboard(p, bb)).sum();

        if debug {
            println!("{color:?}");
            println!("  material: {material_score}");
        }

        material_score
    }

    fn score_piece(piece: Piece) -> f64 {
        match piece {
            Piece::Pawn => 1.0,
            Piece::Rook => 5.0,
            Piece::Knight => 2.8,
            Piece::Bishop => 3.0,
            Piece::Queen => 9.0,
            Piece::King => 0.0,
        }
    }

    fn score_bitboard(piece: Piece, bb: BitBoard) -> f64 {
        Bits::count(bb) as f64 * Self::score_piece(piece)
    }
}
