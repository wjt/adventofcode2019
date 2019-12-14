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

struct Game {
    grid: [[i64; 35]; 25],
    paddle: Point,
    ball_dir: i64,
    ball: Point,
    score: i64,
}
impl Game {
    fn new() -> Self {
        Self {
            grid: [[0; 35]; 25],
            paddle: Point { x: 0, y: 0 },
            ball_dir: 0,
            ball: Point { x: 0, y: 0 },
            score: 0,
        }
    }

    fn set(&mut self, x: i64, y: i64, cell: i64) {
        if x == -1 && y == 0 {
            self.score = cell;
        } else {
            self.grid[y as usize][x as usize] = cell;
            if (cell == 3) {
                self.paddle = Point { x, y };
            } else if (cell == 4) {
                self.ball_dir = (x - self.ball.x).signum();
                self.ball = Point { x, y };
            }
        }
    }

    fn render_cell(cell: i64) -> char {
        match cell {
            0 => ' ',
            1 => '█',
            2 => '▣',
            3 => '━',
            4 => '⏺',
            _ => panic!("{}", cell),
        }
    }

    fn render(&self) {
        for row in &self.grid {
            println!(
                "{}",
                row.iter()
                    .map(|&cell| Self::render_cell(cell))
                    .collect::<String>()
            );
        }
        println!("{:^36}", self.score);
    }

    fn update(&mut self, vm: &mut VM) {
        for chunk in &vm.drain_output().into_iter().chunks(3) {
            if let Some((x, y, cell)) = chunk.collect_tuple() {
                self.set(x, y, cell);
            }
        }
        self.render();
    }
}

fn main() {
    let mut program = program_from_stdin();
    program[0] = 2;

    let mut vm = VM::new(&program);
    let mut game = Game::new();
    while let State::NeedInput = vm.run() {
        game.update(&mut vm);

        let joystick = (game.ball.x - game.paddle.x).signum();
        vm.input.push_back(joystick);
        println!("");
    }
    game.update(&mut vm);
}
