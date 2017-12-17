// cargo-deps: regex="0.2", lazy_static="1.0.0"
#[macro_use] extern crate lazy_static;
extern crate regex;
use std::io::{self, BufRead};
use std::mem;
use regex::Regex;

fn spin(programs: &mut Vec<char>, x: usize) {
    let pg = programs.clone();
    let (left, right) = pg.split_at(programs.len() - x);
    let concat: Vec<char> = [right, left].concat();
    mem::replace(programs, concat);
}

fn exchange(programs: &mut Vec<char>, a: usize, b: usize) {
    let val_a = programs[a];
    let val_b = programs[b];
    programs[a] = val_b;
    programs[b] = val_a;
}

fn partner(programs: &mut Vec<char>, a: char, b: char) {
    for i in 0..programs.len() {
        if programs[i] == a { 
            programs[i] = b;
        } else if programs[i] == b {
            programs[i] = a;
        }
    }
}

fn perform(moves: &str, programs: &mut Vec<char>, dances: usize) {
    lazy_static! {
        static ref MOVES_RE: Regex = Regex::new(r"(\w)(\w+)/?(\w+)?").unwrap();
    }
    let orig_programs = programs.clone();
    let moves: Vec<&str> = moves.split(',').collect();
    let mut states: Vec<Vec<char>> = Vec::new();
    let mut cycles = 0;
    states.push(programs.clone());
    loop {
        cycles += 1;
        for m in &moves {
            let cap = MOVES_RE.captures(m).unwrap();
            match cap[1].parse::<char>().unwrap() {
                's' => spin(programs, cap[2].parse::<usize>().unwrap()),
                'x' => exchange(programs, cap[2].parse::<usize>().unwrap(), cap[3].parse::<usize>().unwrap()),
                'p' => partner(programs, cap[2].parse::<char>().unwrap(), cap[3].parse::<char>().unwrap()),
                _ => {
                  println!("UNKNOWN");
                }
            }
        }
        if programs == &orig_programs {
            println!("Cycle detected: {}", cycles);
            break;
        }
        states.push(programs.clone());
    }
    println!("States: {:?}, cycles: {}, dances: {}", states, cycles, dances);
    let state = states[dances % cycles].clone();
    mem::replace(programs, state);
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap(); //abcdefghijklmnop
        let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();
        perform(&line, &mut programs, 1000000000);
        let result: String = programs.into_iter().collect();
        println!("Output: {}", result);
    }
}

// Tests
#[test]
fn test_spin() {
    let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    let exp = vec!['c', 'd', 'e', 'a', 'b'];
    spin(&mut programs, 3);
    assert_eq!(exp, programs);
}

#[test]
fn test_exchange() {
    let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    let exp = vec!['a', 'b', 'c', 'e', 'd'];
    exchange(&mut programs, 3, 4);
    assert_eq!(exp, programs);
}

#[test]
fn test_partner() {
    let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    let exp = vec!['a', 'e', 'c', 'd', 'b'];
    partner(&mut programs, 'e', 'b');
    assert_eq!(exp, programs);
}

#[test]
fn test_perform_small_1() {
    let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    let exp = vec!['b', 'a', 'e', 'd', 'c'];
    perform("s1,x3/4,pe/b", &mut programs, 1);
    assert_eq!(exp, programs);
}

#[test]
fn test_perform_small_2() {
    let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    let exp = vec!['c', 'e', 'a', 'd', 'b'];
    perform("s1,x3/4,pe/b", &mut programs, 2);
    assert_eq!(exp, programs);
}

#[test]
fn test_perform_small_5() {
    let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    let exp = vec!['c', 'e', 'a', 'd', 'b'];
    perform("s1,x3/4,pe/b", &mut programs, 6);
    assert_eq!(exp, programs);
}

