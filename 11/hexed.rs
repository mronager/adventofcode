use std::io::{self, BufRead};

fn dist(path: &str) -> isize {
    let vec: Vec<&str> = path.split(",").collect();
    let (x,y,z): (isize, isize, isize) = vec.into_iter().fold((0, 0, 0), |(x,y,z),p| {
      let (x2, y2, z2) = match p {
        "nw" => (-1, 1, 0),
        "n"  => (0, 1, -1),
        "ne" => (1, 0, -1),
        "se" => (1, -1, 0),
        "s"  => (0, -1, 1),
        "sw" => (-1, 0, 1),
        _    => (0, 0, 0)
      };
      (x+x2, y+y2, z+z2)
    });
    println!("x: {}, y: {}, z: {}", x, y, z);
    (x.abs() + y.abs() + z.abs()) / 2
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("Distance: {}", dist(&line));
    }
}

// Tests
#[test]
fn test_1() {
    assert_eq!(3, dist("ne,ne,ne"));
}

#[test]
fn test_2() {
    assert_eq!(0, dist("ne,ne,sw,sw"));
}

#[test]
fn test_3() {
    assert_eq!(2, dist("ne,ne,s,s"));
}

#[test]
fn test_4() {
    assert_eq!(3, dist("se,sw,se,sw,sw"));
}
