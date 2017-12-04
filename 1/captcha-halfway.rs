use std::io::{self, BufRead};

fn captcha(n: &Vec<u32>) -> u32 {
    let (a, b) = n.split_at(n.len() / 2);
    let list: Vec<(&u32, &u32)> = a.iter().zip(b).collect();
    let sum: u32 = list.iter()
                       .filter(|&&(a, b)| a == b)
                       .map(|&(a, b)| a)
                       .sum();
    sum * 2
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let chars: Vec<u32> = line.chars()
                                  .map(|x| x.to_digit(10).unwrap())
                                  .collect();
        println!("Input: {:?}", chars);
        println!("Output: {}", captcha(&chars));
    }
}

// Tests
#[test]
fn puzzle_answer_1() {
    assert_eq!(6, captcha(&vec![1, 2, 1, 2]))
}

#[test]
fn puzzle_answer_2() {
    assert_eq!(0, captcha(&vec![1, 2, 2, 1]))
}

#[test]
fn puzzle_answer_3() {
    assert_eq!(4, captcha(&vec![1, 2, 3, 4, 2, 5]))
}

#[test]
fn puzzle_answer_4() {
    assert_eq!(12, captcha(&vec![1, 2, 3, 1, 2, 3]))
}

#[test]
fn puzzle_answer_5() {
    assert_eq!(4, captcha(&vec![1, 2, 1, 3, 1, 4, 1, 5]))
}
