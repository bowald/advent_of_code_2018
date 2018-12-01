extern crate utils;
use utils::utils::read_input;

fn main() {
    let content = read_input().unwrap();
    let numbers: Vec<i32> = content.lines().map(|x| {
        return x.parse().unwrap();
    }).collect();
    println!("{}", numbers.iter().sum::<i32>());
}