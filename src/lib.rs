use std::io;

use board::Board;
use color::Color;
use defs::Sq;
use eval::{Scorer, legacy::LegacyScorer};
use moves::Move;
use search::Search;
use util::{print_board, print_hboard};

pub mod bits;
mod board;
mod color;
pub mod defs;
pub mod eval;
pub mod fen;
pub mod magic;
mod moves;
mod piece;
pub mod pos;
pub mod search;
pub mod squares;
pub mod util;

fn read_sq() -> Sq {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let col: u8 = line.trim().parse().unwrap();
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let row: u8 = line.trim().parse().unwrap();
    row * 8 + col
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

        let result = Search::new(&board, 4, legacy_eval).find();
        if let Some(movement) = result.movement {
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

        let eval = match board.state().mover() {
            Color::B => black_eval,
            Color::W => white_eval,
        };

        let result = Search::new(&board, depth, eval).find();

        if let Some(movement) = result.movement {
            if let Some(mate) = result.mate {
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
