use chessie::{defs::BitBoard, pos::Pos, print_bitboard};

fn pregen_row_slider() -> [BitBoard; 8] {
    let mut ans: [BitBoard; 8] = [0; 8];
    for (sq, bb) in ans.iter_mut().enumerate() {
        let mut v = 1 << (sq * 8);
        for _ in 0..7 {
            v |= v << 1;
        }
        *bb = v;
        print_bitboard(v);
    }

    ans
}

fn pregen_col_slider() -> [BitBoard; 8] {
    let mut ans: [BitBoard; 8] = [0; 8];
    for (sq, bb) in ans.iter_mut().enumerate() {
        let mut v = 1 << sq;
        for _ in 0..7 {
            v |= v << 8;
        }
        *bb = v;
    }

    ans
}

fn pregen_diag_slider() -> [BitBoard; 64] {
    let mut ans: [BitBoard; 64] = [0; 64];
    for (sq, bb) in ans.iter_mut().enumerate().rev() {
        let p: Pos = sq.into();
        let o = 1 << sq;

        let mut v = 0;
        for (s, _) in (p.col()..8).enumerate() {
            v |= o << (8 * s + s);
        }

        for (s, _) in (0..=p.col()).enumerate() {
            v |= o >> (8 * s + s);
        }
        *bb = v;
    }

    ans
}

fn pregen_antidiag_slider() -> [BitBoard; 64] {
    let mut ans: [BitBoard; 64] = [0; 64];
    for (sq, bb) in ans.iter_mut().enumerate().rev() {
        let p: Pos = sq.into();
        let o = 1 << sq;

        let mut v = 0;
        for (s, _) in (0..=p.col()).enumerate() {
            v |= o << (8 * s - s);
        }

        for (s, _) in (p.col()..8).enumerate() {
            v |= o >> (8 * s - s);
        }
        *bb = v;
    }

    ans
}

fn print_const(name: &str, values: &[BitBoard]) {
    println!("#[allow(clippy::unreadable_literal)]");
    println!("pub const {}: [BitBoard; {}] = [", name.to_uppercase(), values.len());
    for value in values {
        println!("  0x{value:x},");
    }
    println!("];");
    println!();
}

fn main() {
    print_const("row_slider", &pregen_row_slider());
    print_const("col_slider", &pregen_col_slider());
    print_const("diag_slider", &pregen_diag_slider());
    print_const("anti_diag_slider", &pregen_antidiag_slider());
}
