use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashSet;


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

fn day_1_part_1(path: &str) -> i32 {
    parse_input(path).iter().sum()
}

fn day_1_part_2(path: &str) -> i32 {
    let xs = parse_input(path);
    let mut seen = HashSet::new();
    let mut freq = 0;
    loop {
        for x in xs.iter() {
            freq += *x;
            if seen.contains(&freq) {
                return freq
            }
            seen.insert(freq);
        }
    }
}


fn main() {
    let path = "/Users/dmb/rust_aoc/inputs/d1.txt";
    println!("part1: {}", day_1_part_1(path));
    println!("part1: {}", day_1_part_2(path));
}
