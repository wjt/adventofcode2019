use adventofcode2019::lcm;
use num_rational::Ratio;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::io::{self, BufRead};
use std::ops::{Add, AddAssign};

type Q = Ratio<usize>;

fn parse_chem(s: &str) -> (&str, Q) {
    let bits = s.split_whitespace().collect::<Vec<_>>();
    let quantity = Q::from_integer(bits[0].parse().expect(s));
    let chemical = bits[1];
    (chemical, quantity)
}

fn parse_input<'a>(
    lines: &'a Vec<std::string::String>,
) -> HashMap<&'a str, (Q, Vec<(&'a str, Q)>)> {
    let mut reactions: HashMap<&str, (Q, Vec<(&str, Q)>)> = HashMap::new();
    for line in lines {
        let bits: Vec<_> = line.split(" => ").collect();
        let inputs: Vec<_> = bits[0].split(", ").map(parse_chem).collect();
        let output = parse_chem(bits[1]);
        reactions.insert(output.0, (output.1, inputs));
    }
    reactions
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: Vec<_> = handle.lines().map(|l| l.unwrap()).collect();
    let reactions = parse_input(&lines);
    println!("{:?}", reactions);
    let mut need: Vec<(&str, Q)> = Vec::new();
    need.push(("FUEL", Q::from_integer(1)));
    let mut need_ore = Q::from_integer(0);
    while let Some((need_chem, need_quantity)) = need.pop() {
        if need_chem == "ORE" {
            need_ore += need_quantity;
        } else if let Some((output_q, inputs)) = reactions.get(need_chem) {
            /* No, must quantize output. */
            /* But this is too early to do so. */
            let q = need_quantity / output_q;
            for (input_chem, input_q) in inputs.iter() {
                let v = (*input_chem, input_q * q);
                println!("for {} {}, need {} {}", need_quantity, need_chem, v.1, v.0);
                need.push(v);
            }
        }
    }
    println!("{}", need_ore);
}
