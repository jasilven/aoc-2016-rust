mod aoc;
use aoc::*;
use std::collections::HashSet;

fn parse_input(fname: &str) -> Vec<(char, i32)> {
    let mut result: Vec<(char, i32)> = Vec::new();
    let data = slurp(fname).expect("error reading file");

    for item in data.trim().split(", ") {
        let turn = item.chars().nth(0).unwrap();
        result.push((turn, item[1..].parse::<i32>().unwrap()))
    }
    result
}

fn get_path(fname: &str) -> Vec<Point> {
    let mut path = Vec::new();
    let mut facing = 0;
    let mut point = Point::origin();
    for dir_steps in parse_input(fname) {
        facing = turn(&facing, dir_steps.0).expect("unable to turn");
        for _ in 0..dir_steps.1 {
            let mut p = point.clone();
            match facing {
                0 => p.y -= 1,
                1 => p.x += 1,
                2 => p.y += 1,
                3 => p.x -= 1,
                _ => panic!("unknown direction: {}", facing),
            };
            point = p.clone();
            path.push(p);
        }
    }
    path
}

fn solve1(fname: &str) -> i32 {
    let path = get_path(fname);
    path.last().unwrap().manh_dist()
}

fn solve2(fname: &str) -> i32 {
    let mut seen = HashSet::new();
    for point in get_path(fname) {
        if seen.contains(&point) {
            return point.manh_dist();
        } else {
            seen.insert(point);
        }
    }
    panic!("Solution not found");
}

fn main() {
    let fname = "resources/day1-input.txt";
    println!("Part 1: {}", solve1(fname)); // correct answer: 231
    println!("Part 2: {}", solve2(fname)); // correct answer: 147
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let v = vec![('R', 5), ('L', 5), ('R', 5), ('R', 3)];
        assert_eq!(v, parse_input("resources/day1-test-input.txt"));
    }

    #[test]
    fn get_path_test() {
        let tv = get_path("resources/day1-test-input.txt"); //[R5, L5, R5, R3]
        assert_eq!(tv.last().unwrap(), &Point { x: 10, y: -2 });
    }

    #[test]
    fn part1_test() {
        assert_eq!(12, solve1("resources/day1-test-input.txt"));
        assert_eq!(2, solve1("resources/day1-test-input2.txt"));
        assert_eq!(5, solve1("resources/day1-test-input3.txt"));
        assert_eq!(0, solve1("resources/day1-test-input4.txt"));
    }
    #[test]
    fn part2_test() {
        assert_eq!(4, solve2("resources/day1-test-input-part2.txt"));
    }
}
