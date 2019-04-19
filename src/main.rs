use std::fs::File;
use std::io::{self, BufReader, BufRead};


fn parse_input(path: &str) -> Vec<i32> {
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    let mut xs: Vec<i32> = Vec::new();

    for line in f.lines() {
        let x = line.unwrap();
        xs.push(x.parse().unwrap());
    }
    xs
}

fn day_1_part_1() {
    let xs: Vec<i32> = parse_input("/Users/dmb/rust_aoc/inputs/d1.txt");
    let sum: i32 = xs.iter().sum();
    println!("sum of xs: {}", sum);
}


fn main() {
    day_1_part_1()
}
