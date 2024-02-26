use crate::{
    board::Board,
    pieces::{BitBoard, Color, Piece},
};

#[derive(Default)]
pub struct Scorer {}

impl Scorer {
    pub fn eval(&self, board: &Board, color: Color) -> f32 {
        self.inner_eval(board, color, false)
    }

    pub fn debug_eval(&self, board: &Board, color: Color) -> f32 {
        self.inner_eval(board, color, true)
    }

    fn inner_eval(&self, board: &Board, color: Color, debug: bool) -> f32 {
        let mut white: f32 = board
            .pieces(Color::White)
            .iter()
            .map(|ps| self.score_bitboard(ps))
            .sum();
        let mut black: f32 = board
            .pieces(Color::Black)
            .iter()
            .map(|ps| self.score_bitboard(ps))
            .sum();

        if debug {
            println!("material score");
            println!(" white: {white}");
            println!(" black: {black}");
        }

        let white_space_score: f32 = board
            .pieces(Color::White)
            .get(Piece::Pawn(Color::White))
            .iter_pos()
            .map(|p| (p.row() as f32 - 1.0) * if p.is_central() { 1.2 } else { 1.0 })
            .sum();

        let black_space_score: f32 = board
            .pieces(Color::Black)
            .get(Piece::Pawn(Color::Black))
            .iter_pos()
            .map(|p| (p.row() as f32 - 6.0) * if p.is_central() { -1.2 } else { -1.0 })
            .sum();

        white += white_space_score / 100.0;
        black += black_space_score / 100.0;

        if debug {
            println!("space score");
            println!(" white: {white_space_score}");
            println!(" black: {black_space_score}");
        }

        match color {
            Color::Black => black - white,
            Color::White => white - black,
        }
    }

    fn score_piece(&self, piece: &Piece) -> f32 {
        match piece {
            Piece::Pawn(_) => 1.0,
            Piece::Rook(_) => 5.0,
            Piece::Knight(_) => 2.8,
            Piece::Bishop(_) => 3.0,
            Piece::Queen(_) => 9.0,
            Piece::King(_) => 1000.0,
        }
    }

    fn score_bitboard(&self, bitboard: &BitBoard) -> f32 {
        bitboard
            .iter_pos()
            .map(|p| self.score_piece(&bitboard.piece()) + if p.is_central() { 0.25 } else { 0.0 })
            .sum()
    }
}
