mod aoc;
use aoc::parse_ints;
use permutohedron::Heap;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum Op {
    SwapPos((usize, usize)),
    SwapLet((u8, u8)),
    RotLeft(usize),
    RotRight(usize),
    RotBased(u8),
    ReversePos((usize, usize)),
    MovePos((usize, usize)),
}

fn parse_operations(fname: &str) -> Vec<Op> {
    let mut result = vec![];
    let file = File::open(fname).expect("unable to open file");
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let is: Vec<usize> = parse_ints(&line).iter().map(move |x| *x as usize).collect();
        match line {
            _ if line.starts_with("swap pos") => result.push(Op::SwapPos((is[0], is[1]))),
            _ if line.starts_with("swap let") => {
                result.push(Op::SwapLet((line.as_bytes()[12], line.as_bytes()[26])))
            }
            _ if line.starts_with("rotate l") => result.push(Op::RotLeft(is[0])),
            _ if line.starts_with("rotate r") => result.push(Op::RotRight(is[0])),
            _ if line.starts_with("rotate b") => {
                result.push(Op::RotBased(line.chars().last().unwrap() as u8))
            }
            _ if line.starts_with("reverse pos") => result.push(Op::ReversePos((is[0], is[1]))),
            _ if line.starts_with("move pos") => result.push(Op::MovePos((is[0], is[1]))),
            _ => panic!("unknown operation"),
        };
    }
    result
}
fn swap_letters(word: &mut [u8], x: u8, y: u8) {
    for ch in word {
        if *ch == x {
            *ch = y;
        } else if *ch == y {
            *ch = x;
        }
    }
}

fn rotate_based_pos(word: &mut [u8], arg: u8) {
    let mut index = 0;
    for i in 0..word.len() {
        if arg == word[i] {
            index = i;
            break;
        }
    }
    let mut cnt = index + 1;
    if index >= 4 {
        cnt += 1;
    }
    word.rotate_right(cnt % word.len());
}

fn move_positions(word: &mut [u8], x: usize, y: usize) {
    let mut tmp = word.to_vec();
    let ch = tmp.remove(x);
    tmp.insert(y, ch);
    word.clone_from_slice(&tmp);
}

fn solve1(ops: &[Op], word: &str) -> String {
    let mut w: Vec<u8> = word.bytes().collect();
    for op in ops {
        match op {
            Op::SwapPos((x, y)) => w.swap(*x, *y),
            Op::SwapLet((x, y)) => swap_letters(&mut w, *x, *y),
            Op::RotLeft(x) => w.rotate_left(*x),
            Op::RotRight(x) => w.rotate_right(*x),
            Op::RotBased(x) => rotate_based_pos(&mut w, *x),
            Op::ReversePos((x, y)) => w[*x..=*y].reverse(),
            Op::MovePos((x, y)) => move_positions(&mut w, *x, *y),
        };
    }
    w.iter().map(|c| *c as char).collect()
}

#[allow(dead_code)]
fn solve2(ops: &[Op], word: &str, target: &str) -> String {
    let target = target.to_owned();
    let mut data: Vec<char> = word.chars().collect();
    let heap = Heap::new(&mut data);
    for perm in heap {
        let s: String = perm.iter().collect();
        let word = solve1(ops, &s);
        if word == target {
            return s;
        }
    }
    "not found".to_owned()
}

fn all_permutations(word: &str) -> Vec<String> {
    let mut result = vec![];
    let mut data: Vec<char> = vec![];
    for ch in word.chars() {
        data.push(ch);
    }
    let heap = Heap::new(&mut data);
    for perm in heap {
        let s: String = perm.iter().collect();
        result.push(s);
    }
    result
}

fn solve2_parallel(ops: &[Op], word: &str, target: &str) -> String {
    all_permutations(word)
        .par_iter()
        .find_first(|s| solve1(ops, s) == *target)
        .unwrap()
        .clone()
}

fn main() {
    let ops = parse_operations("resources/day21-input.txt");
    println!("Part 1: {}", solve1(&ops, "abcdefgh"));
    // correct answer: dbfgaehc
    println!("Part 2: {}", solve2_parallel(&ops, "abcdefgh", "fbgdceah"));
    // correct answer: aghfcdeb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_letters() {
        let mut a = vec!['a' as u8, 'b' as u8, 'c' as u8];
        let b = vec!['c' as u8, 'b' as u8, 'a' as u8];
        swap_letters(&mut a, 'a' as u8, 'c' as u8);
        assert_eq!(a, b);
    }

    #[test]
    fn test_part1() {
        let ops = parse_operations("resources/day21-test-input.txt");
        assert_eq!("decab", solve1(&ops, "abcde"));
    }
}
