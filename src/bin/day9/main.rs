use std::collections::VecDeque;
use std::io::{self, BufRead};

type Memory = Vec<i64>;
type IO = VecDeque<i64>;

struct VM {
    memory: Memory,
    ip: usize,
    relative_base: i64,
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
        let mut memory = program.clone();
        memory.resize(program.len() * 10, 0);
        Self {
            memory,
            ip: 0,
            relative_base: 0,
            input,
            output: IO::new(),
        }
    }

    fn arg(&mut self, i: u32) -> &mut i64 {
        let instruction = self.memory[self.ip];
        let mode = (instruction / (10 * (10_i64.pow(i)))) % 10;
        let j = self.ip + i as usize;
        match mode {
            0 => {
                let val = self.memory[j];
                &mut self.memory[val as usize]
            }
            1 => &mut self.memory[j],
            2 => {
                let address: i64 = self.memory[j] + self.relative_base;
                &mut self.memory[address as usize]
            }
            _ => panic!("{}", mode),
        }
    }

    fn int3<F>(&mut self, f: F)
    where
        F: Fn(i64, i64) -> i64,
    {
        let x = *self.arg(1);
        let y = *self.arg(2);
        *self.arg(3) = f(x, y);
        self.ip += 4
    }

    fn jump_if<F>(&mut self, f: F)
    where
        F: Fn(i64) -> bool,
    {
        if f(*self.arg(1)) {
            self.ip = *self.arg(2) as usize
        } else {
            self.ip += 3;
        }
    }

    fn step(&mut self) -> State {
        let instruction = self.memory[self.ip];
        let masked = instruction % 100;
        match masked {
            1 => self.int3(|a, b| a + b),
            2 => self.int3(|a, b| a * b),
            3 => {
                let val_ = self.input.pop_front();
                match val_ {
                    Some(val) => {
                        *self.arg(1) = val;
                        self.ip += 2;
                    }
                    None => {
                        return State::NeedInput;
                    }
                }
            }
            4 => {
                let val = *self.arg(1);
                self.output.push_back(val);
                self.ip += 2
            }
            5 => self.jump_if(|x| x != 0),
            6 => self.jump_if(|x| x == 0),
            7 => self.int3(|a, b| (a < b) as i64),
            8 => self.int3(|a, b| (a == b) as i64),
            9 => {
                self.relative_base += *self.arg(1);
                self.ip += 2;
            }
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

        vm.memory.resize(program.len(), 0);
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

    #[test]
    fn test_day9_quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let input = IO::new();
        let (_, output) = run_with_input(&program, input);
        assert_eq!(output, program);
    }

    #[test]
    fn test_day9_2() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let input = IO::new();
        let (_, output) = run_with_input(&program, input);
        assert!(output[0] >= 10_i64.pow(15));
        assert!(output[0] < 10_i64.pow(16));
    }

    #[test]
    fn test_day9_large() {
        let program = vec![104, 1125899906842624, 99];
        let input = IO::new();
        let (_, output) = run_with_input(&program, input);
        assert_eq!(output, vec![program[1]]);
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
    for i in 1..3 {
        let input = IO::from(vec![i]);
        let mut vm = VM::new(&program, input);
        vm.run();
        println!("part {}: {:?}", i, vm.output);
    }
}
