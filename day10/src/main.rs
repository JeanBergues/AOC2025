use std::fs::read_to_string;
use std::time::Instant;


fn button_xor(b1: &Vec<bool>, b2: &Vec<bool>) -> Vec<bool> {
    b1.iter().zip(b2).map(|(x1, x2)| x1 ^ x2).collect()
}

fn find_least_presses(
    goal: &Vec<bool>,
    buttons: &Vec<Vec<bool>>,
    state: Vec<bool>,
    from: usize,
    pressed: i32,
) -> i32 {
    if *goal == state {
        return pressed;
    };
    buttons[from..]
        .iter()
        .enumerate()
        .map(|(i, b)| find_least_presses(goal, buttons, button_xor(&state, b), from + i + 1, pressed + 1))
        .min()
        .unwrap_or(i32::MAX)
}

fn main() {
    let f = read_to_string("src/input.txt").unwrap();
    let start = Instant::now();

    let mut answer_a = 0;
    for problem in f.lines() {
        let mut goal_state: Vec<bool> = vec![];
        let mut buttons: Vec<Vec<bool>> = vec![];
        let mut joltages: Vec<i64> = vec![];

        for (i, s) in problem.split(" ").enumerate() {
            if i == 0 {
                goal_state = s[1..s.len() - 1].chars().map(|c| c == '#').collect()
            } else if i == problem.split(" ").count() - 1 {
                joltages = s[1..s.len() - 1]
                    .split(",")
                    .map(|c| c.parse().unwrap())
                    .collect();
            } else {
                let mut new_button: Vec<bool> = vec![false; goal_state.len()];
                for b in s[1..s.len() - 1]
                    .split(",")
                    .map(|c| c.parse::<usize>().unwrap())
                {
                    new_button[b] = true;
                }
                buttons.push(new_button);
            }
        }
        answer_a += find_least_presses(&goal_state, &buttons, vec![false; goal_state.len()], 0, 0);
    }

    let end = start.elapsed();
    println!("Solution A: {}", answer_a);
    println!("Time taken: {} micros", end.as_micros());
}
