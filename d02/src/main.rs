use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(path: &str) -> Vec<String> {
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    let mut xs = Vec::new();
    for line in f.lines() {
        let s = line.unwrap().trim().to_owned();
        xs.push(s);
    }
    xs
}

fn exactly_k(s: &str, k: usize) -> bool {
    let mut m = HashMap::new();
    for c in s.chars() {
        if !m.contains_key(&c) {
            m.insert(c,0usize);
        }
        m.insert(c,
                 m.get(&c).unwrap() + 1);
    }
    m.iter().any( |(_c, i)| *i == k)
}

fn d2_p1(xs: &Vec<String>) -> usize {
    let exactly_two   = |s: &str| exactly_k(s, 2);
    let exactly_three = |s: &str| exactly_k(s, 3);
    let twos   = xs.iter().filter(|x| exactly_two(x)).count();
    let threes = xs.iter().filter(|x| exactly_three(x)).count();
    twos * threes
}

fn ndiff(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn intersect(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _c2)| c1 )
        .collect()
}

fn d2_p2(xs: &Vec<String>) -> String {
    for s1 in xs.iter() {
        for s2 in xs.iter() {
            if ndiff(s1, s2) == 1 {
                return intersect(s1, s2);
            }
        }
    }
    panic!("AHHHH")
}

fn main() {
    let lines = parse_input("/Users/dmb/rust_aoc/inputs/d2.txt");
    println!("{}", d2_p1(&lines));
    println!("{}", d2_p2(&lines))
}
