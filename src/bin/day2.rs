mod aoc;
use aoc::parse_map;
use aoc::point::move_point;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn find_first_by_key<K, V: PartialEq<char>>(hm: &HashMap<K, V>, val: char) -> Option<(&K, &V)> {
    hm.iter().filter(|(_, c)| *c == &val).next()
}

fn solve(fname: &str, kpfname: &str) -> String {
    let keypad = parse_map(kpfname, &[' ']).expect("unable to parse keypad file");
    let mut point = find_first_by_key(&keypad, '5')
        .expect("key not found for value")
        .0
        .clone();
    let mut code = String::new();
    for line in BufReader::new(File::open(&fname).unwrap()).lines() {
        for ch in line.unwrap().chars() {
            let p = move_point(&point, ch, 1).expect("cannot move point");
            if keypad.contains_key(&p) {
                point = p;
            }
        }
        code.push(*keypad.get(&point).unwrap());
    }
    code
}

fn main() {
    let fname = "resources/day2-input.txt";
    println!("Part 1: {}", solve(fname, "resources/day2-keypad1.txt"));
    // correct answer: 74921
    println!("Part 2: {}", solve(fname, "resources/day2-keypad2.txt"));
    // correct answer: A6B35
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            "1985",
            solve(
                "resources/day2-test-input.txt",
                "resources/day2-keypad1.txt"
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            "5DB3",
            solve(
                "resources/day2-test-input.txt",
                "resources/day2-keypad2.txt"
            )
        );
    }
}
