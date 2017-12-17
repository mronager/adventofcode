mod knot;
use std::io::{self, BufRead};

fn compute_grid(key: &str) -> usize {
    let mut squares: usize = 0;
    for i in 0..128 as u8 {
        let mut k: String = key.clone().to_string();
        k.push('-');
        k.push_str(&i.to_string());
        let lengths = knot::parse_line(&k);
        let knot = knot::knot(256, lengths);
        let bin: String = knot.into_iter().map(|x| format!("{:b}", x)).collect();
        squares += bin.chars().fold(0, |s, c| return match c {
          '1' => s + 1,
          _ => s
        });
    }
    squares
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let squares = compute_grid(&line);
        println!("Input: {}", line);
        println!("Squares: {}", squares);
    }
}

#[test]
fn test_squares_1() {
    let key = "flqrgnkx";
    assert_eq!(8108, compute_grid(key));
}
