use chessie::precompute::{antidiag_slider, col_slider, diag_slider, king, knight, row_slider};

fn print_const(name: &str, values: &[u64]) {
    println!("/// This magic bitboard is pre-generated with `cargo run --bin pregen`");
    println!("pub(crate) const {}: [BitBoard; {}] = [", name.to_uppercase(), values.len());
    for value in values {
        println!("  0x{value:x},");
    }
    println!("];");
    println!();
}

fn main() {
    print_const("king_moves", &king());
    print_const("knight_moves", &knight());
    print_const("row_slider", &row_slider());
    print_const("col_slider", &col_slider());
    print_const("diag_slider", &diag_slider());
    print_const("antidiag_slider", &antidiag_slider());
}
