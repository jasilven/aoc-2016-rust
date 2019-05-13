mod aoc;
use aoc::*;

fn valid_triangle((a, b, c): (i32, i32, i32)) -> bool {
    (a + b > c) & (a + c > b) & (b + c > a)
}

fn solve(vec: Vec<Vec<String>>) -> i32 {
    let mut result = 0;
    for cs in vec.iter() {
        let nums: Vec<i32> = cs.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        for chu in nums.chunks(3) {
            result += valid_triangle((chu[0], chu[1], chu[2])) as i32;
        }
    }
    result
}

fn main() {
    let mat = Matrix::parse_matrix("resources/day3-input.txt").expect("parse error");
    println!("Part 1: {}", solve(mat.rows()));
    println!("Part 2: {}", solve(mat.cols().unwrap()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_triangle_test() {
        assert_eq!(false, valid_triangle((5, 10, 25)));
        assert_eq!(true, valid_triangle((5, 5, 5)));
    }
}
