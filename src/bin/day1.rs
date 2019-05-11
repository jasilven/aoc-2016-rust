mod aoc;

fn parse_input(fname: &str) -> Vec<(char, isize)> {
    let mut result: Vec<(char, isize)> = Vec::new();
    let data = aoc::slurp(fname).expect("error reading file");

    for i in data.trim().split(", ") {
        result.push((i.chars().nth(0).unwrap(), i[1..].parse::<isize>().unwrap()))
    }
    result
}

fn solve1(fname: &str) -> isize {
    let mut x_y = (0, 0);
    let mut dir = 0; // N E S W = 0 1 2 3

    for dir_steps in parse_input(fname) {
        dir = match dir_steps.0 {
            'L' => (dir + 3) % 4,
            'R' => (dir + 1) % 4,
            _ => panic!("unknown direction"),
        };
        match dir {
            0 => x_y.1 -= dir_steps.1,
            1 => x_y.0 += dir_steps.1,
            2 => x_y.1 += dir_steps.1,
            3 => x_y.0 -= dir_steps.1,
            _ => panic!("unknown direction"),
        };
    }
    aoc::manh_dist((0, 0), x_y)
}

fn main() {
    let fname = "resources/day1-input.txt";
    println!("Part 1: {}", solve1(fname)); // correct answer: 231
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
    fn part1_test() {
        assert_eq!(12, solve1("resources/day1-test-input.txt"));
        assert_eq!(2, solve1("resources/day1-test-input2.txt"));
        assert_eq!(5, solve1("resources/day1-test-input3.txt"));
        assert_eq!(0, solve1("resources/day1-test-input4.txt"));
    }
}
