use std::fs::read_to_string;
use std::i8::MAX;

fn button_xor(b1: &Vec<bool>, b2: &Vec<bool>) -> Vec<bool> {
    b1.iter().zip(b2).map(|(x1, x2)| x1 ^ x2).collect()
}

fn find_least_presses(goal: &Vec<bool>, buttons: &Vec<Vec<bool>>, start: Vec<bool>, depth: i64) -> i64 {
    if depth 
    if *goal == start { return 0 };
    for b in buttons
}

fn main() {
    let f = read_to_string("src/example.txt").unwrap();
    for problem in f.lines() {
        println!("{:?}", problem);
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
        let mut answer_a = i64::MAX;
        println!("{:?}", goal_state);
        println!("{:?}", buttons);
    }
}
