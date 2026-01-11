use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn trace_from(
    x: usize,
    y: usize,
    system_height: usize,
    splitter_has_split: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    let mut y_pos = y;
    loop {
        y_pos += 1;
        if y_pos >= system_height {
            return 1;
        }
        if let Some(i) = splitter_has_split.get(&(x, y_pos)) {
            return if *i > 0 {
                *i
            } else {
                let paths_from_here = trace_from(x - 1, y_pos, system_height, splitter_has_split)
                    + trace_from(x + 1, y_pos, system_height, splitter_has_split);
                splitter_has_split.insert((x, y_pos), paths_from_here);
                paths_from_here
            };
        }
    }
}

fn main() {
    let f = read_to_string("src/example.txt").unwrap();

    let start = Instant::now();

    // let system: Vec<&[u8]> = f.lines().map(|line| line.as_bytes()).collect();
    // Scan through the system to find splitters and start
    let mut start_xy = (0, 0);
    let mut splitter_has_split: HashMap<(usize, usize), i64> = HashMap::new();
    for (y, row) in f.lines().map(|line| line.as_bytes()).enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch as char == 'S' {
                start_xy = (x, y)
            } else if *ch as char == '^' {
                splitter_has_split.insert((x, y), 0);
            } else {
                continue;
            }
        }
    }

    let system_height = f.lines().count();
    let answer_b = trace_from(start_xy.0, start_xy.1, system_height, &mut splitter_has_split);
    let answer_a = splitter_has_split.values().filter(|v| **v > 0).count();

    let end = start.elapsed();

    println!("Solution A: {}", answer_a);
    println!("Solution B: {}", answer_b);
    println!("Took: {} micros", end.as_micros());
}
