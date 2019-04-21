use std::fs::File;

use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use core::borrow::Borrow;

#[derive(Debug)]
struct Claim {
    id: String,
    col: u32,
    row: u32,
    w: u32,
    h: u32,
}

fn parse_col_row(s: &str) -> (u32, u32) {
    let toks: Vec<&str> = s.split(',').collect();
    (toks[0].parse().unwrap(), toks[1].trim_end_matches(':').parse().unwrap())
}

fn parse_width_height(s: &str) -> (u32, u32) {
    let toks: Vec<&str> = s.split('x').collect();
    (toks[0].parse().unwrap(), toks[1].parse().unwrap())
}

impl Claim {
    fn parse(s: &str) -> Claim {
        let xs: Vec<&str> = s.split_whitespace().collect();
        let id = xs[0].to_owned();
        let (col,row) = parse_col_row(xs[2]);
        let (w,h) = parse_width_height(xs[3]);
        Claim {id, col, row, w, h}
    }

    fn contains(&self, col: &u32, row: &u32) -> bool {
        (self.col <= *col && *col <= self.col + self.w) &&
        (self.row <= *row && *row <= self.row + self.h)
    }
}


fn parse_input(path: &str) -> Vec<Claim> {
    let f = File::open(path).unwrap();
    BufReader::new(f)
        .lines().into_iter()
        .map(|line| Claim::parse(&line.unwrap()))
        .collect()
}

fn place_claim(c: &Claim, counts: &mut HashMap<(u32,u32), usize>) {
    for col in c.col..(c.col + c.w) {
        for row in c.row..(c.row + c.h) {
            if !counts.contains_key(&(col, row)) {
                counts.insert((col, row), 0);
            }
            *counts.get_mut(&(col, row)).unwrap() += 1;
        }
    }
}

fn d3_p1(claims: &Vec<Claim>) -> usize {
    let mut counts: HashMap<(u32,u32), usize> = HashMap::new();
    for c in claims.iter() {
        place_claim(c, &mut counts)
    }
    counts.values().into_iter().filter(|x| **x >= 2usize).count()
}

fn d3_p2(claims: &Vec<Claim>) -> &str {
    let mut counts: HashMap<(u32,u32), usize> = HashMap::new();
    for c in claims.iter() {
        place_claim(c, &mut counts)
    }

    let conflicts: Vec<&(u32,u32)> = counts.iter()
        .filter(|(_, n)| **n >= 2usize)
        .map(|(row_col,_)| row_col)
        .collect();

    let conflict_free = claims.iter()
        .find(|claim|
            !conflicts.iter().any(|(r,c)| claim.contains(r,c))
        )
        .unwrap();

    conflict_free.id.borrow()
}

fn main() {
    let claims = parse_input("input.txt");
    println!("{}", d3_p1(&claims));
    println!("{}", d3_p2(&claims));
}
