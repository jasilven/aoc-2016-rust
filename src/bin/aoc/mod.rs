use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn manh_dist(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

pub fn turn(facing: &i32, ch: char) -> Result<i32, String> {
    let result = match ch {
        'L' => (*facing + 3) % 4,
        'R' => (*facing + 1) % 4,
        _ => return Err(format!("unknown turn '{}'", ch)),
    };
    Ok(result)
}

pub fn move_point(p: &Point, ch: char, steps: i32) -> Result<Point, String> {
    let mut point = p.clone();
    match ch {
        'U' | 'N' => point.y -= steps,
        'R' | 'E' => point.x += steps,
        'D' | 'S' => point.y += steps,
        'L' | 'W' => point.x -= steps,
        _ => return Err(format!("unknown direction: {}", ch)),
    };
    Ok(point)
}

pub fn parse_map(fname: &str, discard: &[char]) -> Result<HashMap<Point, char>, Box<Error>> {
    let mut keypad = HashMap::new();
    let mut x = 0i32;
    let mut y = 0i32;

    for line in BufReader::new(File::open(fname)?).lines() {
        for ch in line.unwrap().chars() {
            if !discard.contains(&ch) {
                keypad.insert(Point { x: x, y: y }, ch);
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    Ok(keypad)
}

#[derive(Debug)]
pub struct Matrix {
    data: Vec<Vec<String>>,
}

impl Matrix {
    pub fn parse_matrix(fname: &str) -> Result<Matrix, Box<Error>> {
        let mut mat: Vec<Vec<String>> = vec![];
        let file = File::open(fname)?;
        for line in BufReader::new(file).lines() {
            let tokens: Vec<String> = line?.split_whitespace().map(|x| x.to_string()).collect();
            mat.push(tokens);
        }
        Ok(Matrix { data: mat })
    }

    pub fn new() -> Matrix {
        let rows: Vec<String> = vec![];
        Matrix { data: vec![rows] }
    }

    pub fn column(&self, col: usize) -> Option<Vec<String>> {
        let mut result: Vec<String> = vec![];
        for row in self.data.iter() {
            result.push(row[col].clone());
        }
        match result.is_empty() {
            true => None,
            _ => Some(result),
        }
    }

    pub fn rows(&self) -> Vec<Vec<String>> {
        self.data.clone()
    }

    pub fn cols(&self) -> Option<Vec<Vec<String>>> {
        let mut result: Vec<Vec<String>> = vec![];
        for c in 0..self.data[0].len() {
            result.push(self.column(c)?);
        }
        Some(result)
    }

    pub fn row(&self, row: usize) -> Option<Vec<String>> {
        Some(self.data.get(row)?.to_vec())
    }

    pub fn cell(&self, (row, col): (i32, i32)) -> String {
        self.data[row as usize][col as usize].clone()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
    pub fn manh_dist(&self) -> i32 {
        manh_dist(self, &Self::origin())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_lr() {
        assert_eq!(1, turn(&0, 'R').unwrap());
        assert_eq!(3, turn(&0, 'L').unwrap());
        assert_eq!(2, turn(&1, 'R').unwrap());
        assert_eq!(0, turn(&1, 'L').unwrap());
        assert_eq!(3, turn(&2, 'R').unwrap());
        assert_eq!(1, turn(&2, 'L').unwrap());
        assert_eq!(0, turn(&3, 'R').unwrap());
        assert_eq!(2, turn(&3, 'L').unwrap());
    }

    #[test]
    fn test_manh_dist() {
        assert_eq!(2, manh_dist(&Point::origin(), &Point { x: 1, y: 1 }));
        assert_eq!(4, manh_dist(&Point { x: -1, y: -1 }, &Point { x: 1, y: 1 }));
        assert_eq!(3, (Point { x: 3, y: 0 }).manh_dist());
    }

    #[test]
    fn test_move_point() {
        assert_eq!(
            Point { x: 0, y: -1 },
            move_point(&Point { x: 0, y: 0 }, 'N', 1).unwrap()
        );
        assert_eq!(
            Point { x: 0, y: -2 },
            move_point(&Point { x: 0, y: 0 }, 'U', 2).unwrap()
        );
        assert_eq!(
            Point { x: 1, y: 0 },
            move_point(&Point { x: 0, y: 0 }, 'R', 1).unwrap()
        );
        assert_eq!(
            Point { x: 2, y: 0 },
            move_point(&Point { x: 0, y: 0 }, 'E', 2).unwrap()
        );
        assert_eq!(
            Point { x: 0, y: 1 },
            move_point(&Point { x: 0, y: 0 }, 'D', 1).unwrap()
        );
        assert_eq!(
            Point { x: 0, y: 2 },
            move_point(&Point { x: 0, y: 0 }, 'S', 2).unwrap()
        );
        assert_eq!(
            Point { x: -1, y: 0 },
            move_point(&Point { x: 0, y: 0 }, 'L', 1).unwrap()
        );
        assert_eq!(
            Point { x: -2, y: 0 },
            move_point(&Point { x: 0, y: 0 }, 'W', 2).unwrap()
        );
        assert_eq!(
            Point { x: 0, y: 0 },
            move_point(&Point { x: 0, y: 0 }, 'L', 0).unwrap()
        );
    }

    #[test]
    fn test_parse_map() {
        let hm = parse_map("resources/day2-keypad1.txt", &[]).unwrap();
        assert_eq!(hm.get(&Point { x: 2, y: 2 }).unwrap(), &'9');
        let hm2 = parse_map("resources/day2-keypad2.txt", &[]).unwrap();
        assert_eq!(hm2.get(&Point { x: 2, y: 4 }).unwrap(), &'D');
    }
}
