use std::fmt;
use std::io::{self, BufRead};
use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct V3 {
    x: i32,
    y: i32,
    z: i32,
}

impl V3 {
    fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl fmt::Display for V3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:>5}, {:>5}, {:>5})", self.x, self.y, self.z)
    }
}

impl Add for V3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for V3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Moon {
    pos: V3,
    vel: V3,
}

impl Moon {
    fn energy(&self) -> i32 {
        self.pos.energy() * self.vel.energy()
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={}, vel={}", self.pos, self.vel)
    }
}

fn step(moons: &mut Vec<Moon>) {
    /* Update velocities */
    for i in 0..moons.len() {
        let m = moons[i].pos;
        let mut delta_v = V3::zero();
        for j in 0..moons.len() {
            if j != i {
                let n = moons[j].pos;
                delta_v += V3 {
                    x: (n.x - m.x).signum(),
                    y: (n.y - m.y).signum(),
                    z: (n.z - m.z).signum(),
                };
            }
        }
        moons[i].vel += delta_v;
    }
    for m in moons {
        m.pos += m.vel;
    }
}

fn energy(moons: &Vec<Moon>) -> i32 {
    moons.iter().map(|m| m.energy()).sum()
}

fn parse_line(line: &str) -> Moon {
    let mut bits = (&line[1..line.len() - 1])
        .split(",")
        .map(|s: &str| (s.trim()).get(2..).expect("oh no").parse().expect("OH NO"));
    let x = bits.next().expect("x");
    let y = bits.next().expect("y");
    let z = bits.next().expect("z");
    let pos = V3 { x, y, z };
    let vel = V3 { x: 0, y: 0, z: 0 };
    Moon { pos, vel }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut moons: Vec<_> = handle
        .lines()
        .map(|l| l.unwrap())
        .map(|s| parse_line(&s))
        .collect();
    for _ in 0..1000 {
        step(&mut moons);
    }
    println!("{}", energy(&moons));
}
