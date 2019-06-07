mod aoc;
use aoc::parse_ints;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const MAXIP: usize = 4294967295;

fn parse_input(fname: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    for line in BufReader::new(File::open(fname).unwrap()).lines() {
        let line = line.unwrap().replace("-", " ");
        let ints = parse_ints(&line);
        result.push((ints[0] as usize, ints[1] as usize));
    }
    result
}

fn get_sorted_cands(ranges: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut cands: Vec<usize> = ranges
        .iter()
        .map(|(_, hi)| {
            if hi < &MAXIP {
                return hi + 1 as usize;
            } else {
                return *hi;
            }
        })
        .collect();
    cands.sort();
    cands
}

fn solve1(ranges: &Vec<(usize, usize)>) -> usize {
    let mut result = 0;
    for cand in get_sorted_cands(&ranges) {
        if ranges.iter().all(|(lo, hi)| (&cand < lo) | (&cand > hi)) {
            result = cand;
            break;
        }
    }
    result
}

fn solve2(ranges: &Vec<(usize, usize)>) -> usize {
    let blocked_cnt = ranges.iter().fold(0, |acc, (lo, hi)| acc + hi - lo + 1);
    println!("{}", &blocked_cnt);
    MAXIP - blocked_cnt + 1
}

fn main() {
    let ranges = parse_input("resources/day20-input.txt");
    println!("Part 1: {:?}", solve1(&ranges));
    // correct answer: 14975795
    println!("Part 2: {:?}", solve2(&ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let ranges = parse_input("resources/day20-test-input.txt");
        assert_eq!(3, solve1(&ranges));
    }
}
