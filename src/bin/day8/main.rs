use itertools::Itertools;
use std::env;
use std::io::{self, BufRead};

fn format_layer(layer: &str, width: usize) {
    for line in &layer.chars().chunks(width) {
        print!("  ");
        for c in line {
            if c == '1' {
                print!(".")
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn count_digit(layer: &str, c: char) -> usize {
    layer.chars().filter(|&d| d == c).count()
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
    let layers: Vec<String> = line
        .chars()
        .chunks(width * height)
        .into_iter()
        .map(|layer| layer.collect::<String>())
        .collect();
    let fewest_0_digits = layers
        .iter()
        .min_by_key(|layer| count_digit(layer, '0'))
        .unwrap();
    format_layer(fewest_0_digits, width);
    println!(
        "{}",
        count_digit(fewest_0_digits, '1') * count_digit(fewest_0_digits, '2')
    );

    let zero: String = "2".repeat(width * height);
    let composited = layers.iter().fold(zero, |upper, lower| {
        upper
            .chars()
            .zip_eq(lower.chars())
            .map(|(c, d)| if c == '2' { d } else { c })
            .collect::<String>()
    });
    format_layer(&composited, width);
}
