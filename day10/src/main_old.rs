use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;

fn button_xor(b1: &Vec<bool>, b2: &Vec<bool>) -> Vec<bool> {
    b1.iter().zip(b2).map(|(x1, x2)| x1 ^ x2).collect()
}

fn button_sub(b1: &Vec<u8>, b2: &Vec<u8>, ntimes: u8) -> Vec<u8> {
    b1.iter().zip(b2).map(|(x1, x2)| x1 - x2 * ntimes).collect()
}

fn most_important_button(buttons: &Vec<Vec<u8>>, button: &Vec<u8>) -> usize {
    let mut res = usize::MAX;
    for (i, v) in button.iter().enumerate() {
        if *v == 0 {continue}
        let mut total = 0;
        for b in buttons {
            if b[i] == 1 { total += 1 }
        }
        if total < res { res = total }
    }
    res
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
        .map(|(i, b)| {
            find_least_presses(
                goal,
                buttons,
                button_xor(&state, b),
                from + i + 1,
                pressed + 1,
            )
        })
        .min()
        .unwrap_or(i32::MAX)
}

fn find_least_joltage_presses_memo(buttons: &Vec<Vec<u8>>, state: Vec<u8>, i: usize, memo: &mut HashMap<(Vec<u8>, usize), Option<i32>>) -> Option<i32> {
    match memo.get(&(state.clone(), i)) {
        Some(v) => {return *v},
        None => {}
    }
    let mut res: Option<i32> = None;
    let mut times = *(state
        .iter()
        .enumerate()
        .filter(|(j, _)| buttons[i][*j] > 0)
        .map(|(_, v)| v)
        .min()
        .unwrap());
    let mut clone = button_sub(&state, &buttons[i], times);
    if clone.iter().map(|x| *x as u32).sum::<u32>() == 0 {
        res = Some(times as i32);
    } else if i == buttons.len() - 1 {
        res = None;
    } else {
        while times > 0 {
            match find_least_joltage_presses_memo(buttons, clone.clone(), i + 1, memo) {
                // Slecht voor performance, al dat clonen
                Some(v) => {
                    // println!("Gelukt - {:?}, b={:?}, t={}", clone, buttons[i], times);
                    res = Some(times as i32 + v);
                    memo.insert((state, i), res);
                    return res;
                }
                None => {
                    // println!("Oeps - {:?}, b={:?}, t={}", state, buttons[i], times);
                    let mut early_return = true;
                    for (j, v) in buttons[i].iter().enumerate() {
                        if *v == 0 { continue }
                        for b in buttons[(i+1)..].iter() {
                            if b[j] == 1 { early_return = false; break }
                        }
                        if !early_return { break }
                    }
                    if early_return { return None }
                    times -= 1;
                    clone = button_sub(&state, &buttons[i], times)
                }
            }
        }
        match find_least_joltage_presses_memo(buttons, clone.clone(), i + 1, memo) {
            // Slecht voor performance, al dat clonen
            Some(v) => {
                // println!("Gelukt - {:?}, b={:?}, t={}", clone, buttons[i], times);
                res = Some(times as i32 + v);
            }
            None => {
                // println!("Oeps - {:?}, b={:?}, t={}", state, buttons[i], times);
                res = None;
            }
        }
    };
    memo.insert((state, i), res);
    res
}

fn find_least_joltage_presses(buttons: &Vec<Vec<u8>>, state: Vec<u8>, i: usize) -> Option<i32> {
    let mut times = *(state
        .iter()
        .enumerate()
        .filter(|(j, _)| buttons[i][*j] > 0)
        .map(|(_, v)| v)
        .min()
        .unwrap());
    let mut clone = button_sub(&state, &buttons[i], times);
    if clone.iter().map(|x| *x as u32).sum::<u32>() == 0 {
        Some(times as i32)
    } else if i == buttons.len() - 1 {
        None
    } else {
        while times > 0 {
            match find_least_joltage_presses(buttons, clone.clone(), i + 1) {
                // Slecht voor performance, al dat clonen
                Some(v) => {
                    // println!("Gelukt - {:?}, b={:?}, t={}", clone, buttons[i], times);
                    return Some(times as i32 + v);
                }
                None => {
                    // println!("Oeps - {:?}, b={:?}, t={}", state, buttons[i], times);
                    let mut early_return = true;
                    for (j, v) in buttons[i].iter().enumerate() {
                        if *v == 0 { continue }
                        for b in buttons[(i+1)..].iter() {
                            if b[j] == 1 { early_return = false; break }
                        }
                        if !early_return { break }
                    }
                    if early_return { return None }
                    times -= 1;
                    clone = button_sub(&state, &buttons[i], times)
                }
            }
        }
        match find_least_joltage_presses(buttons, clone.clone(), i + 1) {
            // Slecht voor performance, al dat clonen
            Some(v) => {
                // println!("Gelukt - {:?}, b={:?}, t={}", clone, buttons[i], times);
                Some(times as i32 + v)
            }
            None => {
                // println!("Oeps - {:?}, b={:?}, t={}", state, buttons[i], times);
                None
            }
        }
    }
}

fn main() {
    let f = read_to_string("src/input.txt").unwrap();
    let start = Instant::now();

    let mut answer_a = 0;
    let mut answer_b = 0;
    for problem in f.lines() {
        let mut goal_state: Vec<bool> = vec![];
        let mut buttons: Vec<Vec<bool>> = vec![];
        let mut nbuttons: Vec<Vec<u8>> = vec![];
        let mut joltages: Vec<u8> = vec![];

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
                let mut new_nbutton: Vec<u8> = vec![0; goal_state.len()];
                for b in s[1..s.len() - 1]
                    .split(",")
                    .map(|c| c.parse::<usize>().unwrap())
                {
                    new_button[b] = true;
                    new_nbutton[b] = 1;
                }
                buttons.push(new_button);
                nbuttons.push(new_nbutton);
            }
        }
        answer_a += find_least_presses(&goal_state, &buttons, vec![false; goal_state.len()], 0, 0);
        let mut sorted_nbuttons = nbuttons.clone();
        //sorted_nbuttons.sort_by(|a, b| most_important_button(&nbuttons, &a).cmp(&most_important_button(&nbuttons, &b)));
        sorted_nbuttons.sort_by(|a, b| b.iter().sum::<u8>().cmp(&a.iter().sum::<u8>()));
        drop(nbuttons);
        println!("{:?}", sorted_nbuttons);
        println!("{:?}", problem);
        // println!("{:?}", joltages);
        // println!("{:?}", nbuttons);

        let mut memo: HashMap<(Vec<u8>, usize), Option<i32>> = HashMap::new();
        //let temp_total = find_least_joltage_presses(&sorted_nbuttons, joltages, 0).unwrap();
        let temp_total = find_least_joltage_presses(&sorted_nbuttons, joltages, 0).unwrap();
        answer_b += temp_total;
        println!("Least presses: {}", temp_total)
    }

    let end = start.elapsed();
    println!("Solution A: {}", answer_a);
    println!("Solution B: {}", answer_b);
    println!("Time taken: {} micros", end.as_micros());
}