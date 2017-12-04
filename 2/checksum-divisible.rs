use std::io::{self, BufRead};

fn checksum_row(row: &Vec<u32>) -> u32 {
    for x in row {
        match row.iter().find(
            |&y| x != y && x % y == 0,
        ) {
            Some(&x2) => return x / x2,
            None => {}
        }
    }
    0
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
    assert_eq!(4, checksum_row(&vec![5, 9, 2, 8]))
}

#[test]
fn checksum_row_2() {
    assert_eq!(3, checksum_row(&vec![9, 4, 7, 3]))
}

#[test]
fn checksum_row_3() {
    assert_eq!(2, checksum_row(&vec![3, 8, 6, 5]))
}

#[test]
fn checksum_1() {
    let grid = vec![
        vec![5, 9, 2, 8],
        vec![9, 4, 7, 3],
        vec![3, 8, 6, 5],
    ];
    assert_eq!(9, checksum(grid))
}
