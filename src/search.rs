use crate::{board::Board, eval::MATE_SCORE, moves::Move};

type EvalFn = fn(board: &Board) -> f64;

#[must_use]
pub(crate) fn find_move(
    board: &Board,
    depth: usize,
    eval_fn: EvalFn,
) -> (Option<Move>, f64, Option<usize>) {
    negamax(&mut board.clone(), depth, eval_fn, (-f64::INFINITY, f64::INFINITY))
}

#[must_use]
fn negamax(
    board: &mut Board,
    ply: usize,
    eval_fn: EvalFn,
    (mut alpha, beta): (f64, f64),
) -> (Option<Move>, f64, Option<usize>) {
    let score = eval_fn(board);
    if score.abs() >= MATE_SCORE {
        return (None, score + (ply as f64), Some(0));
    } else if ply == 0 {
        return (None, score, None);
    }

    let mover = board.state().mover();
    let mut movements = board.movements(mover);
    movements.sort_by(|a, b| b.priority().total_cmp(&a.priority()));
    let first = movements.first().copied();

    let mut best_eval = -MATE_SCORE;
    let mut best_move = None;
    let mut best_mate: Option<usize> = None;

    for movement in movements {
        board.apply_mut(movement);
        let (_, eval, mate) = negamax(board, ply - 1, eval_fn, (-beta, -alpha));
        board.unapply_mut(movement);

        let eval = -eval;
        if eval > best_eval {
            best_eval = eval;
            best_move = Some(movement);
            best_mate = mate;
        }
        alpha = alpha.max(eval);
        if alpha >= beta {
            break;
        }
    }

    // Avoid stalemates
    if best_move.is_none() && !board.in_check(mover) {
        best_eval = 0.0;
    }

    best_mate = best_mate.map(|m| m + 1).or_else(|| {
        if best_eval.abs() >= MATE_SCORE {
            Some(1)
        } else {
            None
        }
    });
    (best_move.or(first), best_eval, best_mate)
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

        let (a, _, mate) = find_move(&board, 4, Scorer::eval);

        print_hboard(&board, &[a.unwrap().to()]);

        assert_eq!(Some(1), mate);
        assert_eq!(Some(Move::Slide { from: sq!(from), to: sq!(to), castling_update: None }), a);
    }

    #[test]
    fn mate_in_two() {
        let mut board = fen::decode("8/8/8/2Q5/7p/1k5P/1N6/1K3B2 w - - 0 101").expect("ook");
        print_hboard(&board, &[]);

        let (a, _, mate) = find_move(&board, 4, Scorer::eval);
        print_hboard(&board, &[a.unwrap().to()]);
        assert_eq!(Some(2), mate);

        board.apply_mut(a.unwrap());
        let (a, _, mate) = find_move(&board, 4, Scorer::eval);
        print_hboard(&board, &[a.unwrap().to()]);
        assert_eq!(Some(1), mate);

        board.apply_mut(a.unwrap());
        let (a, _, mate) = find_move(&board, 4, Scorer::eval);
        print_hboard(&board, &[a.unwrap().to()]);
        assert_eq!(Some(1), mate);

        assert_eq!(Some(Move::Slide { from: sq!(3, 2), to: sq!(2, 2), castling_update: None }), a);
    }
}
