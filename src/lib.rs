use std::io;

use board::Board;
use pieces::{BitBoard, Color};

use crate::pos::Pos;

mod board;
mod pieces;
mod pos;

fn print_board(board: &Board, highlights: &[BitBoard]) {
    for row in (0..8).rev() {
        println!("+---+---+---+---+---+---+---+---+");
        for col in (0..8).rev() {
            let pos = Pos(row, col);
            let mark = highlights.iter().find(|h| h.has_piece(pos)).map(|_| "@");
            let piece = board.at(pos).map(|set| set.piece().as_str()).unwrap_or(" ");
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
    let mut board = Board::default();
    print_board(&board, &[]);

    loop {
        let from = read_pos();
        print_board(&board, &[board.generate_moves(from)]);

        let to = read_pos();
        board.apply_move(from, to);
        print_board(&board, &[]);

        let (from, to, _) = explore(
            &board,
            Color::Black,
            Color::Black,
            -f32::INFINITY,
            f32::INFINITY,
            4,
        );
        board.apply_move(from.unwrap(), to.unwrap());
        print_board(&board, &[]);
    }
}

pub fn auto_play(mut color: Color, moves: u8, depth: u8) {
    let mut board = Board::new();
    for _ in 0..moves {
        let (from, to, eval) = explore(&board, color, color, -f32::INFINITY, f32::INFINITY, depth);
        board.apply_move(from.unwrap(), to.unwrap());
        board.eval(color, true);

        println!("{color:?} to play... {from:?} -> {to:?} ({eval})");
        print_board(&board, &[]);

        color = color.opposite();
    }
}

pub fn explore(
    board: &Board,
    mover: Color,
    maxing_color: Color,
    mut alpha: f32,
    mut beta: f32,
    depth: u8,
) -> (Option<Pos>, Option<Pos>, f32) {
    if depth == 0 {
        let eval = board.eval(maxing_color, false);
        return (None, None, eval);
    }

    let mut value: f32 = if mover == maxing_color {
        -f32::INFINITY
    } else {
        f32::INFINITY
    };
    let mut best_from = None;
    let mut best_to = None;

    for ps in board.pieces(mover).iter() {
        for from in ps.positions() {
            for to in board.generate_moves(from).iter_pos() {
                let mut new_board = board.clone();
                new_board.apply_move(from, to);
                let (_, _, eval) = explore(
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
                        best_from = Some(from);
                        best_to = Some(to);
                    }
                    alpha = alpha.max(value);
                    if value >= beta {
                        return (best_from, best_to, value);
                    }
                } else {
                    if eval < value {
                        value = eval;
                        best_from = Some(from);
                        best_to = Some(to);
                    }
                    beta = beta.min(value);
                    if value <= alpha {
                        return (best_from, best_to, value);
                    }
                }
            }
        }
    }

    (best_from, best_to, value)
}

pub fn main() {
    auto_play(Color::White, 25, 4);
    // play();
}
