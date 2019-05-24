extern crate regex;
use regex::Regex;

fn parse_marker(slice: &str) -> (usize, usize, usize) {
    let re: Regex = Regex::new(r"^\((\d+)x(\d+)\)").expect("invalid regex");
    let cap = re.captures(&slice).unwrap();
    (
        cap[0].len(),
        cap[1].parse::<usize>().unwrap(),
        cap[2].parse::<usize>().unwrap(),
    )
}

fn solve(slice: &str, recursive: bool) -> usize {
    let mut cnt = 0;
    for n in 0..slice.len() {
        if &slice[n..n + 1] == "(" {
            let (marker_len, ch_cnt, repeat_cnt) = parse_marker(&slice[n..slice.len()]);
            let low = n + marker_len;
            let high = low + ch_cnt;
            if recursive {
                return cnt
                    + solve(&slice[high..slice.len()], true)
                    + repeat_cnt * solve(&slice[low..high], true);
            }
            return cnt + solve(&slice[high..slice.len()], false) + repeat_cnt * ch_cnt;
        } else {
            cnt += 1;
        }
    }
    return cnt;
}

fn main() {
    let fname = "resources/day9-input.txt";
    let data = std::fs::read_to_string(fname)
        .unwrap()
        .trim_end()
        .to_owned();
    println!("Part 1: {}", solve(&data, false));
    // correct answer: 120765
    println!("Part 2: {}", solve(&data, true));
    // correct answer: 11658395076
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(6, solve(&String::from("ADVENT"), false));
        assert_eq!(7, solve(&String::from("A(1x5)BC"), false));
        assert_eq!(9, solve(&String::from("(3x3)XYZ"), false));
        assert_eq!(11, solve(&String::from("A(2x2)BCD(2x2)EFG"), false));
        assert_eq!(6, solve(&String::from("(6x1)(1x3)A"), false));
        assert_eq!(18, solve(&String::from("X(8x2)(3x3)ABCY"), false));
    }
}
