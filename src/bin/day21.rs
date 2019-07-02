mod aoc;
use aoc::parse_ints;
use permutohedron::Heap;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_operations(fname: &str) -> Vec<String> {
    let mut result = vec![];
    let file = File::open(fname).expect("unable to open file");
    for line in BufReader::new(file).lines() {
        result.push(line.unwrap());
    }
    result
}

fn swap_pos(word: &mut [u8], args: &[isize]) {
    let x = args[0] as usize;
    let y = args[1] as usize;
    word.swap(x, y);
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

fn rotate_left(word: &mut [u8], args: &[isize]) {
    let cnt = (args[0] as usize) % word.len();
    word.rotate_left(cnt);
}

fn rotate_right(word: &mut [u8], args: &[isize]) {
    let cnt = (args[0] as usize) % word.len();
    word.rotate_right(cnt);
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
    rotate_right(word, &[cnt as isize]);
}

fn reverse_positions(word: &mut [u8], args: &[isize]) {
    let x = args[0] as usize;
    let y = args[1] as usize;
    word[x..=y].reverse();
}

fn move_positions(word: &mut [u8], args: &[isize]) {
    let x = args[0] as usize;
    let y = args[1] as usize;
    let mut tmp = word.to_vec();
    let ch = tmp.remove(x);
    tmp.insert(y, ch);
    word.clone_from_slice(&tmp);
}

fn solve1(ops: &[String], word: &str) -> String {
    let mut w: Vec<u8> = word.bytes().collect();
    for op in ops {
        match op {
            _ if op.starts_with("swap position") => swap_pos(&mut w, &parse_ints(&op)),
            _ if op.starts_with("swap letter") => {
                swap_letters(&mut w, op.as_bytes()[12], op.as_bytes()[26])
            }
            _ if op.starts_with("rotate left") => rotate_left(&mut w, &parse_ints(&op)),
            _ if op.starts_with("rotate right") => rotate_right(&mut w, &parse_ints(&op)),
            _ if op.starts_with("rotate based") => {
                rotate_based_pos(&mut w, op.chars().last().unwrap() as u8)
            }
            _ if op.starts_with("reverse positions") => reverse_positions(&mut w, &parse_ints(&op)),
            _ if op.starts_with("move position") => move_positions(&mut w, &parse_ints(&op)),
            _ => panic!("unknown operation"),
        };
    }
    w.iter().map(|c| *c as char).collect()
}

#[allow(dead_code)]
fn solve2(ops: &[String], word: &str, target: &str) -> String {
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

fn solve2_parallel(ops: &[String], word: &str, target: &str) -> String {
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
    fn test_swap_pos() {
        let mut a = vec!['a' as u8, 'b' as u8, 'c' as u8];
        let b = vec!['c' as u8, 'b' as u8, 'a' as u8];
        let args = vec![0, 2];
        swap_pos(&mut a, &args);
        assert_eq!(a, b);
    }

    #[test]
    fn test_swap_letters() {
        let mut a = vec!['a' as u8, 'b' as u8, 'c' as u8];
        let b = vec!['c' as u8, 'b' as u8, 'a' as u8];
        swap_letters(&mut a, 'a' as u8, 'c' as u8);
        assert_eq!(a, b);
    }

    #[test]
    fn test_rotate_left() {
        let mut a = vec!['a' as u8, 'b' as u8, 'a' as u8];
        let b = vec!['b' as u8, 'a' as u8, 'a' as u8];
        rotate_left(&mut a, &vec![1]);
        assert_eq!(a, b);

        let mut a = vec!['a' as u8, 'b' as u8, 'a' as u8];
        let b = vec!['a' as u8, 'a' as u8, 'b' as u8];
        rotate_left(&mut a, &vec![2]);
        assert_eq!(a, b);

        let mut a = vec!['a' as u8, 'b' as u8, 'a' as u8];
        let b = vec!['a' as u8, 'b' as u8, 'a' as u8];
        rotate_left(&mut a, &vec![3]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_rotate_right() {
        let mut a = vec!['a' as u8, 'b' as u8, 'a' as u8];
        let b = vec!['a' as u8, 'a' as u8, 'b' as u8];
        rotate_right(&mut a, &vec![1]);
        assert_eq!(a, b);

        let mut a = vec!['a' as u8, 'b' as u8, 'a' as u8];
        let b = vec!['b' as u8, 'a' as u8, 'a' as u8];
        rotate_right(&mut a, &vec![2]);
        assert_eq!(a, b);

        let mut a = vec!['a' as u8, 'b' as u8, 'a' as u8];
        let b = vec!['a' as u8, 'b' as u8, 'a' as u8];
        rotate_right(&mut a, &vec![3]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_reverse_positions() {
        let mut a = vec!['a' as u8, 'b' as u8, 'c' as u8];
        let b = vec!['c' as u8, 'b' as u8, 'a' as u8];
        reverse_positions(&mut a, &vec![0, 2]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_part1() {
        let ops = parse_operations("resources/day21-test-input.txt");
        assert_eq!("decab", solve1(&ops, "abcde"));
    }
}
