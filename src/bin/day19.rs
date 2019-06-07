use std::collections::VecDeque;

fn prune_elves(elves: &mut Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = elves.iter().step_by(2).cloned().collect();
    if (elves.len() % 2) == 1 {
        result.remove(0);
    }
    result
}

fn solve1(cnt: u32) -> u32 {
    let mut elves: Vec<u32> = (1..=cnt).collect();
    while elves.len() > 1 {
        elves = prune_elves(&mut elves);
    }
    elves[0]
}

fn solve2(cnt: u32) -> u32 {
    let mut elves: VecDeque<u32> = (1..=cnt).collect();
    while elves.len() > 1 {
        elves.remove(elves.len() / 2);
        let e = elves.pop_front().unwrap();
        elves.push_back(e);
    }
    elves[0]
}

fn main() {
    println!("Part 1: {}", solve1(3014603));
    // correct answer: 1834903
    println!("Part 2: {}", solve2(3014603));
    // correct answer: 1420280
}
