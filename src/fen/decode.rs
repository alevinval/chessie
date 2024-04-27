use crate::{board::Board, color::Color, defs::Castling, piece::Piece, pos::Pos};

use super::FenError;

#[allow(dead_code)]
pub(crate) fn decode(input: &str) -> Result<Board, FenError> {
    let mut board = Board::default();
    board.clear();

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
            let pos = Pos::new(row as u8, col);
            let (color, piece) = fen_to_piece(ch)?;
            board.add(color, piece, pos);
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
    *board.state_mut().castling_for(Color::W) = Castling::None;
    *board.state_mut().castling_for(Color::B) = Castling::None;

    if input == "-" {
        return Ok(());
    }

    let mut white_left = false;
    let mut white_right = false;
    let mut black_left = false;
    let mut black_right = false;

    for ch in input.chars() {
        match ch {
            'Q' => white_left = true,
            'K' => white_right = true,
            'q' => black_right = true,
            'k' => black_left = true,
            _ => return Err(FenError::Invalid),
        }
    }

    if white_left || white_right {
        *board.state_mut().castling_for(Color::W) = Castling::Some(white_left, white_right);
    }
    if black_left || black_right {
        *board.state_mut().castling_for(Color::B) = Castling::Some(black_left, black_right);
    }

    Ok(())
}

fn decode_enpassant(_board: &mut Board, input: &str) -> Result<(), FenError> {
    if input != "-" {
        return Err(FenError::NotSupported("en-passant".to_string()));
    }
    Ok(())
}

fn decode_halfmove(_board: &mut Board, input: &str) -> Result<(), FenError> {
    if input != "0" {
        return Err(FenError::NotSupported("half-move".to_string()));
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
        FenError::NotSupported("en-passant".to_string())
    )]
    #[test_case(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 2 1", 
        FenError::NotSupported("half-move".to_string())
    )]
    fn not_supported(input: &str, expected: FenError) {
        assert_eq!(expected, decode(input).expect_err("should not be supported"));
    }
}
