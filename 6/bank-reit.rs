use std::io::{self, BufRead};
use std::collections::HashMap;

fn redistribute(blocks: &mut Vec<u16>) {
    let mut max: u16 = 0;
    let mut idx: usize = 0;
    let len = blocks.len();
    for (i, &item) in blocks.iter().enumerate() {
        if item > max {
          max = item;
          idx = i;
        }
    }
    blocks[idx] = 0;
    for i in 0..max as usize {
        blocks[(idx + i + 1) % len] += 1
    }
}

fn redistribute_all(blocks: &mut Vec<u16>) -> usize {
    let mut map: HashMap<Vec<u16>, usize> = HashMap::new();
    let mut steps: usize = 0;
    loop {
        redistribute(blocks);
        let nb = blocks.clone();
        println!("Step: {:?}", nb);
        match map.insert(nb, steps) {
            None  => steps += 1,
            Some(x) => return steps - x
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut valid = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let vec: Vec<&str> = line.split_whitespace().collect();
        let mut vec: Vec<u16> = vec.iter()
                               .map(|&x| x.parse().unwrap())
                               .collect();
        println!("Input: {:?}", vec);
        println!("Output: {}", redistribute_all(&mut vec));
    }
}

// Tests
#[test]
fn step_1() {
    let mut blocks: Vec<u16> = vec![0, 2, 7, 0];
    redistribute(&mut blocks);
    assert_eq!(vec![2, 4, 1, 2], blocks)
}

#[test]
fn step_2() {
    let mut blocks: Vec<u16> = vec![2, 4, 1, 2];
    redistribute(&mut blocks);
    assert_eq!(vec![3, 1, 2, 3], blocks)
}

#[test]
fn step_3() {
    let mut blocks: Vec<u16> = vec![3, 1, 2, 3];
    redistribute(&mut blocks);
    assert_eq!(vec![0, 2, 3, 4], blocks)
}

#[test]
fn steps() {
    let mut blocks: Vec<u16> = vec![0, 2, 7, 0];
    assert_eq!(4, redistribute_all(&mut blocks))
}
