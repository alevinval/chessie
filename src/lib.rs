use std::io;

use board::Board;
use color::Color;
use defs::Sq;
use eval::{legacy::LegacyScorer, Scorer};
use moves::Move;
use search::find_move;
use util::{print_board, print_hboard};

mod bits;
mod board;
mod color;
mod defs;
mod eval;
mod fen;
mod magic;
mod moves;
mod piece;
mod pos;
pub mod precompute;
mod search;
mod util;

fn read_sq() -> Sq {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let col: u8 = line.trim().parse().unwrap();
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let row: u8 = line.trim().parse().unwrap();
    sq!(row, col)
}

pub fn play() {
    let legacy_eval = LegacyScorer::eval;
    let mut board = Board::default();
    print_board(&board);
    loop {
        let from = read_sq();
        // print_board(&board, &[board.generate_moves(from).map(|p| )]);

        let to = read_sq();

        board.apply_mut(Move::Slide { from, to, castling_update: None });
        print_board(&board);

        let (movement, _, _) = find_move(&board, 4, legacy_eval);
        if let Some(movement) = movement {
            board.apply_mut(movement);
            print_board(&board);
        } else {
            println!("Game over");
            return;
        }
    }
}

pub fn auto_play(moves: usize, depth: usize) {
    let white_eval = Scorer::eval;
    let black_eval = LegacyScorer::eval;
    let mut board = Board::default();

    for _ in 0..moves {
        let pc = board.count_pieces();
        let bonus = if pc < 6 {
            3
        } else if pc < 10 {
            2
        } else {
            1
        };
        let depth = match board.state().mover() {
            Color::B => depth,
            Color::W => depth + bonus,
        };
        let (movement, _, mate) = find_move(
            &board,
            depth,
            match board.state().mover() {
                Color::B => black_eval,
                Color::W => white_eval,
            },
        );

        if let Some(movement) = movement {
            if let Some(mate) = mate {
                println!("{movement}, mate in {mate}");
            } else {
                println!("{movement}");
            }
            board.apply_mut(movement);
            print_hboard(&board, &[movement.from()]);
        } else {
            if board.in_check(board.state().mover()) {
                println!("{:?} wins by checkmate", board.state().mover().flip());
                return;
            }
            println!("stalemate");
            return;
        }
    }
}

pub fn main() {
    auto_play(500, 4);
    // play();
}
