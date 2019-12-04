use std::io::{self, BufRead};
use std::cmp;

fn evaluate(program: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut memory = program.clone();
    memory[1] = noun;
    memory[2] = verb;

    let mut ip = 0;

    loop {
        /*
        for j in &memory {
            print!("{},", j);
        }
        println!("");
        */
        let j = memory[ip + 1];
        let k = memory[ip + 2];
        let l = memory[ip + 3];
        match memory[ip] {
            1  => memory[l] = memory[j] + memory[k],
            2  => memory[l] = memory[j] * memory[k],
            99 => break,
            _  => panic!("at the discotheque {}", ip)
        }
        ip += 4
    }

    memory[0]
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let line = handle.lines().map(|l| l.unwrap()).next().unwrap();
    let program: Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();

    println!("part1: {}", evaluate(&program, 12, 2));
    for noun in 0..100 {
        for verb in 0..100 {
            if evaluate(&program, noun, verb) == 19690720 {
                println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
            }
        }
    }
}
