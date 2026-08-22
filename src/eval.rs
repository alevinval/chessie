use crate::{bits, board::Board, color::Color, defs::BitBoard, piece::Piece};

pub mod legacy;

pub const MATE_SCORE: i32 = 100_000_000;

const PHASE_MAX: i32 = 24;
const PHASE_WEIGHT: [i32; 6] = [0, 3, 3, 4, 8, 0]; // P N B R Q K

#[derive(Default)]
pub struct Scorer {}

impl Scorer {
    pub fn eval(board: &Board) -> i32 {
        let mover = board.state().mover();
        let phase = Self::phase(board);
        Self::score(board, mover, phase, false) - Self::score(board, mover.flip(), phase, false)
    }

    #[allow(dead_code)]
    pub(crate) fn debug_eval(board: &Board) -> i32 {
        let mover = board.state().mover();
        let phase = Self::phase(board);
        Self::score(board, mover, phase, true) - Self::score(board, mover.flip(), phase, true)
    }

    // 24 = full material (middlegame), 0 = bare kings (endgame).
    fn phase(board: &Board) -> i32 {
        [Color::W, Color::B]
            .iter()
            .flat_map(|color| board.pieces(*color))
            .map(|(piece, bb)| PHASE_WEIGHT[piece.idx()] * bits::count(bb) as i32)
            .sum::<i32>()
            .clamp(0, PHASE_MAX)
    }

    fn score(board: &Board, color: Color, phase: i32, debug: bool) -> i32 {
        let material_score: i32 = board
            .pieces(color)
            .map(|(piece, bb)| Self::score_bitboard(piece, bb, phase, color))
            .sum();

        if debug {
            println!("{color:?}");
            println!("  material: {material_score}");
        }

        material_score
    }

    fn score_bitboard(piece: Piece, bb: BitBoard, phase: i32, color: Color) -> i32 {
        score_material::score(piece, bb)
            + score_centrality::score(piece, bb)
            + score_king::score(piece, bb, phase, color)
    }
}

pub(crate) mod score_material {
    use super::*;

    pub(super) fn score(piece: Piece, bb: BitBoard) -> i32 {
        bits::count(bb) as i32 * piece_value(piece)
    }

    pub const fn piece_value(piece: Piece) -> i32 {
        match piece {
            Piece::Pawn => 100,
            Piece::Rook => 500,
            Piece::Knight => 280,
            Piece::Bishop => 300,
            Piece::Queen => 900,
            Piece::King => 0,
        }
    }
}

mod score_centrality {
    use super::*;

    #[rustfmt::skip]
const CENTRALITY_BIAS: [i32; 64] = [
     0,  1,  2,  3,  3,  2,  1,  0,
     1,  2,  3,  4,  4,  3,  2,  1,
     2,  3,  4,  5,  5,  4,  3,  2,
     3,  4,  5,  6,  6,  5,  4,  3,
     3,  4,  5,  6,  6,  5,  4,  3,
     2,  3,  4,  5,  5,  4,  3,  2,
     1,  2,  3,  4,  4,  3,  2,  1,
     0,  1,  2,  3,  3,  2,  1,  0,
];

    pub(super) fn score(piece: Piece, bb: BitBoard) -> i32 {
        if !matches!(piece, Piece::Knight | Piece::Bishop) {
            return 0;
        }
        bits::pos(bb).into_iter().map(|sq| CENTRALITY_BIAS[sq as usize]).sum()
    }
}

mod score_king {
    use super::*;

    #[rustfmt::skip]
    const KING_MID: [i32; 64] = [
        3,  3,  4,  5,  5,  4,  3,  3,
        2,  3,  3,  4,  4,  3,  3,  2,
       -1, -2, -2, -3, -3, -2, -2, -1,
       -2, -3, -3, -4, -4, -3, -3, -2,
       -3, -4, -4, -5, -5, -4, -4, -3,
       -3, -4, -4, -5, -5, -4, -4, -3,
       -3, -4, -4, -5, -5, -4, -4, -3,
       -3, -4, -4, -5, -5, -4, -4, -3,
    ];

    #[rustfmt::skip]
    const KING_END: [i32; 64] = [
        0,  0,  1,  1,  1,  1,  0,  0,
        1,  1,  2,  2,  2,  2,  1,  1,
        2,  2,  3,  3,  3,  3,  2,  2,
        3,  3,  4,  4,  4,  4,  3,  3,
        3,  3,  3,  3,  3,  3,  3,  3,
        2,  2,  2,  2,  2,  2,  2,  2,
        1,  1,  1,  1,  1,  1,  1,  1,
        0,  0,  0,  0,  0,  0,  0,  0,
    ];

    pub(super) fn score(piece: Piece, bb: BitBoard, phase: i32, color: Color) -> i32 {
        if piece != Piece::King {
            return 0;
        }
        let Some(sq) = bits::first_pos(bb) else {
            return 0;
        };
        // Tables are written for White; mirror ranks for Black.
        let sq = if color == Color::B { sq ^ 56 } else { sq };
        let mid = KING_MID[sq as usize];
        let end = KING_END[sq as usize];
        (mid * phase + end * (PHASE_MAX - phase)) / PHASE_MAX
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        fen::decode, piece::Piece, pos, squares::A1, squares::D4, squares::E4, squares::G1,
    };

    use super::*;

    fn eval_fen(fen: &str) -> i32 {
        Scorer::eval(&decode(fen).unwrap())
    }

    mod eval {
        use super::*;

        #[test]
        fn start_position_is_zero() {
            assert_eq!(eval_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"), 0);
        }
    }

    mod phase {
        use super::*;

        #[test]
        fn phase_weights() {
            let start = decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
            assert_eq!(Scorer::phase(&start), 24);

            let bare_kings = decode("k7/8/8/8/8/8/8/K7 w - - 0 1").unwrap();
            assert_eq!(Scorer::phase(&bare_kings), 0);

            let rooks_only = decode("r3k3/8/8/8/8/8/8/K3R3 w - - 0 1").unwrap();
            assert_eq!(Scorer::phase(&rooks_only), 8);
        }
    }

    mod material {
        use super::*;

        #[test]
        fn piece_values() {
            assert_eq!(score_material::piece_value(Piece::Pawn), 100);
            assert_eq!(score_material::piece_value(Piece::Knight), 280);
            assert_eq!(score_material::piece_value(Piece::Bishop), 300);
            assert_eq!(score_material::piece_value(Piece::Rook), 500);
            assert_eq!(score_material::piece_value(Piece::Queen), 900);
            assert_eq!(score_material::piece_value(Piece::King), 0);
        }

        #[test]
        fn score_scales_with_piece_count() {
            assert_eq!(score_material::score(Piece::Queen, pos::bb(E4)), 900);
            assert_eq!(score_material::score(Piece::Pawn, pos::bb(E4) | pos::bb(D4)), 200);
        }
    }

    mod centrality {
        use super::*;

        #[test]
        fn only_minor_pieces_score() {
            assert_eq!(score_centrality::score(Piece::Knight, pos::bb(E4)), 6);
            assert_eq!(score_centrality::score(Piece::Bishop, pos::bb(E4)), 6);
            assert_eq!(score_centrality::score(Piece::Pawn, pos::bb(E4)), 0);
            assert_eq!(score_centrality::score(Piece::Queen, pos::bb(E4)), 0);
        }

        #[test]
        fn corner_scores_zero() {
            assert_eq!(score_centrality::score(Piece::Knight, pos::bb(A1)), 0);
        }
    }

    mod king {
        use super::*;

        #[test]
        fn castled_king_scores_positive_in_middlegame() {
            assert_eq!(eval_fen("r1q3r1/4k3/8/8/8/8/8/R1Q1K2R w - - 0 1"), 5 - 4);
        }

        #[test]
        fn central_king_scores_negative_in_middlegame() {
            assert_eq!(eval_fen("r1q3r1/8/4k3/8/4K3/8/8/R1Q3R1 w - - 0 1"), -4 - (-3));
        }

        #[test]
        fn black_king_table_is_rank_mirrored() {
            assert_eq!(eval_fen("r1q1k2r/8/8/8/8/8/8/R1Q1K2R w - - 0 1"), 0);
        }

        #[test]
        fn endgame_flips_king_preference() {
            assert_eq!(eval_fen("4k3/8/8/8/4K3/8/8/8 w - - 0 1"), 4 - 1);
        }

        #[test]
        fn king_table_values() {
            assert_eq!(score_king::score(Piece::King, pos::bb(G1), 24, Color::W), 3);
            assert_eq!(score_king::score(Piece::King, pos::bb(G1 ^ 56), 24, Color::B), 3);
            assert_eq!(score_king::score(Piece::King, pos::bb(E4), 24, Color::W), -4);
            assert_eq!(score_king::score(Piece::King, pos::bb(E4), 0, Color::W), 4);
        }
    }
}
