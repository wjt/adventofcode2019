use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Vector {
    x: i32,
    y: i32,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Q {
    d: i32,
    v: Vector,
}

impl Vector {
    fn angle(&self) -> f64 {
        (self.y as f64).atan2(self.x as f64)
    }
}

fn risky_cmp<T>(a: &T, b: &T) -> Ordering
where
    T: PartialOrd<T>,
{
    a.partial_cmp(b).unwrap()
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

fn get_slope(a: &Vector, b: &Vector) -> Q {
    let x = b.x - a.x;
    let y = b.y - a.y;
    let d = gcd(x.abs(), y.abs());
    Q {
        d,
        v: Vector { x: x / d, y: y / d },
    }
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

fn visible_from(asteroids: &Vec<Vector>, i: usize) -> BTreeMap<Vector, Vec<Q>> {
    let a = &asteroids[i];
    let mut equivs = BTreeMap::new();
    for (j, b) in asteroids.iter().enumerate() {
        if i != j {
            let slope = get_slope(a, b);
            equivs
                .entry(slope.v)
                .or_insert_with(|| Vec::new())
                .push(Q { d: slope.d, v: *b });
        }
    }
    for v in equivs.values_mut() {
        v.sort()
    }
    equivs
}

fn part1(asteroids: &Vec<Vector>) -> (BTreeMap<Vector, Vec<Q>>, usize) {
    (0..asteroids.len())
        .map(|i| (visible_from(&asteroids, i), i))
        .max_by_key(|(v, _)| v.len())
        .unwrap()
}

fn part2(mut visible: BTreeMap<Vector, Vec<Q>>) -> Option<i32> {
    let mut dirs: Vec<Vector> = visible.keys().map(|v| *v).collect();
    dirs.sort_by(|a, b| risky_cmp(&a.angle(), &b.angle()));
    let (i, _) = dirs
        .iter()
        .enumerate()
        .filter(|(_, v)| v.angle() >= (-std::f64::consts::PI / 2.))
        .next()
        .unwrap();
    let mut deque: VecDeque<&Vector> = dirs.iter().collect();
    deque.rotate_left(i);
    let mut j = 0;
    let mut two_hundred = None;
    for dir in deque.iter().cycle() {
        if let Some(qs) = visible.get_mut(dir) {
            let q = qs.remove(0);
            println!("{} {:?}", j, q);
            if qs.is_empty() {
                visible.remove(dir);
            }
            if j == 199 {
                println!("^^^^^^^^");
                two_hundred = Some(q.v.x * 100 + q.v.y);
            }
            if visible.is_empty() {
                break;
            }
            j += 1;
        }
    }

    two_hundred
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let asteroids = parse_map(lines);

    let (v, i) = part1(&asteroids);
    println!("{:?} {:?}", v, asteroids[i]);
    println!("{}", part2(v).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let lines: Vec<String> = vec![
            String::from(".#..#"),
            String::from("....."),
            String::from("#####"),
            String::from("....#"),
            String::from("...##"),
        ];
        let asteroids = parse_map(lines);
        let (v, _) = part1(&asteroids);
        assert_eq!(v.len(), 8);
    }

    #[test]
    fn test_part2_example() {
        let lines: Vec<String> = vec![
            String::from(".#....#####...#.."),
            String::from("##...##.#####..##"),
            String::from("##...#...#.#####."),
            String::from("..#.....#...###.."),
            String::from("..#.#.....#....##"),
        ];
        let asteroids = parse_map(lines);
        let i = asteroids
            .iter()
            .position(|a| a == &Vector { x: 8, y: 3 })
            .expect("should find it");
        let visible = visible_from(&asteroids, i);
        part2(visible);
    }

    #[test]
    fn test_big_example() {
        let lines: Vec<String> = vec![
            String::from(".#..##.###...#######"),
            String::from("##.############..##."),
            String::from(".#.######.########.#"),
            String::from(".###.#######.####.#."),
            String::from("#####.##.#.##.###.##"),
            String::from("..#####..#.#########"),
            String::from("####################"),
            String::from("#.####....###.#.#.##"),
            String::from("##.#################"),
            String::from("#####.##.###..####.."),
            String::from("..######..##.#######"),
            String::from("####.##.####...##..#"),
            String::from(".#####..#.######.###"),
            String::from("##...#.##########..."),
            String::from("#.##########.#######"),
            String::from(".####.#.###.###.#.##"),
            String::from("....##.##.###..#####"),
            String::from(".#.#.###########.###"),
            String::from("#.#.#.#####.####.###"),
            String::from("###.##.####.##.#..##"),
        ];
        let asteroids = parse_map(lines);
        let (visible, i) = part1(&asteroids);
        let best = asteroids[i];
        assert_eq!(best.x, 11);
        assert_eq!(best.y, 13);
        assert_eq!(Some(802), part2(visible));
    }
}
