extern crate utils;

use utils::utils::read_input;
use std::cmp;

fn reacts(a: u8, b: u8) -> bool {
    let v : i8 = a as i8 - b as i8;
    v.abs() == 32
}

fn tuplify(input: &String) -> Vec<(u8, u8)> {
    let evens: Vec<u8> = input.as_bytes()
                     .iter()
                     .enumerate()
                     .filter(|&(i,_)| i % 2 == 0)
                     .map(|(_,e)| *e).collect();

    let odds: Vec<u8> = input.as_bytes()
                    .iter()
                    .enumerate()
                    .filter(|&(i,_)| i % 2 == 1)
                    .map(|(_,e)| *e).collect();
    evens.iter().zip(odds.iter()).map(|(a,b)| (*a,*b)).collect()
}

fn stringify(bytetuples: &Vec<(u8, u8)>) -> String {
    let mut s = String::new();
    for (x, y) in bytetuples.iter() {
        s.push(*x as char);
        s.push(*y as char);
    }
    s
}

fn filter_units(poly: String) -> String{
    let mut output = poly;
    loop {
        let before = output.len();
        output = stringify(&tuplify(&output).iter().filter(|(a,b)| !reacts(*a, *b)).map(|x| *x).collect());
        if before == output.len(){
            break;
        }
    }
    output
}

fn padd_odd(s: &mut String) -> bool{
    if s.len() % 2 == 1 {
        s.push('-');
        return true;
    }
    false
}

fn padd(s: &mut String) {
    s.insert(0, '-');
    s.push('-');
}

fn unpadd(s: &mut String) {
    s.remove(0);
    s.pop();
}

fn solve_a(unstable_poly: &String) -> i32 {

    let mut poly = unstable_poly.clone();

    let padded = padd_odd(&mut poly);

    loop {
        let length_before = poly.len();
        poly = filter_units(poly);
        padd(&mut poly);
        poly = filter_units(poly);
        unpadd(&mut poly);

        if length_before == poly.len() {
            break;
        }
    }

    if padded {
        poly.pop();
    }

    poly.len() as i32
}

fn solve_b(unstable_poly: &String) -> i32 {

    let mut shortest = 9999999;

    for i in 65..=90 {
        let bytes : Vec<u8> =unstable_poly.as_bytes().iter().filter(|&c| *c != i as u8 && *c != i+32 as u8).map(|x| *x).collect();
        let s = String::from_utf8(bytes).unwrap();
        let res = solve_a(&s);
        shortest = cmp::min(shortest, res);
    }
    shortest
}

fn main() {
    let content = read_input().unwrap();
    println!("A: {}", solve_a(&content));
    println!("B: {}", solve_b(&content));
}
