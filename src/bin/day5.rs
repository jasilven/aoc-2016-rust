extern crate md5;

fn solve1(s: &str) -> String {
    (0..)
        .map(|i| md5::compute(format!("{}{}", s, i)))
        .map(|s| format!("{:x}", &s))
        .filter(|s| s.starts_with("00000"))
        .take(8)
        .map(|s| s.chars().nth(5).expect("parse error"))
        .collect::<String>()
}

fn solve2_functional(s: &str) -> String {
    const SIZE: usize = 8;
    let mut result: [char; SIZE] = ['#'; SIZE];
    let _ = (0..)
        .map(|i| md5::compute(format!("{}{}", s, i)))
        .map(|s| format!("{:x}", &s))
        .filter(|s| s.starts_with("00000"))
        .map(|s| {
            (
                (s.chars().nth(5).unwrap() as usize) - 48,
                s.chars().nth(6).unwrap(),
            )
        })
        .filter(|(i, c)| {
            if *i < SIZE && result[*i] == '#' {
                result[*i] = *c;
                true
            } else {
                false
            }
        })
        .take(SIZE)
        .fold(0, |acc, _| acc + 1); // force evaluation
    result.iter().collect::<String>()
}

fn solve2_imperative(s: &str) -> String {
    const SIZE: usize = 8;
    let mut result: [char; SIZE] = ['#'; SIZE];
    let mut cnt = 0;
    for n in 0.. {
        if cnt == SIZE {
            break;
        }
        let hash = format!("{:x}", md5::compute(format!("{}{}", s, n)));
        if hash.starts_with("00000") {
            let i = (hash.chars().nth(5).unwrap() as usize) - 48;
            let c = hash.chars().nth(6).unwrap();
            if i < SIZE && result[i] == '#' {
                result[i] = c;
                cnt += 1;
            }
        }
    }
    result.iter().collect::<String>()
}

fn main() {
    println!("Part 1: {}", solve1("cxdnnyjw"));
    // correct answer: f77a0e6e
    println!("Part 2 functional: {}", solve2_functional("cxdnnyjw"));
    println!("Part 2 imperative: {}", solve2_imperative("cxdnnyjw"));
    // correct answer: 999828ec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        assert_eq!(
            true,
            format!("{:x}", md5::compute("abc3231929")).starts_with("00000")
        );
    }

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!("18f47a30", solve1("abc"));
    }

    #[test]
    #[ignore]
    fn test_part2_functional() {
        assert_eq!("05ace8e3", solve2_functional("abc"));
    }

    #[test]
    #[ignore]
    fn test_part2_imperative() {
        assert_eq!("05ace8e3", solve2_imperative("abc"));
    }
}
