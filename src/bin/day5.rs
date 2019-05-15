extern crate regex;
use regex::Regex;

fn parse_ints(s: &str) -> Vec<i32> {
    let re = Regex::new(r"(\d+)").unwrap();
    let nums: Vec<i32> = re
        .captures_iter(&s)
        .map(|x| x[1].parse::<i32>().unwrap())
        .collect();
    dbg!(&nums);
    nums
}

fn main() {
    dbg!(parse_ints("10 kdkd 200 fj,,. 3000"));
}
