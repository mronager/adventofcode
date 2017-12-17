use std::io::{self, BufRead};
use std::collections::HashMap;

fn generate(seed: usize, factor: usize, multiple: usize) -> usize {
    let divisor = 2147483647;
    let mut prev = seed;
    loop {
        prev = (prev * factor) % divisor;
        if (prev % multiple) == 0 {
            return prev;
        }
    }
    
}

fn pairs(mut seed_a: usize, mut seed_b: usize, count: usize) -> usize {
    let mut pairs = 0;
    for _ in 0..count {
        seed_a = generate(seed_a, 16807, 4);
        seed_b = generate(seed_b, 48271, 8);
        if (seed_a as u16 == seed_b as u16) {
            pairs += 1;
        }
    }
    pairs
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let seeds: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
        let pairs = pairs(seeds[0], seeds[1], 5000000);
        println!("Seeds: {:?}, pairs: {}", seeds, pairs);
    }
}

// Tests
#[test]
fn test_generator_a() {
    let val = generate(65, 16807, 4);
    assert_eq!(1352636452, val);
}

#[test]
fn test_generator_b() {
    let val = generate(8921, 48271, 8);
    assert_eq!(1233683848, val);
}

#[test]
fn test_pairs() {
    let pairs = pairs(65, 8921, 5);
    assert_eq!(0, pairs);
}

#[test]
fn test_pairs_2() {
    let pairs = pairs(65, 8921, 5000000);
    assert_eq!(309, pairs);
}
