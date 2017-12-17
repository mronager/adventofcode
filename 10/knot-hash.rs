use std::io::{self, BufRead};

fn dense_hash(list: Vec<usize>) -> usize {
    list.iter().fold(0, |i, x| i ^ x)
}

fn reverse_circular(list: &mut Vec<usize>, offset: usize, length: usize) {
    let mut newlist = Vec::new();
    let len = list.len();
    for i in 0..length {
        newlist.push(list[(offset + i) % len]);
    }
    newlist.reverse();
    for i in 0..length {
        list[(offset + i) % len] = newlist[i]
    }
}

pub fn knot(numbers: usize, mut lengths: Vec<usize>) -> Vec<usize> {
    let mut pos = 0;
    let mut skip_size = 0;
    let mut list: Vec<usize> = (0..numbers).collect();
    let magic: Vec<usize> = vec!(17, 31, 73, 47, 23);
    for i in magic {
        lengths.push(i);
    }
    for i in 0..64 {
        for len in &lengths {
            reverse_circular(&mut list, pos % numbers, *len);
            pos += len + skip_size;
            skip_size += 1;
        }
    }
    let mut hash: Vec<usize> = Vec::new();
    for i in 0..16 {
        let offset = i * 16;
        let slice: Vec<usize> = list[offset..offset+16].to_vec();
        hash.push(dense_hash(slice));
    }
    hash
}

fn make_hash(hash: Vec<usize>) -> String {
     let hash_hex: Vec<String> = hash.into_iter()
                                      .map(|x| format!("{:02x}", x))
                                      .collect();
      hash_hex.join("")
}

pub fn parse_line(line: &str) -> Vec<usize> {
    line.to_string().into_bytes().into_iter().map(|x| x as usize).collect()
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let lengths: Vec<usize> = parse_line(&line);
        let knot = knot(256, lengths);
        println!("Output: {}", make_hash(knot));
    }
}

#[test]
fn test_xor() {
    let inp: Vec<usize> = vec!(65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22);
    assert_eq!(64, dense_hash(inp));
}

#[test]
fn test_parse() {
    let exp: Vec<usize> = vec!(49, 44, 50, 44, 51);
    assert_eq!(exp, parse_line("1,2,3"));
}

#[test]
fn test_hash_1() {
    let lengths: Vec<usize> = parse_line("1,2,3");
    let hash = "3efbe78a8d82f29979031a4aa0b16a9d";
    let knot = knot(256, lengths);
    assert_eq!(hash, make_hash(knot));
}

#[test]
fn test_hash_2() {
    let lengths: Vec<usize> = parse_line("1,2,4");
    let hash = "63960835bcdc130f0b66d7ff4f6a5a8e";
    let knot = knot(256, lengths);
    assert_eq!(hash, make_hash(knot));
}
