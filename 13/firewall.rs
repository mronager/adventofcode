use std::io::{self, BufRead};
use std::collections::HashMap;

struct Firewall {
    rules: HashMap<usize, usize>
}

impl Firewall {
    fn add_layer(&mut self, idx: usize, size: usize) {
        self.rules.insert(idx, size);
    }

    fn severity(&mut self, size: usize) -> usize {
        let mut severity: usize = 0;
        for i in 0..size+1 {
            let size = &self.rules.get(&i);
            if size.is_some() {
                let s = size.unwrap();
                let roundtrip = s + (s - 2);
                println!("i: {}, roundtrip: {}", i, roundtrip);
                if (i % roundtrip == 0) {
                    severity += i * s;
                }
            }
        }
        severity
    }
}

fn parse_input(input: &str) -> (usize, usize) {
    let elements: Vec<&str> = input.split(": ").collect();
    let elements: Vec<usize> =  elements.iter()
                                  .map(|&x| x.parse().unwrap())
                                  .collect();
    (elements[0], elements[1])
}

fn main() {
    let stdin = io::stdin();
    let mut firewall = Firewall { rules: HashMap::new() };
    let mut max = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let (idx, size) = parse_input(&line);
        firewall.add_layer(idx, size);
        max = idx;
    }
    println!("Output: {}", firewall.severity(max));
}

// Tests
#[test]
fn test_severity() {
    let mut firewall = Firewall { rules: HashMap::new() };
    firewall.add_layer(0, 3);
    firewall.add_layer(1, 2);
    firewall.add_layer(4, 4);
    firewall.add_layer(6, 4);
    assert_eq!(24, firewall.severity(6));
}

#[test]
fn test_parse() {
    assert_eq!((0, 3), parse_input("0: 3"));
}
