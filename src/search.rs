mod tt;

use crate::{board::Board, eval::MATE_SCORE, moves::Move};
use tt::{Bound, TT, TTEntry};

type EvalFn = fn(board: &Board) -> i32;

/// Search infinity. Must exceed any possible |eval| (mate scores are
/// MATE_SCORE plus/minus ply) so windows never clip real values, while
/// staying small enough that `-INF` is representable in i32.
const INF: i32 = MATE_SCORE + 10_000;

pub struct Search {
    board: Board,
    depth: usize,
    eval_fn: EvalFn,
    nodes: usize,
}

pub struct Stats {
    pub nodes: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SearchResult {
    pub eval: i32,
    pub movement: Option<Move>,
    pub mate_dist: Option<usize>,
}

impl Search {
    pub fn new(board: &Board, depth: usize, eval_fn: EvalFn) -> Self {
        Self { board: board.clone(), depth, eval_fn, nodes: 0 }
    }

    #[must_use]
    pub fn find(mut self) -> SearchResult {
        self.negamax(0, (-INF, INF))
    }

    #[must_use]
    pub fn find_with_stats(mut self) -> (SearchResult, Stats) {
        (self.negamax(0, (-INF, INF)), Stats { nodes: self.nodes })
    }

    #[must_use]
    fn negamax(&mut self, ply: usize, (mut alpha, mut beta): (i32, i32)) -> SearchResult {
        self.nodes += 1;

        let alpha_orig = alpha;

        let hash: u64 = self.board.hash().get();
        let remaining_depth = self.depth.saturating_sub(ply);
        let mut tt_move = None;

        if let Some(entry) = TT.probe(hash, ply, remaining_depth) {
            let mut result = entry.result;
            if result.mate_dist.is_some() {
                result.eval -= result.eval.signum() * (ply - entry.ply) as i32;
            }
            tt_move = result.movement;
            match entry.bound {
                Bound::Exact => return result,
                Bound::Lower => {
                    alpha = alpha.max(result.eval);
                }
                Bound::Upper => {
                    beta = beta.min(result.eval);
                }
            };
            if alpha >= beta {
                return result;
            };
        }

        if ply == self.depth {
            let eval = (self.eval_fn)(&self.board);
            return SearchResult {
                movement: None,
                eval,
                mate_dist: (eval <= -MATE_SCORE).then_some(0),
            };
        }

        let mover = self.board.state().mover();
        let mut movements = self.board.movements(mover);
        if movements.is_empty() {
            let result = if self.board.in_check(mover) {
                SearchResult { movement: None, eval: -MATE_SCORE + ply as i32, mate_dist: Some(0) }
            } else {
                SearchResult { movement: None, eval: 0, mate_dist: None }
            };
            TT.store(TTEntry { hash, bound: Bound::Exact, ply, depth: remaining_depth, result });
            return result;
        }

        movements.sort_by(|a, b| {
            let a_tt = Some(*a) == tt_move;
            let b_tt = Some(*b) == tt_move;
            b_tt.cmp(&a_tt).then_with(|| b.priority().cmp(&a.priority()))
        });

        let fallback_move = movements.first().copied();
        let mut best_eval = i32::MIN;
        let mut best_move = fallback_move;
        let mut mate_dist = None;

        for movement in movements {
            self.board.apply_mut(movement);
            let result = self.negamax(ply + 1, (-beta, -alpha));
            self.board.unapply_mut(movement);

            let eval = -result.eval;
            if eval > best_eval {
                best_eval = eval;
                best_move = Some(movement);
                mate_dist = result.mate_dist.map(|d| d + 1);
                if mate_dist == Some(1) {
                    break;
                }
            }

            alpha = alpha.max(eval);
            if alpha >= beta {
                break;
            }
        }

        let result = SearchResult { movement: best_move, eval: best_eval, mate_dist };
        let bound = if best_eval <= alpha_orig {
            Bound::Upper
        } else if best_eval >= beta {
            Bound::Lower
        } else {
            Bound::Exact
        };
        TT.store(TTEntry { hash, bound, ply, depth: remaining_depth, result });
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{defs::Sq, eval::Scorer, fen, squares::*, util::print_hboard};
    use test_case::test_case;

    #[test_case("8/8/8/8/2Q4p/k6P/1N6/1K3B2 w - - 0 101", C4, C3)]
    #[test_case("8/8/8/2Q5/k6p/3N3P/8/1K3B2 w - - 0 101", C5, B4)]
    #[test_case("8/8/8/2Q5/2B4p/2k2p1P/5N2/1K6 w - - 0 101", F2, E4)]
    fn mate_in_one(input: &str, from: Sq, to: Sq) {
        let board = fen::decode(input).unwrap();

        let result = Search::new(&board, 4, Scorer::eval).find();
        print_hboard(&board, &[result.movement.unwrap().to()]);

        assert_eq!(Some(1), result.mate_dist);
        assert_eq!(Some(Move::Slide { from, to, castling_update: None }), result.movement);
    }

    #[test]
    fn mate_in_two() {
        let mut board = fen::decode("8/8/8/2Q5/7p/1k5P/1N6/1K3B2 w - - 0 101").expect("ook");
        print_hboard(&board, &[]);

        let result = Search::new(&board, 4, Scorer::eval).find();
        print_hboard(&board, &[result.movement.unwrap().to()]);
        assert_eq!(Some(3), result.mate_dist);

        board.apply_mut(result.movement.unwrap());
        let result = Search::new(&board, 4, Scorer::eval).find();
        print_hboard(&board, &[result.movement.unwrap().to()]);
        assert_eq!(Some(2), result.mate_dist);

        board.apply_mut(result.movement.unwrap());
        let result = Search::new(&board, 4, Scorer::eval).find();
        print_hboard(&board, &[result.movement.unwrap().to()]);
        assert_eq!(Some(1), result.mate_dist);

        assert_eq!(Some(Move::Slide { from: C5, to: B4, castling_update: None }), result.movement);
    }

    mod tt_mate {
        use super::*;

        const MATED_FEN: &str = "k7/2K5/8/8/8/8/8/R7 b - - 0 1";

        #[test]
        fn mate_leaf_stored_and_rebased() {
            let board = fen::decode(MATED_FEN).unwrap();
            let mut search = Search::new(&board, 4, Scorer::eval);

            let ply: usize = 2;
            let first = search.negamax(ply, (-INF, INF));
            assert_eq!(first.eval, -MATE_SCORE + ply as i32);
            assert_eq!(first.mate_dist, Some(0));

            let deeper = ply + 3;
            let second = search.negamax(deeper, (-INF, INF));
            assert_eq!(second.eval, -MATE_SCORE + deeper as i32);
            assert_eq!(second.mate_dist, Some(0));
        }

        #[test]
        fn stalemate_leaf_stored_exact() {
            let board = fen::decode("k7/8/1Q6/8/8/8/2K5/8 b - - 0 1").unwrap();
            let mut search = Search::new(&board, 4, Scorer::eval);

            let ply: usize = 1;
            let result = search.negamax(ply, (-INF, INF));
            assert_eq!(result.eval, 0);
            assert_eq!(result.mate_dist, None);

            let again = search.negamax(ply + 2, (-INF, INF));
            assert_eq!(again.eval, 0);
            assert_eq!(again.mate_dist, None);
        }
    }

    mod distance {
        use super::*;

        const MATE_IN_TWO_FEN: &str = "5rk1/7p/6r1/Q3p3/P1P5/4NpPq/1P3P1P/6RK b - - 0 33";

        #[test]
        fn mate_in_one_at_min_depth_reports_distance() {
            let board =
                fen::decode("r3kb1r/ppp3pp/2n1p3/3p3P/3PP3/2NK4/PPP2q1P/R1BQ1B1R b kq - 0 12")
                    .unwrap();
            let result = Search::new(&board, 2, Scorer::eval).find();
            assert_eq!(result.eval, MATE_SCORE - 1);
            assert_eq!(result.mate_dist, Some(1));
        }

        #[test]
        fn mate_in_two_at_min_depth_reports_distance() {
            let board = fen::decode(MATE_IN_TWO_FEN).unwrap();
            let result = Search::new(&board, 4, Scorer::eval).find();
            assert_eq!(result.eval, MATE_SCORE - 3);
            assert_eq!(result.mate_dist, Some(3));
        }

        #[test]
        fn mate_exactly_at_horizon_is_not_a_mate_score() {
            let board = fen::decode(MATE_IN_TWO_FEN).unwrap();
            let result = Search::new(&board, 3, Scorer::eval).find();
            assert!(result.eval.abs() < MATE_SCORE - 3);
            assert_eq!(result.mate_dist, None);
        }
    }
}
