mod utils;

fn main() {
    let content = utils::read_input().unwrap();
    let numbers: Vec<i32> = content.lines().map(|x| {
        return x.parse().unwrap();
    }).collect();
    println!("{}", numbers.iter().sum::<i32>());
}