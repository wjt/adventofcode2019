extern crate adventofcode2019;

use adventofcode2019::{program_from_stdin, State, VM};
use std::collections::BTreeMap;
use std::fmt;

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

const WALL: char = '█';
const VALVE: char = '⊛';
const CORRIDOR: char = '·';

struct Game {
    known: BTreeMap<Point, char>,
    cost: BTreeMap<Point, usize>,
    droid: Point,
    valve: Option<Point>,
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
            valve: None,
            stack: vec![Direction::N, Direction::E, Direction::S, Direction::W],
            last: None,
        }
    }

    fn render_cell(&self, p: Point) -> char {
        if p == self.droid {
            '⚙'
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

    fn update(&mut self, vm: &mut VM) {
        if let Some(d) = &self.last {
            let mut o = vm.drain_output();
            let result = o.pop_front().unwrap();
            assert_eq!(o.len(), 0);
            let target = d.advance(self.droid);
            let cost = self.cost[&self.droid] + 1;
            if result == 0 {
                self.known.insert(target, WALL);
            } else {
                if result == 2 {
                    self.valve = Some(target);
                }

                self.droid = target;
                if !self.known.contains_key(&target) {
                    self.known
                        .insert(target, if result == 2 { VALVE } else { CORRIDOR });
                    self.cost.insert(target, cost);
                    let (back, succs) = d.succs();
                    self.stack.push(back);
                    for succ in succs {
                        if !self.known.contains_key(&succ.advance(self.droid)) {
                            self.stack.push(succ);
                        }
                    }
                }
            };
        }
    }

    fn submit(&mut self, vm: &mut VM) -> bool {
        self.last = self.stack.pop();
        if let Some(d) = &self.last {
            vm.input.push_back(*d as i64);
            true
        } else {
            false
        }
    }

    fn reset(&mut self) {
        let p = self.valve.unwrap();
        assert_eq!(p, self.droid);
        self.known.clear();
        self.known.insert(p, VALVE);
        self.cost.clear();
        self.cost.insert(p, 0);
        self.stack = vec![Direction::N, Direction::E, Direction::S, Direction::W];
    }
}

fn main() {
    let program = program_from_stdin();
    let mut vm = VM::new(&program);
    let mut game = Game::new();
    game.submit(&mut vm);
    while let State::NeedInput = vm.run() {
        game.update(&mut vm);
        if let Some(p) = &game.valve {
            println!("{} {}", p, game.cost[p]);
            game.render();
            break;
        }

        if !game.submit(&mut vm) {
            panic!("oh no");
        }
    }

    /* Now do it again, starting at the same point. */
    game.reset();
    while game.submit(&mut vm) {
        if let State::NeedInput = vm.run() {
            game.update(&mut vm);
        } else {
            break;
        }
    }
    game.render();
    println!("{}", game.cost.values().max().unwrap());
}
