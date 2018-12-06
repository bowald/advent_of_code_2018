extern crate utils;
extern crate regex;

use std::collections::HashMap;
use std::cmp::Ordering;
use utils::utils::read_input;
use regex::Regex;

#[derive(Eq,PartialEq)]
enum Action {
    Starts,
    Sleeps,
    Awakes
}
#[derive(Eq)]
pub struct Event {ordering: i32, minute: usize, action: Action, guardid: i32}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.ordering.cmp(&other.ordering)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.ordering == other.ordering
    }
}

fn awakes(line: &str) -> bool {
    return line.contains("wakes up");
}

fn falls_asleep(line: &str) -> bool {
    return line.contains("falls asleep");
}

fn parse_guard_id(line: &str) -> i32 {
    let re = Regex::new(r"#(\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    caps[1].parse::<i32>().unwrap()
}

fn parse_timestamp(line: &str) -> (i32, usize) {
    let re = Regex::new(r"1518-(\d{2})-(\d{2})\s(\d{2}):(\d{2})").unwrap();
    let caps = re.captures(line).unwrap();
    return (
        caps[1].parse::<i32>().unwrap() * 1000000 + caps[2].parse::<i32>().unwrap() * 10000 + caps[3].parse::<i32>().unwrap() * 100 + caps[4].parse::<i32>().unwrap(),
        caps[4].parse::<usize>().unwrap(),
    );
}

fn parse_action(line: &str) -> (Action, i32) {
    return
    if awakes(line) {
        (Action::Awakes, -1)
    } else if falls_asleep(line) {
        (Action::Sleeps, -1)
    } else {
        (Action::Starts, parse_guard_id(line))
    };
}

fn parse_events(lines: &Vec<&str>) -> Vec<Event> {
    return lines.iter().map(|l| {
        let (o, m) = parse_timestamp(l);
        let (a, id) = parse_action(l);
        Event{ordering: o, minute: m, action: a, guardid: id}
        }).collect();
}

fn calc_sleeping_table(events: &Vec<Event>) -> HashMap<i32, Vec<i32>> {
    let mut sleepeing_table : HashMap<i32, Vec<i32>> = HashMap::new();
    let mut guardid = -1;
    let mut prev_index = 0;

    for e in events {
        match e.action {
            Action::Starts => {
                sleepeing_table.entry(e.guardid).or_insert(vec![0;60]);
                guardid = e.guardid;
                prev_index = e.minute;
            },
            Action::Awakes => {
                for n in &mut sleepeing_table.get_mut(&guardid).unwrap()[prev_index..e.minute] { *n += 1 }
                prev_index = e.minute;
            }
            Action::Sleeps => {
                prev_index = e.minute;
            }

        }
    }
    sleepeing_table
}

fn solve_a_guardid(table: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut max_sleep = 0;
    let mut id = -1;
    for (guardid, minutes) in table.into_iter() {
        let minutes_sleep = minutes.iter().sum();
        if minutes_sleep > max_sleep {
            max_sleep = minutes_sleep;
            id = *guardid;
        }
    }
    id
}

fn solve_a(table: &HashMap<i32, Vec<i32>>) -> i32 {
    let id = solve_a_guardid(table);
    let mut index = 0;
    let mut maxvalue : i32 = 0;
    let minutes : &Vec<i32> = table.get(&id).unwrap();
    for (i, &m) in minutes.iter().enumerate() {
        if maxvalue < m {
            maxvalue = m;
            index = i;
        }
    }
    id * (index as i32)
}

fn solve_b_guardid(table: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut max_frequent = 0;
    let mut id = -1;
    for (guardid, minutes) in table.into_iter() {
        let freq = minutes.iter().max().unwrap();
        if *freq > max_frequent {
            max_frequent = *freq;
            id = *guardid;
        }
    }
    id
}

fn solve_b(table: &HashMap<i32, Vec<i32>>) -> i32 {
    let id = solve_b_guardid(table);
    let mut index = 0;
    let mut maxvalue : i32 = 0;
    let minutes : &Vec<i32> = table.get(&id).unwrap();
    for (i, &m) in minutes.iter().enumerate() {
        if maxvalue < m {
            maxvalue = m;
            index = i;
        }
    }
    id * (index as i32)
}

fn solve(content: String) {
    let lines : Vec<&str> = content.lines().collect();
    let mut events : Vec<Event> = parse_events(&lines);
    events.sort();
    let sleepeing_table = calc_sleeping_table(&events);

    println!("A: {}", solve_a(&sleepeing_table));
    println!("B: {}", solve_b(&sleepeing_table));
}

fn main() {
    let content = read_input().unwrap();
    solve(content);
}
