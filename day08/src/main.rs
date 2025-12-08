use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::Hash;

struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

fn union<T: Eq + Hash + Copy>(h1: &HashSet<T>, h2: &HashSet<T>) -> HashSet<T> {
    let mut res: HashSet<T> = HashSet::new();
    for a in h1 {
        res.insert(*a);
    }
    for b in h2 {
        res.insert(*b);
    }
    res
}

fn main() {
    let f = read_to_string("src/input.txt").unwrap();
    let n_connections = 1000;
    let junction_boxes: Vec<JunctionBox> = f
        .lines()
        .map(|line| {
            let mut n_it = line.split(",").map(|n| n.parse::<i64>().unwrap());
            JunctionBox {
                x: n_it.next().unwrap(),
                y: n_it.next().unwrap(),
                z: n_it.next().unwrap(),
            }
        })
        .collect();
    let mut box_pair_dists: Vec<(usize, usize, f64)> = vec![];
    for (i, box1) in junction_boxes.iter().enumerate() {
        for j in i + 1..junction_boxes.len() {
            box_pair_dists.push((i, j, box1.distance_to(&junction_boxes[j])));
        }
    }
    box_pair_dists.sort_by(|a, b| a.2.total_cmp(&b.2));

    let mut circuits: Vec<HashSet<usize>> = vec![];
    let mut answer_a: u64 = 0;
    let mut answer_b: i64 = 0;
    for i in 0..box_pair_dists.len() {
        let (b1, b2, _) = box_pair_dists[i];
        let mut found_matches: Vec<usize> = vec![];
        for (i, circuit) in circuits.iter_mut().enumerate() {
            if circuit.contains(&b1) || circuit.contains(&b2) {
                circuit.insert(b1);
                circuit.insert(b2);
                found_matches.push(i);
            }
        }
        if found_matches.len() > 1 {
            circuits[found_matches[0]] =
                union(&circuits[found_matches[0]], &circuits[found_matches[1]]);
            circuits.remove(found_matches[1]);
        }
        if found_matches.len() == 0 {
            circuits.push(HashSet::from([b1, b2]));
        }
        if i == n_connections - 1 {
            let mut circuit_sizes: Vec<u64> = circuits.iter().map(|c| c.len() as u64).collect();
            circuit_sizes.sort();
            answer_a = circuit_sizes.last_chunk::<3>().unwrap().iter().product();
        }
        if circuits.len() == 1 && circuits[0].len() == junction_boxes.len() {
            answer_b = junction_boxes[b1].x * junction_boxes[b2].x;
            break;
        }
    }

    println!("Solution A: {}", answer_a);
    println!("Solution B: {}", answer_b);
}
