use std::io::{self, BufRead};

fn spiral(n: u32) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir: u32 = 3; // start by moving right

    for i in 1..n {
        if x.abs() == y.abs() {
            dir += 1;
        }
        match dir {
            0 => y -= 1, // up
            1 => x -= 1, // left
            2 => y += 1, // down
            3 => x += 1, // right
            4 => { // right up
              x += 1;
              dir = 0
            },
            _ => {}
        }
    }
    println!("X: {}, Y: {}", x, y);
    x.abs() + y.abs()
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("Input: {}", line);
        println!("Distance: {}", spiral(line.parse().unwrap()))
    }
}

// Tests
#[test]
fn spiral_1() {
    assert_eq!(0, spiral(1))
}

#[test]
fn spiral_2() {
    assert_eq!(3, spiral(12))
}

#[test]
fn spiral_3() {
    assert_eq!(2, spiral(23))
}

#[test]
fn spiral_4() {
    assert_eq!(31, spiral(1024))
}
