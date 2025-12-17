use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let f = read_to_string("src/input.txt").unwrap();
    let n_shapes = 6;
    let start_time = Instant::now();

    let f_iter = f.split("\r\n\r\n");
    let shape_sizes: Vec<usize> = f_iter
        .take(n_shapes)
        .map(|s| s.chars().filter(|c| *c == '#').count())
        .collect();
    let problems = f.split("\r\n\r\n").last().unwrap();

    let mut answer_a = 0;
    for problem in problems.lines() {
        let (dims, shape_counts) = problem.split_once(":").unwrap();
        let area: u64 = dims.split("x").map(|n| n.parse::<u64>().unwrap()).product();
        let required_space: u64 = shape_counts
            .split_ascii_whitespace()
            .enumerate()
            .map(|(i, v)| v.parse::<u64>().unwrap() * shape_sizes[i] as u64)
            .sum();
        if required_space <= area { answer_a += 1 };
    }

    let end_time = start_time.elapsed();
    println!("Solution A: {}", answer_a);
    println!("Took {} micros", end_time.as_micros());
}
