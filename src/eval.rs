use crate::{bits::Bits, board::Board, color::Color, defs::BitBoard, piece::Piece};

pub mod legacy;

#[derive(Default)]
pub(crate) struct Scorer {}

impl Scorer {
    pub(crate) fn eval(board: &Board, maxer: Color) -> f64 {
        Self::inner_eval(board, maxer, false)
    }

    #[allow(dead_code)]
    pub(crate) fn debug_eval(board: &Board, maxer: Color) -> f64 {
        Self::inner_eval(board, maxer, true)
    }

    fn inner_eval(board: &Board, maxer: Color, debug: bool) -> f64 {
        if board.get(maxer, Piece::King) == 0 {
            return f64::NEG_INFINITY;
        } else if board.get(maxer.flip(), Piece::King) == 0 {
            return f64::INFINITY;
        }

        let white = Self::score(board, Color::W, debug);
        let black = Self::score(board, Color::B, debug);
        let score = match maxer {
            Color::B => black - white,
            Color::W => white - black,
        };
        score / 100.0
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

    fn score_bitboard(piece: Piece, bb: BitBoard) -> f64 {
        Bits::count(bb) as f64 * score_piece(piece)
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
