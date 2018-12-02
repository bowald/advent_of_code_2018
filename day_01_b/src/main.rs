extern crate utils;
use utils::utils::read_input;
use std::collections::HashSet;

fn main() {
    let content = read_input().unwrap();
    let sequence: Vec<i32> = content.lines().map(|x| {
        return x.parse().unwrap();
    }).collect();
    let mut frequencies: HashSet<i32> = HashSet::new();
    frequencies.insert(0);
    let mut index = 0;
    let mut res = 0;
    loop {
        res += sequence[index];
        if frequencies.contains(&res) {
            println!("{}", res);
            break;
        }
        frequencies.insert(res);
        index = (index + 1) % sequence.len();
    }
}