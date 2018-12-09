extern crate utils;
extern crate regex;

use utils::utils::read_input;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use regex::Regex;

#[derive(Eq, Clone)]
struct Step {
    id: char,
    children: Vec<char>,
    requires: Vec<char>,
    seconds_left: u8
}

impl Step {
    fn from_id(id: char) -> Step {
        Step{id: id, children: vec![], requires: vec![], seconds_left: (id as u8) - 4}
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Step) -> Ordering {
        other.id.cmp(&self.id)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Step) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Step) -> bool {
        self.id == other.id
    }
}

#[derive(Clone)]
struct Worker {
    step: Option<Step>,
}

impl Worker {
    fn work(&mut self) {
        if let Some(step) = self.step.as_mut() {
            if step.seconds_left > 0 {
                step.seconds_left -= 1;
            }
        }
    }

    fn is_idle(&self) -> bool {
        match self.step.as_ref() {
            Some(step) => step.seconds_left == 0,
            None => true,
        }
    }

    fn get_complete(&self) -> Option<&Step>{
        if let Some(step) = self.step.as_ref() {
            if step.seconds_left == 0 {
              return Some(step);
            }
        }
        None
    }
}

fn parse_line(line : &str) -> (char, char) {
    let re = Regex::new(r"Step\s([A-Z]).*step\s*([A-Z])").unwrap();
    let caps = re.captures(line).unwrap();
    return (
        caps[1].parse::<char>().unwrap(),
        caps[2].parse::<char>().unwrap(),
    );
}

fn parse_steps(input : &String) -> HashMap<char, Step> {
    let mut steps_dict : HashMap<char, Step> = HashMap::new();
    for line in input.lines(){
        let (id, unlocks) = parse_line(line);
        {
            let step = steps_dict.entry(id).or_insert(Step::from_id(id));
            step.children.push(unlocks);
        }
        {
            let step = steps_dict.entry(unlocks).or_insert(Step::from_id(unlocks));
            step.requires.push(id);
        }
    }
    steps_dict
}

fn find_starts(dict: &HashMap<char, Step>) -> Vec<&Step> {
    let mut out : Vec<&Step> = Vec::new();
    for v in dict.values() {
        if v.requires.is_empty(){
            out.push(v);
        }
    }
    out
}

fn solve_a(dict : &HashMap<char, Step>) -> String {
    let mut completed : Vec<char> = vec![];
    let mut frontier: BinaryHeap<&Step> = BinaryHeap::new();

    for s in find_starts(&dict).iter() {
        frontier.push(s);
    }

    while let Some(completed_step) = frontier.pop() {
        completed.push(completed_step.id);
        for child in completed_step.children.iter() {
            if completed.contains(child) {
                continue;
            }

            if let Some(step) = dict.get(&child) {
                if step.requires.iter().all(|id| completed.contains(id)) {
                    frontier.push(step);
                }
            }
        }
    }
    completed.into_iter().collect()
}

fn solve_b(dict : &HashMap<char, Step>) -> i32 {
    let mut completed : Vec<char> = vec![];
    let mut frontier: BinaryHeap<&Step> = BinaryHeap::new();
    let mut workers: Vec<Worker> = vec![Worker{step: None};5];

    for s in find_starts(&dict).iter() {
        frontier.push(s);
    }
    let mut seconds = 0;
    while !frontier.is_empty() || workers.iter().any(|w| !w.is_idle()) {
        for worker in workers.iter_mut().filter(|w| w.is_idle()) {
            if let Some(next_step) = frontier.pop() {
                worker.step = Some(next_step.clone());
            }
        }

        for worker in workers.iter_mut().filter(|w| !w.is_idle()) {
            worker.work();

            // check complete
            if let Some(completed_step) = worker.get_complete() {
                completed.push(completed_step.id);
                for child in completed_step.children.iter() {
                    if completed.contains(child) {
                        continue;
                    }

                    if let Some(step) = dict.get(&child) {
                        if step.requires.iter().all(|id| completed.contains(id)) {
                            frontier.push(step);
                        }
                    }
                }
            }
        }
        seconds += 1;
    }
    seconds
}

fn main() {
    let content = read_input().unwrap();
    let mut step_dict = parse_steps(&content);
    println!("A: {}", solve_a(&mut step_dict));
    println!("B: {}", solve_b(&mut step_dict));
}
