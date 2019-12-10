use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Vector {
    x: i32,
    y: i32,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    assert!(x >= 0);
    assert!(y >= 0);
    let (mut a, mut b) = if x > y { (x, y) } else { (y, x) };

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}

fn get_slope(a: &Vector, b: &Vector) -> Vector {
    let x = b.x - a.x;
    let y = b.y - a.y;
    let d = gcd(x.abs(), y.abs());
    Vector { x: x / d, y: y / d }
}

fn parse_map(lines: Vec<String>) -> Vec<Vector> {
    let mut asteroids = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                asteroids.push(Vector {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    asteroids
}

fn visible_from(asteroids: &Vec<Vector>, i: usize) -> usize {
    let a = &asteroids[i];
    let mut equivs = BTreeMap::new();
    for (j, b) in asteroids.iter().enumerate() {
        if i != j {
            let slope = get_slope(a, b);
            *equivs.entry(slope).or_insert(0) += 1;
        }
    }
    equivs.len()
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let asteroids = parse_map(lines);

    let (v, i) = (0..asteroids.len())
        .map(|i| (visible_from(&asteroids, i), i))
        .max()
        .unwrap();
    println!("{:?} {:?}", v, asteroids[i],);
}
