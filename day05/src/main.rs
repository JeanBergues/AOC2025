use std::fs::read_to_string;
use std::cmp::max;

fn main() {
    let f = read_to_string("src/input.txt").unwrap();
    let (fresh_id_ranges, ingredient_ids) = f.split_once("\n\n").unwrap();

    let mut ranges: Vec<_> = fresh_id_ranges.lines()
        .map(|range| {
            let (l, r) = range.split_once("-").unwrap();
            l.parse::<i64>().unwrap() ..= r.parse::<i64>().unwrap()
        }).collect();

    let n_fresh_ingredients = ingredient_ids.lines()
        .map(|id_str| {
            let id: i64 = id_str.parse().unwrap();
            let mut is_fresh = false;
            for range in ranges.iter() {
                if range.contains(&id) { is_fresh = true }
            }
            is_fresh
        }).filter(|b| *b).count();

    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut n_fresh_ids: i64 = 0;
    let mut highest_range_end: i64 = 0;
    for range in ranges.iter() {
        if range.end() < &highest_range_end { continue };
        n_fresh_ids += 1 + range.end() - max(range.start(), &highest_range_end);
        highest_range_end = 1 + max(*range.end(), highest_range_end);
    }

    println!("Solution A: {}", n_fresh_ingredients);
    println!("Solution B: {}", n_fresh_ids);
}
