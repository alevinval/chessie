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

pub fn auto_play(mut mover: Color, moves: u8, depth: u8) {
    let mut board = Board::default();

    for _ in 0..moves {
        let (movement, eval) = explore(&board, mover, -f32::INFINITY, f32::INFINITY, depth);

        println!("{mover:?} to play... {movement:?} ({eval})");
        if matches!(movement, Move::None) {
            let king_pos = board.pieces().king.iter_pos().next().unwrap();

            board.next_turn();
            if board
                .pseudo_movements()
                .iter()
                .filter_map(|m| m.to())
                .any(|p| p == king_pos)
            {
                println!("{:?} wins by checkmate", mover.opposite());
            } else {
                println!("stalemate");
            }
            return;
        }

        board = movement.apply(&board);
        Scorer::debug_eval(&board, mover);

        print_board(
            &board,
            &movement.from().map(|f| vec![f]).unwrap_or_default(),
        );

        mover = mover.opposite();
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
    if depth == 0 {
        return (Move::None, Scorer::eval(board, maxer));
    }

    let mut value: f32 = if board.mover() == maxer {
        -f32::INFINITY
    } else {
        f32::INFINITY
    };

    let movements = board.movements();
    let mut evaluated_movements: Vec<_> = movements
        .iter()
        .map(|movement| {
            let next = movement.apply(board);
            let eval = Scorer::eval(&next, maxer);
            (next, eval, movement)
        })
        .collect();
    evaluated_movements.sort_by(|a, b| b.1.total_cmp(&a.1));

    let mut best = *movements.first().unwrap_or(&Move::None);
    for (new_board, _, movement) in evaluated_movements {
        let (_, eval) = explore(&new_board, maxer, alpha, beta, depth - 1);

        if board.mover() == maxer {
            if eval > value {
                value = eval;
                best = *movement;
            }
            alpha = alpha.max(value);
            if value >= beta {
                break;
            }
        } else {
            if eval < value {
                value = eval;
                best = *movement;
            }
            beta = beta.min(value);
            if value <= alpha {
                break;
            }
        }
    }

    (best, value)
}

pub fn main() {
    auto_play(Color::W, u8::MAX, 5);
    // play();
}
