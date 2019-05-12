use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn slurp(fname: &str) -> Result<String, std::io::Error> {
    let mut data = String::new();
    let mut file = File::open(fname)?;
    file.read_to_string(&mut data)?;
    Ok(data)
}

pub fn manh_dist(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

pub fn turn(facing: &i32, ch: char) -> i32 {
    match ch {
        'L' => (*facing + 3) % 4,
        'R' => (*facing + 1) % 4,
        _ => panic!("unknown turn"),
    }
}

pub fn move_point(p: &Point, ch: char, steps: i32) -> Point {
    let mut point = p.clone();
    match ch {
        'U' | 'N' => point.y -= steps,
        'R' | 'E' => point.x += steps,
        'D' | 'S' => point.y += steps,
        'L' | 'W' => point.x -= steps,
        _ => panic!("unknown direction: {}", ch),
    };
    point
}

pub fn parse_map(fname: &str, discard: &[char]) -> HashMap<Point, char> {
    let mut keypad = HashMap::new();
    let mut x = 0i32;
    let mut y = 0i32;
    for line in slurp(fname).expect("cannot read keypad file").lines() {
        for ch in line.chars() {
            if !discard.contains(&ch) {
                keypad.insert(Point { x: x, y: y }, ch);
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    keypad
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
    fn slurp_test() {
        let s = slurp("src/bin/aoc/mod.rs");
        assert!(s.is_ok());
        assert!(!s.unwrap().is_empty());
    }

    #[test]
    fn turn_test_lr() {
        assert_eq!(1, turn(&0, 'R'));
        assert_eq!(3, turn(&0, 'L'));
        assert_eq!(2, turn(&1, 'R'));
        assert_eq!(0, turn(&1, 'L'));
        assert_eq!(3, turn(&2, 'R'));
        assert_eq!(1, turn(&2, 'L'));
        assert_eq!(0, turn(&3, 'R'));
        assert_eq!(2, turn(&3, 'L'));
    }

    #[test]
    fn manh_dist_test() {
        assert_eq!(2, manh_dist(&Point::origin(), &Point { x: 1, y: 1 }));
        assert_eq!(4, manh_dist(&Point { x: -1, y: -1 }, &Point { x: 1, y: 1 }));
        assert_eq!(3, (Point { x: 3, y: 0 }).manh_dist());
    }

    #[test]
    fn move_point_test() {
        assert_eq!(
            Point { x: 0, y: -1 },
            move_point(&Point { x: 0, y: 0 }, 'N', 1)
        );
        assert_eq!(
            Point { x: 0, y: -2 },
            move_point(&Point { x: 0, y: 0 }, 'U', 2)
        );
        assert_eq!(
            Point { x: 1, y: 0 },
            move_point(&Point { x: 0, y: 0 }, 'R', 1)
        );
        assert_eq!(
            Point { x: 2, y: 0 },
            move_point(&Point { x: 0, y: 0 }, 'E', 2)
        );
        assert_eq!(
            Point { x: 0, y: 1 },
            move_point(&Point { x: 0, y: 0 }, 'D', 1)
        );
        assert_eq!(
            Point { x: 0, y: 2 },
            move_point(&Point { x: 0, y: 0 }, 'S', 2)
        );
        assert_eq!(
            Point { x: -1, y: 0 },
            move_point(&Point { x: 0, y: 0 }, 'L', 1)
        );
        assert_eq!(
            Point { x: -2, y: 0 },
            move_point(&Point { x: 0, y: 0 }, 'W', 2)
        );
        assert_eq!(
            Point { x: 0, y: 0 },
            move_point(&Point { x: 0, y: 0 }, 'L', 0)
        );
    }

    #[test]
    fn parse_map_test() {
        let hm = parse_map("resources/day2-keypad1.txt", &[]);
        assert_eq!(hm.get(&Point { x: 2, y: 2 }).unwrap(), &'9');
        let hm2 = parse_map("resources/day2-keypad2.txt", &[]);
        assert_eq!(hm2.get(&Point { x: 2, y: 4 }).unwrap(), &'D');
    }
}
