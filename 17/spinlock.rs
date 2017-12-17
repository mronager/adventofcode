use std::io::{self, BufRead};

fn spinlock(len: usize, step: usize) -> usize {
    let mut state: Vec<usize> = Vec::new();
    let mut position: usize = 0;
    let mut nextval: usize = 1;
    // Initial
    state.push(0);
    for i in 1..len {
        let size = state.len();
        position = (position + step) % size;
        state.insert(position + 1, nextval);
        position += 1;
        nextval += 1;
    }
    state[position + 1]
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let val: usize = line.parse().unwrap();
        let spinlock = spinlock(2018, val);
        println!("Output: {}", spinlock);
    }
}

// Tests
#[test]
fn test_spinlock_small() {
    assert_eq!(5, spinlock(10, 3));
}

#[test]
fn test_spinlock_large() {
    assert_eq!(638, spinlock(2018, 3));
}
