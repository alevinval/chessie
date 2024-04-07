use crate::{board::Board, moves::Move};

type EvalFn = fn(board: &Board) -> f64;

#[must_use]
pub(crate) fn find_move(
    board: &Board,
    depth: usize,
    eval_fn: EvalFn,
) -> (Option<Move>, f64, Option<usize>) {
    negamax(board, depth, eval_fn, (-f64::INFINITY, f64::INFINITY))
}

#[must_use]
fn negamax(
    board: &Board,
    ply: usize,
    eval_fn: EvalFn,
    (mut alpha, beta): (f64, f64),
) -> (Option<Move>, f64, Option<usize>) {
    let score = eval_fn(board);
    if score.is_infinite() {
        return (None, score, Some(board.state().n()));
    } else if ply == 0 {
        return (None, score, None);
    }

    let mover = board.state().mover();
    let mut movements = board.movements(mover);
    movements.sort_by(|a, b| b.priority().total_cmp(&a.priority()));

    let first = movements.first().copied();
    let mut best_move = None;
    let mut best_eval = f64::NEG_INFINITY;
    let mut best_mate: Option<usize> = None;

    for movement in movements {
        let child = movement.apply(board);
        let (_, eval, mate) = negamax(&child, ply - 1, eval_fn, (-beta, -alpha));
        let eval = -eval;
        if let Some(proposal) = mate {
            best_mate = best_mate.map(|curr| curr.min(proposal)).or(mate);
        }
        if eval > best_eval {
            best_eval = eval;
            best_move = Some(movement);
        }
        alpha = alpha.max(best_eval);
        if alpha >= beta {
            break;
        }
    }

    // Avoid stalemates
    if best_move.is_none() && !board.in_check(mover) {
        return (None, 0.0, best_mate);
    }

    (best_move.or(first), best_eval, best_mate)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{eval::Scorer, fen, pos::Pos, util::print_board};
    use test_case::test_case;

    #[test_case("8/8/8/8/2Q4p/k6P/1N6/1K3B2 w - - 0 101", (3, 2), (2,2))]
    #[test_case("8/8/8/2Q5/k6p/3N3P/8/1K3B2 w - - 0 101", (4,2), (3,1))]
    fn mate_in_one<P: Into<Pos>>(input: &str, from: P, to: P) {
        let board = fen::decode(input).unwrap();

        let (a, _, mate) = find_move(&board, 4, Scorer::eval);

        print_board(&board, &[a.unwrap().to()]);

        assert_eq!(Some(202), mate);
        assert_eq!(Some(Move::Slide { from: from.into(), to: to.into() }), a);
    }

    #[test]
    fn mate_in_two() {
        let board = fen::decode("8/8/8/2Q5/7p/1k5P/1N6/1K3B2 w - - 0 101").expect("ook");
        print_board(&board, &[]);

        let (a, _, mate) = find_move(&board, 4, Scorer::eval);
        print_board(&board, &[a.unwrap().to()]);
        assert_eq!(Some(204), mate);

        let board = a.unwrap().apply(&board);
        let (a, _, mate) = find_move(&board, 4, Scorer::eval);
        print_board(&board, &[a.unwrap().to()]);
        assert_eq!(Some(204), mate);

        let board = a.unwrap().apply(&board);
        let (a, _, mate) = find_move(&board, 4, Scorer::eval);
        print_board(&board, &[a.unwrap().to()]);
        assert_eq!(Some(204), mate);

        assert_eq!(Some(Move::Slide { from: Pos::new(3, 2), to: Pos::new(2, 2) }), a);
    }
}
