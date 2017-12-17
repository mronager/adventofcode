// cargo-deps: regex="0.2"
extern crate regex;
use std::borrow::ToOwned;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp;
use regex::Regex;

struct Register {
    registers: HashMap<String, isize>
}

impl Register {
    fn instruct(&mut self, instruction: &str) {
        //b inc 5 if a > 1
        let re = Regex::new(r"(\w+) (inc|dec) ([0-9-]+) if (\w+) ([!=><]+) ([0-9-]+)").unwrap();
        let cap = re.captures(instruction).unwrap();
        let register_a: &str = &cap[1];
        let register_b: &str = &cap[4];
        let val_a = *self.registers.entry(register_a.to_owned()).or_insert(0);
        let val_b = *self.registers.entry(register_b.to_owned()).or_insert(0);
        let val: isize = cap[3].parse().unwrap();
        let multiplier = match &cap[2] {
            "inc" => 1,
            "dec" => -1,
            _     => {
              println!("Unknown multiplier");
              0
            }
        };
        let check: isize = cap[6].parse().unwrap();
        let multiply = match &cap[5] {
            "==" => val_b == check,
            ">" => val_b > check,
            "<" => val_b < check,
            ">=" => val_b >= check,
            "!=" => val_b != check,
            "<=" => val_b <= check,
            _    => {
                println!("Unknown multiply");
                false
            }
        };
        if multiply {
            self.registers.insert(register_a.to_owned(), val_a + (val * multiplier));
        }
    }

    fn get_largest(&mut self) -> isize {
        self.registers.iter().fold(0, |m, (_, &i)| cmp::max(i, m))
    }
}

fn main() {
    let stdin = io::stdin();
    let mut registers = Register { registers: HashMap::new() };
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        registers.instruct(&line);
    }
    println!("Max value: {}", registers.get_largest());
}

// Tests
#[test]
fn test_1() {
    let mut register = Register { registers: HashMap::new() };
    let line = "b inc 5 if a > 1";
    register.instruct(line);
    assert_eq!(&0isize, register.registers.get("a").unwrap());
    assert_eq!(&0isize, register.registers.get("b").unwrap());
}

#[test]
fn test_2() {
    let mut register = Register { registers: HashMap::new() };
    register.instruct("b inc 5 if a > 1");
    register.instruct("a inc 1 if b < 5");
    register.instruct("c dec -10 if a >= 1");
    register.instruct("c inc -20 if c == 10");
    assert_eq!(&1isize, register.registers.get("a").unwrap());
    assert_eq!(&0isize, register.registers.get("b").unwrap());
    assert_eq!(&-10isize, register.registers.get("c").unwrap());
}

#[test]
fn test_3() {
    let mut register = Register { registers: HashMap::new() };
    register.instruct("b inc 5 if a > 1");
    register.instruct("a inc 1 if b < 5");
    register.instruct("c dec -10 if a >= 1");
    register.instruct("c inc -20 if c == 10");
    assert_eq!(1isize, register.get_largest());
}
