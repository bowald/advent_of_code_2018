extern crate utils;
extern crate regex;

use utils::utils::read_input;
use regex::Regex;
use std::cmp::max;

pub struct Point {x: i32, y: i32}
pub struct Fabric {id: String, x: i32, y: i32, w: i32, h: i32}

fn maxpoint(fabrics: &Vec<Fabric>) -> Point {
    let mut p = Point {
        x:0,
        y:0
    };

    for fabric in fabrics {
        p.x = max(fabric.x + fabric.w, p.x);
        p.y = max(fabric.y + fabric.h, p.y);
    }
    return p;
}

fn overlapping(fabric: &Vec<String>) -> i32 {
    return fabric.iter().filter(|x| *x == "X").count() as i32;
}

fn map_fabrics(claims: &Vec<Fabric>, max_point: &Point) -> Vec<String> {
    let mut entire_fabric : Vec<String> = vec![".".to_string(); (max_point.x * max_point.y) as usize];
    for f in claims.iter() {
        for x in 0..f.w {
            for y in 0..f.h {
                let p = Point{x: f.x + x, y: f.y + y};
                let index = (p.x + max_point.x * p.y) as usize;
                match entire_fabric[index].as_ref() {
                    "." => entire_fabric[index] = f.id.clone(),
                    _ => entire_fabric[index] = "X".to_string(),
                }
            }
        }
    }
    return entire_fabric;
}

fn capture(line: &str) -> Fabric {
    let re = Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    return Fabric{
        id: caps[1].to_string(),
        x: caps[2].parse::<i32>().unwrap(),
        y: caps[3].parse::<i32>().unwrap(),
        w: caps[4].parse::<i32>().unwrap(),
        h: caps[5].parse::<i32>().unwrap(),
    };
}

fn solve(content: String) -> i32 {
    let lines : Vec<&str> = content.lines().collect();
    let claims : Vec<Fabric> = lines.iter().map(|l| capture(l)).collect();
    let max_point = maxpoint(&claims);
    let entire_fabric = map_fabrics(&claims, &max_point);
    return overlapping(&entire_fabric);
}

fn main() {
    let content = read_input().unwrap();
    let s = solve(content);
    println!("{}", s);
}
