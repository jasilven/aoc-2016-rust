mod aoc;
use aoc::parse_ints;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Disc {
    id: usize,
    pos_cnt: usize,
    pos: usize,
}

fn parse_input(fname: &str) -> Vec<Disc> {
    let mut result: Vec<Disc> = Vec::new();
    for line in BufReader::new(File::open(fname).unwrap()).lines() {
        let ints = parse_ints(&line.unwrap());
        result.push(Disc {
            id: ints[0] as usize,
            pos_cnt: ints[1] as usize,
            pos: ints[3] as usize,
        });
    }
    result
}

fn solve(discs: &Vec<Disc>) -> usize {
    let mut wait: usize = 0;
    while discs
        .iter()
        .any(|d| 0 != ((d.id + ((wait + d.pos) % d.pos_cnt)) % d.pos_cnt))
    {
        wait += 1;
    }
    return wait;
}

fn main() {
    let mut discs = parse_input("resources/day15-input.txt");
    println!("Part 1: {}", solve(&discs));
    // correct answer: 400589
    discs.push(Disc {
        id: discs.len() + 1,
        pos_cnt: 11,
        pos: 0,
    });
    println!("Part 2: {}", solve(&discs));
    // correct answer: 3045959
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let discs = parse_input("resources/day15-test-input.txt");
        assert_eq!(5, solve(&discs));
    }
}
