use std::collections::BTreeMap;
use std::io::{self, BufRead};
/*
use std::rc::Rc;
use std::cell::{


struct Node {
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}
*/

type Children<'a> = BTreeMap<&'a str, Vec<&'a str>>;

fn f(n: &str, d: u32, children: &Children) -> u32 {
    let e: u32 = match children.get(n) {
        None => 0,
        Some(cs) => cs.iter().map(|c| f(c, d + 1, children)).sum(),
    };
    d + e
}

fn ancestors<'a>(mut n: &'a str, parents: &BTreeMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut v = Vec::new();
    loop {
        match parents.get(n) {
            None => break,
            Some(p) => {
                v.push(*p);
                n = p;
            }
        }
    }
    v
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut children: Children = BTreeMap::new();
    children.insert("COM", Vec::new());
    let mut parents = BTreeMap::new();

    let lines: Vec<String> = handle.lines().map(|line| line.unwrap()).collect();
    let rows: Vec<Vec<&str>> = lines.iter().map(|line| line.split(')').collect()).collect();
    for row in rows {
        children.entry(row[0]).or_insert(Vec::new()).push(row[1]);
        parents.insert(row[1], row[0]);
    }

    let part1 = f("COM", 0, &children);
    println!("{}", part1);

    let mut you = ancestors("YOU", &parents);
    let mut san = ancestors("SAN", &parents);
    while you.last() == san.last() {
        you.pop();
        san.pop();
    }
    println!("{}", you.len() + san.len());
}
