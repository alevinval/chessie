use std::io;

use board::Board;
use eval::Scorer;
use movement::Moves;
use pieces::{BitBoard, Color};
use pos::ORIGIN;

use crate::pos::Pos;

mod board;
mod eval;
pub mod movement;
mod pieces;
mod pos;

fn print_board(board: &Board, highlights: &[BitBoard]) {
    for row in (0..8).rev() {
        println!("+---+---+---+---+---+---+---+---+");
        for col in (0..8).rev() {
            let pos: Pos = (row, col).into();
            let mark = highlights.iter().find(|h| h.has_piece(pos)).map(|_| "@");
            let piece = board.at(pos).map_or(" ", |set| set.piece().as_str());
            print!("| {} ", mark.unwrap_or(piece));
        }
        println!("| {}", row + 1);
    }
    println!("+---+---+---+---+---+---+---+---+");
    println!("  8   7   6   5   4   3   2   1  ");
}

fn read_pos() -> Pos {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let col: u8 = line.trim().parse().unwrap();
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let row: u8 = line.trim().parse().unwrap();
    (row - 1, col - 1).into()
}

pub fn play() {
    let eval = Scorer::default();
    let mut board = Board::default();
    print_board(&board, &[]);

    loop {
        let from = read_pos();
        // print_board(&board, &[board.generate_moves(from).map(|p| )]);

        let to = read_pos();
        board.apply_move(from, to);
        print_board(&board, &[]);

        let (from, to, _) = explore(
            &eval,
            &board,
            Color::Black,
            Color::Black,
            -f32::INFINITY,
            f32::INFINITY,
            4,
        );
        board.apply_move(from, to);
        print_board(&board, &[]);
    }
}

pub fn auto_play(mut color: Color, moves: u8, depth: u8) {
    let scorer = Scorer::default();
    let mut board = Board::new();

    for _ in 0..moves {
        let (from, to, eval) = explore(
            &scorer,
            &board,
            color,
            color,
            -f32::INFINITY,
            f32::INFINITY,
            depth,
        );
        board.apply_move(from, to);
        scorer.debug_eval(&board, color);

        println!("{color:?} to play... {from:?} -> {to:?} ({eval})");
        print_board(&board, &[]);

        color = color.opposite();
    }
}

pub fn explore(
    scorer: &Scorer,
    board: &Board,
    mover: Color,
    maxing_color: Color,
    mut alpha: f32,
    mut beta: f32,
    depth: u8,
) -> (Pos, Pos, f32) {
    if depth == 0 {
        let eval = scorer.eval(board, maxing_color);
        return (ORIGIN, ORIGIN, eval);
    }

    let mut value: f32 = if mover == maxing_color {
        -f32::INFINITY
    } else {
        f32::INFINITY
    };

    let (mut best_from, mut best_to) = (ORIGIN, ORIGIN);

    let mut movements: Vec<_> = board
        .pieces(mover)
        .iter()
        .flat_map(BitBoard::iter_pos)
        .map(|p| board.generate_moves(p))
        .flat_map(Moves::iter_pos)
        .map(|(from, to)| {
            let mut b = board.clone();
            b.apply_move(from, to);
            let eval = scorer.eval(&b, maxing_color);
            (b, eval, from, to)
        })
        .collect();

    movements.sort_by(|a, b| b.1.total_cmp(&a.1));

    for (new_board, _, from, to) in movements {
        let (_, _, eval) = explore(
            scorer,
            &new_board,
            mover.opposite(),
            maxing_color,
            alpha,
            beta,
            depth - 1,
        );

        if mover == maxing_color {
            if eval > value {
                value = eval;
                best_from = from;
                best_to = to;
            }
            alpha = alpha.max(value);
            if value >= beta {
                return (best_from, best_to, value);
            }
        } else {
            if eval < value {
                value = eval;
                best_from = from;
                best_to = to;
            }
            beta = beta.min(value);
            if value <= alpha {
                return (best_from, best_to, value);
            }
        }
    }

    debug_assert!(
        best_from != best_to,
        "should always find a legal move for the moment"
    );

    (best_from, best_to, value)
}

pub fn main() {
    auto_play(Color::White, u8::MAX, 4);
    // play();
}
