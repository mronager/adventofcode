mod knot;
use std::io::{self, BufRead};

fn cleanup_region(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {

}

fn compute_grid(key: &str) -> usize {
    let size = 1;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut squares: usize = 0;
    for i in 0..size {
        let mut k: String = key.clone().to_string();
        k.push('-');
        k.push_str(&i.to_string());
        let lengths = knot::parse_line(&k);
        let knot = knot::knot(256, lengths);
        let bin: String = knot.into_iter().map(|x| format!("{:b}", x)).collect();
        grid.push(bin.chars().collect());
        println!("Grid: {:?}", grid[i]);
    }
    for i in 0..size {
        for j in 0..size {
            if grid[i][j] == '1' {

            }
        }
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
