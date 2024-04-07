use crate::{board::Board, color::Color, moves::Move};

type EvalFn = fn(board: &Board, maxer: Color) -> f64;

#[must_use]
pub(crate) fn find_move(
    board: &Board,
    depth: usize,
    eval: EvalFn,
) -> (Option<Move>, f64, Option<usize>) {
    alpha_beta(
        board,
        depth,
        eval,
        board.state().mover(),
        true,
        true,
        (-f64::INFINITY, f64::INFINITY),
    )
}

#[must_use]
fn alpha_beta(
    board: &Board,
    ply: usize,
    eval_fn: EvalFn,
    maxer: Color,
    is_root: bool,
    is_maxer: bool,
    (mut alpha, mut beta): (f64, f64),
) -> (Option<Move>, f64, Option<usize>) {
    let score = eval_fn(board, maxer);
    if score.is_infinite() {
        return (None, score, Some(board.state().n()));
    } else if ply == 0 {
        return (None, score, None);
    }

    let mover = board.state().mover();
    let mut movements = board.movements(mover);
    movements.sort_by(|a, b| b.priority().total_cmp(&a.priority()));

    let mut best_move = if is_root { movements.first().copied() } else { None };
    let mut best_eval = if is_maxer { f64::NEG_INFINITY } else { f64::INFINITY };
    let mut shortest_mate: Option<usize> = None;
    for movement in movements {
        let child = movement.apply(board);
        let (_, child_eval, mate) =
            alpha_beta(&child, ply - 1, eval_fn, maxer, false, !is_maxer, (alpha, beta));
        if let Some(proposal) = mate {
            shortest_mate = shortest_mate.map(|current| current.min(proposal)).or(mate);
        }
        if is_maxer {
            if child_eval > best_eval {
                best_eval = child_eval;
                best_move = Some(movement);
            }
            alpha = alpha.max(best_eval);
            if best_eval >= beta {
                break;
            }
        } else {
            if child_eval < best_eval {
                best_eval = child_eval;
                best_move = Some(movement);
            }
            beta = beta.min(best_eval);
            if best_eval <= alpha {
                break;
            }
        }
    }

    // Avoid stalemates
    if best_move.is_none() && !is_maxer && !board.in_check(mover) {
        return (None, 0.0, None);
    }

    (best_move, best_eval, shortest_mate)
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

        let (a, _, _) = find_move(&board, 4, Scorer::eval);
        print_board(&board, &[a.unwrap().to()]);

        let board = a.unwrap().apply(&board);
        let (a, _, _) = find_move(&board, 4, Scorer::eval);
        print_board(&board, &[a.unwrap().to()]);

        let board = a.unwrap().apply(&board);
        let (a, _, _) = find_move(&board, 4, Scorer::eval);
        print_board(&board, &[a.unwrap().to()]);

        assert_eq!(Some(Move::Slide { from: Pos::new(3, 2), to: Pos::new(2, 2) }), a);
    }
}
