use crate::piece::Piece;

pub mod legacy;

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
