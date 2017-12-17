// cargo-deps: regex="0.2"
extern crate regex;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

struct Plumber {
    tree: HashMap<usize, Vec<usize>>
}

impl Plumber {
    fn add_node(&mut self, root: usize, children: Vec<usize>) {
        self.tree.insert(root, children);
    }

    /// Breadth-first search
    fn count_groups(&mut self) -> usize {
        let mut seen: HashSet<usize> = HashSet::new();
        let mut groups: usize = 0;
        for (node, chldrn) in &self.tree {
            if !seen.contains(&node) {
                let mut nodes: Vec<usize> = chldrn.clone();
                println!("Seen: {:?}, node: {}", seen, node);
                groups += 1;
                loop {
                    let mut bfs_nodes: Vec<usize> = Vec::new();
                    for n in &nodes {
                        if seen.insert(*n) {
                            let children = self.tree.get(n).unwrap();
                            for c in children {
                                bfs_nodes.push(*c);
                            }
                        }
                    }
                    if bfs_nodes.len() == 0 {
                        break
                    } else {
                        nodes = bfs_nodes;
                    }
                }
            }
        }
        groups
    }
}

fn parse_input(input: &str) -> (usize, Vec<usize>) {
    let re = Regex::new(r"(\d+) <-> ([0-9, ]+)").unwrap();
    let cap = re.captures(input).unwrap();
    let children: Vec<&str> = cap[2].split(", ").collect();
    let vec: Vec<usize> = children.iter()
                            .map(|&x| x.parse().unwrap())
                            .collect();
    (cap[1].parse().unwrap(), vec)
}

fn main() {
    let stdin = io::stdin();
    let mut plmb = Plumber { tree: HashMap::new() };
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let (node, children) = parse_input(&line);
        plmb.add_node(node, children);
    }
    println!("Output: {}", plmb.count_groups());
}

// Tests
#[test]
fn test_count() {
    let mut plmb = Plumber { tree: HashMap::new() };
    &plmb.add_node(0, vec!(2));
    &plmb.add_node(1, vec!(1));
    &plmb.add_node(2, vec!(0, 3, 4));
    &plmb.add_node(3, vec!(2, 4));
    &plmb.add_node(4, vec!(2, 3, 6));
    &plmb.add_node(5, vec!(6));
    &plmb.add_node(6, vec!(4, 5));
    assert_eq!(&2usize, &plmb.count_groups());
}

#[test]
fn test_input_1() {
    let input = "0 <-> 2";
    assert_eq!((0, vec!(2)), parse_input(input));
}

#[test]
fn test_input_2() {
    let input = "2 <-> 0, 3, 4";
    assert_eq!((2, vec!(0, 3, 4)), parse_input(input));
}
