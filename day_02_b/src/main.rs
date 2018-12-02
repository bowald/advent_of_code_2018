extern crate utils;
use utils::utils::read_input;

fn get_common(xs: &str, ys: &str) -> String {
    return xs.as_bytes().iter().zip(ys.as_bytes())
            .filter(|(&x, &y)| x==y)
            .map(|(&x, _)| x as char)
            .collect();
}

fn solve(content: String) -> Option<String> {
    let ids : Vec<&str> = content.lines().collect();
    for (i, id) in ids.iter().enumerate() {
        for other in ids[i+1..].iter() {
            let common = get_common(id, other);
            if common.len() == id.len() - 1 {
                return Some(common);
            }
        }
    }
    None
}

fn main() {
    let content = read_input().unwrap();
    let s = solve(content).unwrap();
    println!("{}", s);

}
