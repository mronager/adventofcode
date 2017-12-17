use std::io::{self, BufRead};

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

fn knot(numbers: usize, lengths: Vec<usize>) -> usize {
    let mut pos = 0;
    let mut skip_size = 0;
    let mut list: Vec<usize> = (0..numbers).collect();
    println!("List: {:?}", list);
    for len in lengths {
        reverse_circular(&mut list, pos % numbers, len);
        pos += len + skip_size;
        skip_size += 1;
        println!("List: {:?}", list);
    }
    &list[0] * &list[1]
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let lengths: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
        println!("Lengths: {:?}", lengths);
        println!("Output: {}", knot(256, lengths));
    }
}

// Tests
#[test]
fn test_1() {
    let mut inp: Vec<usize> = vec!(2, 1, 0, 3, 4);
    let exp: Vec<usize> = vec!(4, 3, 0, 1, 2);
    reverse_circular(&mut inp, 3, 4);
    assert_eq!(exp, inp);
}

#[test]
fn test_knot() {
    let lengths: Vec<usize> = vec!(3, 4, 1, 5);
    assert_eq!(12, knot(5, lengths));
}
