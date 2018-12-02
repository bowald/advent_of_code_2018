extern crate utils;
use utils::utils::read_input;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let content = read_input().unwrap();
    let mut pairs = 0;
    let mut triples = 0;

    content.lines().for_each(|line| {
        let mut char_count = HashMap::new();
        let chars: Vec<char> = line.chars().collect();

        for c in chars {
            char_count.entry(c).or_insert(0);
            *char_count.get_mut(&c).unwrap() += 1;
        }

        let set : HashSet<i32> = char_count.values().map(|x| *x).collect();
        for v in set {
            match v {
                2 => pairs += 1,
                3 => triples += 1,
                _ => (),
            }
        }
    });
    println!("{}", pairs * triples);
}