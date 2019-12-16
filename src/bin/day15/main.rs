extern crate adventofcode2019;

use adventofcode2019::{program_from_stdin, Memory, State, VM};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    N = 1,
    E = 4,
    S = 2,
    W = 3,
}

impl Direction {
    fn advance(&self, p: Point) -> Point {
        match self {
            Direction::N => Point { x: p.x, y: p.y + 1 },
            Direction::E => Point { x: p.x + 1, y: p.y },
            Direction::S => Point { x: p.x, y: p.y - 1 },
            Direction::W => Point { x: p.x - 1, y: p.y },
        }
    }

    /* Stack of moves after successful move in direction of self.
     * Backwards is at the bottom.
     */
    fn succs(&self) -> (Self, Vec<Self>) {
        match self {
            Self::N => (Self::S, vec![Self::W, Self::E, Self::N]),
            Self::E => (Self::W, vec![Self::N, Self::S, Self::E]),
            Self::S => (Self::N, vec![Self::W, Self::E, Self::S]),
            Self::W => (Self::E, vec![Self::N, Self::S, Self::W]),
        }
    }
}

struct Game {
    known: BTreeMap<Point, char>,
    cost: BTreeMap<Point, usize>,
    droid: Point,
    stack: Vec<Direction>,
    last: Option<Direction>,
}

impl Game {
    fn new() -> Self {
        let mut cost = BTreeMap::new();
        let mut known = BTreeMap::new();
        let droid = Point { x: 0, y: 0 };
        known.insert(*&droid, '.');
        cost.insert(*&droid, 0);
        Self {
            known,
            cost,
            droid,
            stack: vec![Direction::N, Direction::E, Direction::S, Direction::W],
            last: None,
        }
    }

    fn render_cell(&self, p: Point) -> char {
        if p == self.droid {
            'D'
        } else {
            *self.known.get(&p).unwrap_or(&' ')
        }
    }

    fn render(&self) {
        let pmin = Point {
            x: self.known.keys().map(|p| p.x).min().unwrap(),
            y: self.known.keys().map(|p| p.y).min().unwrap(),
        };
        let pmax = Point {
            x: self.known.keys().map(|p| p.x).max().unwrap(),
            y: self.known.keys().map(|p| p.y).max().unwrap(),
        };
        for y in (pmin.y..pmax.y + 1).rev() {
            let line = (pmin.x..pmax.x + 1)
                .map(|x| self.render_cell(Point { x, y }))
                .collect::<String>();
            println!("{}", line);
        }
        println!("");
    }

    fn update(&mut self, vm: &mut VM) -> Option<(Point, usize)> {
        if let Some(d) = &self.last {
            let mut o = vm.drain_output();
            let result = o.pop_front().unwrap();
            assert_eq!(o.len(), 0);
            let target = d.advance(self.droid);
            let cost = self.cost[&self.droid] + 1;
            let ret = match result {
                0 => {
                    self.known.insert(target, '#');
                    None
                }
                1 => {
                    self.droid = target;
                    if !self.known.contains_key(&target) {
                        self.known.insert(target, '.');
                        self.cost.insert(target, cost);
                        let (back, succs) = d.succs();
                        self.stack.push(back);
                        for succ in succs {
                            if !self.known.contains_key(&succ.advance(self.droid)) {
                                self.stack.push(succ);
                            }
                        }
                    } /* else we are retracing our steps */
                    None
                }
                2 => Some((target, cost)),
                _ => panic!("oh no {}", result),
            };
            //self.render();
            ret
        } else {
            None
        }
    }

    fn submit(&mut self, vm: &mut VM) -> bool {
        self.last = self.stack.pop();
        if let Some(d) = &self.last {
            //println!("Going {:?}\n", *d);
            vm.input.push_back(*d as i64);
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut program = program_from_stdin();
    let mut vm = VM::new(&program);
    let mut game = Game::new();
    let mut stack = vec![1, 2, 3, 4];
    game.submit(&mut vm);
    while let State::NeedInput = vm.run() {
        if let Some(d) = game.update(&mut vm) {
            println!("{:?}", d);
            break;
        }

        if !game.submit(&mut vm) {
            break;
        }
    }
}
