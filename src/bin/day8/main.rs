use itertools::Itertools;
use std::env;
use std::io::{self, BufRead};

fn count_digit(layer: &Vec<char>, c: char) -> usize {
    layer.iter().filter(|&d| *d == c).count()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let width: usize = args[0].parse().unwrap();
    let height: usize = args[1].parse().unwrap();
    let stdin = io::stdin();
    let handle = stdin.lock();
    let line = handle
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("");
    let layers: Vec<Vec<char>> = line
        .chars()
        .chunks(width * height)
        .into_iter()
        .map(|layer| layer.collect::<Vec<char>>())
        .collect();
    let fewest_0_digits = layers
        .iter()
        .min_by_key(|layer| count_digit(layer, '0'))
        .unwrap();
    println!(
        "{}",
        count_digit(fewest_0_digits, '1') * count_digit(fewest_0_digits, '2')
    );
}
