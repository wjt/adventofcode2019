use std::env;
use std::io::{self, BufRead};

struct VM {
    memory: Vec<i64>,
    ip: usize,
    input: Vec<i64>,
    output: Vec<i64>,
}

impl VM {
    fn new(program: &Vec<i64>, input: Vec<i64>) -> Self {
        VM {
            memory: program.clone(),
            ip: 0,
            input,
            output: Vec::new(),
        }
    }

    fn arg(&self, i: u32) -> i64 {
        let instruction = self.memory[self.ip];
        let mode = (instruction / (10 * (10_i64.pow(i)))) % 10;
        let val = self.memory[self.ip + i as usize];
        match mode {
            0 => self.memory[val as usize],
            1 => val,
            _ => panic!("{}", mode),
        }
    }

    fn int3<F>(&mut self, f: F)
    where
        F: Fn(i64, i64) -> i64,
    {
        let l = self.memory[self.ip + 3];
        self.memory[l as usize] = f(self.arg(1), self.arg(2));
        self.ip += 4
    }

    fn step(&mut self) -> bool {
        let instruction = self.memory[self.ip];
        match instruction % 100 {
            1 => self.int3(|a, b| a + b),
            2 => self.int3(|a, b| a * b),
            3 => {
                let j = self.memory[self.ip + 1];
                self.memory[j as usize] = self.input.pop().unwrap();
                self.ip += 2
            }
            4 => {
                self.output.push(self.arg(1));
                self.ip += 2
            }
            5 => {
                if self.arg(1) != 0 {
                    self.ip = self.arg(2) as usize
                } else {
                    self.ip += 3;
                }
            }
            6 => {
                if self.arg(1) == 0 {
                    self.ip = self.arg(2) as usize
                } else {
                    self.ip += 3;
                }
            }
            7 => self.int3(|a, b| (a < b) as i64),
            8 => self.int3(|a, b| (a == b) as i64),
            99 => return false,
            x => panic!("the discotheque: ip {}: {}", self.ip, x),
        }
        true
    }

    fn run(&mut self) {
        loop {
            if !self.step() {
                break;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let input = args.iter().map(|s| s.parse().unwrap()).collect();
    let stdin = io::stdin();
    let handle = stdin.lock();
    let line = handle.lines().map(|l| l.unwrap()).next().unwrap();
    let program: Vec<i64> = line.split(",").map(|s| s.parse().unwrap()).collect();
    let mut vm = VM::new(&program, input);

    vm.run();

    /*
    for (i, x) in vm.memory.iter().enumerate() {
        println!("memory[{}] = {}", i, x);
    }
    */
    for x in vm.output {
        println!("{}", x);
    }
}
