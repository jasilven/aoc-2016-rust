use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// parse input file to hashmap where key is (x,y) and value is (size, used, avail)
fn parse_input(fname: &str) -> HashMap<(usize, usize), (usize, usize, usize)> {
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
                caps[3].parse::<usize>().unwrap(),
                caps[4].parse::<usize>().unwrap(),
                caps[5].parse::<usize>().unwrap(),
            ),
        );
    }
    result
}

fn solve1(input_map: &HashMap<(usize, usize), (usize, usize, usize)>) -> usize {
    let mut result = 0;
    for ((_, a1, a2), (_, b1, b2)) in input_map.values().tuple_combinations() {
        if (*a1 > 0) && (a1 <= b2) {
            result += 1;
        }
        if (*b1 > 0) && (b1 <= a2) {
            result += 1;
        }
    }
    result
}

fn solve2(input_map: &HashMap<(usize, usize), (usize, usize, usize)>) -> usize {
    let (max_x, _) = input_map.keys().max_by(|a, b| a.0.cmp(&b.0)).unwrap();
    let (_, max_y) = input_map.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let (large_limit, _, _) = input_map.values().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    for y in 0..=*max_y {
        for x in 0..=*max_x {
            let (_, used, _) = input_map.get(&(x, y)).unwrap();
            if (x == *max_x) && (y == 0) {
                print!("G");
            } else if (x == 0) && (y == 0) {
                print!("H");
            } else if *used == 0 {
                print!("_");
            } else if (*used > 0) && (*used <= *large_limit) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    let result = 17 + 22 + 34 + (34 * 5) + 1;
    println!("Play sliding puzzle and count the steps:");
    println!("17 + 22 + 34 + (34 * 5) + 1");
    result
}

fn main() {
    let input_map = parse_input("resources/day22-input.txt");
    println!("Part 1: {}", solve1(&input_map));
    println!("Part 2: {}", solve2(&input_map));
}
