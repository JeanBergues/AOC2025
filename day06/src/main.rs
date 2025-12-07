use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let f = read_to_string("src/input.txt").unwrap();
    let problems: Vec<Vec<i64>> = f
        .lines()
        .enumerate()
        .filter(|&(i, _)| i != f.lines().count() - 1)
        .map(|(_, row)| {
            row.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    let mut operations: Vec<&str> = f.lines().last().unwrap().split_whitespace().collect();

    let mut answer_a: i64 = 0;
    for (i, op) in operations.iter().enumerate() {
        let column_iter = problems.iter().map(|row| row[i]);
        answer_a += if *op == "+" {
            column_iter.sum::<i64>()
        } else {
            column_iter.product()
        };
    }

    println!("Solution to part A: {}", answer_a);

    // Oude versie, volledig in memory
    let start_oud = Instant::now();
    let problems_ascii_vec: Vec<&[u8]> = f.lines().map(|l| l.as_bytes()).collect();
    let mut cephalopod_numbers: Vec<i64> = vec![-1; problems_ascii_vec[0].len()];

    for i in 0..problems_ascii_vec[0].len() {
        let column_iter: Vec<u8> = problems_ascii_vec
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != problems_ascii_vec.len() - 1)
            .map(|(_, row)| row[i])
            .collect();
        cephalopod_numbers[i] = String::from_utf8(column_iter)
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap_or(-1);
    }
    let _answer_b: i64 = cephalopod_numbers
        .split(|n| n == &-1)
        .enumerate()
        .map(|(i, problem)| {
            if operations[i] == "+" {
                problem.into_iter().sum::<i64>()
            } else {
                problem.into_iter().product()
            }
        })
        .sum();

    let end_oud = start_oud.elapsed();
    println!("Oud took {} microseconds.", end_oud.as_micros());

    // Nieuwe versie, met minder collects
    let start_nieuw = Instant::now();
    operations.reverse(); // Need to reverse as vec only implements popping the last element
    let problems_ascii_vec: Vec<&[u8]> = f
        .lines()
        .enumerate()
        .filter(|&(i, _)| i != f.lines().count() - 1)
        .map(|(_, l)| l.as_bytes())
        .collect();
    let mut answer_b: i64 = 0;
    let mut current_operator = operations.pop().unwrap();
    let mut problem_total: i64 = if current_operator == "+" { 0 } else { 1 };
    for i in 0..problems_ascii_vec[0].len() {
        let column_string: String = String::from_utf8(
            problems_ascii_vec
                .iter()
                .map(|row| row[i])
                .collect::<Vec<u8>>(),
        )
        .unwrap();
        match column_string.trim().parse::<i64>() {
            Ok(i) => {
                if current_operator == "+" {
                    problem_total += i
                } else {
                    problem_total *= i
                }
            }
            Err(_e) => {
                answer_b += problem_total;
                current_operator = operations.pop().unwrap();
                problem_total = if current_operator == "+" { 0 } else { 1 };
            }
        }
    }

    // Add the last sum
    answer_b += problem_total;

    let end_nieuw = start_nieuw.elapsed();
    println!("Nieuw took {} microseconds.", end_nieuw.as_micros());
    // OUD = ~2500 micros, NIEUW = ~1700 micros, significant sneller dus

    println!("Solution to part B: {}", answer_b);
}
