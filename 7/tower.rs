// cargo-deps: regex="0.2"
extern crate regex;
use std::borrow::ToOwned;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

struct Tower {
    discs: HashMap<String, Option<String>>
}

impl Tower {
    fn add_disc(&mut self, name: String, discs: Vec<String>) {
        self.discs.entry(name.clone()).or_insert(None);
        for disc in discs {
            self.discs.insert(disc, Some(name.clone()));
        }
    }

    fn get_root(&mut self) -> Option<String> {
        for (disc, parent) in &self.discs {
            if parent.is_none() {
                return Some(disc.to_string())
            }
        }
        None
    }
}

fn parse_line(line: &str) -> (String, Vec<String>) {
    let re = Regex::new(r"(\w+) \(([0-9]+)\)( -> ([a-z, ]+))?").unwrap();
    let cap = re.captures(line).unwrap();
    let name: String = cap[1].to_owned();
    if cap.get(4).is_some() {
        let discs: String = cap[4].to_owned();
        let discs: Vec<String> = discs.split(", ").map(|x| x.to_owned()).collect();
        return (name, discs)
    } else {
        let discs: Vec<String> = Vec::new();
        return (name, discs)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut valid = 0;
    let mut tower = Tower { discs: HashMap::new() };
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let (name, discs) = parse_line(&line);
        tower.add_disc(name, discs);
        println!("Input: {}", line);
    }
    println!("Root: {:?}", tower.get_root());
}

// Tests
#[test]
fn parse_line_test() {
    let line = "havc (66)";
    let name: String = "havc".to_owned();
    let discs: Vec<String> = Vec::new();
    assert_eq!((name, discs), parse_line(line));
}

#[test]
fn parse_line_with_discs_test() {
    let line = "havc (66) -> ktlj, cntj, xhth";
    let name: String = "havc".to_owned();
    let discs: Vec<String> = vec!["ktlj", "cntj", "xhth"].into_iter().map(|x| x.to_owned()).collect();
    assert_eq!((name, discs), parse_line(line))
}

#[test]
fn step_1() {
    let mut tower = Tower { discs: HashMap::new() };
    let line = "havc (66) -> ktlj, cntj, xhth";
    let (name, discs) = parse_line(line);
    tower.add_disc(name, discs);
    assert_eq!(4, tower.discs.len());
    assert_eq!(Some("havc".to_string()), tower.get_root());
}



