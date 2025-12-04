use std::fs;

fn count_neighbours(diagram: &Vec<Vec<bool>>, x: i32, y: i32) -> u32 {
    let mut neighbours = 0;
    let (max_x, max_y) = (diagram[0].len(), diagram.len());
    for dx in -1..=1 {
        for dy in -1..=1 {
            if y + dy < 0 || y + dy >= max_y as i32 { continue }
            if x + dx < 0 || x + dx >= max_x as i32 { continue }
            if dx == 0 && dy == 0 { continue }
            if diagram[(y+dy) as usize][(x+dx) as usize] {
                neighbours += 1;
            }
        }
    }
    neighbours
}

fn main() {
    let f = fs::read_to_string("src/input.txt").unwrap();
    let mut diagram: Vec<Vec<bool>> = f.lines()
        .map(|line| line.chars()
            .map(|c| c == '@')
            .collect()
        ).collect();

    let mut accessible_rolls = 0;
    let mut all_rolls_inaccessible = false;
    let mut printed_solution_a = false;

    while !all_rolls_inaccessible {
        let mut rolls_to_remove: Vec<(usize, usize)> = vec!();
        for x in 0..diagram[0].len() {
            for y in 0..diagram.len() {
                if diagram[y][x] && count_neighbours(&diagram, x as i32, y as i32) < 4 {
                    accessible_rolls += 1;
                    rolls_to_remove.push((x, y));
                }
            }
        }

        all_rolls_inaccessible = rolls_to_remove.len() == 0;
        for (x, y) in rolls_to_remove {
            diagram[y][x] = false;
        }
        if !printed_solution_a {
            println!("Solution A: {}", accessible_rolls);
            printed_solution_a = true;
        }
    }
    println!("Solution B: {}", accessible_rolls);
}
