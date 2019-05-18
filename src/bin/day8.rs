mod aoc;

use aoc::parse_ints;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn build_matrix(rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    for _ in 0..rows {
        result.push(vec!['.'; cols]);
    }
    result
}

fn rotate_row(matrix: &mut Vec<Vec<char>>, row: usize, n: usize) {
    let n = n % matrix[row].len();
    matrix[row].rotate_right(n);
}

fn rotate_col(matrix: &mut Vec<Vec<char>>, col: usize, n: usize) {
    let mut column: Vec<char> = vec![];
    for row in matrix.iter() {
        column.push(row[col]);
    }

    let n = n % column.len();
    column.rotate_right(n);
    for i in 0..column.len() {
        matrix[i][col] = column[i];
    }
}

fn rect(matrix: &mut Vec<Vec<char>>, x: usize, y: usize) {
    for row in 0..y {
        for col in 0..x {
            matrix[row][col] = '#';
        }
    }
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            print!("{}", matrix[row][col]);
        }
        println!();
    }
}

fn execute(fname: &str, rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut matrix = build_matrix(rows, cols);
    for line in BufReader::new(File::open(fname).expect("file open error")).lines() {
        let line: String = line.expect("line parse error");
        let nums = parse_ints(&line);
        match true {
            true if line.starts_with("rotate column") => rotate_col(&mut matrix, nums[0], nums[1]),
            true if line.starts_with("rotate row") => rotate_row(&mut matrix, nums[0], nums[1]),
            true if line.starts_with("rect") => rect(&mut matrix, nums[0], nums[1]),
            _ => panic!("input data error"),
        }
    }
    matrix
}

fn solve1(matrix: &Vec<Vec<char>>) -> usize {
    matrix.iter().flatten().filter(|c| *c == &'#').count()
}

fn solve2(matrix: &Vec<Vec<char>>) {
    print_matrix(matrix);
}

fn main() {
    let fname = "resources/day8-input.txt";
    let mut matrix = execute(fname, 6, 50);
    println!("Part 1: {}", solve1(&mut matrix));
    println!("Part 2:");
    solve2(&matrix);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_build_matrix() {
        assert_eq!('.', build_matrix(3, 3)[1][1]);
    }

    #[test]
    fn test_rotate_row() {
        let mut vec1 = vec![vec!['a', 'b', 'c']];
        rotate_row(&mut vec1, 0, 1);
        let vec2 = vec![vec!['c', 'a', 'b']];
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_rotate_col() {
        let mut vec1 = vec![vec!['a', 'b', 'c'], vec!['1', '2', '3']];
        rotate_col(&mut vec1, 1, 1);
        assert_eq!('2', vec1[0][1]);
    }
}
