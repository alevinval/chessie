use crate::{
    board::{Board, GameState},
    color::Color,
    piece::Piece,
    sq,
};

pub(crate) fn encode(board: &Board) -> String {
    let mut out = String::new();
    let state = board.state();

    encode_ranks(&mut out, board);
    encode_mover(&mut out, state);
    encode_castling(&mut out, state);
    encode_enpassant(&mut out);
    encode_moves(&mut out, state);
    out
}

fn encode_ranks(out: &mut String, board: &Board) {
    for rank in (0..8).rev() {
        let mut acc = 0;
        for col in 0..8 {
            let piece = board.at(sq!(rank, col));
            if let Some((color, piece, _)) = piece {
                if acc != 0 {
                    out.push_str(&acc.to_string());
                    acc = 0;
                }
                out.push(piece_to_fen(color, piece));
            } else {
                acc += 1;
            }
        }
        if acc != 0 {
            out.push_str(&acc.to_string());
        }

        if rank != 0 {
            out.push('/');
        }
    }
}

fn encode_mover(out: &mut String, state: &GameState) {
    out.push(' ');
    out.push(match state.mover() {
        Color::B => 'b',
        Color::W => 'w',
    });
}

fn encode_castling(out: &mut String, state: &GameState) {
    out.push(' ');

    let (white_left, white_right) = state.castling(Color::W);
    let (black_left, black_right) = state.castling(Color::B);

    if !white_left && !white_right && !black_left && !black_right {
        out.push('-');
        return;
    }

    if white_right {
        out.push('K');
    }
    if white_left {
        out.push('Q');
    }
    if black_left {
        out.push('k');
    }
    if black_right {
        out.push('q');
    }
}

fn encode_enpassant(out: &mut String) {
    out.push(' ');
    out.push('-');
}

fn encode_moves(out: &mut String, state: &GameState) {
    out.push(' ');
    out.push('0');
    out.push(' ');
    out.push_str(&state.fullmove().to_string());
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        let board = Board::default();

        assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", encode(&board));
    }
}
