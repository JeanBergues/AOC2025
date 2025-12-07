use std::fs::read_to_string;

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

    let operations: Vec<i64> = f
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|op| match op {
            "+" => 0,
            "*" => 1,
            _ => -1,
        })
        .collect();

    let mut answer_a: i64 = 0;
    for (i, op) in operations.iter().enumerate() {
        let column_iter = problems.iter().map(|row| row[i]);
        answer_a += if op == &0 {
            column_iter.sum::<i64>()
        } else {
            column_iter.product()
        };
    }

    println!("Solution to part A: {}", answer_a);

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
    let answer_b: i64 = cephalopod_numbers
        .split(|n| n == &-1)
        .enumerate()
        .map(|(i, problem)| {
            if operations[i] == 0 {
                problem.into_iter().sum::<i64>()
            } else {
                problem.into_iter().product()
            }
        })
        .sum();

    println!("Solution to part B: {}", answer_b);
}
