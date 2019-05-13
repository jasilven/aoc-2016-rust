// mod aoc;
// use aoc::*;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_input(fname: &str) -> Vec<(String, String, String)> {
    let mut result: Vec<(String, String, String)> = vec![];
    let file = File::open(fname).expect("cannot open file");
    let re = Regex::new(r"^(\D+)(\d+)\[([a-z]{5})\]").expect("Regex::new() error");
    for line in BufReader::new(file).lines() {
        let s = line.unwrap();
        let caps = re.captures(&s).expect("re.captures() error");
        result.push((
            caps[1].to_owned().replace("-", ""),
            caps[2].to_owned(),
            caps[3].to_owned(),
        ));
    }
    result
}

fn checksum(s: &String) -> String {
    let mut hm: HashMap<char, i32> = HashMap::new();
    for ch in s.chars() {
        let counter = hm.entry(ch).or_insert(0);
        *counter += 1;;
    }
    let mut kvs: Vec<(&char, &i32)> = hm.iter().collect();
    kvs.sort_by(|a, b| match a.1 == b.1 {
        true => a.0.cmp(b.0),
        _ => a.1.cmp(b.1).reverse(),
    });
    kvs.iter()
        .take(5)
        .fold(String::new(), |acc, x| format!("{}{}", acc, *x.0))
}

fn solve1(input: &Vec<(String, String, String)>) -> i32 {
    let mut result = 0;
    for item in input {
        match checksum(&item.0).eq(&item.2) {
            true => result += &item.1.parse::<i32>().unwrap(),
            _ => (),
        };
    }
    result
}

//"northpole object storage"
fn solve2(input: Vec<(String, String)>) -> String {
    String::from("jee")
}

fn main() {
    let input = parse_input("resources/day4-test-input.txt");
    dbg!(&input);
    println!("Part 1: {:?}", solve1(&input));
    // println!("Part 1:", solve1());
}

#[cfg(test)]
mod test_a {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(1514, solve1(&parse_input("resources/day4-test-input.txt")));
    }
}
