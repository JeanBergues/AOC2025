use std::fs::read_to_string;
use std::collections::HashMap;
use std::time::Instant;

fn paths_from(start: &str, goal: &str, connections: &HashMap<&str, Vec<&str>>, memo: &mut HashMap<(String, String), i64>) -> i64 {
    if start == "out" && goal != "out" { return 0 };
    if start == goal { return 1 };
    if memo.contains_key(&(start.to_string(), goal.to_string())) { return memo[&(start.to_string(), goal.to_string())] };
    let res: i64 = connections[start].iter().map(|s| paths_from(*s, goal, connections, memo)).sum();
    memo.insert((start.to_string(), goal.to_string()), res.clone());
    res
}

fn main() {
    let f = read_to_string("src/input.txt").unwrap();
    let start_time = Instant::now();
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in f.lines() {
        let (device, connected_to) = line.split_once(":").unwrap();
        connections.insert(device, connected_to.split_ascii_whitespace().collect());
    }
    connections.insert("out", vec!["svr"]);

    let mut paths_memo: HashMap<(String, String), i64> = HashMap::new();
    let answer_a = paths_from("svr", "fft", &connections, &mut paths_memo);
    let fft_dac_paths = paths_from("svr", "fft", &connections, &mut paths_memo) * paths_from("fft", "dac", &connections, &mut paths_memo) * paths_from("dac", "out", &connections, &mut paths_memo);
    let dac_fft_paths = paths_from("svr", "dac", &connections, &mut paths_memo) * paths_from("dac", "fft", &connections, &mut paths_memo) * paths_from("fft", "out", &connections, &mut paths_memo);
    let answer_b = fft_dac_paths + dac_fft_paths;

    let end_time = start_time.elapsed();
    println!("Solution A: {}", answer_a);
    println!("Solution B: {}", answer_b);
    println!("Took {:?} micros", end_time.as_micros());
}
