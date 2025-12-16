use std::fs::read_to_string;
use std::time::Instant;
use good_lp::variable::ProblemVariables;
use good_lp::{
    constraint, default_solver, variable, variables, Constraint, Expression, Solution, SolverModel,
    Variable,
};

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

// GOOD_LP DEFINITION STRUCTS
struct ButtonProblem {
    buttons: ProblemVariables,
    pressed: Expression,
    machine: JoltageMachine,
}

struct Button {
    b0: u8,
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8,
    b5: u8,
    b6: u8,
    b7: u8,
    b8: u8,
    b9: u8,
}

impl Button {
    fn from_vec(v: &Vec<bool>) -> Button {
        assert_eq!(v.len(), 10);
        Button {
            b0: if v[0] {1} else {0},
            b1: if v[1] {1} else {0},
            b2: if v[2] {1} else {0},
            b3: if v[3] {1} else {0},
            b4: if v[4] {1} else {0},
            b5: if v[5] {1} else {0},
            b6: if v[6] {1} else {0},
            b7: if v[7] {1} else {0},
            b8: if v[8] {1} else {0},
            b9: if v[9] {1} else {0},
        }
    }
}

struct Joltage {
    joltage_goal: u16,
    joltage_value: Expression
}

struct JoltageMachine {
    j0: Joltage,
    j1: Joltage,
    j2: Joltage,
    j3: Joltage,
    j4: Joltage,
    j5: Joltage,
    j6: Joltage,
    j7: Joltage,
    j8: Joltage,
    j9: Joltage,
}

impl ButtonProblem {
    fn new(goal: Vec<u16>) -> ButtonProblem {
        assert_eq!(goal.len(), 10);
        ButtonProblem {
            buttons: variables!(),
            pressed: 0.into(),
            machine: JoltageMachine {
                j0: Joltage {joltage_goal: goal[0], joltage_value: 0.into()},
                j1: Joltage {joltage_goal: goal[1], joltage_value: 0.into()},
                j2: Joltage {joltage_goal: goal[2], joltage_value: 0.into()},
                j3: Joltage {joltage_goal: goal[3], joltage_value: 0.into()},
                j4: Joltage {joltage_goal: goal[4], joltage_value: 0.into()},
                j5: Joltage {joltage_goal: goal[5], joltage_value: 0.into()},
                j6: Joltage {joltage_goal: goal[6], joltage_value: 0.into()},
                j7: Joltage {joltage_goal: goal[7], joltage_value: 0.into()},
                j8: Joltage {joltage_goal: goal[8], joltage_value: 0.into()},
                j9: Joltage {joltage_goal: goal[9], joltage_value: 0.into()},
            },
        }
    }

    fn add(&mut self, button: Button) -> Variable {
        let times_to_press = self.buttons.add(variable().min(0).integer());
        self.pressed += times_to_press;
        self.machine.j0.joltage_value += times_to_press * button.b0;
        self.machine.j1.joltage_value += times_to_press * button.b1;
        self.machine.j2.joltage_value += times_to_press * button.b2;
        self.machine.j3.joltage_value += times_to_press * button.b3;
        self.machine.j4.joltage_value += times_to_press * button.b4;
        self.machine.j5.joltage_value += times_to_press * button.b5;
        self.machine.j6.joltage_value += times_to_press * button.b6;
        self.machine.j7.joltage_value += times_to_press * button.b7;
        self.machine.j8.joltage_value += times_to_press * button.b8;
        self.machine.j9.joltage_value += times_to_press * button.b9;
        times_to_press
    }

    fn constraints(m: JoltageMachine) -> Vec<Constraint> {
        let mut constraints = Vec::with_capacity(10);
        for c in [m.j0, m.j1, m.j2, m.j3, m.j4, m.j5, m.j6, m.j7, m.j8, m.j9] {
            constraints.push(constraint!(c.joltage_value == c.joltage_goal));
        }
        constraints
    }

    fn least_needed_presses(self) -> impl Solution {
        let objective = self.pressed;
        self.buttons.minimise(objective).using(default_solver).with_all(Self::constraints(self.machine)).solve().unwrap()
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
        let mut joltages: Vec<u16> = vec![0; 10];

        for (i, s) in problem.split(" ").enumerate() {
            if i == 0 {
                goal_state = s[1..s.len() - 1].chars().map(|c| c == '#').collect()
            } else if i == problem.split(" ").count() - 1 {
                for j in s[1..s.len() - 1]
                    .split(",")
                    .enumerate()
                    .map(|(i, c)| (i, c.parse::<u16>().unwrap()))
                {
                    joltages[j.0] = j.1;
                }
            } else {
                let mut new_button: Vec<bool> = vec![false; 10];
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

        let mut pb = ButtonProblem::new(joltages);
        let variables: Vec<_> = buttons.iter().map(|b| pb.add(Button::from_vec(b))).collect();
        let solution = pb.least_needed_presses();
        let presses_per_button: Vec<i32> = variables.iter().map(|&v| solution.value(v).round() as i32).collect();
        println!("{}", problem);
        println!("{:?}", presses_per_button);
        answer_b += presses_per_button.iter().sum::<i32>();
    }

    let end = start.elapsed();
    println!("Solution A: {}", answer_a);
    println!("Solution B: {}", answer_b);
    println!("Time taken: {} micros", end.as_micros());
}
