use std::io::{self, BufRead};

fn spinlock(len: usize, step: usize) -> usize {
    let mut position: usize = 0;
    let mut size: usize = 0;
    let mut ret: usize = 0;
    for i in 1..len {
        size += 1;
        position = (position + step) % size;
        if position == 0 {
            ret = i;
        }
        position += 1;
    }
    ret
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let val: usize = line.parse().unwrap();
        let spinlock = spinlock(50000000, val);
        println!("Output: {}", spinlock);
    }
}

// Tests
#[test]
fn test_spinlock_small() {
    assert_eq!(9, spinlock(10, 3));
}

#[test]
fn test_spinlock_large() {
    assert_eq!(1226, spinlock(2018, 3));
}

//#[test]
//fn test_spinlock_super() {
//    assert_eq!(1226, spinlock(50000000, 3));
//}
