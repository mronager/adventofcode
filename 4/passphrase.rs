use std::io::{self, BufRead};

fn valid_passphrase(pass: &Vec<&str>) -> bool {
    let mut new_pass = pass.clone();
    new_pass.sort();
    new_pass.dedup();
    new_pass.len() == pass.len()
}

fn main() {
    let stdin = io::stdin();
    let mut valid = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let passphrase: Vec<&str> = line.split_whitespace().collect();
        let is_valid = valid_passphrase(&passphrase);
        if (is_valid) {
            valid += 1;
        }
    }
    println!("Total valid: {}", valid);
}

// Tests
#[test]
fn valid_pass_1() {
    assert_eq!(true, valid_passphrase(&vec!["aa", "bb", "cc", "dd", "ee"]))
}

#[test]
fn valid_pass_2() {
    assert_eq!(false, valid_passphrase(&vec!["aa", "aa"]))
}

#[test]
fn valid_pass_3() {
    assert_eq!(false, valid_passphrase(&vec!["aa", "bb", "cc", "dd", "aa"]))
}

#[test]
fn valid_pass_4() {
    assert_eq!(true, valid_passphrase(&vec!["aa", "bb", "cc", "dd", "aaa"]))
}
