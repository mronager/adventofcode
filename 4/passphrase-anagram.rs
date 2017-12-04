use std::io::{self, BufRead};

fn valid_passphrase(pass: &Vec<&str>) -> bool {
    let mut pass_list: Vec<String> = Vec::new();
    for s in pass {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        pass_list.push(chars.into_iter().collect());
    }
    pass_list.sort();
    pass_list.dedup();
    pass_list.len() == pass.len()
}

fn main() {
    let stdin = io::stdin();
    let mut valid = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let passphrase: Vec<&str> = line.split_whitespace().collect();
        let is_valid = valid_passphrase(&passphrase);
        if is_valid {
            valid += 1;
        }
    }
    println!("Total valid: {}", valid);
}

// Tests
#[test]
fn valid_pass_1() {
    assert_eq!(true, valid_passphrase(&vec!["abcde", "fghij"]))
}

#[test]
fn valid_pass_2() {
    assert_eq!(false, valid_passphrase(&vec!["abcde", "xyz", "ecdab"]))
}

#[test]
fn valid_pass_3() {
    assert_eq!(true, valid_passphrase(&vec!["a", "ab", "abc", "abd", "abf", "abj"]))
}

#[test]
fn valid_pass_4() {
    assert_eq!(true, valid_passphrase(&vec!["iiii", "oiii", "ooii", "oooi", "oooo"]))
}

#[test]
fn valid_pass_5() {
    assert_eq!(false, valid_passphrase(&vec!["oiii", "ioii", "iioi", "iiio"]))
}
