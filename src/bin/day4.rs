extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_input(fname: &str) -> Vec<(String, String, String)> {
    let mut result = vec![];
    let file = File::open(fname).expect("file open error");
    let re = Regex::new(r"^(\D+)(\d+)\[([a-z]{5})\]").expect("invalid regex");
    for line in BufReader::new(file).lines() {
        let s = line.expect("file read error");
        let caps = re.captures(&s).expect("input parse error");
        result.push((
            caps[1].to_owned().replace("-", ""),
            caps[2].to_owned(),
            caps[3].to_owned(),
        ));
    }
    result
}

fn checksum(s: &str) -> String {
    let mut hm: HashMap<char, i32> = HashMap::new();
    for ch in s.chars() {
        let counter = hm.entry(ch).or_insert(0);
        *counter += 1;
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
    input
        .iter()
        .filter(|x| checksum(&x.0) == x.2)
        .map(|x| x.1.parse::<i32>().unwrap())
        .fold(0, |acc, i| acc + i)
}

fn decrypt(s: &str, n: u32) -> String {
    s.chars()
        .map(|c| 97 + ((n + (c as u32) - 97) % 26) as u8)
        .map(|i| i as char)
        .collect::<String>()
}

fn solve2(input: &Vec<(String, String, String)>) -> String {
    for item in input {
        let id: u32 = item.1.parse().expect("sector id parse error");
        if decrypt(&item.0, id) == "northpoleobjectstorage" {
            return item.1.to_owned();
        }
    }
    panic!("solution not found");
}

fn main() {
    let input = parse_input("resources/day4-input.txt");
    println!("Part 1: {}", solve1(&input));
    // correct answer: 409147
    println!("Part 2: {}", solve2(&input));
    // correct answer: 991
}

#[cfg(test)]
mod test_a {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1514, solve1(&parse_input("resources/day4-test-input.txt")));
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(
            "veryencryptedname",
            decrypt(&String::from("qzmtzixmtkozyivhz"), 343)
        )
    }
}
