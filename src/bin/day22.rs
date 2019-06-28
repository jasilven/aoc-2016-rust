use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// parse input file to hashmap where key is (x,y) and value is (used, avail)
fn parse_input(fname: &str) -> HashMap<(usize, usize), (usize, usize)> {
    let mut result = HashMap::new();
    let re = Regex::new(r"^.*-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T.+$").unwrap();
    let f = File::open(fname).expect("unable to open file");
    for line in BufReader::new(f).lines().skip(2) {
        let l = line.unwrap();
        let caps = re.captures(&l).expect("regex parse error");
        result.insert(
            (
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
            ),
            (
                caps[4].parse::<usize>().unwrap(),
                caps[5].parse::<usize>().unwrap(),
            ),
        );
    }
    result
}

fn solve1(input_map: &HashMap<(usize, usize), (usize, usize)>) -> usize {
    let mut result = 0;
    for ((a1, a2), (b1, b2)) in input_map.values().tuple_combinations() {
        if (*a1 > 0) & (a1 <= b2) {
            result += 1;
        }
        if (*b1 > 0) & (b1 <= a2) {
            result += 1;
        }
    }
    result
}

fn main() {
    let input_map = parse_input("resources/day22-input.txt");
    println!("Part 1: {:?}", solve1(&input_map));
}
