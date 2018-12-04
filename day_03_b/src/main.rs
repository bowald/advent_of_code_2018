extern crate utils;
extern crate regex;

use utils::utils::read_input;
use std::collections::HashSet;
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

// fn overlap(a: &Fabric, b: &Fabric) -> bool {
//     // return a.x < b.x + b.w && a.x + a.w > b.x && a.y > b.y + b.h && a.y + a.h < b.y;
//     if a.x >= b.x + b.w || b.x >= a.x + a.w {
//         return false;
//     }

//     // If one rectangle is above other
//     if a.y < b.y + b.h || b.y < a.y + a.h {
//         return false;
//     }

//     return true;
// }

// fn non_overlapping(fabrics: &Vec<Fabric>) -> Option<String> {
//     for f in fabrics.iter() {
//         let mut overlapped = false;
//         for other in fabrics.iter() {
//             if f.id == other.id {
//                 continue;
//             }

//             if overlap(f, other) {
//                 println!("overlapped {}", f.id);
//                 overlapped = true;
//                 break;
//             }
//         }
//         // if !overlapped {
//         return Some(f.id.clone());
//             // println!("{}", f.id);
//         // }
//     }
//     None
// }

fn map_fabrics(claims: &Vec<Fabric>, max_point: &Point) -> String {
    let mut candidates : HashSet<String> = HashSet::new();
    let mut entire_fabric : Vec<String> = vec![".".to_string(); (max_point.x * max_point.y) as usize];
    for f in claims.iter() {
        candidates.insert(f.id.clone());
        for x in 0..f.w {
            for y in 0..f.h {
                let p = Point{x: f.x + x, y: f.y + y};
                let index = (p.x + max_point.x * p.y) as usize;
                match entire_fabric[index].as_ref() {
                    "." => {
                        entire_fabric[index] = f.id.clone();
                    },
                    _ => {
                        candidates.remove(&f.id);
                        candidates.remove(&entire_fabric[index]);
                        entire_fabric[index] = "X".to_string()
                    },
                }
            }
        }
    }
    for x in candidates.iter() {
        return x;
    }
    return "".to_string();
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

fn solve(content: String) -> String {
    let lines : Vec<&str> = content.lines().collect();
    let claims : Vec<Fabric> = lines.iter().map(|l| capture(l)).collect();
    let max_point = maxpoint(&claims);
    return map_fabrics(&claims, &max_point);
}


fn main() {
    let content = read_input().unwrap();
    let s = solve(content);
    println!("{}", s);
}
