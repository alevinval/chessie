use std::io;

use board::Board;
use color::Color;
use moves::Move;
use pos::Pos;
use search::minmax;
use util::print_board;

mod bits;
mod board;
mod color;
mod defs;
mod eval;
mod magic;
mod moves;
mod piece;
mod pos;
pub mod precompute;
mod search;
mod util;

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
        let (movement, eval, mate) =
            minmax(&board, depth, -f64::INFINITY, f64::INFINITY, true, board.state().mover());

        if let Some(movement) = movement {
            if let Some(mate) = mate {
                println!("Mate in {}", mate - board.state().n());
            }
            println!(
                "{} => {:?} to play... {movement:?} ({eval}) (depth={depth}",
                board.state().n(),
                board.state().mover()
            );
            board = movement.apply(&board);
            print_board(&board, &[movement.from()]);
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
