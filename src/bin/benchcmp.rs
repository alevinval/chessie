use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug)]
struct BenchResult {
    name: String,
    duration_ms: i64,
    total_nodes: i64,
    nodes_per_sec: i64,
}

fn parse_line(line: &str) -> Option<BenchResult> {
    let parts: Vec<_> = line.split(',').collect();
    if parts.len() < 4 {
        return None;
    }
    Some(BenchResult {
        name: parts[0].trim_matches('"').to_string(),
        duration_ms: parts[1].parse().ok()?,
        total_nodes: parts[2].parse().ok()?,
        nodes_per_sec: parts[3].parse().ok()?,
    })
}

fn read_csv(path: &str) -> (Vec<BenchResult>, HashMap<String, BenchResult>) {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut order = Vec::new();
    let mut map = HashMap::new();
    for line in reader.lines().skip(1) {
        if let Ok(l) = line
            && let Some(res) = parse_line(&l)
        {
            order.push(res.clone());
            map.insert(res.name.clone(), res);
        }
    }
    (order, map)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: benchcmp <old.csv> <new.csv>");
        std::process::exit(1);
    }
    let (_, old_map) = read_csv(&args[1]);
    let (new_order, _) = read_csv(&args[2]);

    fn diff_str(old: i64, new: i64) -> String {
        let diff = new - old;
        if diff == 0 {
            " 0".to_string()
        } else if diff > 0 {
            format!("+{}", diff)
        } else {
            format!("{}", diff)
        }
    }

    println!(
        "{:<25} {:>10} {:>10} {:>8} {:>10} {:>10} {:>8} {:>10} {:>10} {:>8}",
        "Name",
        "OldTime",
        "NewTime",
        "ΔTime",
        "OldNodes",
        "NewNodes",
        "ΔNodes",
        "OldNPS",
        "NewNPS",
        "ΔNPS"
    );
    for new_res in &new_order {
        if let Some(old_res) = old_map.get(&new_res.name) {
            println!(
                "{:<25} {:>10} {:>10} {:>8} {:>10} {:>10} {:>8} {:>10} {:>10} {:>8}",
                new_res.name,
                old_res.duration_ms,
                new_res.duration_ms,
                diff_str(old_res.duration_ms, new_res.duration_ms),
                old_res.total_nodes,
                new_res.total_nodes,
                diff_str(old_res.total_nodes, new_res.total_nodes),
                old_res.nodes_per_sec,
                new_res.nodes_per_sec,
                diff_str(old_res.nodes_per_sec, new_res.nodes_per_sec)
            );
        }
    }
}
