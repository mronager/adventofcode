use std::cmp;
use std::io::{self, BufRead};

fn checksum_row(row: &Vec<u32>) -> u32 {
    let (min, max) = row.iter().fold(
        (std::u32::MAX, std::u32::MIN),
        |(min, max): (u32, u32),
         &x| {
            (cmp::min(min, x), cmp::max(max, x))
        },
    );
    max - min
}

fn checksum(grid: Vec<Vec<u32>>) -> u32 {
    grid.iter()
        .map(|row| checksum_row(row))
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let vec: Vec<&str> = line.split_whitespace().collect();
        let vec: Vec<u32> = vec.iter()
                               .map(|&x| x.parse().unwrap())
                               .collect();
        grid.push(vec);
    }
    println!("Grid: {:?}", grid);
    println!("Answer: {}", checksum(grid))
}

// Tests
#[test]
fn checksum_row_1() {
    assert_eq!(8, checksum_row(&vec![5, 1, 9, 5]))
}

#[test]
fn checksum_row_2() {
    assert_eq!(4, checksum_row(&vec![7, 5, 3]))
}

#[test]
fn checksum_row_3() {
    assert_eq!(6, checksum_row(&vec![2, 4, 6, 8]))
}

#[test]
fn checksum_row_4() {
    assert_eq!(0, checksum_row(&vec![5]))
}

#[test]
fn checksum_1() {
    let sheet = vec![
        vec![5, 1, 9, 5],
        vec![7, 5, 3],
        vec![2, 4, 6, 8],
    ];
    assert_eq!(18, checksum(sheet))
}
