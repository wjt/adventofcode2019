use std::iter::Map;
use std::io::{self, BufRead, Lines, Result};
use std::cmp;

/* day 1 */
fn fuel_for(mass: u32) -> u32 {
    cmp::max(mass / 3, 2) - 2
}

/* day 2 */
fn fuel_for_rec(mass: u32) -> u32 {
    let fuel = fuel_for(mass);
    if fuel > 0 {
        fuel + fuel_for_rec(fuel)
    } else {
        fuel
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let l: Lines<_> = handle.lines();
    let lines: Map<_, _> = l.map(|l: Result<String>| l.unwrap());
    let ints: _ = lines.map(|l| l.parse().unwrap());
    let fuel: u32 = ints.map(fuel_for_rec).sum();

    println!("{}", fuel);
}
