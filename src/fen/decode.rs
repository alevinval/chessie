use crate::{board::Board, color::Color, defs::CastlingUpdate, piece::Piece};

use super::FenError;

#[allow(dead_code)]
pub(crate) fn decode(input: &str) -> Result<Board, FenError> {
    let mut board = Board::empty();

    let mut input = input.split_whitespace();

    let ranks = input.next().ok_or(FenError::Invalid)?;
    decode_ranks(&mut board, ranks)?;

    let mover = input.next().ok_or(FenError::Invalid)?;
    decode_mover(&mut board, mover)?;

    let castling = input.next().ok_or(FenError::Invalid)?;
    decode_castling(&mut board, castling)?;

    let enpassant = input.next().ok_or(FenError::Invalid)?;
    decode_enpassant(&mut board, enpassant)?;

    let halfmove = input.next().ok_or(FenError::Invalid)?;
    decode_halfmove(&mut board, halfmove)?;

    let fullmove = input.next().ok_or(FenError::Invalid)?;
    decode_fullmove(&mut board, fullmove)?;

    board.calculate_occupancies();

    Ok(board)
}

fn decode_ranks(board: &mut Board, input: &str) -> Result<(), FenError> {
    for (row, rank) in input.split('/').rev().enumerate() {
        if row > 8 {
            return Err(FenError::Invalid);
        }
        let mut col: u8 = 0;
        for ch in rank.chars() {
            if ch.is_numeric() {
                let n: u8 = ch.to_string().parse().map_err(|_| FenError::Invalid)?;
                col += n;
                continue;
            }
            #[allow(clippy::cast_possible_truncation)]
            let sq: u8 = row as u8 * 8 + col;
            let (color, piece) = fen_to_piece(ch)?;
            board.add(color, piece, sq);
            col += 1;
        }
        if col != 8 {
            return Err(FenError::Invalid);
        }
    }
    Ok(())
}

fn decode_mover(board: &mut Board, input: &str) -> Result<(), FenError> {
    let mover = match input {
        "w" => Color::W,
        "b" => Color::B,
        _ => return Err(FenError::Invalid),
    };
    board.state_mut().set_mover(mover);
    Ok(())
}

fn decode_castling(board: &mut Board, input: &str) -> Result<(), FenError> {
    let state = board.state_mut();
    state.set_castling(Color::W, CastlingUpdate::Both, false);
    state.set_castling(Color::B, CastlingUpdate::Both, false);

    if input == "-" {
        return Ok(());
    }

    for ch in input.chars() {
        match ch {
            'Q' => state.set_castling(Color::W, CastlingUpdate::Left, true),
            'K' => state.set_castling(Color::W, CastlingUpdate::Right, true),
            'q' => state.set_castling(Color::B, CastlingUpdate::Right, true),
            'k' => state.set_castling(Color::B, CastlingUpdate::Left, true),
            _ => return Err(FenError::Invalid),
        }
    }

    Ok(())
}

fn decode_enpassant(_board: &mut Board, input: &str) -> Result<(), FenError> {
    if input != "-" {
        return Err(FenError::NotSupportedEnPassant);
    }
    Ok(())
}

fn decode_halfmove(_board: &mut Board, input: &str) -> Result<(), FenError> {
    if input != "0" {
        return Err(FenError::NotSupportedHalfMove);
    }
    Ok(())
}

fn decode_fullmove(board: &mut Board, input: &str) -> Result<(), FenError> {
    let fullmove: usize = input.parse().map_err(|_| FenError::Invalid)?;
    board.state_mut().set_fullmove(fullmove);
    Ok(())
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

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;
    use crate::{fen::encode, util::print_board};

    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")]
    #[test_case("rnbqkbnr/pppppppp/8/3P1p2/8/8/PPPPPPPP/RNBQKBNR b KQ - 0 7")]
    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qq - 0 25")]
    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b Kk - 0 10")]
    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 12")]
    fn test_symmetry(input: &str) {
        let decoded = &decode(input).expect("should be OK");
        print_board(decoded);
        assert_eq!(input, encode(decoded));
    }

    #[test_case("rnbqknr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")]
    #[test_case("rnbMqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")]
    #[test_case("rnbqkbnr/pppppppp//8/3P1p2/8/8/PPPPPPPP/RNBQKBNR b KQ - 0 7")]
    #[test_case("rnbqkbnr/pppppppp/8/3P1p2/8/8/PPPPPPPP/RNBQKBNR b KQ - 0 9999999999999999999999")]
    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkXq - 0 1")]
    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/8/8/8/8 w KQkXq - 0 1")]
    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR X KQkq - 0 1")]
    fn test_invalid_fen(input: &str) {
        assert_eq!(FenError::Invalid, decode(input).expect_err("should be invalid FEN"));
    }

    #[test_case(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq a1 0 1",
        FenError::NotSupportedEnPassant
    )]
    #[test_case(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 2 1",
        FenError::NotSupportedHalfMove
    )]
    fn not_supported(input: &str, expected: FenError) {
        assert_eq!(expected, decode(input).expect_err("should not be supported"));
    }
}
