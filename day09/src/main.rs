use std::cmp::max;
use std::cmp::min;
use std::fs::read_to_string;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug)]
struct RedTile {
    x: i64,
    y: i64,
}

impl RedTile {
    fn area(&self, other: &RedTile) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

#[derive(Debug, PartialEq)]
struct ParseRedTileError;

impl FromStr for RedTile {
    type Err = ParseRedTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (xs, ys) = s.split_once(",").ok_or(ParseRedTileError)?;
        Ok(RedTile {
            x: xs.parse().or(Err(ParseRedTileError))?,
            y: ys.parse().or(Err(ParseRedTileError))?,
        })
    }
}

#[derive(Debug)]
struct Vline {
    x: i64,
    ys: i64,
    ye: i64,
}

#[derive(Debug)]
struct Hline {
    y: i64,
    xs: i64,
    xe: i64,
}

fn do_lines_cross(h: &Hline, v: &Vline) -> bool {
    (h.xs..=h.xe).contains(&v.x) && (v.ys..=v.ye).contains(&h.y)
}

fn main() {
    let f = read_to_string("src/input.txt").unwrap();

    let start = Instant::now();

    let mut red_tiles: Vec<RedTile> = f
        .lines()
        .map(|line| RedTile::from_str(line).unwrap())
        .collect();

    red_tiles.sort_by(|a, b| a.x.cmp(&b.x));
    let mut vert_lines: Vec<Vline> = vec![];
    for (i, w) in red_tiles.windows(2).enumerate() {
        if i % 2 > 0 {
            continue;
        };
        vert_lines.push(Vline {
            x: w[0].x,
            ys: min(w[0].y, w[1].y),
            ye: max(w[0].y, w[1].y),
        });
    }
    // println!("{:?}", vert_lines);

    red_tiles.sort_by(|a, b| a.y.cmp(&b.y));
    let mut hor_lines: Vec<Hline> = vec![];
    for (i, w) in red_tiles.windows(2).enumerate() {
        if i % 2 > 0 {
            continue;
        };
        hor_lines.push(Hline {
            y: w[0].y,
            xs: min(w[0].x, w[1].x),
            xe: max(w[0].x, w[1].x),
        });
    }
    // println!("{:?}", hor_lines);

    let mut rectangles: Vec<(&RedTile, &RedTile, i64)> = vec![];
    for i in 0..red_tiles.len() {
        for j in i..red_tiles.len() {
            rectangles.push((
                &red_tiles[i],
                &red_tiles[j],
                red_tiles[i].area(&red_tiles[j]),
            ));
        }
    }
    rectangles.sort_by(|a, b| b.2.cmp(&a.2));
    // println!("{:?}", rectangles);
    let answer_a = rectangles[0].2;
    println!("Solution A: {}", answer_a);

    for rectangle in rectangles.iter() {
        let mut possible = true;
        let (minx, maxx, miny, maxy) = (
            min(rectangle.0.x, rectangle.1.x),
            max(rectangle.0.x, rectangle.1.x),
            min(rectangle.0.y, rectangle.1.y),
            max(rectangle.0.y, rectangle.1.y),
        );
        let (h1, h2) = (
            Hline {
                y: miny + 1,
                xs: minx + 1,
                xe: maxx - 1,
            },
            Hline {
                y: maxy - 1,
                xs: minx + 1,
                xe: maxx - 1,
            },
        );
        for v in vert_lines.iter() {
            if do_lines_cross(&h1, v) || do_lines_cross(&h2, v) {
                possible = false;
                break;
            }
        }
        if !possible {
            continue;
        };

        let (v1, v2) = (
            Vline {
                x: minx + 1,
                ys: miny + 1,
                ye: maxy - 1,
            },
            Vline {
                x: maxx - 1,
                ys: miny + 1,
                ye: maxy - 1,
            },
        );
        for h in hor_lines.iter() {
            if do_lines_cross(h, &v1) || do_lines_cross(h, &v2) {
                possible = false;
                break;
            }
        }

        if possible {
            // println!("{:?} {:?}", rectangle.0, rectangle.1);
            println!("Solution B: {}", rectangle.2);
            let end = start.elapsed();
            println!("Solution took {} micros", end.as_micros());
            break;
        }
    }
}
