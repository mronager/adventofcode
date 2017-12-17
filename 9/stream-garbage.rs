// cargo-deps: regex="0.2"
extern crate regex;
use std::io::{self, BufRead};
use regex::Regex;

fn count_garbage(stream: &str) -> usize {
    let re_excl = Regex::new(r"!.").unwrap();
    let re_garb = Regex::new(r"<(.*?)>").unwrap();
    let mut chars: usize = 0;
    let stream = re_excl.replace_all(&stream, ""); // !. = remove
    for cap in re_garb.captures_iter(&stream) {
        chars += &cap[1].len();
        println!("Captured: {}", &cap[1]);
    }
    chars
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let score = count_garbage(&line);
        println!("Score: {}", score);
    }
}

// Tests
#[test]
fn test_garbage() {
    assert_eq!(0, count_garbage("<>"));
    assert_eq!(17, count_garbage("<random characters>"));
    assert_eq!(3, count_garbage("<<<<>"));
    assert_eq!(2, count_garbage("<{!>}>"));
    assert_eq!(0, count_garbage("<!!>"));
    assert_eq!(0, count_garbage("<!!!>>"));
    assert_eq!(10, count_garbage("<{o\"i!a,<{i<a>"));
}

