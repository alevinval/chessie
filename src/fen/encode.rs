use crate::{
    board::{Board, GameState},
    color::Color,
    defs::Castling,
    piece::Piece,
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
            let piece = board.at((rank, col));
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

    let white = state.castling(Color::W);
    let black = state.castling(Color::B);
    if matches!(white, Castling::None) && matches!(black, Castling::None) {
        out.push('-');
    }

    if let Castling::Some(left, right) = white {
        if right {
            out.push('K');
        }
        if left {
            out.push('Q');
        }
    }

    if let Castling::Some(left, right) = black {
        if left {
            out.push('k');
        }
        if right {
            out.push('q');
        }
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
