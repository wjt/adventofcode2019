use std::cmp::{max, min};
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Segment {
    dir: Direction,
    /* TODO: should be unsigned */
    dist: i32,
    p: i32,
    start: i32,
    end: i32,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let points = match self.dir {
            Direction::Horizontal => (
                Point {
                    x: self.p,
                    y: self.start,
                },
                Point {
                    x: self.p,
                    y: self.end,
                },
            ),
            Direction::Vertical => (
                Point {
                    x: self.start,
                    y: self.p,
                },
                Point {
                    x: self.end,
                    y: self.p,
                },
            ),
        };
        write!(f, "{} → {}", points.0, points.1)
    }
}

type Wire = Vec<Segment>;
type Crossing = (Point, i32);

fn intersects(a: &Segment, b: &Segment) -> Option<Crossing> {
    // yuck
    if a.dir != b.dir
        && min(a.start, a.end) <= b.p
        && b.p <= max(a.start, a.end)
        && min(b.start, b.end) <= a.p
        && a.p <= max(b.start, b.end)
    {
        let point = match a.dir {
            Direction::Horizontal => Point { x: a.p, y: b.p },
            Direction::Vertical => Point { x: b.p, y: a.p },
        };
        let a_partial = (b.p - a.start).abs();
        let b_partial = (a.p - b.start).abs();
        let dist = a.dist + b.dist + a_partial + b_partial;
        Some((point, dist))
    } else {
        None
    }
}

fn parse_line(line: String) -> Wire {
    let mut x = 0;
    let mut y = 0;
    let mut dist = 0;
    let mut wire = Vec::new();

    for op in line.split(",") {
        let len: i32 = op[1..].parse().unwrap();
        match &op[0..1] {
            "L" => {
                wire.push(Segment {
                    dir: Direction::Horizontal,
                    dist,
                    p: y,
                    start: x,
                    end: x - len,
                });
                x -= len;
            }
            "R" => {
                wire.push(Segment {
                    dir: Direction::Horizontal,
                    dist,
                    p: y,
                    start: x,
                    end: x + len,
                });
                x += len;
            }
            "U" => {
                wire.push(Segment {
                    dir: Direction::Vertical,
                    dist,
                    p: x,
                    start: y,
                    end: y + len,
                });
                y += len;
            }
            "D" => {
                wire.push(Segment {
                    dir: Direction::Vertical,
                    dist,
                    p: x,
                    start: y,
                    end: y - len,
                });
                y -= len;
            }
            _ => panic!("{}", op),
        }
        dist += len;
    }

    wire
}

fn crossings(w0: &Wire, w1: &Wire) -> Vec<Crossing> {
    w0.into_iter()
        .flat_map(move |a| w1.into_iter().filter_map(move |b| intersects(&a, &b)))
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let wires: Vec<Wire> = handle.lines().map(|l| l.unwrap()).map(parse_line).collect();
    let w0 = &wires[0];
    println!("{:?}", w0);
    let w1 = &wires[1];
    println!("{:?}", w1);

    let points = crossings(&w0, &w1);

    for point in &points {
        println!("{:?}", point);
    }
    let part1 = points
        .clone()
        .into_iter()
        .map(|(p, _)| p.x.abs() + p.y.abs())
        .filter(|d| d > &0)
        .min();
    println!("{:?}", part1);

    let part2 = points
        .into_iter()
        .map(|(_, dist)| dist)
        .filter(|d| d > &0)
        .min();
    println!("{:?}", part2);
    /* Just take the min non-zero manhattan */
}
