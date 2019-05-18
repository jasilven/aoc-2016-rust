extern crate regex;
mod aoc;
use aoc::{is_palindrome, partition_by};
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_ip(line: &str) -> Vec<String> {
    let re: Regex = Regex::new(r"\[[^\]]*\]").expect("invalid regex");
    let s = re.replace_all(line, ",");
    s.splitn(100000, ",")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}

fn parse_hyper(line: &str) -> Vec<String> {
    let re: Regex = Regex::new(r"\[([^\]]*)\]").expect("invalid regex");
    re.captures_iter(line)
        .map(|caps| caps[1].to_owned())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
}

fn solve1(fname: &str) -> i32 {
    let mut result = 0;
    let wlen = 4;
    for line in BufReader::new(File::open(fname).expect("file open error")).lines() {
        let line = line.expect("line parse error");
        let ips = parse_ip(&line);
        let hyper = parse_hyper(&line);
        if ips
            .iter()
            .any(|s| partition_by(&s, wlen).iter().any(|s| is_palindrome(&s)))
            & hyper
                .iter()
                .all(|s| partition_by(&s, wlen).iter().all(|s| !is_palindrome(&s)))
        {
            result += 1;
        }
    }
    result
}

fn aba_to_bab(aba: &str) -> String {
    if aba.len() != 3 {
        panic!("input length not 3");
    }
    format!(
        "{}{}{}",
        aba.get(1..2).unwrap(),
        aba.get(0..1).unwrap(),
        aba.get(1..2).unwrap()
    )
}

fn solve2(fname: &str) -> i32 {
    let mut result = 0;
    let wlen = 3;
    for line in BufReader::new(File::open(fname).expect("file open error")).lines() {
        let line = line.expect("line parse error");
        let ipss = parse_ip(&line)
            .iter()
            .map(|s| partition_by(&s, wlen))
            .flatten()
            .filter(|s| is_palindrome(&s))
            .map(|s| aba_to_bab(&s))
            .collect::<HashSet<String>>();
        let hypers = parse_hyper(&line)
            .iter()
            .map(|s| partition_by(&s, wlen))
            .flatten()
            .filter(|s| is_palindrome(&s))
            .collect::<HashSet<String>>();
        if !ipss
            .intersection(&hypers)
            .collect::<HashSet<&String>>()
            .is_empty()
        {
            result += 1;
        }
    }
    result
}

fn main() {
    println!("Part 1: {}", solve1("resources/day7-input.txt"));
    // correct answer: 118
    println!("Part 2: {}", solve2("resources/day7-input.txt"));
    // correct answer: 260
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ip() {
        let t1 = "eka,toka,kolmas"
            .split(",")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(t1, parse_ip("eka[moikka]toka[kdk]kolmas"));
        assert_eq!(true, parse_ip("").is_empty());
        assert_eq!(true, parse_ip("[moikka]").is_empty());
    }

    #[test]
    fn test_parse_hyper_seq() {
        let t1: Vec<String> = vec![String::from("AA"), String::from("B")];
        assert_eq!(t1, parse_hyper("eka[AA]toka[B]kolmas"));
        assert_eq!(true, parse_hyper("").is_empty());
        assert_eq!(true, parse_hyper("ksksksk]jsd[").is_empty());
    }

    #[test]
    fn test_part1() {
        assert_eq!(2, solve1("resources/day7-test-input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3, solve2("resources/day7-test-input2.txt"));
    }

    #[test]
    fn test_aba_to_bab() {
        assert_eq!("BAB", aba_to_bab("ABA"));
    }
}
