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

fn solve1(slice: &str) -> usize {
    let mut cnt = 0;
    for n in 0..slice.len() {
        if &slice[n..n + 1] == "(" {
            let (marker_len, ch_cnt, repeat_cnt) = parse_marker(&slice[n..slice.len()]);
            let low = n + marker_len;
            let high = low + ch_cnt;
            return cnt + solve1(&slice[high..slice.len()]) + repeat_cnt * ch_cnt;
        }
        cnt += 1;
    }
    return cnt;
}

fn solve2(slice: &str) -> usize {
    let mut cnt = 0;
    for n in 0..slice.len() {
        if &slice[n..n + 1] == "(" {
            let (marker_len, ch_cnt, repeat_cnt) = parse_marker(&slice[n..slice.len()]);
            let low = n + marker_len;
            let high = low + ch_cnt;
            return cnt
                + solve2(&slice[high..slice.len()])
                + repeat_cnt * solve2(&slice[low..high]);
        }
        cnt += 1;
    }
    return cnt;
}

fn main() {
    let fname = "resources/day9-input.txt";
    println!("Part 1: {}", &solve1(std::fs::read_to_string(fnane)));
    // println!("{}",&solve1(std::fs::read_to_string(fname)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(6, solve1(&String::from("ADVENT")));
        assert_eq!(7, solve1(&String::from("A(1x5)BC")));
        assert_eq!(9, solve1(&String::from("(3x3)XYZ")));
        assert_eq!(11, solve1(&String::from("A(2x2)BCD(2x2)EFG")));
        assert_eq!(6, solve1(&String::from("(6x1)(1x3)A")));
        assert_eq!(18, solve1(&String::from("X(8x2)(3x3)ABCY")));
    }
}
