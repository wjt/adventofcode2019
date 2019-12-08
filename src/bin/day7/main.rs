use itertools::Itertools;
use std::collections::VecDeque;
use std::io::{self, BufRead};

type Memory = Vec<i64>;
type IO = VecDeque<i64>;

struct VM {
    memory: Memory,
    ip: usize,
    input: IO,
    output: IO,
}

enum State {
    Running,
    Halted,
    NeedInput,
}

impl VM {
    fn new(program: &Memory, input: IO) -> Self {
        VM {
            memory: program.clone(),
            ip: 0,
            input,
            output: IO::new(),
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

    fn step(&mut self) -> State {
        let instruction = self.memory[self.ip];
        match instruction % 100 {
            1 => self.int3(|a, b| a + b),
            2 => self.int3(|a, b| a * b),
            3 => {
                let j = self.memory[self.ip + 1];
                match self.input.pop_front() {
                    Some(val) => {
                        self.memory[j as usize] = val;
                        self.ip += 2;
                    }
                    None => {
                        return State::NeedInput;
                    }
                }
            }
            4 => {
                self.output.push_back(self.arg(1));
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
            99 => return State::Halted,
            x => panic!("the discotheque: ip {}: {}", self.ip, x),
        }
        State::Running
    }

    fn run(&mut self) -> State {
        loop {
            match self.step() {
                State::Running => {}
                x => return x,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let input = VecDeque::new();
        let mut vm = VM::new(&program, input);
        vm.run();

        assert_eq!(vm.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    fn run_with_input(program: &Memory, input: IO) -> (Memory, IO) {
        let mut vm = VM::new(&program, input);
        vm.run();

        (vm.memory, vm.output)
    }

    #[test]
    fn test_day5() {
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let input = IO::from(vec![7]);
        let (_, output) = run_with_input(&program, input);
        assert_eq!(output, IO::from(vec![1]));
    }
}

fn amplify(program: &Memory, sequence: Vec<i64>) -> i64 {
    // println!("- {:?}", sequence);
    let mut input = IO::from(vec![0]);
    for phase in sequence {
        // NB: prepend
        input.push_front(phase);
        // println!("{:?}", input);
        let mut vm = VM::new(&program, input);
        vm.run();
        input = vm.output;
    }
    // println!("  --> {:?}", input);
    input.pop_front().expect("at least one output")
}

fn amplify_loop(program: &Memory, sequence: Vec<i64>) -> i64 {
    let mut vms: Vec<VM> = sequence
        .iter()
        .map(|phase| {
            let input = IO::from(vec![*phase]);
            VM::new(&program, input)
        })
        .collect();
    vms[0].input.push_back(0);
    let n = vms.len();
    loop {
        for i in 0..n {
            let (result, mut output): (State, IO) = {
                let vm = &mut vms[i];
                let r = vm.run();
                let o = vm.output.drain(..).collect();
                (r, o)
            };
            if let State::Halted = result {
                if i == n - 1 {
                    return output
                        .pop_front()
                        .expect("at least one output when final machine halts");
                }
            }
            let next = &mut vms[(i + 1) % n];
            next.input.append(&mut output);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let line = handle
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("");
    let program: Memory = line.split(",").map(|s| s.parse().unwrap()).collect();

    let part1 = (0..5)
        .permutations(5)
        .map(|sequence| amplify(&program, sequence))
        .max()
        .expect("at least one element");
    println!("{}", part1);

    let part2 = (5..10)
        .permutations(5)
        .map(|sequence| amplify_loop(&program, sequence))
        .max()
        .expect("at least one element");
    println!("{}", part2);
}
