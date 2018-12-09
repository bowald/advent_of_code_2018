use std::collections::VecDeque;

// Similar to the python function rotate
// https://docs.python.org/2/library/collections.html#collections.deque.rotate
fn rotate(ring: &mut VecDeque<u32>, amount: i32){
    (0..amount.abs()).for_each(|_| {
        if amount > 0 {
            let tmp = ring.pop_back().expect("Fail in rotate");
            ring.push_front(tmp);
        }
        else {
            let tmp = ring.pop_front().expect("Fail in rotate");
            ring.push_back(tmp);
        }
    });
}

fn solve(nr_of_players : u32, last_marble : u32) -> u32 {
    let mut score = vec![0; nr_of_players as usize];
    let mut ring : VecDeque<u32> = VecDeque::with_capacity(last_marble as usize);
    ring.push_back(0);
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            let current_player = ((marble - 1) % nr_of_players) as usize;
            rotate(&mut ring, 7);
            score[current_player] += marble + ring.pop_back().unwrap();
            rotate(&mut ring, -1);
        }
        else {
            rotate(&mut ring, -1);
            ring.push_back(marble);
        }
    }
    *score.iter().max().unwrap()
}

fn main() {
    let nr_of_players : u32 = 428;
    let last_marble : u32 = 72061;
    println!("A: {}", solve(nr_of_players, last_marble));
    println!("B: {}", solve(nr_of_players, last_marble * 100));
}
