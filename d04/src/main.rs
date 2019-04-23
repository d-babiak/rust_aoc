use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use chrono::prelude::*;
use std::convert::TryInto;

fn parse_input(path: &str) -> Vec<String> {
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    let xs: Vec<String> = f.lines()
        .map(|s| s.unwrap().trim().to_owned())
        .collect();
    xs
}

#[derive(PartialEq)]
#[derive(Debug)]
enum EventType {
    Sleep,
    Awake,
}

#[derive(Debug)]
struct Event {
    event_type: EventType,
    dt: DateTime<Utc>,
}

fn parse_log(log: &Vec<String>) -> HashMap<u64, Vec<Event>> {
    let mut events_by_guard: HashMap<u64, Vec<Event>> = HashMap::new();

    let mut guard_id: u64 = 0; // u_u

    let mut I = log.iter();

    loop {
        let mut maybe_line: Option<&String> = I.next();

        if maybe_line.is_none() {
            return events_by_guard;
        }

        let line = maybe_line.unwrap();

        if line.contains("begins") {
            let s: Vec<&str> = line[26..].split_whitespace().collect();
            guard_id = s[0].parse().unwrap();
            continue;
        }
        let event_type = parse_event_type(line);
        let dt = parse_dt(line);
        let events = events_by_guard.entry(guard_id).or_insert_with(Vec::new);
        events.push(Event { event_type, dt });
    }
}

fn parse_event_type(s: &str) -> EventType {
    if s.contains("begins") || s.contains("wakes up") {
        EventType::Awake
    } else {
        EventType::Sleep
    }
}

fn total_minutes(events: &Vec<Event>) -> i64 {
    let sleeps = events.iter().filter(|x| x.event_type == EventType::Sleep);
    let awakes = events.iter().filter(|x| x.event_type == EventType::Awake);
    let mut acc = 0i64;

    for (s, a) in sleeps.zip(awakes) {
        acc += (a.dt.timestamp_millis() - s.dt.timestamp_millis()) / (1000 * 60);
    }
    acc
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    Utc.datetime_from_str(&s[1..17], "%Y-%m-%d %H:%M").unwrap()
}

fn modal_minute(events: &Vec<Event>) -> (u32, u64) {
    let sleeps = events.iter().filter(|x| x.event_type == EventType::Sleep);
    let awakes = events.iter().filter(|x| x.event_type == EventType::Awake);
    let mut m: HashMap<u32, u64> = HashMap::new();
    for (s, a) in sleeps.zip(awakes) {
        for i in s.dt.minute()..a.dt.minute() {
            let n = m.entry(i).or_insert(0);
            *n += 1;
        }
    }
    let (x,y) = m.iter().max_by(|(_, n1), (_, n2)| n1.cmp(n2)).unwrap();
    (x.clone(), y.clone())

}

fn main() {
    // rust is extraordinarily fast
    let log = parse_input("/Users/dmb/rust_aoc/inputs/d4.txt");

    let m = parse_log(&log);

    let (laziest_guard, n) =
        m.iter()
            .map(|(id, events)| (id, total_minutes(events)))
            .max_by(|(_, n1),(_, n2)| n1.cmp(n2))
            .unwrap();

    println!("{}: {}", laziest_guard, n);

    let mode = modal_minute(m.get(laziest_guard).unwrap());

    println!("mode: {:?}", mode);

}
