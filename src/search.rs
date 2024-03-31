use crate::{board::Board, color::Color, eval::Scorer, moves::Move, piece::Piece};

#[must_use]
pub(crate) fn minmax(
    board: &Board,
    depth: usize,
    mut alpha: f64,
    mut beta: f64,
    maxer: bool,
    maxer_color: Color,
) -> (Option<Move>, f64, Option<usize>) {
    if depth == 0 || board.get(board.state().mover(), Piece::King) == 0 {
        let eval = Scorer::eval(board, maxer_color);
        return (None, eval, if eval.is_infinite() { Some(board.state().n()) } else { None });
    }

    let mut movements = board.movements(board.state().mover());
    movements.sort_by(|a, b| b.priority().total_cmp(&a.priority()));

    let mut best_move = movements.first().copied();
    let mut best_eval = if maxer { f64::NEG_INFINITY } else { f64::INFINITY };
    let mut shortest_mate: Option<usize> = None;
    for movement in movements {
        let child = movement.apply(board);
        let (_, curr_eval, mate) = minmax(&child, depth - 1, alpha, beta, !maxer, maxer_color);
        if let Some(proposal) = mate {
            shortest_mate = shortest_mate.map(|current| current.min(proposal)).or(mate);
        }
        if maxer {
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

    if best_move.is_none() && !maxer && !board.in_check(board.state().mover()) {
        (None, f64::NEG_INFINITY, None)
    } else {
        (
            best_move,
            best_eval,
            if best_eval.is_infinite() { shortest_mate.or(Some(board.state().n())) } else { None },
        )
    }
}
