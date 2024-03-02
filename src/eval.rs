use crate::{
    board::Board,
    pieces::{BitBoard, Piece},
    Color,
};

#[derive(Default)]
pub struct Scorer {}

impl Scorer {
    pub fn eval(board: &Board, color: Color) -> f32 {
        Self::inner_eval(board, color, false)
    }

    pub fn debug_eval(board: &Board, color: Color) -> f32 {
        Self::inner_eval(board, color, true)
    }

    fn inner_eval(board: &Board, color: Color, debug: bool) -> f32 {
        let white = Scorer::score(board, Color::W, debug);
        let black = Scorer::score(board, Color::B, debug);
        match color {
            Color::B => black - white,
            Color::W => white - black,
        }
    }

    fn score(board: &Board, color: Color, debug: bool) -> f32 {
        let material_score: f32 = board
            .pieces_for(color)
            .iter()
            .map(Self::score_bitboard)
            .sum();

        let mut space_score: f32 = board
            .pieces_for(color)
            .pawns
            .iter_pos()
            .map(|p| if p.is_central() { 1.2 } else { 1.0 })
            .sum();
        space_score /= 100.0;

        let mut king_score = 0.0;

        let king = &board.pieces_for(color).king;
        if let Piece::King(_, moved) = king.piece() {
            if let Piece::Rook(_, left, right) = &board.pieces_for(color).rooks.piece() {
                if !moved && !right {
                    king_score -= 0.2;
                } else if !moved && !left {
                    king_score -= 0.1;
                }
            }
        }

        let total = material_score + space_score + king_score;

        if debug {
            println!("----");
            println!("{color:?}:");
            println!("  material: {material_score}");
            println!("     space: {space_score}");
            println!("      king: {king_score}");
            println!("     total: {total}");
        }

        total
    }

    fn score_piece(piece: Piece) -> f32 {
        match piece {
            Piece::Pawn(_) => 1.0,
            Piece::Rook(_, _, _) => 5.0,
            Piece::Knight(_) => 2.8,
            Piece::Bishop(_) => 3.0,
            Piece::Queen(_) => 9.0,
            Piece::King(_, _) => 25.0,
        }
    }

    fn score_bitboard(bitboard: &BitBoard) -> f32 {
        bitboard
            .iter_pos()
            .map(|p| Self::score_piece(bitboard.piece()) + if p.is_central() { 0.25 } else { 0.0 })
            .sum()
    }
}
