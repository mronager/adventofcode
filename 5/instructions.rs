use std::io::{self, BufRead};

struct InstructionSet {
    instructions: Vec<i16>,
    pos: usize
}

impl InstructionSet {
    fn step(&mut self) -> usize {
        self.instructions[self.pos] += 1;
        self.pos = (self.pos as i16 + self.instructions[self.pos] - 1) as usize;
        self.pos
    }
}

fn steps(instructions: Vec<i16>) -> usize {
    let mut pos = 0;
    let mut steps = 0;
    let size = instructions.len();
    let mut set = InstructionSet {
        instructions: instructions,
        pos: pos
    };
    while pos < size {
        steps += 1;
        pos = set.step();
    }
    steps
}

fn main() {
    let stdin = io::stdin();
    let mut instructions: Vec<i16> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        instructions.push(line.parse().unwrap())
    }
    println!("Vec: {:?}", instructions);
    println!("Steps: {}", steps(instructions));
}

// Tests
#[test]
fn step_1() {
    let mut set = InstructionSet {
        instructions: vec![0, 3, 0, 1, -3],
        pos: 0
    };
    assert_eq!(0, set.step())
}

#[test]
fn step_2() {
    let mut set = InstructionSet {
        instructions: vec![1, 3, 0, 1, -3],
        pos: 0
    };
    assert_eq!(1, set.step())
}

#[test]
fn step_3() {
    let mut set = InstructionSet {
        instructions: vec![1, 3, 0, 1, -3],
        pos: 1
    };
    assert_eq!(4, set.step())
}

#[test]
fn step_4() {
    assert_eq!(5, steps(vec![0, 3, 0, 1, -3]))
}


