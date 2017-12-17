// cargo-deps: regex="0.2"
extern crate regex;
use std::io::{self, BufRead};
use regex::Regex;

fn remove_garbage(stream: &str) -> String {
    let re_excl = Regex::new(r"!.").unwrap();
    let re_garb = Regex::new(r"<.*?>").unwrap();
    let stream = re_excl.replace_all(&stream, ""); // !. = remove
    let stream = re_garb.replace_all(&stream, "").to_string(); // <.*> = remove
    stream
}

fn score(stream: &str) -> usize {
    let cleaned = remove_garbage(stream);
    let (score, _) = cleaned.chars().fold((0, 0), |(score, next), c| {
        match c {
          '{' => return (score + next + 1, next + 1),
          '}' => return (score, next - 1),
          _   => return (score, next)
        }
    });
    score
}


fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let score = score(&line);
        println!("Score: {}", score);
    }
}

// Tests
#[test]
fn test_garbage() {
    assert_eq!("", remove_garbage("<>"));
    assert_eq!("", remove_garbage("<random characters>"));
    assert_eq!("", remove_garbage("<<<<>"));
    assert_eq!("", remove_garbage("<{!>}>"));
    assert_eq!("", remove_garbage("<!!>"));
    assert_eq!("", remove_garbage("<!!!>>"));
    assert_eq!("", remove_garbage("<{o\"i!a,<{i<a>"));
}

#[test]
fn test_groups_garbage() {
    assert_eq!("{,,,}", remove_garbage("{<a>,<a>,<a>,<a>}"));
    assert_eq!("{{},{},{},{}}", remove_garbage("{{<a>},{<a>},{<a>},{<a>}}"));
    assert_eq!("{{}}", remove_garbage("{{<!>},{<!>},{<!>},{<a>}}"));
}

#[test]
fn test_score() {
    assert_eq!(16, score("{{{},{},{{}}}}"));
    assert_eq!(9, score("{{<a>},{<a>},{<a>},{<a>}}"));
    assert_eq!(3, score("{{<!>},{<!>},{<!>},{<a>}}"));
}
