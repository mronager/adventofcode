use std::io::{self, BufRead};
use std::collections::HashMap;

fn generate(seed: usize, factor: usize) -> usize {
    let divisor = 2147483647;
    (seed * factor) % divisor
}

fn pairs(mut seed_a: usize, mut seed_b: usize, count: usize) -> usize {
    let mut pairs = 0;
    for _ in 0..count {
        seed_a = generate(seed_a, 16807);
        seed_b = generate(seed_b, 48271);
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
        let pairs = pairs(seeds[0], seeds[1], 40000000);
        println!("Seeds: {:?}, pairs: {}", seeds, pairs);
    }
}

// Tests
#[test]
fn test_generator_a() {
    let val = generate(65, 16807);
    assert_eq!(1092455, val);
}

#[test]
fn test_generator_b() {
    let val = generate(8921, 48271);
    assert_eq!(430625591, val);
}

#[test]
fn test_pairs() {
    let pairs = pairs(65, 8921, 5);
    assert_eq!(1, pairs);
}

#[test]
fn test_pairs_2() {
    let pairs = pairs(65, 8921, 40000000);
    assert_eq!(588, pairs);
}
