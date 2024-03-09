use std::io;

use bitboard::BitBoard;
use board::Board;
pub use color::Color;
use eval::Scorer;
use moves::Move;
use piece::Piece;
pub use pos::Pos;

use crate::bitboard::Bits;

mod bitboard;
mod board;
mod color;
mod eval;
mod exp;
mod magic;
mod moves;
mod piece;
mod pos;

pub fn print_bitboard(bb: BitBoard) {
    for row in (0..8).rev() {
        println!("+---+---+---+---+---+---+---+---+");
        for col in 0..8 {
            let pos: Pos = (row, col).into();
            let piece = if Bits::has_piece(bb, pos) { "@" } else { " " };
            print!("| {piece} ");
        }
        println!("| {row}");
    }
    println!("+---+---+---+---+---+---+---+---+");
    println!("  0   1   2   3   4   5   6   7  ");
}

fn print_board(board: &Board, highlights: &[Pos]) {
    for row in (0..8).rev() {
        println!("+---+---+---+---+---+---+---+---+");
        for col in 0..8 {
            let pos: Pos = (row, col).into();
            let mark = highlights.iter().find(|p| **p == pos).map(|_| "•");
            let piece = board.at(pos).map_or(" ", |(c, piece, _)| piece.as_str(c));
            print!("| {} ", mark.unwrap_or(piece));
        }
        println!("| {row}");
    }
    println!("+---+---+---+---+---+---+---+---+");
    println!("  0   1   2   3   4   5   6   7  ");
}

fn read_pos() -> Pos {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let col: u8 = line.trim().parse().unwrap();
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let row: u8 = line.trim().parse().unwrap();
    (row, col).into()
}

pub fn play() {
    let mut board = Board::default();
    print_board(&board, &[]);

    loop {
        let from = read_pos();
        // print_board(&board, &[board.generate_moves(from).map(|p| )]);

        let to = read_pos();

        board = Move::Slide { from, to }.apply(&board);
        print_board(&board, &[]);

        let (movement, _, _) = minmax(&board, 4, -f64::INFINITY, f64::INFINITY, true, Color::B);
        if let Some(movement) = movement {
            board = movement.apply(&board);
            print_board(&board, &[]);
        } else {
            println!("Game over");
            return;
        }
    }
}

pub fn auto_play(moves: usize, depth: usize) {
    let mut board = Board::default();

    for _ in 0..moves {
        let pc = board.piece_count();
        let bonus = if pc < 6 {
            3
        } else if pc < 10 {
            2
        } else if pc < 16 {
            1
        } else {
            1
        };
        let depth = match board.mover() {
            Color::B => depth,
            Color::W => depth + bonus,
        };
        let (movement, eval, mate) = minmax(
            &board,
            depth,
            -f64::INFINITY,
            f64::INFINITY,
            true,
            board.mover(),
        );

        if let Some(movement) = movement {
            if let Some(mate) = mate {
                println!("Mate in {}", mate - board.n());
            }
            println!(
                "{} => {:?} to play... {movement:?} ({eval}) (depth={depth}",
                board.n(),
                board.mover()
            );
            board = movement.apply(&board);
            print_board(&board, &[movement.from()]);
        } else {
            if board.in_check(board.mover()) {
                println!("{:?} wins by checkmate", board.mover().opposite());
                return;
            }
            println!("stalemate");
            return;
        }
    }
}

#[must_use]
pub fn minmax(
    board: &Board,
    depth: usize,
    mut alpha: f64,
    mut beta: f64,
    maxer: bool,
    maxer_color: Color,
) -> (Option<Move>, f64, Option<usize>) {
    if depth == 0 || board.get_piece(board.mover(), Piece::King) == 0 {
        let eval = Scorer::eval(board, maxer_color, false);
        return (
            None,
            eval,
            if eval.is_infinite() {
                Some(board.n())
            } else {
                None
            },
        );
    }

    let mover = board.mover();
    let mut movements: Vec<_> = board
        .movements(mover)
        .into_iter()
        .map(|movement| {
            let next = movement.apply(board);
            let eval = Scorer::eval(&next, mover, false);
            // let eval: f64 = Scorer::eval(&next, mover, board.n() < 16 || board.piece_count() < 14)
            // + movement.priority();
            (next, movement, eval)
        })
        .collect();
    movements.sort_by(|a, b| b.2.total_cmp(&a.2));

    let mut best_move = movements.first().map(|r| r.1);
    let mut best_eval = if maxer {
        f64::NEG_INFINITY
    } else {
        f64::INFINITY
    };
    let mut shortest_mate: Option<usize> = None;
    for (child, movement, _) in movements {
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

    if best_move.is_none() && !maxer && !board.in_check(board.mover()) {
        (None, f64::NEG_INFINITY, None)
    } else {
        (
            best_move,
            best_eval,
            if best_eval.is_infinite() {
                shortest_mate.or(Some(board.n()))
            } else {
                None
            },
        )
    }
}

pub fn main() {
    auto_play(500, 3);
    // play();
}
