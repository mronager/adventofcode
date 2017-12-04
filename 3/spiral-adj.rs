use std::io::{self, BufRead};
use std::collections::HashMap;

fn spiral(n: u32) -> u32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir: u32 = 3; // start by moving right
    let mut grid = HashMap::new();
    let mut maxval: u32 = 0;
    grid.insert((x,y), 1); // Block 0 is always 1

    while maxval <= n {
        if x.abs() == y.abs() {
            dir += 1;
        }
        match dir {
            0 => y -= 1, // up
            1 => x -= 1, // left
            2 => y += 1, // down
            3 => x += 1, // right
            4 => {       // right up
                x += 1;
                dir = 0
            },
            _ => {}
        }
        let mut val = 0;
        for x1 in -1..2 {
            for y1 in -1..2 {
                match grid.get(&(x + x1, y + y1)) {
                    Some(val1) => {
                      val += val1
                    },
                    None => {}
                }
            }
        }
        grid.insert((x,y), val);
        maxval = val;
    }
    maxval
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("Input: {}", line);
        println!("Output: {}", spiral(line.parse().unwrap()))
    }
}

// Tests
#[test]
fn spiral_1() {
    assert_eq!(2, spiral(1))
}
#[test]
fn spiral_2() {
    assert_eq!(4, spiral(2))
}

#[test]
fn spiral_3() {
    assert_eq!(5, spiral(4))
}

#[test]
fn spiral_4() {
    assert_eq!(10, spiral(5))
}
