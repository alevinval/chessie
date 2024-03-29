use rand::Rng;

use crate::{bits::Bits, board::Board, defs::BitBoard, piece::Piece, Color};

#[derive(Default)]
pub(crate) struct Scorer {}

impl Scorer {
    pub(crate) fn eval(board: &Board, maxer: Color, jitter: bool) -> f64 {
        Self::inner_eval(board, maxer, false, jitter)
    }

    #[allow(dead_code)]
    pub(crate) fn debug_eval(board: &Board, maxer: Color) -> f64 {
        Self::inner_eval(board, maxer, true, false)
    }

    fn inner_eval(board: &Board, maxer: Color, debug: bool, jitter: bool) -> f64 {
        if board.get(maxer, Piece::King) == 0 {
            return f64::NEG_INFINITY;
        } else if board.get(maxer.flip(), Piece::King) == 0 {
            return f64::INFINITY;
        }

        let offset = if jitter { rand::thread_rng().gen_range(-0.00001..0.00001) } else { 0.0 };
        let white = Scorer::score(board, Color::W, debug);
        let black = Scorer::score(board, Color::B, debug);
        let score = match maxer {
            Color::B => black - white,
            Color::W => white - black,
        };
        score / 100.0 + offset
    }

    fn score(board: &Board, color: Color, debug: bool) -> f64 {
        let material_score: f64 =
            board.pieces(color).map(|(piece, bb)| Self::score_bitboard(piece, bb)).sum();

        if debug {
            println!("{color:?}");
            println!("  material: {material_score}");
        }

        material_score
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

    fn score_bitboard(piece: Piece, bb: BitBoard) -> f64 {
        Bits::count(bb) as f64 * Self::score_piece(piece)
    }
}
