use std::io::{self, BufRead};

fn captcha(n: &Vec<u32>) -> u32 {
    let last = n.last().unwrap();
    let (sum, _) = n.iter().fold(
        (0, last),
        |(sum, prev): (u32, &u32), curr| if prev == curr {
            return (sum + curr, curr);
        } else {
            return (sum, curr);
        },
    );
    sum
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

// Test
#[test]
fn puzzle_answer_1() {
    assert_eq!(3, captcha(&vec![1, 1, 2, 2]))
}

#[test]
fn puzzle_answer_2() {
    assert_eq!(4, captcha(&vec![1, 1, 1, 1]))
}

#[test]
fn puzzle_answer_3() {
    assert_eq!(0, captcha(&vec![1, 2, 3, 4]))
}

#[test]
fn puzzle_answer_4() {
    assert_eq!(9, captcha(&vec![9, 1, 2, 1, 2, 1, 2, 9]))
}
