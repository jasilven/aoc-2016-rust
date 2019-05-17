mod aoc;
use std::collections::HashMap;
use std::fs;

fn solve(fname: &str, puzzle_part: i32) -> String {
    let mut result = "".to_owned();
    let mut hm: HashMap<char, i32> = HashMap::new();
    let data = fs::read_to_string(fname).expect("error reading file");

    for n in 0..data.lines().nth(0).unwrap().len() {
        for line in data.lines() {
            let ch = line.chars().nth(n).expect("parse error");
            if let Some(i) = hm.get_mut(&ch) {
                *i += 1;
            } else {
                hm.insert(ch.clone(), 1);
            }
        }
        let mut vec: Vec<(char, i32)> = hm.iter().map(|(ch, i)| (ch.clone(), *i)).collect();
        vec.sort_by(|(_, i1), (_, i2)| i1.cmp(i2));
        match puzzle_part {
            1 => result.push(vec.last().unwrap().0.clone()),
            2 => result.push(vec.first().unwrap().0.clone()),
            _ => panic!("puzzle part should be 1 or 2"),
        }
        hm.clear();
    }
    result
}

fn main() {
    println!("Part 1: {}", solve("resources/day6-input.txt", 1));
    // correct answer: nabgqlcw
    println!("Part 2: {}", solve("resources/day6-input.txt", 2));
    // correct answer: ovtrjcjh
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("easter", solve("resources/day6-test-input.txt", 1));
        assert_eq!("advent", solve("resources/day6-test-input.txt", 2));
    }
}
