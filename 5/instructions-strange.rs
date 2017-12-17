use std::io::{self, BufRead};

struct InstructionSet {
    instructions: Vec<i16>,
    pos: usize
}

impl InstructionSet {
    fn step(&mut self) -> usize {
        let inc = if self.instructions[self.pos] >= 3 { -1 } else { 1 };
        self.instructions[self.pos] += inc;
        self.pos = (self.pos as i16 + self.instructions[self.pos] - inc) as usize;
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
    assert_eq!(10, steps(vec![0, 3, 0, 1, -3]))
}


