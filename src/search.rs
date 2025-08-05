use crate::{board::Board, eval::MATE_SCORE, moves::Move};

type EvalFn = fn(board: &Board) -> f64;

pub(crate) struct Search {
    board: Board,
    depth: usize,
    eval_fn: EvalFn,
}

pub(crate) struct SearchResult {
    pub movement: Option<Move>,
    pub eval: f64,
    pub mate: Option<usize>,
}

impl Search {
    pub(crate) fn new(board: &Board, depth: usize, eval_fn: EvalFn) -> Self {
        Self { board: board.clone(), depth, eval_fn }
    }

    #[must_use]
    pub(crate) fn find(mut self) -> SearchResult {
        self.negamax(self.depth, (-f64::INFINITY, f64::INFINITY))
    }

    #[must_use]
    fn negamax(&mut self, ply: usize, (mut alpha, beta): (f64, f64)) -> SearchResult {
        let score = (self.eval_fn)(&self.board);
        if score.abs() >= MATE_SCORE {
            return SearchResult { movement: None, eval: score + (ply as f64), mate: Some(0) };
        } else if ply == 0 {
            return SearchResult { movement: None, eval: score + (ply as f64), mate: None };
        }

        let mover = self.board.state().mover();
        let mut movements = self.board.movements(mover);
        movements.sort_by(|a, b| b.priority().total_cmp(&a.priority()));
        let first = movements.first().copied();

        let mut best_eval = -MATE_SCORE;
        let mut best_move = None;
        let mut best_mate: Option<usize> = None;

        for movement in movements {
            self.board.apply_mut(movement);
            let result = self.negamax(ply - 1, (-beta, -alpha));
            self.board.unapply_mut(movement);

            let eval = -result.eval;
            if eval > best_eval {
                best_eval = eval;
                best_move = Some(movement);
                best_mate = result.mate;
            }
            alpha = alpha.max(eval);
            if alpha >= beta {
                break;
            }
        }

        // Avoid stalemates
        if best_move.is_none() && !self.board.in_check(mover) {
            best_eval = 0.0;
        }

        best_mate = best_mate.map(|m| m + 1).or_else(|| {
            if best_eval.abs() >= MATE_SCORE {
                Some(1)
            } else {
                None
            }
        });

        SearchResult { movement: best_move.or(first), eval: best_eval, mate: best_mate }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{eval::Scorer, fen, sq, util::print_hboard};
    use test_case::test_case;

    #[test_case("8/8/8/8/2Q4p/k6P/1N6/1K3B2 w - - 0 101", (3, 2), (2,2))]
    #[test_case("8/8/8/2Q5/k6p/3N3P/8/1K3B2 w - - 0 101", (4,2), (3,1))]
    #[test_case("8/8/8/2Q5/2B4p/2k2p1P/5N2/1K6 w - - 0 101", (1,5), (3,4))]
    fn mate_in_one(input: &str, from: (u8, u8), to: (u8, u8)) {
        let board = fen::decode(input).unwrap();

        let result = Search::new(&board, 4, Scorer::eval).find();
        print_hboard(&board, &[result.movement.unwrap().to()]);

        assert_eq!(Some(1), result.mate);
        assert_eq!(
            Some(Move::Slide { from: sq!(from), to: sq!(to), castling_update: None }),
            result.movement
        );
    }

    #[test]
    fn mate_in_two() {
        let mut board = fen::decode("8/8/8/2Q5/7p/1k5P/1N6/1K3B2 w - - 0 101").expect("ook");
        print_hboard(&board, &[]);

        let result = Search::new(&board, 4, Scorer::eval).find();
        print_hboard(&board, &[result.movement.unwrap().to()]);
        assert_eq!(Some(2), result.mate);

        board.apply_mut(result.movement.unwrap());
        let result = Search::new(&board, 4, Scorer::eval).find();
        print_hboard(&board, &[result.movement.unwrap().to()]);
        assert_eq!(Some(1), result.mate);

        board.apply_mut(result.movement.unwrap());
        let result = Search::new(&board, 4, Scorer::eval).find();
        print_hboard(&board, &[result.movement.unwrap().to()]);
        assert_eq!(Some(1), result.mate);

        assert_eq!(
            Some(Move::Slide { from: sq!(3, 2), to: sq!(2, 2), castling_update: None }),
            result.movement
        );
    }
}
