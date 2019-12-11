extern crate adventofcode2019;

use adventofcode2019::{Memory, State, VM};
use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    N,
    E,
    S,
    W,
}

impl Direction {
    fn left(self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
    fn right(self) -> Self {
        match self {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }

    fn advance(&self, p: Point) -> Point {
        match self {
            Direction::N => Point { x: p.x, y: p.y + 1 },
            Direction::E => Point { x: p.x + 1, y: p.y },
            Direction::S => Point { x: p.x, y: p.y - 1 },
            Direction::W => Point { x: p.x - 1, y: p.y },
        }
    }
}

fn paint(program: &Memory, start: i64) -> BTreeMap<Point, i64> {
    let mut vm = VM::new(program);
    let mut colours: BTreeMap<Point, i64> = BTreeMap::new();
    let mut point = Point { x: 0, y: 0 };
    let mut direction = Direction::N;

    colours.insert(point, start);

    loop {
        vm.input.push_back(*colours.get(&point).unwrap_or(&0));
        match vm.run() {
            State::Halted => break,
            State::NeedInput => {
                let colour = vm.output.pop_front().unwrap();
                let turn = vm.output.pop_front().unwrap();
                // println!("{} {}", colour, turn,);
                colours.insert(point, colour);
                direction = match turn {
                    0 => direction.left(),
                    1 => direction.right(),
                    _ => panic!("{}", turn),
                };
                point = direction.advance(point);
            }
        }
    }

    colours
}

fn render(hull: BTreeMap<Point, i64>) {
    let pmin = Point {
        x: hull.keys().map(|p| p.x).min().unwrap(),
        y: hull.keys().map(|p| p.y).min().unwrap(),
    };
    let pmax = Point {
        x: hull.keys().map(|p| p.x).max().unwrap(),
        y: hull.keys().map(|p| p.y).max().unwrap(),
    };

    for y in (pmin.y..(pmax.y + 1)).rev() {
        let row = (pmin.x..(pmax.x + 1))
            .rev()
            .map(|x| *hull.get(&Point { x, y }).unwrap_or(&0))
            .map(|c| if c == 0 { "  " } else { "🚀" })
            .collect::<String>();
        println!("{}", row);
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let line = handle
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("");
    let program: Memory = line.split(",").map(|s| s.parse().unwrap()).collect();

    let part1 = paint(&program, 0);
    println!("{}", part1.len());
    render(part1);
    let part2 = paint(&program, 1);
    render(part2);
}
