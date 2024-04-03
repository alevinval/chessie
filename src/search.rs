use crate::{board::Board, color::Color, moves::Move, piece::Piece};

type EvalFn = fn(board: &Board, maxer: Color) -> f64;

#[must_use]
pub(crate) fn find_move(
    board: &Board,
    depth: usize,
    eval: EvalFn,
) -> (Option<Move>, f64, Option<usize>) {
    alpha_beta(board, depth, eval, board.state().mover(), true, -f64::INFINITY, f64::INFINITY)
}

#[must_use]
fn alpha_beta(
    board: &Board,
    depth: usize,
    eval_fn: EvalFn,
    maxer: Color,
    is_maxer: bool,
    mut alpha: f64,
    mut beta: f64,
) -> (Option<Move>, f64, Option<usize>) {
    if depth == 0 || board.get(board.state().mover(), Piece::King) == 0 {
        let score = eval_fn(board, maxer);
        return (None, score, if score.is_infinite() { Some(board.state().n()) } else { None });
    }

    let mut movements = board.movements(board.state().mover());
    movements.sort_by(|a, b| b.priority().total_cmp(&a.priority()));

    let mut best_move = movements.first().copied();
    let mut best_eval = if is_maxer { f64::NEG_INFINITY } else { f64::INFINITY };
    let mut shortest_mate: Option<usize> = None;
    for movement in movements {
        let child = movement.apply(board);
        let (_, curr_eval, mate) =
            alpha_beta(&child, depth - 1, eval_fn, maxer, !is_maxer, alpha, beta);
        if let Some(proposal) = mate {
            shortest_mate = shortest_mate.map(|current| current.min(proposal)).or(mate);
        }
        if is_maxer {
            if curr_eval > best_eval {
                best_eval = curr_eval;
                best_move = Some(movement);
            }
            alpha = alpha.max(best_eval);
            if best_eval >= beta {
                break;
            }
        } else {
            if curr_eval < best_eval {
                best_eval = curr_eval;
                best_move = Some(movement);
            }
            beta = beta.min(best_eval);
            if best_eval <= alpha {
                break;
            }
        }
    }

    if best_move.is_none() && !is_maxer && !board.in_check(board.state().mover()) {
        (None, f64::NEG_INFINITY, None)
    } else {
        (
            best_move,
            best_eval,
            if best_eval.is_infinite() { shortest_mate.or(Some(board.state().n())) } else { None },
        )
    }
}
