use std::io;

use board::Board;
pub use color::Color;
use eval::Scorer;
use moves::Move;
use pieces::BitBoard;
pub use pos::Pos;

use crate::moves::MoveGen;

mod board;
mod color;
mod eval;
mod moves;
mod pieces;
mod pos;

fn print_board(board: &Board, highlights: &[BitBoard]) {
    for row in (0..8).rev() {
        println!("+---+---+---+---+---+---+---+---+");
        for col in 0..8 {
            let pos: Pos = (row, col).into();
            let mark = highlights.iter().find(|h| h.has_piece(pos)).map(|_| "@");
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

        Move::Basic { from, to }.apply(&mut board);

        print_board(&board, &[]);

        let (mov, _) = explore(&board, Color::B, Color::B, -f32::INFINITY, f32::INFINITY, 4);
        mov.apply(&mut board);
        print_board(&board, &[]);
    }
}

pub fn auto_play(mut color: Color, moves: u8, depth: u8) {
    let mut board = Board::default();

    for _ in 0..moves {
        let (mov, eval) = explore(&board, color, color, -f32::INFINITY, f32::INFINITY, depth);
        println!("{color:?} to play... {mov:?} ({eval})");

        mov.apply(&mut board);
        Scorer::debug_eval(&board, color);

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
) -> (Move, f32) {
    if depth == 0 {
        return (Move::None, Scorer::eval(board, maxing_color));
    }

    let mut value: f32 = if mover == maxing_color {
        -f32::INFINITY
    } else {
        f32::INFINITY
    };

    let mut best = Move::None;

    let mut movements: Vec<_> = board
        .pieces(mover)
        .iter()
        .flat_map(BitBoard::iter_pos)
        .flat_map(|p| MoveGen::new(board, mover, p).generate())
        .map(|mov| {
            let mut b = board.clone();
            mov.apply(&mut b);
            let eval = Scorer::eval(&b, maxing_color);
            (b, eval, mov)
        })
        .collect();

    movements.sort_by(|a, b| b.1.total_cmp(&a.1));

    for (new_board, _, mov) in movements {
        let (_, eval) = explore(
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
                best = mov;
            }
            alpha = alpha.max(value);
            if value >= beta {
                return (best, value);
            }
        } else {
            if eval < value {
                value = eval;
                best = mov;
            }
            beta = beta.min(value);
            if value <= alpha {
                return (best, value);
            }
        }
    }

    debug_assert!(
        best != Move::None,
        "should always find a legal move for the moment"
    );

    (best, value)
}

pub fn main() {
    auto_play(Color::W, u8::MAX, 3);
    // play();
}
