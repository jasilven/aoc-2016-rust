mod aoc;
use aoc::parse_ints;
use permutohedron::Heap;
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

fn swap_pos(word: &mut String, args: &[isize]) {
    let x = args[0] as usize;
    let y = args[1] as usize;
    let x_str = &word[x..=x].to_owned();
    let y_str = &word[y..=y].to_owned();
    word.replace_range(x..=x, y_str);
    word.replace_range(y..=y, x_str);
}

fn swap_letters(word: &mut String, x: &str, y: &str) {
    let mut tmp = word.replace(x, "#");
    tmp = tmp.replace(y, x);
    tmp = tmp.replace("#", y);
    word.clear();
    word.push_str(&tmp);
}

fn rotate_left(word: &mut String, args: &[isize]) {
    let cnt = (args[0] as usize) % word.len();
    let (first, last) = word.split_at(cnt);
    let mut tmp = "".to_owned();
    tmp.push_str(&last);
    tmp.push_str(&first);
    word.clear();
    word.push_str(&tmp);
}

fn rotate_right(word: &mut String, args: &[isize]) {
    let cnt = (word.len() - (args[0] as usize) % word.len()) as usize;
    let (first, last) = word.split_at(cnt);
    let mut tmp = "".to_owned();
    tmp.push_str(&last);
    tmp.push_str(&first);
    word.clear();
    word.push_str(&tmp);
}

fn rotate_based_pos(word: &mut String, arg: char) {
    let index = word.find(arg).unwrap();
    let mut cnt = index + 1;
    if index >= 4 {
        cnt += 1;
    }
    rotate_right(word, &[cnt as isize]);
}

fn reverse_positions(word: &mut String, args: &[isize]) {
    let x = args[0] as usize;
    let y = args[1] as usize;
    let tmp: String = word[x..=y].to_owned().chars().rev().collect();
    word.replace_range(x..=y, &tmp);
}

fn move_positions(word: &mut String, args: &[isize]) {
    let x = args[0] as usize;
    let y = args[1] as usize;
    let ch = word.remove(x);
    word.insert(y, ch);
}

fn solve1(ops: &[String], word: &str) -> String {
    let mut w = word.to_owned();
    for op in ops {
        match true {
            true if op.starts_with("swap position") => swap_pos(&mut w, &parse_ints(&op)),
            true if op.starts_with("swap letter") => swap_letters(&mut w, &op[12..13], &op[26..27]),
            true if op.starts_with("rotate left") => rotate_left(&mut w, &parse_ints(&op)),
            true if op.starts_with("rotate right") => rotate_right(&mut w, &parse_ints(&op)),
            true if op.starts_with("rotate based") => {
                rotate_based_pos(&mut w, op.chars().last().unwrap())
            }
            true if op.starts_with("reverse positions") => {
                reverse_positions(&mut w, &parse_ints(&op))
            }
            true if op.starts_with("move position") => move_positions(&mut w, &parse_ints(&op)),
            _ => panic!("unknown operation"),
        };
    }
    w
}

fn solve2(ops: &[String], word: &str, target: &str) -> String {
    let target = target.to_owned();
    let mut data: Vec<char> = vec![];
    for ch in word.chars() {
        data.push(ch);
    }
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

fn main() {
    let ops = parse_operations("resources/day21-input.txt");
    println!("Part 1: {}", solve1(&ops, "abcdefgh"));
    // correct answer: dbfgaehc
    println!("Part 2: {}", solve2(&ops, "abcdefgh", "fbgdceah"));
    // correct answer: aghfcdeb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_pos() {
        let mut word = String::from("abc");
        let args = vec![0, 2];
        swap_pos(&mut word, &args);
        assert_eq!("cba", word);
    }

    #[test]
    fn test_swap_letters() {
        let mut word = String::from("abc");
        swap_letters(&mut word, "a", "c");
        assert_eq!("cba", word);
    }

    #[test]
    fn test_rotate_left() {
        let mut word = String::from("aba");
        rotate_left(&mut word, &vec![1]);
        assert_eq!("baa", word);

        word = String::from("aba");
        rotate_left(&mut word, &vec![2]);
        assert_eq!("aab", word);

        word = String::from("aba");
        rotate_left(&mut word, &vec![3]);
        assert_eq!("aba", word);
    }

    #[test]
    fn test_rotate_right() {
        let mut word = String::from("aba");
        rotate_right(&mut word, &vec![1]);
        assert_eq!("aab", word);

        word = String::from("aba");
        rotate_right(&mut word, &vec![2]);
        assert_eq!("baa", word);

        word = String::from("aba");
        rotate_right(&mut word, &vec![3]);
        assert_eq!("aba", word);
    }

    #[test]
    fn test_reverse_positions() {
        let mut word = String::from("12345");
        reverse_positions(&mut word, &vec![0, 2]);
        assert_eq!("32145", word);
    }

    #[test]
    fn test_part1() {
        let ops = parse_operations("resources/day21-test-input.txt");
        assert_eq!("decab", solve1(&ops, "abcde"));
    }
}
