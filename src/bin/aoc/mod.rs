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
}
