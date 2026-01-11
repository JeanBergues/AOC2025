use std::fs::read_to_string;
use std::time::Instant;
use std::cmp::{min, max};
use std::collections::HashSet;

fn invalid_ids_in_range(start: i64, stop: i64, even_split: bool) -> i64 {
    let mut total = 0;
    for i in start..=stop {
        let i_str = i.to_string();
        let len_str = i_str.chars().count();

        // Loop over all possible sequence lengths, checking whether at least one repeats
        let mut invalid = false;
        let max_divisor = if even_split { 2 } else { len_str };
        for divisor in 2..=max_divisor {
            if len_str % divisor != 0 {
                continue;
            }; // If you cannot split the number evenly, skip

            // Split the entire number into evenly sized chunks
            let i_chunks = i_str
                .as_bytes()
                .chunks(len_str / divisor)
                .map(str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .expect("One of the elements is not valid UTF-8");

            // Only if all chunks are equal, the ID is invalid
            if i_chunks.iter().all(|chunk| chunk == &i_chunks[0]) {
                invalid = true;
                break;
            }
        }
        if invalid {
            total += i
        };
    }
    total
}

// TODO: ipv alle getallen in range checken, alle mogelijke invalid ID's constructen en kijken of die in range liggen.
fn repeated_number(digit: i64, n: u8) -> i64 {
    let mut res: i64 = 0;
    for _ in 0..n {
        res = res * 10i64.pow(digit.ilog10() + 1) + digit;
    }
    res
}

fn new_invalid_ids_in_range(start: i64, stop: i64, even_split: bool) -> i64 {
    // Assumes start and stop contain equally many digits
    let mut total: i64 = 0;
    let mut added_numbers: HashSet<i64> = HashSet::new();
    for n_length in (start.ilog10() + 1)..=(stop.ilog10() + 1) {
        let n_start = max(start, 10i64.pow(n_length-1));
        let n_stop = min(stop, 10i64.pow(n_length) - 1);

        let max_divisor = if even_split { 2 } else { n_length };
        for divisor in 2..=max_divisor {
            if n_length % divisor != 0 {
                continue;
            }; // If you cannot split the number evenly, skip
            let piece_length = n_length / divisor;
            for first_seq in (n_start / 10i64.pow(n_length - piece_length))
                ..=(n_stop / 10i64.pow(n_length - piece_length))
            {
                let n = repeated_number(first_seq, divisor as u8);
                if (start..=stop).contains(&n) && !added_numbers.contains(&n) {
                    total += n;
                    added_numbers.insert(n);
                }
            }
        }
    }
    total
}

fn main() {
    let f = read_to_string("src/input.txt").unwrap();
    let start = Instant::now();
    let invalid_id_sum_a: i64 = f
        .split(",")
        .map(|range| {
            let mut split_range = range.split("-");
            new_invalid_ids_in_range(
                split_range
                    .next()
                    .unwrap()
                    .parse()
                    .expect("Is not a number"),
                split_range
                    .next()
                    .unwrap()
                    .parse()
                    .expect("Is not a number"),
                true,
            )
        })
        .sum();
    let invalid_id_sum_b: i64 = f
        .split(",")
        .map(|range| {
            let mut split_range = range.split("-");
            new_invalid_ids_in_range(
                split_range
                    .next()
                    .unwrap()
                    .parse()
                    .expect("Is not a number"),
                split_range
                    .next()
                    .unwrap()
                    .parse()
                    .expect("Is not a number"),
                false,
            )
        })
        .sum();
    let time_take = start.elapsed();
    println!("Solution: {}", invalid_id_sum_a);
    println!("Solution: {}", invalid_id_sum_b);
    println!("Took {} micros", time_take.as_micros());
}
