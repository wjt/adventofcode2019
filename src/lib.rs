use std::collections::VecDeque;

pub type Memory = Vec<i64>;
pub type IO = VecDeque<i64>;

pub struct VM {
    memory: Memory,
    ip: usize,
    relative_base: i64,
    pub input: IO,
    pub output: IO,
}

pub enum State {
    Halted,
    NeedInput,
}

impl VM {
    pub fn new(program: &Memory) -> Self {
        let mut memory = program.clone();
        memory.resize(program.len() * 10, 0);
        Self {
            memory,
            ip: 0,
            relative_base: 0,
            input: IO::new(),
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

    fn step(&mut self) -> Option<State> {
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
                        return Some(State::NeedInput);
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
            99 => return Some(State::Halted),
            x => panic!("the discotheque: ip {}: {}", self.ip, x),
        }
        None
    }

    pub fn run(&mut self) -> State {
        loop {
            if let Some(x) = self.step() {
                return x;
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
        let mut vm = VM::new(&program);
        vm.run();

        vm.memory.resize(program.len(), 0);
        assert_eq!(vm.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    fn run_with_input(program: &Memory, mut input: IO) -> (Memory, IO) {
        let mut vm = VM::new(&program);
        vm.input.append(&mut input);
        vm.run();

        (vm.memory, vm.output)
    }

    #[test]
    fn test_day5() {
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let (_, output) = run_with_input(&program, IO::from(vec![7]));
        assert_eq!(output, IO::from(vec![1]));
    }

    #[test]
    fn test_day9_quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let (_, output) = run_with_input(&program, IO::new());
        assert_eq!(output, program);
    }

    #[test]
    fn test_day9_2() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let (_, output) = run_with_input(&program, IO::new());
        assert!(output[0] >= 10_i64.pow(15));
        assert!(output[0] < 10_i64.pow(16));
    }

    #[test]
    fn test_day9_large() {
        let program = vec![104, 1125899906842624, 99];
        let (_, output) = run_with_input(&program, IO::new());
        assert_eq!(output, vec![program[1]]);
    }
}

pub fn gcd(x: i64, y: i64) -> i64 {
    assert!(x >= 0);
    assert!(y >= 0);
    let (mut a, mut b) = if x > y { (x, y) } else { (y, x) };

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}

pub fn lcm(x: i64, y: i64) -> i64 {
    assert!(x >= 0);
    assert!(y >= 0);

    let g = gcd(x, y);
    (x / g) * y
}
