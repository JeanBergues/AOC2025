use std::fs::read_to_string;

fn find_maximum_joltage(bank: Vec<u32>, n_batteries: u32) -> u64 {
    let mut total_joltage: u64 = 0;
    let mut slice_start = 0;

    for battery in 1..=n_batteries {
        let padding = (n_batteries - battery) as usize;
        let largest_number = bank[slice_start..bank.len()-padding].iter().max().unwrap();
        slice_start += 1 + bank[slice_start..bank.len()-padding].iter().position(|n| n == largest_number).unwrap();
        total_joltage = 10 * total_joltage + (*largest_number as u64);
    }

    total_joltage
}

fn main() {
    let f = read_to_string("src/input.txt").unwrap();
    let total_joltage_part_a: u64 = f.lines()
        .map(|bank| {
            let bank_vec: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
            find_maximum_joltage(bank_vec, 2)
        })
        .sum();
    let total_joltage_part_b: u64 = f.lines()
        .map(|bank| {
            let bank_vec: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
            find_maximum_joltage(bank_vec, 12)
        })
        .sum();
    println!("Solution A: {}", total_joltage_part_a);
    println!("Solution B: {}", total_joltage_part_b);
}
