use std::io;

use board::Board;
pub use color::Color;
use eval::Scorer;
use moves::Move;
pub use pos::Pos;

mod board;
mod color;
mod eval;
mod moves;
mod pieces;
mod pos;

fn print_board(board: &Board, highlights: &[Pos]) {
    for row in (0..8).rev() {
        println!("+---+---+---+---+---+---+---+---+");
        for col in 0..8 {
            let pos: Pos = (row, col).into();
            let mark = highlights.iter().find(|p| **p == pos).map(|_| "•");
            let piece = board.at(pos).map_or(" ", |set| set.piece().as_str());
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

        let (movement, _) = explore(&board, Color::B, -f32::INFINITY, f32::INFINITY, 4);
        board = movement.apply(&board);
        print_board(&board, &[]);
    }
}

pub fn auto_play(moves: usize, depth: u8) {
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
            0
        };
        let depth = match board.mover() {
            Color::B => depth,
            Color::W => depth + bonus,
        };
        let (movement, eval) = explore(&board, board.mover(), -f32::INFINITY, f32::INFINITY, depth);

        println!("{:?} to play... {movement:?} ({eval})", board.mover());
        if matches!(movement, Move::None) {
            if board.in_check(board.mover()) {
                println!("{:?} wins by checkmate", board.mover().opposite());
                return;
            }

            println!("stalemate");
            return;
        }

        board = movement.apply(&board);

        print_board(
            &board,
            &movement.from().map(|f| vec![f]).unwrap_or_default(),
        );
    }
}

#[must_use]
pub fn explore(
    board: &Board,
    maxer: Color,
    mut alpha: f32,
    mut beta: f32,
    depth: u8,
) -> (Move, f32) {
    if depth == 0
        || board.pieces().king.is_empty()
        || board.pieces_for(board.mover().opposite()).king.is_empty()
    {
        return (Move::None, Scorer::eval(board, maxer, false));
    }

    let mut value: f32 = if board.mover() == maxer {
        -f32::INFINITY
    } else {
        f32::INFINITY
    };

    let movements = board.movements(board.mover());
    let mut evaluated_movements: Vec<_> = movements
        .iter()
        .map(|movement| {
            let next = movement.apply(board);
            let eval = Scorer::eval(&next, maxer, true);
            (next, movement, eval)
        })
        .collect();
    evaluated_movements.sort_by(|a, b| b.2.total_cmp(&a.2));

    let mut best = *evaluated_movements
        .first()
        .map(|r| r.1)
        .unwrap_or(&Move::None);

    for (child, movement, _) in evaluated_movements {
        let (_, eval) = explore(&child, maxer, alpha, beta, depth - 1);

        if board.mover() == maxer {
            if eval > value {
                value = eval;
                alpha = alpha.max(value);
                best = *movement;
            }
            if value >= beta {
                break;
            }
        } else {
            if eval < value {
                value = eval;
                beta = beta.min(value);
                best = *movement;
            }
            if value <= alpha {
                break;
            }
        }
    }

    if best == Move::None && board.mover() != maxer && !board.in_check(board.mover()) {
        return (Move::None, f32::NEG_INFINITY);
    }

    (best, value)
}

pub fn main() {
    auto_play(500, 4);
    // play();
}
