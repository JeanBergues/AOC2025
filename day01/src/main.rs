use std::fs::read_to_string;

fn main() {
    let numbers_on_safe = 100;
    let f = read_to_string("src/input.txt").unwrap();

    let mut dial_position = 50;
    let mut count_zero_hits = 0;
    let mut count_zero_passes = 0;

    for line in f.lines() {
        let dir = match line.chars().nth(0).unwrap() {
            'L' => -1,
            'R' => 1,
            _ => 0,
        };
        let distance: i32 = line[1..].parse().unwrap();

        // First calculate the number of times we pass 0 due to making full rotations
        count_zero_passes += distance / numbers_on_safe;

        // Calculate if we pass 0 with the remaining rotation (after the full rotations are completed)
        let mod_move = dir * (distance % numbers_on_safe);
        if (dial_position + mod_move <= 0 || dial_position + mod_move > numbers_on_safe - 1)
            && dial_position != 0
        {
            count_zero_passes += 1;
        }

        // Update the position and correct to lie on the safe
        dial_position += dir * distance;
        dial_position = dial_position.rem_euclid(numbers_on_safe); // Using rem_euclid because I want negative numbers to wrap to positive values
        if dial_position == 0 {
            count_zero_hits += 1;
        }
    }

    println!("Solution A: {}", count_zero_hits);
    println!("Solution B: {}", count_zero_passes);
}
