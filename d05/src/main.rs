fn reacts_with(c1: char, c2: char) -> bool {
    // why tf does to_lowercase return an iterator? (b/c unicode)
//    c1.to_lowercase().next().unwrap() == c2.to_lowercase().next().unwrap() && c1 != c2
    c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2
}

fn d5_p1(s: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match stack.last() {
            None => stack.push(c),
            Some(c1) => {
                if reacts_with(c1.clone(), c) {
                    stack.pop();
                } else {
                    stack.push(c)
                }

            },
        }
    }
    stack.len()
}

fn without_char(s: &str, c: char) -> String {
    let c = c.to_ascii_lowercase();

    s.chars().into_iter()
        .filter(|x| x.to_ascii_lowercase() != c)
        .collect()
}

fn main() {
    let s = std::fs::read_to_string("../inputs/d5.txt").unwrap();
    let s = s.trim();
    let n = d5_p1(s);
    println!("{}", n);
    let s2 = "dabAcCaCBAcCcaDA";
    let s3 = without_char(s2, 'a');
    println!("{}", s3);
}
