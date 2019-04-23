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

fn alphabet() -> Vec<char> {
    //  https://stackoverflow.com/questions/45343345/is-there-a-simple-way-to-generate-the-lowercase-and-uppercase-english-alphabet-i/45344161
    let letters = (b'a'..=b'z')             // Start as u8
        .map(|c| c as char)                     // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect::<Vec<_>>();
    letters
}

fn without_char(s: &str, c: char) -> String {
    let c = c.to_ascii_lowercase();

    s.chars().into_iter()
        .filter(|x| x.to_ascii_lowercase() != c)
        .collect()
}

fn d5_p2(s: &str) -> usize {
    // is there a "max int" type thing?
    let mut maybe_n: Option<usize> = None;
    for c in alphabet().iter() {
        let len_without_c = d5_p1(&without_char(s, c.clone()));
        println!("Len without {}: {}", c, len_without_c);
        maybe_n = match maybe_n {
            None => Some(len_without_c),
            Some(n) => Some(n.min(len_without_c)),
        }
    }
    maybe_n.unwrap()
}

fn main() {
    let s = std::fs::read_to_string("../inputs/d5.txt").unwrap();
    let s = s.trim();
    let n1 = d5_p1(s);
    println!("{}", n1);
    let n2 = d5_p2(s);
    println!("{}", n2);
}
