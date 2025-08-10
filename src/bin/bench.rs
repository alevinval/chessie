use chessie::{eval::Scorer, fen, search::Search, util::print_board};
use std::{fs::File, io::Write, time::Instant};

fn main() {
    let positions = [
        ("Start position", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        (
            "Tactical middlegame",
            "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 4",
        ),
        (
            "Complex middlegame",
            "r2q1rk1/pp1b1ppp/2n1pn2/2bp4/2P5/2N1PN2/PP1BBPPP/R2Q1RK1 w - - 0 9",
        ),
        // ("Mate in one", "8/8/8/8/2Q4p/k6P/1N6/1K3B2 w - - 0 1"),
        // ("Mate in one variation", "8/8/8/2Q5/k6p/3N3P/8/1K3B2 w - - 0 1"),
        // ("Mate in one variation two", "8/8/8/2Q5/2B4p/2k2p1P/5N2/1K6 w - - 0 1"),
        // ("Endgame", "8/8/8/8/8/4k3/8/4K3 w - - 0 1"),
        // ("Stalemate", "7k/5K2/6Q1/8/8/8/8/8 b - - 0 1"),
        // ("Promotion", "8/P7/8/8/8/8/8/k6K w - - 0 1"),
    ];

    let depth = 6;

    let mut file = File::create(format!("bench_results_depth{}.csv", depth)).unwrap();
    writeln!(file, "name,duration_ms,nodes,nodes_sec").unwrap();
    for (scenario_name, fen_str) in positions {
        let board = fen::decode(fen_str).unwrap();
        print_board(&board);

        let mut nodes = 0usize;
        let start = Instant::now();
        let search = Search::new(&board, depth, Scorer::eval);
        let (_, stats) = search.find_with_stats();
        let duration = start.elapsed();
        let secs = duration.as_secs_f64();
        let nps = (stats.nodes - nodes) as f64 / secs;
        nodes = stats.nodes;
        writeln!(file, "\"{}\",{},{},{}", scenario_name, duration.as_millis(), nodes, nps as u64)
            .unwrap();
    }
}
