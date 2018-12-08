extern crate utils;
use utils::utils::read_input;

struct Node {
    metadata:  Vec<u32>,
    children: Vec<Node>
}

fn solve_a(node: &Node) -> u32 {
    let s = node.metadata.iter().sum();
    node.children.iter().fold(s, |acc, child| acc + solve_a(child))
}

fn solve_b(node: &Node) -> u32 {
    if node.children.is_empty() {
        node.metadata.iter().sum()
    } else {
        node.metadata.iter().fold(0, |acc, childindex| {
            let i = (*childindex - 1) as usize;
            if i < node.children.len() {
                acc + solve_b(&node.children[i])
            } else {
                acc
            }
        })
    }
}

fn parse_node(data: &mut Vec<u32>) -> Node {
    let nr_of_children = data.remove(0) as usize;
    let nr_of_metadata = data.remove(0) as usize;
    let mut n = Node {metadata: vec![],children: vec![]};

    for _ in 0..nr_of_children {
        n.children.push(parse_node(data));
    }
    n.metadata = data.drain(0..nr_of_metadata).collect();
    n
}

fn parse_tree(input: String) -> Node {
    let mut data : Vec<u32> = input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    parse_node(&mut data)
}

fn main() {
    let content = read_input().unwrap();
    let root_node = parse_tree(content);
    println!("{}", solve_a(&root_node));
    println!("{}", solve_b(&root_node));
}
