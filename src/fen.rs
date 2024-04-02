pub(crate) use encode::encode;
pub(crate) use error::FenError;

use crate::{color::Color, piece::Piece};

mod decode;
mod encode;
mod error;

#[must_use]
const fn piece_to_fen(color: Color, piece: Piece) -> char {
    let fen = match piece {
        Piece::Pawn => 'p',
        Piece::Knight => 'n',
        Piece::Bishop => 'b',
        Piece::Rook => 'r',
        Piece::Queen => 'q',
        Piece::King => 'k',
    };

    if matches!(color, Color::W) {
        fen.to_ascii_uppercase()
    } else {
        fen
    }
}

fn fen_to_piece(ch: char) -> Result<(Color, Piece), FenError> {
    let piece = match ch.to_ascii_lowercase() {
        'p' => Piece::Pawn,
        'n' => Piece::Knight,
        'b' => Piece::Bishop,
        'r' => Piece::Rook,
        'q' => Piece::Queen,
        'k' => Piece::King,
        _ => return Err(FenError::Invalid),
    };

    let color = if ch.is_lowercase() { Color::B } else { Color::W };

    Ok((color, piece))
}
