extern crate md5;
extern crate regex;
mod aoc;

use aoc::partition_by;
use std::collections::HashSet;
use std::collections::VecDeque;

fn repetitive_char(rcnt: usize, s: &str) -> Option<char> {
    let mut result = None;
    partition_by(&s, rcnt).iter().any(|s| {
        let mut v: Vec<char> = s.chars().collect();
        v.dedup();
        if v.len() == 1 {
            result = Some(v[0]);
            return true;
        }
        return false;
    });
    result
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ChIndex {
    index: i32,
    ch: char,
}

fn build_que(size: usize) -> VecDeque<ChIndex> {
    let mut result = VecDeque::new();
    for _ in 0..size {
        result.push_front(ChIndex { index: -1, ch: ' ' });
    }
    result
}

fn gen_hash(s: &str, cnt: usize) -> String {
    let mut result = String::from(s);
    for _ in 0..cnt {
        result = format!("{:x}", md5::compute(&mut result))
    }
    result
}

fn solve(limit: usize, salt: &str, hash_cnt: usize) -> i32 {
    let mut que = build_que(1000);
    let mut keys: HashSet<ChIndex> = HashSet::new();
    for n in 0.. {
        if keys.len() >= limit {
            break;
        }
        let hash = gen_hash(&format!("{}{}", &salt, n), hash_cnt);
        if let Some(ch) = repetitive_char(5, &hash) {
            for item in que.iter().filter(|item| item.ch == ch) {
                keys.insert(item.clone());
            }
        }
        if let Some(ch) = repetitive_char(3, &hash) {
            que.push_back(ChIndex { index: n, ch: ch });
        } else {
            que.push_back(ChIndex { index: n, ch: ' ' });
        }
        que.pop_front().unwrap();
    }
    let mut ks: Vec<ChIndex> = keys.iter().cloned().collect();
    ks.sort_by(|a, b| a.index.cmp(&b.index));

    ks[limit - 1].index
}

fn main() {
    let salt = "yjdafjpo";
    println!("Part 1: {}", solve(64, &salt, 1));
    // correct answer: 25427
    println!("Part 2: {}", solve(64, &salt, 2017));
    // correct answer: 22045
}
