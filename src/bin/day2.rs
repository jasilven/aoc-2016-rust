mod aoc;
use aoc::*;
use std::collections::HashMap;

fn key_for_val(hm: &HashMap<Point, char>, val: &char) -> Point {
    for (k, v) in hm {
        if val == v {
            return k.clone();
        }
    }
    panic!("cannot find key for value: {}", val);
}

fn solve(fname: &str, kpfname: &str) -> String {
    let keypad = parse_map(kpfname, &[' ']);
    let mut point = key_for_val(&keypad, &'5');
    let mut code = String::new();

    for line in slurp(fname).expect("cannot read input file").lines() {
        for ch in line.chars() {
            let p = move_point(&point, ch, 1);
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
    fn part1_test() {
        assert_eq!(
            "1985",
            solve(
                "resources/day2-test-input.txt",
                "resources/day2-keypad1.txt"
            )
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            "5DB3",
            solve(
                "resources/day2-test-input.txt",
                "resources/day2-keypad2.txt"
            )
        );
    }
}
